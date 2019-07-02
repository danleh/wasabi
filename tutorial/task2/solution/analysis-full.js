// One way (of many possible ones) how to write the full analysis.

(function () {
    const callGraphEdges = new Set();

    Wasabi.callGraph = callGraphEdges;

    Wasabi.saveCallGraph = function() { saveCallGraph(callGraphEdges) };

    function fctName(fctId) {
        const fct = Wasabi.module.info.functions[fctId];
        if (fct.export[0] !== undefined) return fct.export[0];
        if (fct.import !== null) return fct.import;
        return fctId;
    }

    Wasabi.analysis = {
        call_pre(location, targetFunc, args, indirectTableIdx) {
            let caller = fctName(location.func);
            let callee = fctName(targetFunc);
            // Optional: Only add call graph edge if either of the two functions has a name.
            // Gives more legible call graph, but misses (many, in the case of non-debug build) edges.
            if (typeof caller !== "number" || typeof callee !== "number") {
                callGraphEdges.add(caller + " -> " + callee);
            }
        },
    };
})();
