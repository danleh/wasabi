import Long from "./lib/long.js";

type InstructionLocation = {
    func: number,
    instr: number,
};


/* Numeric/test instructions */

// i32, f32, and f64 are represented as number, i64 as Long, results of testop's and relop's are boolean
type Val = boolean | number | Long;

type UnaryOp = "clz" | "ctz" /* ...TODO */;
type BinaryOp = "add" | "shr_sx" /* ...TODO */;
type ConversionOp = "wrap" | "extend" | "trunc" | "demote" | "promote" | "convert_u" | "convert_s" | "reinterpret";

function const_(i: InstructionLocation, value: Val) {
}

function unary(i: InstructionLocation, op: UnaryOp, input: Val, result: Val) {
}

function binary(i: InstructionLocation, op: BinaryOp, first: Val, second: Val, result: Val) {
}

function conversion(i: InstructionLocation, op: ConversionOp, before: Val, after: Val) {
}


/* Type-parametric instructions */

function drop(i: InstructionLocation, value: Val) {
}

function select(i: InstructionLocation, cond: boolean, first: Val, second: Val, result: Val) {
}


/* Variable instructions */

function local(i: InstructionLocation, op: "get" | "set" | "tee", index: number, value: Val) {
}

function global(i: InstructionLocation, op: "get" | "set", index: number, value: Val) {
}

// TODO how to inspect, e.g., the stack and/or locals of the current function inside other hooks?
// a) always give it to every hook as arguments (horribly inefficient and only possible up to a certain depth w.o. allocation)
// b) with inlining the analysis it would be possible!?

// TODO how to inspect globals from within an analysis? can we call into the "host" module? FIXME cyclic imports?


/* Memory instructions */

type MemArg = {
    offset: number,
    align: number,
}

type LoadSize = null | "8_s" | "8_u" | "16_s" | "16_u" | "32_s" | "32_u";
type StoreSize = null | 8 | 16 | 32;

function load(i: InstructionLocation, memArg: MemArg, storageSize: LoadSize, value: Value) {
}

function store(i: InstructionLocation, memArg: MemArg, storageSize: StoreSize, value: Value) {
}

function current_memory(i: InstructionLocation, currentSizePages: number) {
}

function grow_memory(i: InstructionLocation, byPages: number, previousSizePages: number) {
}


/* Control instructions */

// functions

function call(i: InstructionLocation, targetFunc: number, args: Val[]) {
}

function call_indirect(i: InstructionLocation, tableIndex: number, args: Val[]) {
}

// TODO or make the translation from tableIndex -> targetFunc in low-level JS hook and present higher-level hook, such as:
function call(i: InstructionLocation, targetFunc: number, indirect: boolean, args: Val[]) {
}

function return_(i: InstructionLocation, results: Val[]) {
}

function function_begin(func: number, args: Val[]) {
}

function function_end(func: number, results: Val[]) {
}

// blocks

// NOTE currently blocks cannot take arguments (unlike functions), but this may be lifted in the future
// see https://github.com/WebAssembly/multi-value/blob/master/proposals/multi-value/Overview.md
// then we might unify block_begin with function_begin
function block_begin(i: InstructionLocation, block: "block" | "loop" | "if" | "else") {
}

// TODO or maybe more specialized versions of these events?
function if_(i: InstructionLocation, cond: boolean) {
}

function block_end(i: InstructionLocation, block: "block" | "loop" | "if" | "else", results: Val[]) {
}

// branches

function br(i: InstructionLocation, targetLabel: number) {
}

// TODO or something higher-level?
function continue_(i: InstructionLocation, loopHead: InstructionLocation) {
}

function br_indirect(i: InstructionLocation, tableIndex: number) {
}

function br_if(i: InstructionLocation, cond: boolean, targetLabel: number) {
}


/* Static information about the functions, ... */

type FunctionInfo = {
    import?: [string, string],
    export?: string,
    type: FunctionType,
    locals: Type[]
}

type FunctionType = {
    params: Type[],
    results: Type[],
}

type Type = "i32" | "i64" | "f32" | "f64";

function func_info(func: number): FunctionInfo {
    // TODO...
    return {
        type: {params: [], results: []},
        locals: []
    };
}

// TODO do we need this?
type InstructionInfo = {};

function instr_info(i: InstructionLocation): InstructionInfo {
    // TODO
    return {}
}

// map table index to function index (for indirect calls)
function function_table(index: number): number {
    // TODO
    return 0;
}

// map branch table index to label (for indirect branches)
function branch_table(i: InstructionLocation, index: number): number {
    // TODO
    return 0;
}

/* low-level hooks, converting to "high-level" JS repr for above hooks */

function _return_i(func: number, instr: number, value: number) {
    return_({func, instr}, [value]);
}

// TODO how to return 2 values from a JavaScript function to WASM?
// for manipulating return values
function _return_I(func: number, instr: number, low: number, high: number) {
    return_({func, instr}, [new Long(low, high)]);
}