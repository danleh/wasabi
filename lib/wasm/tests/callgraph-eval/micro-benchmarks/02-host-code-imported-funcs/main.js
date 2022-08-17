const fs = require('fs');

let wasm_exports;

function imported() {
    wasm_exports.export2();
}

(async () => {
    const binary = fs.readFileSync('./main.wasm');
    const importObject = {
        'host': {
            'print': (arg) => {
                console.log(arg);
            },
            imported
        },
    };
    const result = await WebAssembly.instantiate(binary, importObject);
    wasm_exports = result.instance.exports;
    wasm_exports.main();
})();
