/*
 * Wasabi loader (monkey-patches WebAssembly.instantiate()) and runtime (e.g., for resolving call_indirect).
 */

let Wasabi = {
    HOOK_NAMES: [
        "if_",
        "br",
        "br_if",
        "br_table",
        "begin",
        "end",
        "nop",
        "unreachable",
        "drop",
        "select",
        "call_",
        "return_",
        "call_result_",
        "const_",
        "unary",
        "binary",
        "load",
        "store",
        "memory_size",
        "memory_grow",
        "local",
        "global"
    ],

    // map a table index to a function index
    resolveTableIdx: function (tableIdx) {
        if (Wasabi.module.exports === undefined || Wasabi.module.table === undefined) {
            console.warn("cannot resolve table index without exports and table (possible reason: exports and table are not available during Wasm start function)");
            return undefined;
        }

        // TODO implement
        return 0;
    },

    // add "info" and "lowlevelHooks" at instrumentation time, add "exports" and "table" after instantiation
    module: {},
};

// provide analysis callback stubs if they were not defined by the user
for (const hook of Wasabi.HOOK_NAMES) {
    if (window[hook] === undefined) {
        window[hook] = function () {
        };
    }
}

// monkey-patch WebAssembly functions to add Wasabi
{
    const oldInstantiate = WebAssembly.instantiate;
    WebAssembly.instantiate = function () {
        if (Wasabi.module.lowlevelHooks === undefined) {
            throw "missing low-level hooks, should be generated beforehand at instrumentation time";
        }

        let importsObject = arguments[1] || {};
        importsObject.wasabi_hooks = Wasabi.module.lowlevelHooks;
        arguments[1] = importsObject;

        const result = oldInstantiate.apply(this, arguments);
        // as soon as instance is available, save exports and table
        result.then(({module, instance}) => {
            Wasabi.module.exports = instance.exports;
            Wasabi.module.table = instance.exports[Wasabi.module.info.tableExportName];
        });
        return result;
    };

    // TODO patch instantiateStreaming
    // TODO patch WebAssembly.Instance to throw error (Chrome does not support it >4KB modules anyway, so we cannot execute Wasabi synchronously)
}