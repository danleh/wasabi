const fs = require('fs');

(async () => {
    const binary = fs.readFileSync('./main.wasm');
    const importObject = {
        'host': {
            'print': (arg) => {
                console.log(arg);
            },
            // Changes at which place the elements will be placed.
            'element_offset': 1,
        },
    };
    const result = await WebAssembly.instantiate(binary, importObject);
    result.instance.exports.main();
})();
