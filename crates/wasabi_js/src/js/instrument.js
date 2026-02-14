const wasabi = require("../../pkg");
const fs = require('fs');
const process = require('process');
const path = require('node:path'); 
const arg = require('arg');

// Parse arguments
const args = arg({
	'-o': String, // output directory
});
const INPUT_PATH = args._[0];
const INPUT_BASE = path.basename(INPUT_PATH, '.wasm');
const OUTPUT_DIR = args['-o'] || process.cwd() + '/out';

// Instrument wasm
const buf = fs.readFileSync(INPUT_PATH);
const arr = new Uint8Array(buf);
const { instrumented, js } = wasabi.instrument_wasm(arr);

// Write output
if (!fs.existsSync(OUTPUT_DIR)) {
    fs.mkdirSync(OUTPUT_DIR);
}
fs.writeFileSync(path.join(OUTPUT_DIR, INPUT_BASE + '.wasm'), new Uint8Array(instrumented));
fs.writeFileSync(path.join(OUTPUT_DIR, INPUT_BASE + ".wasabi.js"), js);
