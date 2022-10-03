{
    const coverage = [];

    function addLocation({func, instr}) {
        coverage[func] = coverage[func] || [];
        // JS engine optimizes this to sparse array anyway, no need for Sets
        coverage[func][instr] = true;
    }

    // just let every callback add its location to coverage
    for (const hook of Wasabi.HOOK_NAMES) {
        Wasabi.analysis[hook] = addLocation;
    }

    Wasabi.analysisResult = coverage;

    console.log(coverage);
    //console.log(Wasabi.analysisResult);
}


/*{
    const coverage = [];

    // branch can be boolean (for if and br_if) or integer (for br_table, i.e., switches)
    function addBranch({func, instr}, branch) {
        coverage[func] = coverage[func] || [];
        coverage[func][instr] = coverage[func][instr] || [];
        if (!coverage[func][instr].includes(branch)) {
            coverage[func][instr].push(branch);
        }
    }

    Wasabi.analysis = {
        if_(loc, cond) { addBranch(loc, cond) },
        br_if(loc, target, cond) { addBranch(loc, cond) },
        br_table(loc, tbl, df, idx) { addBranch(loc, idx) },
        select(loc, cond) { addBranch(loc, cond) },
    };

    Wasabi.analysisResult = coverage;
    console.log(coverage);
    //console.log(Wasabi.analysisResult);
}*/
