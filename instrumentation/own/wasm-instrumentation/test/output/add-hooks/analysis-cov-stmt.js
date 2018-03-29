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

// just let every callback report add its location to coverageData
for (const callback of analysisCallbacks) {
    this[callback] = addLocation;
}