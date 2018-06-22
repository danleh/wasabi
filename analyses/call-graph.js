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
        if (fct.export !== null) return fct.export;
        if (fct.import !== null) return fct.import;
        return fctId;
    }

    Wasabi.analysis = {
        call_pre(location, targetFunc, args, indirectTableIdx) {
            const caller = fctName(location.func);
            const callee = fctName(targetFunc);
            if (caller.indexOf("__wasabi_function_") === -1 && callee.indexOf("__wasabi_function_") === -1) {
                callGraphEdges.add(caller + " --> " + callee);
            }
        },
    };

})();

