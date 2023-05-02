// Author: Michael Pradel

/*
 * Simple taint analysis that considers explicit flows only
 * (i.e., no flows caused by control flow dependencies, but only data flow).
 */

(function() {

    const debug = false;

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
        // FIXME || [] is a hack to get it to run without exceptions
        return stack.peek().blocks.peek() || [];
    }

    const memory = [];
    const globals = [];

    let returnValue; // to propagate return value's taint from end() to call_post()

    /*
     * Taint policy: sources and sink
     */
    function findSourceSinkFcts() {
        let sourceFctIdx = -1;
        let sinkFctIdx = -1;
        for (let i = 0; i < Wasabi.module.info.functions.length; i++) {
            const fct = Wasabi.module.info.functions[i];
            if (fct.export === "taint_source" || fct.export === "_markSource") {
                sourceFctIdx = i;
            }
            if (fct.export === "taint_sink" || fct.export === "_sink") {
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

    function ensureTaint(value, location) {
        if (value instanceof Taint) {
            return value;
        } else {
            if (debug) console.log("ensureTaint: creating taint for value at ", location);
            return new Taint();
        }
    }

    Wasabi.analysis = {

        start(location) {
            // create taints for all globals
            for (let i = 0; i < Wasabi.module.info.globals.length; i++) {
                const global = Wasabi.module.info.globals[i];
                if (debug) console.log("Creating taint for globals[" + i + "]");
                globals[i] = new Taint();
            }

            // any other data for which need to initialize taints?
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

        drop(location, value) {
            values().pop()
        },

        select(location, cond, first, second) {
            values().pop();
            const taint2 = ensureTaint(values().pop(), location);
            const taint1 = ensureTaint(values().pop(), location);
            if (cond) values().push(taint1);
            else values().push(taint2);
        },

        begin(location, type) {
            stack.peek().blocks.push([]);
        },

        end(location, type, beginLocation) {
            const resultTaintArr = stack.peek().blocks.pop();
            // FIXME sometimes pop() returns undefined, not just an empty []. Why? 
            // hacky workaround: just return early if [resultTaint] pattern match would fail
            if (resultTaintArr === undefined) return;
            const [resultTaint] = resultTaintArr;
            if (type === "function" && resultTaint !== undefined) {
                returnValue = ensureTaint(resultTaint, location);
                if (debug) console.log("end(): Storing return value's taint ", returnValue, " at ", location);
            }
        },

        call_pre(location, targetFunc, args, indirectTableIdx) {
            if (indirectTableIdx !== undefined) {
                values().pop();
            }
            const argTaints = [];
            for (const arg of args) {
                const taint = ensureTaint(values().pop(), location);
                argTaints.push(taint);
                if (targetFunc == sourceFctIdx) {
                    taint.label = 1;
                    console.log("Source: Marking value as ", taint);
                }
                if (targetFunc == sinkFctIdx && taint.label == 1) {
                    console.log("Tainted value reached sink at ", location);
                }
            }
            stack.push({
                blocks:[],
                locals:argTaints,
            });
        },

        call_post(location, vals) {
            stack.pop();
            if (returnValue !== undefined) {
                if (debug) console.log("Found return value's taint in call_post at ", location);
                values().push(returnValue);
                returnValue = undefined;
            }
        },

        return_(location, values) {
            // Note on interaction between end() and return_():
            //  * end() may or may not be called on function returns
            //  * return_() may or may not be called on function returns
            //  * end() always happens before return_()
            //  * We try to retrieve the return value taint in end(),
            //    and if none found, we try to retrieve it in return_()
            if (returnValue === undefined && stack.peek().blocks.length !== 0) {
                const [resultTaint] = stack.peek().blocks.pop();
                returnValue = ensureTaint(resultTaint, location);
                if (debug) console.log("return_(): Storing return value's taint ", returnValue, " at ", location);
            }
        },

        const_(location, op, value) {
            if (debug) console.log("New taint at ", location);
            values().push(new Taint());
        },

        unary(location, op, input, result) {
            const taint = ensureTaint(values().pop(), location);
            const taintResult = new Taint();
            taintResult.label = taint.label;
            values().push(taintResult);
        },

        binary(location, op, first, second, result) {
            const taint1 = ensureTaint(values().pop(), location);
            const taint2 = ensureTaint(values().pop(), location);
            const taintResult = join(taint1, taint2);
            if (debug) console.log("Result of binary is ", taintResult, " at ", location);
            values().push(taintResult);
        },

        load(location, op, memarg, value) {
            values().pop();
            const effectiveAddr = memarg.addr + memarg.offset;
            const taint = ensureTaint(memory[effectiveAddr], location);
            if (debug) console.log("Memory load from address " + effectiveAddr + " with taint " + taint);
            values().push(taint);
        },

        store(location, op, memarg, value) {
            const taint = ensureTaint(values().pop(), location);
            values().pop();
            const effectiveAddr = memarg.addr + memarg.offset;
            if (debug) console.log("Memory store to address " + effectiveAddr + " with taint " + taint);
            memory[effectiveAddr] = taint;
        },

        memory_size(location, currentSizePages) {
            values().push(new Taint());
        },

        memory_grow(location, byPages, previousSizePages) {
            values().pop();
            values().push(new Taint());
        },

        local(location, op, localIndex, value) {
            switch (op) {
                case "local.set": {
                    const taint = ensureTaint(values().pop(), location);
                    stack.peek().locals[localIndex] = taint;
                    if (debug) console.log("Setting local variable with ", taint, " at ", location);
                    return;
                }
                case "local.tee": {
                    const taint = ensureTaint(values().peek(), location);
                    stack.peek().locals[localIndex] = taint;
                    return;
                }
                case "local.get": {
                    const taint = ensureTaint(stack.peek().locals[localIndex], location);
                    values().push(taint);
                    if (debug) console.log("Getting local variable with ", taint, " at ", location);
                    return;
                }
            }
        },

        global(location, op, globalIndex, value) {
            let taint;
            switch (op) {
                case "global.set":
                    taint = ensureTaint(values().pop(), location);
                    globals[globalIndex] = taint;
                    return;
                case "global.get":
                    taint = ensureTaint(globals[globalIndex], location);
                    values().push(taint);
                    return;
            }
        },
    };

})();
