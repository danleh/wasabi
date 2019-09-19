{
    const counts = {};

    Wasabi.analysisResult = function () {
        const keysSorted = Object.keys(counts).sort(function(a,b){return counts[b]-counts[a]});
        for (const key of keysSorted) {
            console.log(key, "\t", counts[key]);
        }
    };

    function incInstr(instr) {
        counts[instr] = (counts[instr] | 0) + 1;
    }

    Wasabi.analysis = {
        // hooks that directly correspond to one instruction
        nop(loc) { incInstr("nop") },
        unreachable(loc) { incInstr("unreachable") },
        if_(loc, cond) { incInstr("if") },
        br(loc, target) { incInstr("br") },
        br_if(loc, target, cond) { incInstr("br_if") },
        br_table(loc, table, def, idx) { incInstr("br_table") },
        drop(loc, val) { incInstr("drop") },
        select(loc, fst, snd, cond) { incInstr("select") },
        memory_size(loc, val) { incInstr("memory_size") },
        memory_grow(loc, delta, old) { incInstr("memory_grow") },

        // hooks that correspond to multiple instructions -> use op argument
        unary(loc, op, input, result) { incInstr(op) },
        binary(loc, op, first, second, result) { incInstr(op) },
        load(loc, op, memarg, val) { incInstr(op) },
        store(loc, op, memarg, val) { incInstr(op) },
        local(loc, op, idx, val) { incInstr(op) },
        global(loc, op, idx, val) { incInstr(op) },

        // special cases
        call_pre(loc, func, args, tableIdx) {
            incInstr((tableIdx === undefined) ? "call" : "call_indirect");
        },
        const_(loc, op, val) {
            incInstr(op);
        },
        begin(loc, type, end) {
            // if is already counted by if_ hook, function begin is implicit
            if (type !== "if" && type !== "function") incInstr(type);
        },
        return_({func, instr}, vals) {
            // do not count implicit returns
            if (instr >= 0) incInstr("return");
        },

        // not hooked: end (not really executed anyway), start (no real instruction), call_post (already counted)
    };
}