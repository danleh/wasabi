// Author: Michael Pradel

/*
 * Simple analysis to trace all calls and build a call graph.
 */

(function() {

    console.log("Starting call tracing. Check for results in Wasabi.analysisResult.");

    const callGraphEdges = new Set();
    Wasabi.analysisResult = callGraphEdges;

    function fctName(fctId) {
        const fct = Wasabi.module.info.functions[fctId];
        if (fct.export[0] !== undefined) return fct.export[0];
        if (fct.import !== null) return fct.import;
        return fctId;
    }

    Wasabi.analysis = {
        call_pre(location, targetFunc, args, indirectTableIdx) {
            const caller = fctName(location.func);
            const callee = fctName(targetFunc);
            callGraphEdges.add(caller + " --> " + callee);
        },
    };

})();

