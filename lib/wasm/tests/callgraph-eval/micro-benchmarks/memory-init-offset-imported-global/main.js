const fs = require('fs');

(async () => {
    const binary = fs.readFileSync('./main.wasm');
    const importObject = {
        'host': {
            'print': (arg) => {
                console.log(arg);
            },
            // Changes at which place the elements will be placed.
            'data_offset': 1337,
        },
    };
    const result = await WebAssembly.instantiate(binary, importObject);
    result.instance.exports.main();
})();
