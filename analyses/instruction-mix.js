{
    const counts = {};

    Wasabi.analysisResult = function () {
        const keysSorted = Object.keys(counts).sort(function(a,b){return counts[b]-counts[a]});
        for (const key of keysSorted) {
            console.log(key, "\t", counts[key]);
        }
    };

    function incInstr(instr) {
        counts[instr] = (counts[instr] || 0) + 1;
    }

    Wasabi.analysis = {
        // hooks that directly correspond to one instruction
        nop() { incInstr("nop") },
        unreachable() { incInstr("unreachable") },
        if_() { incInstr("if") },
        br() { incInstr("br") },
        br_if() { incInstr("br_if") },
        br_table() { incInstr("br_table") },
        drop() { incInstr("drop") },
        select() { incInstr("select") },
        memory_size() { incInstr("memory_size") },
        memory_grow() { incInstr("memory_grow") },

        // hooks that correspond to multiple instructions -> use op argument
        unary(loc, op) { incInstr(op) },
        binary(loc, op) { incInstr(op) },
        load(loc, op) { incInstr(op) },
        store(loc, op) { incInstr(op) },
        local(loc, op) { incInstr(op) },
        global(loc, op) { incInstr(op) },

        // special cases
        call_pre(loc, func, args, tableIdx) {
            incInstr((tableIdx === undefined) ? "call" : "call_indirect");
        },
        const_() {
            // FIXME cannot determine const type, so record just as T.const
            incInstr("T.const");
        },
        begin(loc, type) {
            // if is already counted by if_ hook, function begin is implicit
            if (type !== "if" && type !== "function") incInstr(type);
        },
        return_({func, instr}) {
            // do not count implicit returns
            if (instr >= 0) incInstr("return");
        },

        // not hooked: end (not really executed anyway), start (no real instruction), call_post (already counted)
    };
}