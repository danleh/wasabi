// Author: Michael Pradel

/*
 * Simple taint analysis that considers explicit flows only
 * (i.e., no flows caused by control flow dependencies, but only data flow).
 * TODO work-in-progress ...
 */

(function() {

    console.log("=========================");
    console.log("Starting taint analysis");

    Array.prototype.peek = function() {
        return this[this.length - 1];
    };

    /*
     * Mirror program state; track taints instead of actual value
     * TODO: Move memory tracking into reusable analysis component
     */
    const stack = [{
        blocks:[], // the value stack for each function contains substacks for each block
        locals:[]
    }];

    function values() {
        return stack.peek().blocks.peek();
    }

    const memory = [];
    const globals = [];

    /*
     * Taint policy: sources and sink
     */
    function findSourceSinkFcts() {
        let sourceFctIdx = -1;
        let sinkFctIdx = -1;
        for (let i = 0; i < Wasabi.module.info.functions.length; i++) {
            const fct = Wasabi.module.info.functions[i];
            if (fct.export === "taint_source") {
                sourceFctIdx = i;
            }
            if (fct.export === "taint_sink") {
                sinkFctIdx = i;
            }
        }
        if (sourceFctIdx === -1) console.log("Warning: No exported source function found.");
        if (sinkFctIdx === -1) console.log("Warning: No exported sink function found.");
        return {sourceFctIdx:sourceFctIdx, sinkFctIdx:sinkFctIdx};
    }
    const {sourceFctIdx, sinkFctIdx} = findSourceSinkFcts();

    function Taint() {
        this.label = 0; // can hold any kind of more complex label; for now, just 0 (not tainted) and 1 (tainted)
    }

    Taint.prototype.toString = function() {
        return "taint-" + this.label;
    };

    function join(taint1, taint2) {
        const resultTaint = new Taint();
        if (taint1.label == 1 || taint2.label == 1)
            resultTaint.label = 1;
        return resultTaint;
    }

    function ensureTaint(value) {
        if (value instanceof Taint) {
            return value;
        } else {
            console.log("ensureTaint: creating taint for value");
            return new Taint();
        }
    }

    Wasabi.analysis = {

        start(location) {
            // create taints for all globals
            for (let i = 0; i < Wasabi.module.info.globals.length; i++) {
                const global = Wasabi.module.info.globals[i];
                console.log("Creating taint for globals[" + i + "]");
                globals[i] = new Taint();
            }
        },

        if_(location, condition) {
            values().pop();
        },

        br(location, target) {
            stack.peek().blocks.pop();
        },

        br_if(location, conditionalTarget, condition) {
            values().pop();
            if (condition) {
                stack.peek().blocks.pop();
            }
        },

        br_table(location, table, defaultTarget, tableIdx) {
            values().pop();
            stack.peek().blocks.pop();
        },

        begin(location, type) {
            // TODO if type is "function", set locals to parameter values of function?
            stack.peek().blocks.push([]);
        },

        end(location, type, beginLocation) {
            const [resultTaint] = stack.peek().blocks.pop();
            // assumes that resultTaint is defined iff the end is due to a function return
            if (resultTaint !== undefined && stack.length > 1) {
                // push return value onto caller's frame
                stack[stack.length-2].blocks.peek().push(ensureTaint(resultTaint));
            }
        },

        drop(location, value) {
            values().pop()
        },

        select(location, cond, first, second) {
            values().pop();
            values().pop();
            values().pop();
        },

        call_pre(location, targetFunc, args, indirectTableIdx) {
            if (indirectTableIdx !== undefined) {
                values().pop();
            }
            for (const arg of args) {
                const taint = ensureTaint(values().pop());
                if (targetFunc == sourceFctIdx) {
                    taint.label = 1;
                    console.log("Source: Marking value as tainted -- " + taint);
                }
                if (targetFunc == sinkFctIdx && taint.label == 1) {
                    console.log("Tainted value reached sink at ", location);
                }
            }
            stack.push({
                blocks:[],
                locals:[],
            });
        },

        call_post(location, vals) {
            stack.pop();
        },

        return_(location, values) {
            // TODO how does it influence the stack? Is this already handled by end_function?
        },

        const_(location, value) {
            console.log("New taint at ", location);
            values().push(new Taint());
        },

        unary(location, op, input, result) {
            const taint = ensureTaint(values().pop());
            const taintResult = new Taint();
            taintResult.label = taint.label;
            values().push(taintResult);
        },

        binary(location, op, first, second, result) {
            const taint1 = ensureTaint(values().pop());
            const taint2 = ensureTaint(values().pop());
            const taintResult = join(taint1, taint2);
            values().push(taintResult);
        },

        load(location, op, memarg, value) {
            values().pop();
            const effectiveAddr = memarg.addr + memarg.offset;
            const taint = ensureTaint(memory[effectiveAddr]);
            console.log("Memory load from address " + effectiveAddr + " with taint " + taint);
            values().push(taint);
        },

        store(location, op, memarg, value) {
            const taint = ensureTaint(values().pop());
            values().pop();
            const effectiveAddr = memarg.addr + memarg.offset;
            console.log("Memory store to address " + effectiveAddr + " with taint " + taint);
            memory[effectiveAddr] = taint;
        },

        memory_size(location, currentSizePages) {
            values().push(currentSizePages);
        },

        memory_grow(location, byPages, previousSizePages) {
            values().pop();
            values().push(previousSizePages);
        },

        local(location, op, localIndex, value) {
            switch (op) {
                case "set_local": {
                    const taint = ensureTaint(values().pop());
                    stack.peek().locals[localIndex] = taint;
                    console.log("Setting local variable with taint " + taint);
                    return;
                }
                case "tee_local": {
                    const taint = ensureTaint(values().peek());
                    stack.peek().locals[localIndex] = taint;
                    return;
                }
                case "get_local": {
                    const taint = ensureTaint(stack.peek().locals[localIndex]);
                    values().push(taint);
                    console.log("Getting local variable with taint " + taint);
                    return;
                }
            }
        },

        global(location, op, globalIndex, value) {
            let taint;
            switch (op) {
                case "set_global":
                    taint = ensureTaint(values().pop());
                    globals[globalIndex] = taint;
                    return;
                case "get_global":
                    taint = ensureTaint(globals[globalIndex]);
                    values().push(taint);
                    return;
            }
        },
    };

})();
