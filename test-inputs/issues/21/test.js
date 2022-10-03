const assert = require('assert');
const fs = require('fs');
const Wasabi = require('./out/test.wasabi.js');

let last_resolved_indirect_call_target_idx = undefined;

Wasabi.analysis.call_pre = function(location, targetFunc, args, indirectTableIdx) {
    if (indirectTableIdx !== undefined) {
        console.log('call_pre:', location, targetFunc, args, indirectTableIdx);
        last_resolved_indirect_call_target_idx = targetFunc;
    }
}

let imports = {
    env: {
        print: (...args) => {
            console.log('print:', ...args);

            // In this particular test binary, the print function is always invoked at indirect
            // calls, with the index of the target function in the original binary as the argument.
            let original_function_idx = args[0];
            if (last_resolved_indirect_call_target_idx !== original_function_idx) {
                throw Error(`Test failure: wrong resolved target function index for call_indirect, should be ${original_function_idx}, but was ${last_resolved_indirect_call_target_idx}`);
            }
        }
    }
};

(async () => {
    let { module: wasm_module, instance } = await WebAssembly.instantiate(fs.readFileSync('out/test.wasm'), imports);
    console.log('module:', wasm_module);
    console.log('instance:', instance);
    console.log('instance exports:', instance.exports);

    console.log('running main()...')
    instance.exports.main();
})();
