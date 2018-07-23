{
    const coverageData = [];
    Wasabi.analysisResult = coverageData;

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

    // just let every callback add its location to coverageData
    for (const hook of Wasabi.HOOK_NAMES) {
        Wasabi.analysis[hook] = addLocation;
    }
}