/*
 * convenience functions
 */
Array.prototype.peek = function () {
    return this[this.length - 1];
};

function check(op, location, jsValue, wasmValue) {
    if (jsValue === undefined) {
        jsValue = "undefined";
    }
    if (wasmValue === undefined) {
        wasmValue = "undefined";
    }
    if (jsValue.toString() !== wasmValue.toString()) {
        console.warn(op, location, jsValue, wasmValue);
        if (!op.includes("load"))
            throw "mismatch";
    }
}

/*
 * WebAssembly program state, mirrored in JavaScript
 */

const stack = [[]];
const locals = [];

const memory = [];
const globals = [];

/*
 * "mirror execution" analysis, that performs all "writing operations" (like memory stores or global.set) also in
 * JavaScript and compares "reading operation" results between JavaScript and WebAssembly.
 */

Wasabi.analysis = {
    if_(location, condition) {
        const jsCondition = stack.peek().pop() === 1;
        check("if", location, jsCondition, condition);
    },

    br_if(location, conditionalTarget, condition) {
        const jsCondition = stack.peek().pop() === 1;
        check("br_if", location, jsCondition, condition);
    },

    br_table(location, table, defaultTarget, tableIdx) {
        const jsTableIdx = stack.peek().pop();
        check("br_table", location, jsTableIdx, tableIdx);
    },

    begin(location, type) {
        console.log("begin", location, type, stack);

        stack.push([]);
        if (type === "function")
            locals.push([]);
    },

    end(location, type, beginLocation) {
        console.log("end", location, beginLocation, type, stack);

        const [result] = stack.pop();
        if (result !== undefined) {
            console.log("end result", type, result)
            // stack.peek().push(result);
        }
        if (type === "function")
            locals.pop();
    },

    drop(location, value) {
        check("drop", location, stack.peek().pop(), value);
    },

    select(location, condition, first, second) {
        check("select", location, stack.peek().pop(), second);
        check("select", location, stack.peek().pop(), first);
        const jsCondition = stack.peek().pop() === 1;
        check("select", location, jsCondition, condition);
    },

    call_pre(location, targetFunc, args, indirectTableIdx) {
        if (indirectTableIdx !== undefined) {
            const jsTargetTableIdx = stack.peek().pop();
            check("call_indirect table idx", location, jsTargetTableIdx, indirectTableIdx);
        }
        for (const arg of args.reverse()) {
            const jsArg = stack.peek().pop();
            check("call args", location, jsArg, arg);
        }
        // TODO locals setup in callee or caller (here)?
        locals.push(args);
    },

    call_post(location, vals) {
        for (const val of vals) {
            stack.peek().push(val);
        }
        locals.pop();
    },

    return_(location, values) {
        console.log("return", location, values)
        // TODO check return values on stack?
    },

    const_(location, op, value) {
        stack.peek().push(value);
    },

    unary(location, op, input, result) {
        const jsInput = stack.peek().pop();
        check(op, location, jsInput, input);
        stack.peek().push(result);
    },

    binary(location, op, first, second, result) {
        const jsSecond = stack.peek().pop();
        check(op + " second arg", location, jsSecond, second);
        const jsFirst = stack.peek().pop();
        check(op + " first arg", location, jsFirst, first);
        stack.peek().push(result);
    },

    load(location, op, memarg, value) {
        const jsAddr = stack.peek().pop();
        check(op + " addr", location, jsAddr, memarg.addr);
        const effectiveAddr = memarg.addr + memarg.offset;
        const jsValue = memory[effectiveAddr];
        check(op + " value @ " + effectiveAddr + " (0x" + effectiveAddr.toString(16) + ")", location, jsValue, value); // FIXME doesn't work for initialized memory by Data section...
        stack.peek().push(value);
    },

    store(location, op, memarg, value) {
        const jsValue = stack.peek().pop();
        check(op + " value", location, jsValue, value);
        const jsAddr = stack.peek().pop();
        check(op + " addr", location, jsAddr, memarg.addr);
        const effectiveAddr = memarg.addr + memarg.offset;
        memory[effectiveAddr] = value;
    },

    memory_size(location, currentSizePages) {
        stack.peek().push(currentSizePages);
    },

    memory_grow(location, byPages, previousSizePages) {
        let jsByPages = stack.peek().pop();
        check("memory_grow", location, jsByPages, byPages);
        stack.peek().push(previousSizePages);
    },

    local(location, op, localIndex, value) {
        switch (op) {
            case "local.set":
                const jsValue = stack.peek().pop();
                check(op, location, jsValue, value);
                locals.peek()[localIndex] = value;
                return;
            case "local.tee":
                const jsValue2 = stack.peek().peek();
                check(op, location, jsValue2, value);
                locals.peek()[localIndex] = value;
                return;
            case "local.get":
                stack.peek().push(value);
                return;
        }
    },

    global(location, op, globalIndex, value) {
        switch (op) {
            case "global.set":
                const jsValue = stack.peek().pop();
                check(op, location, jsValue, value);
                globals[globalIndex] = value;
                return;
            case "global.get":
                stack.peek().push(value);
                return;
        }
    },
};