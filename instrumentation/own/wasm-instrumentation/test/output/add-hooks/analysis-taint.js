// convenience
Array.prototype.peek = function () {
    return this[this.length - 1];
};

function check(op, location, jsValue, wasmValue) {
    if ((jsValue === undefined ? "undefined" : jsValue).toString() !== wasmValue.toString()) {
        console.log(op, location, jsValue, wasmValue);
    }
}

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

let valueCounter = 0;

const sourceFnIdx = [];
const sinkFnIdx = [];

/*
 * User-facing API for dynamic analyses.
 */

function if_(location, condition) {
    const jsCondition = values().pop() === 1;
    check("if", location, jsCondition, condition);
}

function br(location, target) {
    const clearThisBlock = stack.peek().blocks.pop();
}

function br_if(location, conditionalTarget, condition) {
    const jsCondition = values().pop() === 1;
    check("br_if", location, jsCondition, condition);
    if (condition) {
        const clearThisBlock = stack.peek().blocks.pop();
    }
}

function br_table(location, table, defaultTarget, tableIdx) {
    const jsTableIdx = values().pop();
    check("br_table", location, jsTableIdx, tableIdx);
    const clearThisBlock = stack.peek().blocks.pop();
}

function begin(location, type) {
    if (type === "function") {
        // TODO set locals to parameter values of function?
        stack.peek().func = location.func;
    }
    stack.peek().blocks.push([]);
}

function end(location, type, beginLocation) {
    const [result] = stack.peek().blocks.pop();
    if (result !== undefined) {
        stack.peek().blocks.peek().push(result);
    }
}

function drop(location) {
    values().pop(); // FIXME cannot compare since wasm value would need monomorphization, which needs stack typing
}

function select(location, condition) {
    values().pop(); // FIXME cannot compare since wasm value would need monomorphization, which needs stack typing
    values().pop(); // FIXME cannot compare since wasm value would need monomorphization, which needs stack typing
    const jsCondition = values().pop() === 1;
    check("select", location, jsCondition, condition);
}

function call_(location, targetFunc, indirect, args) {
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
}

function call_result_(location, vals) {
    // clear stack frame
    stack.pop();
    for (const val of vals) {
        values().push(val);
    }
}

function return_(location, values) {
    // FIXME how does it influence the stack?
}

function const_(location, value) {
    values().push(value);
}

function unary(location, op, input, result) {
    const jsInput = values().pop();
    check(op, location, jsInput, input);
    values().push(result);
}

function binary(location, op, first, second, result) {
    const jsSecond = values().pop();
    check(op + " second arg", location, jsSecond, second);
    const jsFirst = values().pop();
    check(op + " first arg", location, jsFirst, first);
    values().push(result);
}

function load(location, op, memarg, value) {
    const jsAddr = values().pop();
    check(op + " addr", location, jsAddr, memarg.addr);
    const effectiveAddr = memarg.addr + memarg.offset;
    const jsValue = memory[effectiveAddr];
    check(op + " value @ " + effectiveAddr + " (0x" + effectiveAddr.toString(16) + ")", location, jsValue, value); // FIXME doesn't work for initialized memory by Data section...
    values().push(value);
}

function store(location, op, memarg, value) {
    const jsValue = values().pop();
    check(op + " value", location, jsValue, value);
    const jsAddr = values().pop();
    check(op + " addr", location, jsAddr, memarg.addr);
    const effectiveAddr = memarg.addr + memarg.offset;
    memory[effectiveAddr] = value;
}

function current_memory(location, currentSizePages) {
    values().push(currentSizePages);
}

function grow_memory(location, byPages, previousSizePages) {
    let jsByPages = values().pop();
    check("grow_memory", location, jsByPages, byPages);
    values().push(previousSizePages);
}

function local(location, op, localIndex, value) {
    switch (op) {
        case "set":
            const jsValue = values().pop();
            check("set_local", location, jsValue, value);
            stack.peek().locals[localIndex] = value;
            return;
        case "tee":
            const jsValue2 = values().peek();
            check("tee_local", location, jsValue2, value);
            stack.peek().locals[localIndex] = value;
            return;
        case "get":
            values().push(value);
            return;
    }
}

function global(location, op, globalIndex, value) {
    switch (op) {
        case "set":
            const jsValue = values().pop();
            check("set_global", location, jsValue, value);
            globals[globalIndex] = value;
            return;
        case "get":
            values().push(value);
            return;
    }
}