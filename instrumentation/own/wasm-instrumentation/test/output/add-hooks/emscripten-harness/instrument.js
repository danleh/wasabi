// your instrumentation goes here...
function return_(location, values) {
    console.log("return @", location);
    console.log("values:", values.map(v => v.toString()));
}

// low-level 
let oldInstantiate = WebAssembly.instantiate;
WebAssembly.instantiate = function () {
    let importsObject = arguments[1] || {};
    importsObject.hooks = {
        _return_: function (func, instr) {
            return_({func, instr}, []);
        },
        _return_I: function (func, instr, low, high) {
            return_({func, instr}, [new Long(low, high)]);
        },
        _return_i: function (func, instr, v) {
            return_({func, instr}, [v]);
        },
        _return_f: function (func, instr, v) {
            return_({func, instr}, [v]);
        },
        _return_F: function (func, instr, v) {
            return_({func, instr}, [v]);
        },
    };
    console.log(importsObject);

    arguments[1] = importsObject;
    return oldInstantiate.apply(this, arguments);
}
