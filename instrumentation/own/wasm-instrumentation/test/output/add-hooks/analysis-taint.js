// convenience
Array.prototype.peek = function () {
    return this[this.length - 1];
};

function check(jsValue, wasmValue) {
    if (jsValue.toString() !== wasmValue.toString()) {
        console.log(jsValue, wasmValue);
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

const memory = {};
const globals = [];

let valueCounter = 0;

const sourceFnIdx = [];
const sinkFnIdx = [];

// TODO add push and pop for every instruction
// TODO correctness: are popped values == on JS replicated stack

/*
 * User-facing API for dynamic analyses.
 */

function if_(location, condition) {
    const jsCondition = values().pop() === 1;
    if (jsCondition !== condition)
        console.log(jsCondition, condition);
}

function br(location, target) {
    const clearThisBlock = stack.peek().blocks.pop();
}

function br_if(locataion, conditionalTarget, condition) {
    const jsCondition = values().pop() === 1;
    // console.log(jsCondition, condition);
    if (condition) {
        const clearThisBlock = stack.peek().blocks.pop();
    }
}

function br_table(location, table, defaultTarget, tableIdx) {
    const clearThisBlock = stack.peek().blocks.pop();
    const jsTableIdx = values().pop();
    // console.log(jsTableIdx, tableIdx);
}

function begin(location, type) {
    if (type === "function") {
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
    values().pop();
}

function select(location, cond) {
    values().pop();
    values().pop();
    values().pop();
}

function call_(location, targetFunc, indirect, args) {
    for (const arg of args) {
        const jsArg = values().pop();
        // console.log(jsArg, arg);
    }
    if (indirect) {
        const jsTargetTableIdx = values().pop();
        // console.log(jsTargetTableIdx, targetFunc);
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
    values().push(result);
}

function binary(location, op, first, second, result) {
    const jsSecond = values().pop();
    const jsFirst = values().pop();
    if (jsFirst !== first)
        console.log(jsFirst, first);
    if (jsSecond !== second)
        console.log(jsSecond, second);
    values().push(result);
}

function load(location, op, memarg, value) {
    const jsAddr = values().pop();
    if (jsAddr !== memarg.addr) {
        console.log(location);
    }
    const jsValue = memory[memarg.addr + memarg.offset];
    if (jsValue !== value) {
        console.log(location, jsValue, value);
    }

    values().push(value);
}

function store(location, op, memarg, value) {
    const jsValue = values().pop();
    if (jsValue.toString() !== value.toString()) {
        console.log(location);
    }
    const jsAddr = values().pop();
    check(jsAddr, memarg.addr);
    memory[memarg.addr + memarg.offset] = value;
}

function current_memory(location, currentSizePages) {
    values().push(currentSizePages);
}

function grow_memory(location, byPages, previousSizePages) {
    let jsByPages = values().pop();
    values().push(previousSizePages);
}

// TODO local/global: uninitialized should return 0, right? i.e. do || 0 for get

function local(location, op, localIndex, value) {
    switch (op) {
        case "set":
            const jsValue = values().pop();
            // if (jsValue !== value) console.log(location, jsValue, value);
            stack.peek().locals[localIndex] = value;
            return;
        case "tee":
            const jsValue2 = values().peek();
            if (jsValue2 !== value) console.log(location, jsValue2, value);
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
            if (jsValue !== value) console.log(location, jsValue, value);
            globals[globalIndex] = value;
            return;
        case "get":
            values().push(value);
            return;
    }
}