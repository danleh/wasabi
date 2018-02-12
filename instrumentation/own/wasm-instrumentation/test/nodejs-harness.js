#!/opt/node-v9.4.0-linux-x64/bin/node

const fs = require("fs");

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

const importObject = {
	imports: {
		output: console.log,
	}
};

const instance = new WebAssembly.Instance(wasmModule, importObject)

// instance.exports."start"() is automatically called

// TODO call instrumentation functions, e.g., to retrieve counters