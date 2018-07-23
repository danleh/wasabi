{
    const coverageData = [];
    Wasabi.analysisResult = coverageData;

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

    Wasabi.analysis = {
        if_(location, condition) {
            addBranch(location, condition);
        },

        br_if(location, conditionalTarget, condition) {
            addBranch(location, condition);
        },

        br_table(location, table, defaultTarget, tableIdx) {
            addBranch(location, tableIdx);
        },

        select(location, condition) {
            addBranch(location, condition);
        },
    };
}