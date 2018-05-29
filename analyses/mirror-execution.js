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
        console.log(op, location, jsValue, wasmValue);
    }
}

/*
 * WebAssembly program state, mirrored in JavaScript
 */

const stack = [{
    func: undefined,
    // the value stack for each function contains substacks for each block
    blocks: [],
    locals: []
}];

function values() {
    return stack.peek().blocks.peek();
}

const memory = [];
const globals = [];

/*
 * "mirror execution" analysis, that performs all "writing operations" (like memory stores or set_global) also in
 * JavaScript and compares "reading operation" results between JavaScript and WebAssembly.
 */

Wasabi.analysis = {
    if_(location, condition) {
        const jsCondition = values().pop() === 1;
        check("if", location, jsCondition, condition);
    },

    br(location, target) {
        const clearThisBlock = stack.peek().blocks.pop();
    },

    br_if(location, conditionalTarget, condition) {
        const jsCondition = values().pop() === 1;
        check("br_if", location, jsCondition, condition);
        if (condition) {
            const clearThisBlock = stack.peek().blocks.pop();
        }
    },

    br_table(location, table, defaultTarget, tableIdx) {
        const jsTableIdx = values().pop();
        check("br_table", location, jsTableIdx, tableIdx);
        const clearThisBlock = stack.peek().blocks.pop();
    },

    begin(location, type) {
        if (type === "function") {
            // TODO set locals to parameter values of function?
            stack.peek().func = location.func;
        }
        stack.peek().blocks.push([]);
    },

    end(location, type, beginLocation) {
        const [result] = stack.peek().blocks.pop();
        if (result !== undefined) {
            stack.peek().blocks.peek().push(result);
        }
    },

    drop(location, value) {
        check("drop", location, values().pop(), value);
    },

    select(location, condition, first, second) {
        check("select", location, values().pop(), second);
        check("select", location, values().pop(), first);
        const jsCondition = values().pop() === 1;
        check("select", location, jsCondition, condition);
    },

    call_pre(location, targetFunc, indirect, args) {
        if (indirect) {
            const jsTargetTableIdx = values().pop();
            check("call_indirect table idx", location, jsTargetTableIdx, targetFunc);
        }
        for (const arg of args.reverse()) {
            const jsArg = values().pop();
            check("call args", location, jsArg, arg);
        }
        // add stack frame
        stack.push({
            func: targetFunc,
            blocks: [],
            locals: [],
        });
    },

    call_post(location, vals) {
        // clear stack frame
        stack.pop();
        for (const val of vals) {
            values().push(val);
        }
    },

    return_(location, values) {
        // TODO how does it influence the stack? Is this already handled by end_function?
    },

    const_(location, value) {
        values().push(value);
    },

    unary(location, op, input, result) {
        const jsInput = values().pop();
        check(op, location, jsInput, input);
        values().push(result);
    },

    binary(location, op, first, second, result) {
        const jsSecond = values().pop();
        check(op + " second arg", location, jsSecond, second);
        const jsFirst = values().pop();
        check(op + " first arg", location, jsFirst, first);
        values().push(result);
    },

    load(location, op, memarg, value) {
        const jsAddr = values().pop();
        check(op + " addr", location, jsAddr, memarg.addr);
        const effectiveAddr = memarg.addr + memarg.offset;
        const jsValue = memory[effectiveAddr];
        check(op + " value @ " + effectiveAddr + " (0x" + effectiveAddr.toString(16) + ")", location, jsValue, value); // FIXME doesn't work for initialized memory by Data section...
        values().push(value);
    },

    store(location, op, memarg, value) {
        const jsValue = values().pop();
        check(op + " value", location, jsValue, value);
        const jsAddr = values().pop();
        check(op + " addr", location, jsAddr, memarg.addr);
        const effectiveAddr = memarg.addr + memarg.offset;
        memory[effectiveAddr] = value;
    },

    memory_size(location, currentSizePages) {
        values().push(currentSizePages);
    },

    memory_grow(location, byPages, previousSizePages) {
        let jsByPages = values().pop();
        check("memory_grow", location, jsByPages, byPages);
        values().push(previousSizePages);
    },

    local(location, op, localIndex, value) {
        switch (op) {
            case "set_local":
                const jsValue = values().pop();
                check(op, location, jsValue, value);
                stack.peek().locals[localIndex] = value;
                return;
            case "tee_local":
                const jsValue2 = values().peek();
                check(op, location, jsValue2, value);
                stack.peek().locals[localIndex] = value;
                return;
            case "get_local":
                values().push(value);
                return;
        }
    },

    global(location, op, globalIndex, value) {
        switch (op) {
            case "set_global":
                const jsValue = values().pop();
                check(op, location, jsValue, value);
                globals[globalIndex] = value;
                return;
            case "get_global":
                values().push(value);
                return;
        }
    },
};