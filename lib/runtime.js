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
            console.warn("cannot resolve table index without exports and table (possible reason: exports and table are not available during Wasm start function)");
            return undefined;
        }

        // FIXME even though MDN says "name property is the toString() result of the function's index in the wasm module"
        // Firefox seems to give out different names :/ -> bug report, either documentation or implementation is wrong
        // see https://developer.mozilla.org/en-US/docs/WebAssembly/Exported_functions

        // dirty HACK subtract the "name index" of the first function, should correct for the wrong property
        // const firstFunctionIdx = parseInt(Wasabi.module.exports[Wasabi.module.info.firstFunctionExportName].name);
        // const functionIdx = parseInt(Wasabi.module.table.get(tableIdx).name) - firstFunctionIdx;
        const functionIdx = parseInt(Wasabi.module.table.get(tableIdx).name);

        return (functionIdx >= Wasabi.module.info.functions.length) ? 0 : functionIdx;
    },

    // call end hooks for all "intermediate" or "implicitly ended blocks" of a branch table
    endBrTableBlocks: function(brTablesInfoIdx, brTableIdx) {
        const table = Wasabi.module.info.brTables[brTablesInfoIdx].table;
        const default_ = Wasabi.module.info.brTables[brTablesInfoIdx].default;
        const target = (table[brTableIdx] === undefined) ? default_ : table[brTableIdx];

        // call end hooks at runtime for all "implicitly ended blocks" during the branch
        for (const args of target.endArgs) {
            // begin_if is undefined for every block type EXCEPT "else"
            const [end, type, begin, begin_if] = args;
            Wasabi.analysis.end(end, type, begin, begin_if);
        }
    },

    loc2func: function(loc) {
        // TODO
    },

    loc2instr: function(loc) {
        // TODO
    },

    functionType: function(func) {
        // TODO
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
        const_(location, value) {},
        unary(location, op, input, result) {},
        binary(location, op, first, second, result) {},
        load(location, op, memarg, value) {},
        store(location, op, memarg, value) {},
        memory_size(location, currentSizePages) {},
        memory_grow(location, byPages, previousSizePages) {},
        local(location, op, localIndex, value) {},
        global(location, op, globalIndex, value) {},
    }

    const oldInstantiate = WebAssembly.instantiate;
    WebAssembly.instantiate = (sourceBuffer, importObject) => {
        if (Wasabi.module.info === undefined || Wasabi.module.lowlevelHooks === undefined) {
            throw "missing static info or low-level hooks, did you include the Wasabi-generated JavaScript file?";
        }

        for (const hook of Wasabi.HOOK_NAMES) {
            if (Wasabi.analysis[hook] === undefined) {
                console.debug(hook, "hook not provided by Wasabi.analysis, add empty function as fallback");
                Wasabi.analysis[hook] = defaultHooks[hook];
            }
        }

        let importObjectWithHooks = importObject || {};
        importObjectWithHooks.__wasabi_hooks = Wasabi.module.lowlevelHooks;

        const result = oldInstantiate(sourceBuffer, importObjectWithHooks);
        // as soon as instance is available, save exports and table
        result.then(({module, instance}) => {
            Wasabi.module.exports = instance.exports;
            Wasabi.module.table = instance.exports[Wasabi.module.info.tableExportName];
        });
        return result;
    };

    // just fall-back to regular instantiation since Wasabi doesn't support streaming instrumentation (yet) anyway
    const oldInstantiateStreaming = WebAssembly.instantiateStreaming;
    WebAssembly.instantiateStreaming = async (source, importObject) => {
        let response = await source;
        let buffer = await response.arrayBuffer();
        return WebAssembly.instantiate(buffer, importObject);
    };

    WebAssembly.Instance = () => {
        throw "synchronous WebAssembly instantiation is not supported by Wasabi (Chrome does not support it for >4KB modules anyway, so we cannot execute Wasabi itself synchronously)"
    };
}