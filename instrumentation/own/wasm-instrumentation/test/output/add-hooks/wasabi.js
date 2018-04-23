/*
 * Wasabi loader (monkey-patches WebAssembly.instantiate()) and runtime (e.g., for resolving call_indirect).
 */

let resolveTableIdx = function (i) {
    throw "internal error! the monkey-patched WebAssembly.instantiate() should have replaced this function with WebAssembly.Instance.Table.get(i)"
};

const oldInstantiate = WebAssembly.instantiate;
WebAssembly.instantiate = function () {
    console.log(arguments);

    let importsObject = arguments[1] || {};
    importsObject.hooks = lowlevelHooks;

    arguments[1] = importsObject;
    const result = oldInstantiate.apply(this, arguments);
    // as soon as instance is available, use its exported table to resolve indirect calls
    result.then(result => {
        // window.wasm_res = result;
        resolveTableIdx = tableIndex => {
            return tableIndex;
            // FIXME this is not correct: the name of the "Exotic Function Object" is the "function address", not
            // necessarily the function index in the WASM module:
            // documentation here is wrong: https://developer.mozilla.org/en-US/docs/WebAssembly/Exported_functions
            // see https://www.w3.org/TR/wasm-js-api-1/#exported-function-exotic-objects
            // and http://webassembly.org/docs/js/#exported-function-exotic-objects
            return parseInt(result.instance.exports[staticInfo.table_export_name].get(tableIndex).name, 10);
        }
    });
    return result;
};

// provide analysis callback stubs if they were not defined by the user
const analysisCallbacks = [
    "if_", "br", "br_if", "br_table", "begin", "end", "nop", "unreachable", "drop", "select", "call_", "return_", "call_result_", "const_", "unary", "binary", "load", "store", "current_memory", "grow_memory", "local", "global"
];
for (const callback of analysisCallbacks) {
    if (window[callback] === undefined) {
        window[callback] = function () {
        };
    }
}