const fs = require('fs');

(async () => {

    const binary = fs.readFileSync('./main.wasm');

    const table = new WebAssembly.Table({
        'element': 'anyfunc',
        'initial': 1,
    });

    function imported() {
        // Even though the function is not itself exported, it is reachable via the table.
        table.get(0)();
    }

    const importObject = {
        'host': {
            'print': (arg) => {
                console.log(arg);
            },
            table,
            imported
        },
    };

    // The only effect of this is that it initializes the table with an element section,
    // thus making a non-exported function reachable from the host.
    const result = await WebAssembly.instantiate(binary, importObject);

    wasm_exports = result.instance.exports;
    wasm_exports.main();
})();
