// low-level stuff, TODO auto-generated from WASM module to-instrument
const oldInstantiate = WebAssembly.instantiate;
WebAssembly.instantiate = function () {
    let importsObject = arguments[1] || {};
    importsObject.hooks = {
        return_: function (func, instr) {
            return_({func, instr}, []);
        },
        return_i32: function (func, instr, v) {
            return_({func, instr}, [v]);
        },
        return_i64: function (func, instr, low, high) {
            return_({func, instr}, [new Long(low, high)]);
        },
        return_f32: function (func, instr, v) {
            return_({func, instr}, [v]);
        },
        return_f64: function (func, instr, v) {
            return_({func, instr}, [v]);
        },
        i32_const: function (func, instr, v) {
            const_({func, instr}, v);
        },
        i64_const: function (func, instr, v_low, v_high) {
            const_({func, instr}, new Long(v_low, v_high));
        },
        f32_const: function (func, instr, v) {
            const_({func, instr}, v);
        },
        f64_const: function (func, instr, v) {
            const_({func, instr}, v);
        },
        i32_eqz: function (func, instr, input, result) {
            unary({func, instr}, "i32_eqz", input, result);
        },
        i64_eqz: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "i64_eqz", new Long(input_low, input_high), result);
        },
        i32_clz: function (func, instr, input, result) {
            unary({func, instr}, "i32_clz", input, result);
        },
        i32_ctz: function (func, instr, input, result) {
            unary({func, instr}, "i32_ctz", input, result);
        },
        i32_popcnt: function (func, instr, input, result) {
            unary({func, instr}, "i32_popcnt", input, result);
        },
        i64_clz: function (func, instr, input_low, input_high, result_low, result_high) {
            unary({func, instr}, "i64_clz", new Long(input_low, input_high), new Long(result_low, result_high));
        },
        i64_ctz: function (func, instr, input_low, input_high, result_low, result_high) {
            unary({func, instr}, "i64_ctz", new Long(input_low, input_high), new Long(result_low, result_high));
        },
        i64_popcnt: function (func, instr, input_low, input_high, result_low, result_high) {
            unary({func, instr}, "i64_popcnt", new Long(input_low, input_high), new Long(result_low, result_high));
        },
        f32_abs: function (func, instr, input, result) {
            unary({func, instr}, "f32_abs", input, result);
        },
        f32_neg: function (func, instr, input, result) {
            unary({func, instr}, "f32_neg", input, result);
        },
        f32_ceil: function (func, instr, input, result) {
            unary({func, instr}, "f32_ceil", input, result);
        },
        f32_floor: function (func, instr, input, result) {
            unary({func, instr}, "f32_floor", input, result);
        },
        f32_trunc: function (func, instr, input, result) {
            unary({func, instr}, "f32_trunc", input, result);
        },
        f32_nearest: function (func, instr, input, result) {
            unary({func, instr}, "f32_nearest", input, result);
        },
        f64_abs: function (func, instr, input, result) {
            unary({func, instr}, "f64_abs", input, result);
        },
        f64_neg: function (func, instr, input, result) {
            unary({func, instr}, "f64_neg", input, result);
        },
        f64_ceil: function (func, instr, input, result) {
            unary({func, instr}, "f64_ceil", input, result);
        },
        f64_floor: function (func, instr, input, result) {
            unary({func, instr}, "f64_floor", input, result);
        },
        f64_trunc: function (func, instr, input, result) {
            unary({func, instr}, "f64_trunc", input, result);
        },
        f64_nearest: function (func, instr, input, result) {
            unary({func, instr}, "f64_nearest", input, result);
        },
    };
    console.log(importsObject);

    arguments[1] = importsObject;
    return oldInstantiate.apply(this, arguments);
};

// const staticModuleInfo = {
//     functions: [
//         {
//             import: ["env", "bla"],
//             export: "exportedname",
//             type: {
//                 params: ["i32", "i32"],
//                 results: [],
//             },
//             locals: ["i32", "i32"],
//             instructions: 100,
//         }
//     ]
// };

// your instrumentation goes here...
// const coverageData = [];

function return_(location, values) {
    // if (coverageData[location.func] === undefined) {
    //     coverageData[location.func] = new Set();
    // }
    // coverageData[location.func].add(location.instr);
    // if (coverageData[location.func][location.instr] === undefined) {
    //     coverageData[location.func][location.instr] = true;
    // }
}

function const_(location, value) {
    console.log("const @", location, ":", value);
}

function unary(location, op, input, result) {
    console.log(op, "@", location, ":", input, "->", result);
}