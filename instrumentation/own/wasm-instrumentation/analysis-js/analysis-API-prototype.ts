import Long from "./lib/long.js";

/** i32, f32, and f64 are represented as number, i64 as Long */
type Val = number | Long;

interface InstructionLocation {
    func: number,
    instr: number,
}

function call(i: InstructionLocation, func: number) {
}

function return_(i: InstructionLocation, values: [Val]) {
}

function _return_i(func: number, instr: number, value: number) {
    return_({func, instr}, [value]);
}

// TODO how to return 2 values from a JavaScript function to WASM?
// for manipulating return values
function _return_I(func: number, instr: number, low: number, high: number) {
    return_({func, instr}, [new Long(low, high)]);
}