const fs = require('fs');

let wasm_exports;

function imported() {
    // Even though the function is not itself exported, it is reachable via the table.
    wasm_exports.table.get(0)();
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
