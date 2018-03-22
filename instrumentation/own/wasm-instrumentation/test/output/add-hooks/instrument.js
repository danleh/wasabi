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

        // generated:

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
        i32_wrap_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "i32_wrap_i64", new Long(input_low, input_high), result);
        },
        i32_trunc_s_f32: function (func, instr, input, result) {
            unary({func, instr}, "i32_trunc_s_f32", input, result);
        },
        i32_trunc_u_f32: function (func, instr, input, result) {
            unary({func, instr}, "i32_trunc_u_f32", input, result);
        },
        i32_trunc_s_f64: function (func, instr, input, result) {
            unary({func, instr}, "i32_trunc_s_f64", input, result);
        },
        i32_trunc_u_f64: function (func, instr, input, result) {
            unary({func, instr}, "i32_trunc_u_f64", input, result);
        },
        i64_extend_s_i32: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_extend_s_i32", input, new Long(result_low, result_high));
        },
        i64_extend_u_i32: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_extend_u_i32", input, new Long(result_low, result_high));
        },
        i64_trunc_s_f32: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_trunc_s_f32", input, new Long(result_low, result_high));
        },
        i64_trunc_u_f32: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_trunc_u_f32", input, new Long(result_low, result_high));
        },
        i64_trunc_s_f64: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_trunc_s_f64", input, new Long(result_low, result_high));
        },
        i64_trunc_u_f64: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_trunc_u_f64", input, new Long(result_low, result_high));
        },
        f32_convert_s_i32: function (func, instr, input, result) {
            unary({func, instr}, "f32_convert_s_i32", input, result);
        },
        f32_convert_u_i32: function (func, instr, input, result) {
            unary({func, instr}, "f32_convert_u_i32", input, result);
        },
        f32_convert_s_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "f32_convert_s_i64", new Long(input_low, input_high), result);
        },
        f32_convert_u_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "f32_convert_u_i64", new Long(input_low, input_high), result);
        },
        f32_demote_f64: function (func, instr, input, result) {
            unary({func, instr}, "f32_demote_f64", input, result);
        },
        f64_convert_s_i32: function (func, instr, input, result) {
            unary({func, instr}, "f64_convert_s_i32", input, result);
        },
        f64_convert_u_i32: function (func, instr, input, result) {
            unary({func, instr}, "f64_convert_u_i32", input, result);
        },
        f64_convert_s_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "f64_convert_s_i64", new Long(input_low, input_high), result);
        },
        f64_convert_u_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "f64_convert_u_i64", new Long(input_low, input_high), result);
        },
        f64_promote_f32: function (func, instr, input, result) {
            unary({func, instr}, "f64_promote_f32", input, result);
        },
        i32_reinterpret_f32: function (func, instr, input, result) {
            unary({func, instr}, "i32_reinterpret_f32", input, result);
        },
        i64_reinterpret_f64: function (func, instr, input, result_low, result_high) {
            unary({func, instr}, "i64_reinterpret_f64", input, new Long(result_low, result_high));
        },
        f32_reinterpret_i32: function (func, instr, input, result) {
            unary({func, instr}, "f32_reinterpret_i32", input, result);
        },
        f64_reinterpret_i64: function (func, instr, input_low, input_high, result) {
            unary({func, instr}, "f64_reinterpret_i64", new Long(input_low, input_high), result);
        },
        i32_eq: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_eq", first, second, result);
        },
        i32_ne: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_ne", first, second, result);
        },
        i32_lt_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_lt_s", first, second, result);
        },
        i32_lt_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_lt_u", first, second, result);
        },
        i32_gt_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_gt_s", first, second, result);
        },
        i32_gt_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_gt_u", first, second, result);
        },
        i32_le_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_le_s", first, second, result);
        },
        i32_le_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_le_u", first, second, result);
        },
        i32_ge_s: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_ge_s", first, second, result);
        },
        i32_ge_u: function (func, instr, first, second, result) {
            binary({func, instr}, "i32_ge_u", first, second, result);
        },
        i64_eq: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({func, instr}, "i64_eq", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_ne: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({func, instr}, "i64_ne", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_lt_s: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_lt_s", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_lt_u: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_lt_u", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_gt_s: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_gt_s", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_gt_u: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_gt_u", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_le_s: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_le_s", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_le_u: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_le_u", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_ge_s: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_ge_s", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        i64_ge_u: function (func, instr, first_low, first_high, second_low, second_high, result) {
            binary({
                func,
                instr
            }, "i64_ge_u", new Long(first_low, first_high), new Long(second_low, second_high), result);
        },
        f32_eq: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_eq", first, second, result);
        },
        f32_ne: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_ne", first, second, result);
        },
        f32_lt: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_lt", first, second, result);
        },
        f32_gt: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_gt", first, second, result);
        },
        f32_le: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_le", first, second, result);
        },
        f32_ge: function (func, instr, first, second, result) {
            binary({func, instr}, "f32_ge", first, second, result);
        },
        f64_eq: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_eq", first, second, result);
        },
        f64_ne: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_ne", first, second, result);
        },
        f64_lt: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_lt", first, second, result);
        },
        f64_gt: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_gt", first, second, result);
        },
        f64_le: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_le", first, second, result);
        },
        f64_ge: function (func, instr, first, second, result) {
            binary({func, instr}, "f64_ge", first, second, result);
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
    // console.log("const @", location, ":", value);
}

function unary(location, op, input, result) {
    console.log(op, "@", location, ":", input, "->", result);
}

function binary(location, op, first, second, result) {
    console.log(op, "@", location, ":", first, ",", second, "->", result);
}