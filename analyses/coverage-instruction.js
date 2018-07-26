{
	// TypedArray is ca 1s vs 8s of regular array of bools!
    const totalInstr = Wasabi.module.info.instrCountCumulative[Wasabi.module.info.instrCountCumulative.length - 1];
    const coverage = new Uint8Array(totalInstr);

    function addLocation(location) {
        coverage[location] = 1;
    }

    // just let every callback add its location to coverage
    for (const hook of Wasabi.HOOK_NAMES) {
        Wasabi.analysis[hook] = addLocation;
    }

    Wasabi.analysisResult = coverage;
}