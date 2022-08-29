const fs = require('fs');

(async () => {
    const binary = fs.readFileSync('./main.wasm');
    const importObject = {
        'host': {
            'print': (arg) => {
                console.log(arg);
            },
        },
    };
    const result = await WebAssembly.instantiate(binary, importObject);
    // Change the function entry in the table.
    result.instance.exports.table.set(0, result.instance.exports.export2);
    result.instance.exports.main();
})();
