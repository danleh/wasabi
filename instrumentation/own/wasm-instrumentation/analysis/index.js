#!/opt/node-v9.4.0-linux-x64/bin/node
const fs = require("fs");
const Long = require("long");

if (typeof WebAssembly !== "object") {
    console.error("WebAssembly object not available, make sure to run Node version >= 8.");
    process.exit(1);
}

// 0 is node itself, 1 is the script, only 2 is then the wasm file
const wasmFilename = process.argv[2];
if (typeof wasmFilename !== "string") {
    console.error("No wasm file to execute given, script argument wasm", wasmFilename);
    process.exit(1);
}

const wasmBinaryBuffer = fs.readFileSync(wasmFilename);
const wasmModule = new WebAssembly.Module(wasmBinaryBuffer);

function return_(location, values) {
    console.log("return @", location);
    console.log("values:", values);
}

const importObject = {
    hooks: {
        _return_: function (func, instr) {
            return_({func, instr}, []);
        },
        _return_I: function (func, instr, low, high) {
            return_({func, instr}, [new Long(low, high)]);
        }
    }
};

const instance = new WebAssembly.Instance(wasmModule, importObject);