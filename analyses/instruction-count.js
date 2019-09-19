// TODO rename to instruction-mix or so, counting sounds like just counting the total number of instructions

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
    nop(location) {
        incInstr("nop");
    },

    unreachable(location) {
        incInstr("unreachable");
    },

    if_(location, condition) {
        incInstr("if");
    },

    br(location, target) {
        incInstr("br");
    },

    br_if(location, conditionalTarget, condition) {
        incInstr("br_if");
    },

    br_table(location, table, defaultTarget, tableIdx) {
        incInstr("br_table");
    },

    begin(location, type) {
        // if and else are already counted by if_ hook
        if (type === "block" || type === "loop")
            incInstr(type);
    },

    drop(location, value) {
        incInstr("drop");
    },

    select(location, cond, first, second) {
        incInstr("select");
    },

    call_pre(location, targetFunc, args, indirectTableIdx) {
        incInstr((indirectTableIdx === undefined) ? "call" : "call_indirect");
    },

    return_(location, values) {
        incInstr("return");
    },

    const_(location, op, value) {
        incInstr(op);
    },

    unary(location, op, input, result) {
        incInstr(op);
    },

    binary(location, op, first, second, result) {
        incInstr(op);
    },

    load(location, op, memarg, value) {
        incInstr(op);
    },

    store(location, op, memarg, value) {
        incInstr(op);
    },

    memory_size(location, currentSizePages) {
        incInstr("memory_size");
    },

    memory_grow(location, byPages, previousSizePages) {
        incInstr("memory_grow");
    },

    local(location, op, localIndex, value) {
        incInstr(op);
    },

    global(location, op, globalIndex, value) {
        incInstr(op);
    },
};