const fs = require('fs');

let wasm_exports;

// Calls export1, but not not export1.
function imported() {
    wasm_exports.export1();
}

(async () => {
    const binary = fs.readFileSync('./main.wasm');
    const importObject = {
        'host': {
            imported
        },
    };
    const result = await WebAssembly.instantiate(binary, importObject);
    wasm_exports = result.instance.exports;
    wasm_exports.main();
})();
