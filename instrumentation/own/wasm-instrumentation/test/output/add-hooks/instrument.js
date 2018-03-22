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
        i64_const: function (func, instr, low, high) {
            const_({func, instr}, new Long(low, high));
        },
        f32_const: function (func, instr, v) {
            const_({func, instr}, v);
        },
        f64_const: function (func, instr, v) {
            const_({func, instr}, v);
        },
        i32_eqz: function (func, instr, input, result) {
            unary({func, instr}, "i32.eqz", input, result);
        },
        i64_eqz: function (func, instr, input_low, input_high, result_low, result_high) {
            unary({func, instr}, "i64.eqz", new Long(input_low, input_high), new Long(result_low, result_high));
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