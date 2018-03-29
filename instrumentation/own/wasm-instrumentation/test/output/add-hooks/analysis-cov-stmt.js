// analysis specific

const coverageData = [];

function addLocation(location) {
    if (coverageData[location.func] === undefined) {
        coverageData[location.func] = new Set();
    }
    // location.instr == -1 for virtual "begin_function" hook (which corresponds to no instruction)
    if (location.instr >= 0) {
        coverageData[location.func].add(location.instr);
    }
}

function results() {
    for (const [fnIdx, fn] of staticInfo.functions.entries()) {
        // imported functions can naturally not have coverage information
        if (fn.import !== null) {
            continue;
        }
        console.log("function #", fnIdx, "exported as:", fn.export);
        const instrs = coverageData[fnIdx] || new Set();
        console.log("  executed instructions:", instrs.size, ",", [...instrs].sort((a, b) => a - b));
        console.log("  of a total of:", fn.instrCount);
        console.log("  coverage (%):", instrs.size / fn.instrCount * 100);
    }
}

// callbacks from analysis API

function if_(location, condition) {
    addLocation(location);
}

function br(location, target) {
    addLocation(location);
}

function br_if(locataion, conditionalTarget, condition) {
    addLocation(location);
}

function br_table(location, table, defaultTarget, tableIdx) {
    addLocation(location);
}

function begin(location, type) {
    addLocation(location);
}

function end(location, type, beginLocation) {
    addLocation(location);
}

function nop(location) {
    addLocation(location);
}

function unreachable(location) {
    addLocation(location);
}

function drop(location) {
    addLocation(location);
}

function select(location, cond) {
    addLocation(location);
}

function call_(location, targetFunc, indirect, args) {
    addLocation(location);
}

function return_(location, values) {
    addLocation(location);
}

function call_result_(location, values) {
    addLocation(location);
}

function const_(location, value) {
    addLocation(location);
}

function unary(location, op, input, result) {
    addLocation(location);
}

function binary(location, op, first, second, result) {
    addLocation(location);
}

function load(location, op, memarg, value) {
    addLocation(location);
}

function store(location, op, memarg, value) {
    addLocation(location);
}

function current_memory(location, currentSizePages) {
    addLocation(location);
}

function grow_memory(location, byPages, previousSizePages) {
    addLocation(location);
}

function local(location, op, localIndex, value) {
    addLocation(location);
}

function global(location, op, globalIndex, value) {
    addLocation(location);
}