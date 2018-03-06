"use strict";
exports.__esModule = true;
var long_js_1 = require("./lib/long.js");
function call(i, func) {
}
function return_(i, values) {
}
function return_i32(func, instr, value) {
    return_({ func: func, instr: instr }, [value]);
}
// TODO how to return 2 values from a JavaScript function to WASM?
// for manipulating return values
function return_i64(func, instr, low, high) {
    return_({ func: func, instr: instr }, [new long_js_1["default"](low, high)]);
}
