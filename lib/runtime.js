/*
 * Wasabi loader (monkey-patches WebAssembly.instantiate()) and runtime (e.g., for resolving call_indirect).
 */

let Wasabi = {
    HOOK_NAMES: [
        "start",
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
        "call_pre",
        "call_post",
        "return_",
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
            console.warn("Wasabi: cannot resolve table index without module exports and table (possible reason: exports and table are usually not available during execution of the Wasm start function)");
            return undefined;
        }

        const resolvedFunction = Wasabi.module.table.get(tableIdx);
        if (resolvedFunction === null) {
            console.warn("Wasabi: resolving indirectly called function failed because table returned `null` at index " + tableIdx);
            return undefined;
        }

        // NOTE We want to get the _index_ of the resolved function to the analysis code, but the
        // WebAssembly API only gives us a _function object_.
        // HACK We can abuse the `.name` property of the function object to get the index.
        // See the MDN, which says the "name property is the toString() result of the function's 
        // index in the wasm module".
        // https://developer.mozilla.org/en-US/docs/WebAssembly/Exported_functions
        const resolvedFunctionIdx = parseInt(resolvedFunction.name);

        // However, because we inserted a bunch of imported hook functions into the module, the
        // index retrieved above is in terms of the _instrumented_ module. We want to get the
        // function index in the _original_ module however, so we adjust it here:
        if (resolvedFunctionIdx >= Wasabi.module.info.originalFunctionImportsCount) {
            return resolvedFunctionIdx - Object.keys(Wasabi.module.lowlevelHooks).length;
        } else {
            return resolvedFunctionIdx;
        }
    },

    // call end hooks for all "intermediate" or "implicitly ended blocks" of a branch table
    endBrTableBlocks: function(brTablesInfoIdx, brTableIdx, func) {
        const table = Wasabi.module.info.brTables[brTablesInfoIdx].table;
        const default_ = Wasabi.module.info.brTables[brTablesInfoIdx].default;
        const target = (table[brTableIdx] === undefined) ? default_ : table[brTableIdx];

        // NOTE this is a JavaScript impl of the Wasabi to_end_hook_args() function in Rust
        for (const block of target.ends) {
            const [type, begin, end, begin_if] = block;
            Wasabi.analysis.end(
                {func, instr: end},
                type,
                {func, instr: begin},
                // not undefined only for block type "else"
                (begin_if === undefined) ? undefined : {func, instr: begin_if});
        }
    },

    module: {
        // filled at instrumentation time
        // TODO flatten info into module itself, by using Object.assign in generated code
        info: undefined, lowlevelHooks: undefined,
        // filled after instantiation
        exports: undefined, table: undefined,
    },

    // filled by user or with empty hooks (as fallback) before instantiation
    analysis: {}
};

// monkey-patch WebAssembly.instantiate() and .instantiateStreaming() to add Wasabi
{
    // NOTE even though nothing is done with their arguments, we should provide them because it speeds up in Firefox
    // maybe because this way the JIT can inline the functions???
    const defaultHooks = {
        start(location) {},
        nop(location) {},
        unreachable(location) {},
        if_(location, condition) {},
        br(location, target) {},
        br_if(location, conditionalTarget, condition) {},
        br_table(location, table, defaultTarget, tableIdx) {},
        begin(location, type) {},
        end(location, type, beginLocation, ifLocation) {},
        drop(location, value) {},
        select(location, cond, first, second) {},
        call_pre(location, targetFunc, args, indirectTableIdx) {},
        call_post(location, values) {},
        return_(location, values) {},
        const_(location, op, value) {},
        unary(location, op, input, result) {},
        binary(location, op, first, second, result) {},
        load(location, op, memarg, value) {},
        store(location, op, memarg, value) {},
        memory_size(location, currentSizePages) {},
        memory_grow(location, byPages, previousSizePages) {},
        local(location, op, localIndex, value) {},
        global(location, op, globalIndex, value) {},
    }

    const assertInstantiationPrecondition = function() {
        if (Wasabi.module.info === undefined || Wasabi.module.lowlevelHooks === undefined) {
            throw "missing static info or low-level hooks, did you include the Wasabi-generated JavaScript file?";
        }
    }

    const importObjectWithHooks = function(importObject) {
        for (const hook of Wasabi.HOOK_NAMES) {
            if (Wasabi.analysis[hook] === undefined) {
                console.debug("Wasabi: hook", hook, "not provided by Wasabi.analysis, I will use an empty function as a fallback");
                Wasabi.analysis[hook] = defaultHooks[hook];
            }
        }
        let importObjectWithHooks = importObject || {};
        importObjectWithHooks.__wasabi_hooks = Wasabi.module.lowlevelHooks;
        return importObjectWithHooks;
    }

    const wireInstanceExports = function(instance) {
        Wasabi.module.exports = instance.exports;
        Wasabi.module.table = instance.exports[Wasabi.module.info.tableExportName];
    }

    const oldInstantiate = WebAssembly.instantiate;
    WebAssembly.instantiate = (sourceBuffer, importObject) => {
        assertInstantiationPrecondition();
        const result = oldInstantiate(sourceBuffer, importObjectWithHooks(importObject));
        // as soon as instance is available, save exports and table
        result.then(({module, instance}) => {
            wireInstanceExports(instance);
        });

        // FIXME Due to the added imports of __wasabi functions, host code that mutates the table
        // might insert the wrong numerical index into the table.
        // We could at least detect (and warn that this changes behavior), or fix it, by wrapping
        // the exported table in a proxy object, that adapts the inserted get/set calls accordingly.

        return result;
    };

    // just fall-back to regular instantiation since Wasabi doesn't support streaming instrumentation (yet) anyway
    const oldInstantiateStreaming = WebAssembly.instantiateStreaming;
    WebAssembly.instantiateStreaming = async (source, importObject) => {
        let response = await source;
        let buffer = await response.arrayBuffer();
        return WebAssembly.instantiate(buffer, importObject);
    };

    const oldInstance = WebAssembly.Instance;
    const newInstance = function(module, importObject) {
        assertInstantiationPrecondition();
        const instance = new oldInstance(module, importObjectWithHooks(importObject));
        wireInstanceExports(instance);
        return instance;
    };
    WebAssembly.Instance = newInstance;
}
