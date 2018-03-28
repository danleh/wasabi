/*
 * User-facing API for dynamic analyses.
 */

function if_(location, condition) {
    // console.log("if @", location, "condition =", condition);
}

function br(location, target) {
    // console.log("br @", location, "to label #", target.label, " which is @", target.location);
}

function br_if(locataion, conditionalTarget, condition) {
    // console.log("br_if @", location, "possibly to label #", conditionalTarget.label, " which is @", conditionalTarget.label, "condition =", condition);
}

function br_table(location, table, defaultTarget, tableIdx) {
    // console.log("br_table @", location, "table =", table, "default target =", defaultTarget, "table index =", tableIdx);
}

function begin(location, type) {
    // console.log("begin", type, "@", location);
}

function end(location, type, beginLocation) {
    // console.log("end @", location, "for begin", type, "@", begin_location);
}

function nop(location) {
    // console.log("nop @", location);
}

function unreachable(location) {
    // console.log("unreachable @", location);
}

function drop(location) {
    // console.log("drop @", location);
}

function select(location, cond) {
    // console.log("select @", location, "condition =", cond);
}

function call_(location, targetFunc, indirect, args) {
    // console.log("call", (indirect ? "(indirect)" : "(direct)"), "func #", targetFunc, "@", location, "args =", args);
}

function return_(location, values) {
    // console.log("return @", location, "=", values);
}

function call_result_(location, values) {
    // console.log("result from call @", location, "=", values);
}

function const_(location, value) {
    // console.log("const @", location, "=", value);
}

function unary(location, op, input, result) {
    // console.log(op, "@", location, ":", input, "=", result);
}

function binary(location, op, first, second, result) {
    // console.log(op, "@", location, ":", first, ",", second, "=", result);
}

function load(location, op, memarg, value) {
    // console.log(op, "@", location, value, "from", memarg);
}

function store(location, op, memarg, value) {
    // console.log(op, "@", location, value, "to", memarg);
}

function current_memory(location, currentSizePages) {
    // console.log("current_memory @", location, "size (in pages) =", currentSizePages);
}

function grow_memory(location, byPages, previousSizePages) {
    // console.log("grow_memory @", location, "delta (in pages) =", byPages, "previous size (in pages) =", previousSizePages);
}

function local(location, op, index, value) {
    // console.log(op, "local #", index, "@", location, "=", value);
}

function global(location, op, index, value) {
    // console.log(op, "global #", index, "@", location, "=", value);
}