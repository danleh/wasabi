const coverageData = [];

function addLocation(location) {
    if (coverageData[location.func] === undefined) {
        coverageData[location.func] = new Set();
    }
    // ignore "virtual instructions" that are not actually present in the binary
    // (e.g., "begin_function" hook or implicit returns)
    // for them location.instr === -1
    if (location.instr >= 0) {
        coverageData[location.func].add(location.instr);
    }
}

// call after the program-to-analyze has completed
function results() {
    for (const [fnIdx, fn] of Wasabi.module.info.functions.entries()) {
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

// just let every callback add its location to coverageData
for (const hook of Wasabi.HOOK_NAMES) {
    Wasabi.analysis[hook] = addLocation;
}