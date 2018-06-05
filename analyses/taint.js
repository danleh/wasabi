// Author: Michael Pradel

/*
 * Simple taint analysis that considers explicit flows only
 * (i.e., no flows caused by control flow dependencies, but only data flow).
 *
 * TODO work-in-progress ...
 *
 */

(function() {

    Wasabi.analysis = {
        start(location) {
            console.log(location, "start");
        },

        begin(location, type) {
            console.log(location, "begin", type);
        },

        end(location, type, beginLocation) {
            console.log(location, "end", type, "(begin @", beginLocation, ")");
        },

        drop(location, value) {
            console.log(location, "drop, value =", value);
        },

        select(location, cond, first, second) {
            console.log(location, "select, condition =", cond, "first =", first, "second =", second);
        },

        call_pre(location, targetFunc, args, indirectTableIdx) {
            console.log(location, (indirectTableIdx === undefined ? "direct" : "indirect"), "call", "to func #", targetFunc, "args =", args);
        },

        call_post(location, values) {
            console.log(location, "call result =", values);
        },

        return_(location, values) {
            console.log(location, (location.instr === -1) ? "implicit" : "explicit", "return, values = ", values);
        },

        const_(location, value) {
            console.log(location, "const, value =", value);
        },

        unary(location, op, input, result) {
            console.log(location, op, "input =", input, "result =", result);
        },

        binary(location, op, first, second, result) {
            console.log(location, op, "first =", first, " second =", second, "result =", result);
        },

        load(location, op, memarg, value) {
            console.log(location, op, "value =", value, "from =", memarg);
        },

        store(location, op, memarg, value) {
            console.log(location, op, "value =", value, "to =", memarg);
        },

        memory_size(location, currentSizePages) {
            console.log(location, "memory_size, size (in pages) =", currentSizePages);
        },

        memory_grow(location, byPages, previousSizePages) {
            console.log(location, "memory_grow, delta (in pages) =", byPages, "previous size (in pages) =", previousSizePages);
        },

        local(location, op, localIndex, value) {
            console.log(location, op, "local #", localIndex, "value =", value);
        },

        global(location, op, globalIndex, value) {
            console.log(location, op, "global #", globalIndex, "value =", value);
        },
    };

})();