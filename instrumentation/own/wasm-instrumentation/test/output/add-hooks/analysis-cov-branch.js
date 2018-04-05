// analysis specific

// TODO how to get 100% data, i.e., the number of possible branches? would need to know each if/br_if/br_table instr
const coverageData = [];

// branch can be boolean (for if and br_if) or integer (for br_table, i.e., switches)
function addBranch(location, branch) {
    if (coverageData[location.func] === undefined) {
        coverageData[location.func] = [];
    }
    if (coverageData[location.func][location.instr] === undefined) {
        coverageData[location.func][location.instr] = new Set();
    }
    coverageData[location.func][location.instr].add(branch);
}

function results() {
    for (const [fnIdx, fnCov] of coverageData.entries()) {
        if (fnCov !== undefined) {
            for (const [instrIdx, instrCov] of fnCov.entries()) {
                if (instrCov !== undefined)
                    console.log("function", fnIdx, "instruction", instrIdx, "branches covered:", [...instrCov])
            }
        }
    }
}

// callbacks from analysis API

function if_(location, condition) {
    addBranch(location, condition);
}

function br_if(locataion, conditionalTarget, condition) {
    addBranch(location, condition);
}

function br_table(location, table, defaultTarget, tableIdx) {
    addBranch(location, tableIdx);
}

// TODO are selects really branches?
function select(location, condition) {
    addBranch(location, condition);
}

function call_(location, targetFunc, indirect, args) {
    if (indirect) {
        // TODO are indirect calls a form of branch? what is 100% coverage there, the full module table?
    }
}