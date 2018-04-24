/*
 * User-facing API for dynamic analyses.
 */

function if_(location, condition) {
    console.log(location, "if, condition =", condition);
}

function br(location, target) {
    console.log(location, "br, to label #", target.label, "(==", target.location, ")");
}

function br_if(location, conditionalTarget, condition) {
    console.log(location, "br_if, (conditionally) to label #", conditionalTarget.label, " (==", conditionalTarget.label, "), condition =", condition);
}

function br_table(location, table, defaultTarget, tableIdx) {
    console.log(location, "br_table, table =", table, ", default target =", defaultTarget, ", table index =", tableIdx);
}

function begin(location, type) {
    console.log(location, "begin", type);
}

function end(location, type, beginLocation) {
    console.log(location, "end, for begin", type, "@", beginLocation);
}

function nop(location) {
    console.log(location, "nop");
}

function unreachable(location) {
    console.log(location, "unreachable");
}

function drop(location, value) {
    console.log(location, "drop, value =", value);
}

function select(location, cond, first, second) {
    console.log(location, "select, condition =", cond, "first =", first, "second =", second);
}

function call_(location, targetFunc, indirect, args) {
    console.log(location, (indirect ? "indirect" : "direct"), "call", "to func #", targetFunc, "args =", args);
}

function return_(location, values) {
    console.log(location, "return, values = ", values);
}

function call_result_(location, values) {
    console.log(location, "call result =", values);
}

function const_(location, value) {
    console.log(location, "const, value =", value);
}

function unary(location, op, input, result) {
    console.log(location, op, "input =", input, "result =", result);
}

function binary(location, op, first, second, result) {
    console.log(location, op, "first =", first, " second =", second, "result =", result);
}

function load(location, op, memarg, value) {
    console.log(location, op, "value =", value, "from =", memarg);
}

function store(location, op, memarg, value) {
    console.log(location, op, "value =", value, "to =", memarg);
}

function memory_size(location, currentSizePages) {
    console.log(location, "memory_size, size (in pages) =", currentSizePages);
}

function memory_grow(location, byPages, previousSizePages) {
    console.log(location, "memory_grow, delta (in pages) =", byPages, "previous size (in pages) =", previousSizePages);
}

function local(location, op, localIndex, value) {
    console.log(location, op, "local #", localIndex, "value =", value);
}

function global(location, op, globalIndex, value) {
    console.log(location, op, "global #", globalIndex, "value =", value);
}