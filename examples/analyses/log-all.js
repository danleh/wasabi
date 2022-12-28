/*
 * User-facing API for dynamic analyses.
 */

Wasabi.analysis = {
    start(location) {
        console.log(location, "start");
    },

    nop(location) {
        console.log(location, "nop");
    },

    unreachable(location) {
        console.log(location, "unreachable");
    },

    if_(location, condition) {
        console.log(location, "if, condition =", condition);
    },

    br(location, target) {
        console.log(location, "br, to label", target.label, "(==", target.location, ")");
    },

    br_if(location, conditionalTarget, condition) {
        console.log(location, "br_if, (conditionally) to label", conditionalTarget.label, "(==", conditionalTarget.location, "), condition =", condition);
    },

    br_table(location, table, defaultTarget, tableIdx) {
        console.log(location, "br_table, table =", table, ", default target =", defaultTarget, ", table index =", tableIdx);
    },

    begin(location, type) {
        console.log(location, "begin", type);
    },

    // ifLocation === location of the matching if block for else
    end(location, type, beginLocation, ifLocation) {
        console.log(location, "end", type, "(begin @", beginLocation, ", if begin @", ifLocation, ")");
    },

    drop(location, value) {
        console.log(location, "drop, value =", value);
    },

    select(location, cond, first, second) {
        console.log(location, "select, condition =", cond, "first =", first, "second =", second);
    },

    // indirectTableIdx === undefined iff direct call, for indirect calls it is a number
    call_pre(location, targetFunc, args, indirectTableIdx) {
        console.log(location, (indirectTableIdx === undefined ? "direct" : "indirect"), "call", "to func #", targetFunc, "args =", args);
    },

    call_post(location, values) {
        console.log(location, "call result =", values);
    },

    return_(location, values) {
        console.log(location, (location.instr === -1) ? "implicit" : "explicit", "return, values = ", values);
    },

    const_(location, op, value) {
        console.log(location, op, "value =", value);
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