use crate::binary::WasmBinary;
use binary_derive::WasmBinary;

use super::*;

#[derive(Debug, Clone)]
pub struct Module {
    pub sections: Vec<Section>,
}

/// Just a marker; does not save the size itself since that changes during transformations anyway.
#[derive(Debug, Clone)]
pub struct WithSize<T>(pub T);

/// Just a marker to indicate that parallel decoding/encoding is possible.
#[derive(Debug, Clone)]
pub struct Parallel<T>(pub T);

/* Sections */

#[derive(WasmBinary, Debug, Clone)]
pub enum Section {
    #[tag = 0] Custom(Vec<u8>),
    #[tag = 1] Type(WithSize<Vec<FunctionType>>),
    #[tag = 2] Import(WithSize<Vec<Import>>),
    #[tag = 3] Function(WithSize<Vec<Idx<FunctionType>>>),
    #[tag = 4] Table(WithSize<Vec<TableType>>),
    #[tag = 5] Memory(WithSize<Vec<MemoryType>>),
    #[tag = 6] Global(WithSize<Vec<Global>>),
    #[tag = 7] Export(WithSize<Vec<Export>>),
    #[tag = 8] Start(WithSize<Idx<Function>>),
    #[tag = 9] Element(WithSize<Vec<Element>>),
    #[tag = 10] Code(WithSize<Parallel<Vec<WithSize<Code>>>>),
    // Exchange with the following line to disable parallel decoding of code section (instructions).
    // #[tag = 10] Code(WithSize<Vec<WithSize<Code>>>),
    #[tag = 11] Data(WithSize<Vec<Data>>),
}

#[derive(WasmBinary, Debug, Clone)]
pub struct Global {
    pub type_: GlobalType,
    pub init: Expr,
}

#[derive(WasmBinary, Debug, Clone)]
pub struct Element {
    // always 0x00 in WASM version 1
    pub table_idx: Idx<Table>,
    pub offset: Expr,
    pub init: Vec<Idx<Function>>,
}

#[derive(WasmBinary, Debug, Clone)]
pub struct Data {
    // always 0x00 in WASM version 1
    pub memory_idx: Idx<Memory>,
    pub offset: Expr,
    pub init: Vec<u8>,
}

#[derive(WasmBinary, Debug, Clone)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub type_: ImportType,
}

#[derive(WasmBinary, Debug, Clone)]
pub struct Export {
    pub name: String,
    pub type_: ExportType,
}

#[derive(WasmBinary, Debug, Clone)]
pub enum ImportType {
    #[tag = 0x0] Function(Idx<FunctionType>),
    #[tag = 0x1] Table(TableType),
    #[tag = 0x2] Memory(MemoryType),
    #[tag = 0x3] Global(GlobalType),
}

#[derive(WasmBinary, Debug, Clone)]
pub enum ExportType {
    #[tag = 0x0] Function(Idx<Function>),
    #[tag = 0x1] Table(Idx<Table>),
    #[tag = 0x2] Memory(Idx<Memory>),
    #[tag = 0x3] Global(Idx<Global>),
}

// Markers for Idx<T>, since in low-level format Function, Table, and Memory have not one type,
// but are split over multiple sections.

#[derive(Debug, Clone)]
pub struct Function;

#[derive(Debug, Clone)]
pub struct Table;

#[derive(Debug, Clone)]
pub struct Memory;

#[derive(Debug, Clone)]
pub struct Local;


/* Code */

#[derive(WasmBinary, Debug, Clone)]
pub struct Code {
    pub locals: Vec<Locals>,
    pub body: Expr,
}

#[derive(WasmBinary, Debug, Copy, Clone)]
pub struct Locals {
    pub count: u32,
    pub type_: ValType,
}

#[derive(Debug, Clone)]
pub struct Expr(pub Vec<Instr>);

#[derive(WasmBinary, Debug, Clone)]
pub enum Instr {
    #[tag = 0x00] Unreachable,
    #[tag = 0x01] Nop,

    // NOTE block nesting is handled by Expr::decode
    #[tag = 0x02] Block(BlockType),
    #[tag = 0x03] Loop(BlockType),
    #[tag = 0x04] If(BlockType),
    #[tag = 0x05] Else,
    #[tag = 0x0b] End,

    #[tag = 0x0c] Br(Label),
    #[tag = 0x0d] BrIf(Label),
    #[tag = 0x0e] BrTable { table: Vec<Label>, default: Label },

    #[tag = 0x0f] Return,
    #[tag = 0x10] Call(Idx<Function>),
    #[tag = 0x11] CallIndirect(Idx<FunctionType>, /* unused, always 0x00 in WASM version 1 */ Idx<Table>),

    #[tag = 0x1a] Drop,
    #[tag = 0x1b] Select,

    #[tag = 0x20] LocalGet(Idx<Local>),
    #[tag = 0x21] LocalSet(Idx<Local>),
    #[tag = 0x22] LocalTee(Idx<Local>),
    #[tag = 0x23] GlobalGet(Idx<Global>),
    #[tag = 0x24] GlobalSet(Idx<Global>),

    #[tag = 0x28] I32Load(Memarg),
    #[tag = 0x29] I64Load(Memarg),
    #[tag = 0x2a] F32Load(Memarg),
    #[tag = 0x2b] F64Load(Memarg),
    #[tag = 0x2c] I32Load8S(Memarg),
    #[tag = 0x2d] I32Load8U(Memarg),
    #[tag = 0x2e] I32Load16S(Memarg),
    #[tag = 0x2f] I32Load16U(Memarg),
    #[tag = 0x30] I64Load8S(Memarg),
    #[tag = 0x31] I64Load8U(Memarg),
    #[tag = 0x32] I64Load16S(Memarg),
    #[tag = 0x33] I64Load16U(Memarg),
    #[tag = 0x34] I64Load32S(Memarg),
    #[tag = 0x35] I64Load32U(Memarg),
    #[tag = 0x36] I32Store(Memarg),
    #[tag = 0x37] I64Store(Memarg),
    #[tag = 0x38] F32Store(Memarg),
    #[tag = 0x39] F64Store(Memarg),
    #[tag = 0x3a] I32Store8(Memarg),
    #[tag = 0x3b] I32Store16(Memarg),
    #[tag = 0x3c] I64Store8(Memarg),
    #[tag = 0x3d] I64Store16(Memarg),
    #[tag = 0x3e] I64Store32(Memarg),

    #[tag = 0x3f] MemorySize(/* unused, always 0x00 in WASM version 1 */ Idx<Memory>),
    #[tag = 0x40] MemoryGrow(/* unused, always 0x00 in WASM version 1 */ Idx<Memory>),

    #[tag = 0x41] I32Const(i32),
    #[tag = 0x42] I64Const(i64),
    #[tag = 0x43] F32Const(f32),
    #[tag = 0x44] F64Const(f64),

    #[tag = 0x45] I32Eqz,
    #[tag = 0x46] I32Eq,
    #[tag = 0x47] I32Ne,
    #[tag = 0x48] I32LtS,
    #[tag = 0x49] I32LtU,
    #[tag = 0x4a] I32GtS,
    #[tag = 0x4b] I32GtU,
    #[tag = 0x4c] I32LeS,
    #[tag = 0x4d] I32LeU,
    #[tag = 0x4e] I32GeS,
    #[tag = 0x4f] I32GeU,
    #[tag = 0x50] I64Eqz,
    #[tag = 0x51] I64Eq,
    #[tag = 0x52] I64Ne,
    #[tag = 0x53] I64LtS,
    #[tag = 0x54] I64LtU,
    #[tag = 0x55] I64GtS,
    #[tag = 0x56] I64GtU,
    #[tag = 0x57] I64LeS,
    #[tag = 0x58] I64LeU,
    #[tag = 0x59] I64GeS,
    #[tag = 0x5a] I64GeU,
    #[tag = 0x5b] F32Eq,
    #[tag = 0x5c] F32Ne,
    #[tag = 0x5d] F32Lt,
    #[tag = 0x5e] F32Gt,
    #[tag = 0x5f] F32Le,
    #[tag = 0x60] F32Ge,
    #[tag = 0x61] F64Eq,
    #[tag = 0x62] F64Ne,
    #[tag = 0x63] F64Lt,
    #[tag = 0x64] F64Gt,
    #[tag = 0x65] F64Le,
    #[tag = 0x66] F64Ge,
    #[tag = 0x67] I32Clz,
    #[tag = 0x68] I32Ctz,
    #[tag = 0x69] I32Popcnt,
    #[tag = 0x6a] I32Add,
    #[tag = 0x6b] I32Sub,
    #[tag = 0x6c] I32Mul,
    #[tag = 0x6d] I32DivS,
    #[tag = 0x6e] I32DivU,
    #[tag = 0x6f] I32RemS,
    #[tag = 0x70] I32RemU,
    #[tag = 0x71] I32And,
    #[tag = 0x72] I32Or,
    #[tag = 0x73] I32Xor,
    #[tag = 0x74] I32Shl,
    #[tag = 0x75] I32ShrS,
    #[tag = 0x76] I32ShrU,
    #[tag = 0x77] I32Rotl,
    #[tag = 0x78] I32Rotr,
    #[tag = 0x79] I64Clz,
    #[tag = 0x7a] I64Ctz,
    #[tag = 0x7b] I64Popcnt,
    #[tag = 0x7c] I64Add,
    #[tag = 0x7d] I64Sub,
    #[tag = 0x7e] I64Mul,
    #[tag = 0x7f] I64DivS,
    #[tag = 0x80] I64DivU,
    #[tag = 0x81] I64RemS,
    #[tag = 0x82] I64RemU,
    #[tag = 0x83] I64And,
    #[tag = 0x84] I64Or,
    #[tag = 0x85] I64Xor,
    #[tag = 0x86] I64Shl,
    #[tag = 0x87] I64ShrS,
    #[tag = 0x88] I64ShrU,
    #[tag = 0x89] I64Rotl,
    #[tag = 0x8a] I64Rotr,
    #[tag = 0x8b] F32Abs,
    #[tag = 0x8c] F32Neg,
    #[tag = 0x8d] F32Ceil,
    #[tag = 0x8e] F32Floor,
    #[tag = 0x8f] F32Trunc,
    #[tag = 0x90] F32Nearest,
    #[tag = 0x91] F32Sqrt,
    #[tag = 0x92] F32Add,
    #[tag = 0x93] F32Sub,
    #[tag = 0x94] F32Mul,
    #[tag = 0x95] F32Div,
    #[tag = 0x96] F32Min,
    #[tag = 0x97] F32Max,
    #[tag = 0x98] F32Copysign,
    #[tag = 0x99] F64Abs,
    #[tag = 0x9a] F64Neg,
    #[tag = 0x9b] F64Ceil,
    #[tag = 0x9c] F64Floor,
    #[tag = 0x9d] F64Trunc,
    #[tag = 0x9e] F64Nearest,
    #[tag = 0x9f] F64Sqrt,
    #[tag = 0xa0] F64Add,
    #[tag = 0xa1] F64Sub,
    #[tag = 0xa2] F64Mul,
    #[tag = 0xa3] F64Div,
    #[tag = 0xa4] F64Min,
    #[tag = 0xa5] F64Max,
    #[tag = 0xa6] F64Copysign,
    #[tag = 0xa7] I32WrapI64,
    #[tag = 0xa8] I32TruncF32S,
    #[tag = 0xa9] I32TruncF32U,
    #[tag = 0xaa] I32TruncF64S,
    #[tag = 0xab] I32TruncF64U,
    #[tag = 0xac] I64ExtendI32S,
    #[tag = 0xad] I64ExtendI32U,
    #[tag = 0xae] I64TruncF32S,
    #[tag = 0xaf] I64TruncF32U,
    #[tag = 0xb0] I64TruncF64S,
    #[tag = 0xb1] I64TruncF64U,
    #[tag = 0xb2] F32ConvertI32S,
    #[tag = 0xb3] F32ConvertI32U,
    #[tag = 0xb4] F32ConvertI64S,
    #[tag = 0xb5] F32ConvertI64U,
    #[tag = 0xb6] F32DemoteF64,
    #[tag = 0xb7] F64ConvertI32S,
    #[tag = 0xb8] F64ConvertI32U,
    #[tag = 0xb9] F64ConvertI64S,
    #[tag = 0xba] F64ConvertI64U,
    #[tag = 0xbb] F64PromoteF32,
    #[tag = 0xbc] I32ReinterpretF32,
    #[tag = 0xbd] I64ReinterpretF64,
    #[tag = 0xbe] F32ReinterpretI32,
    #[tag = 0xbf] F64ReinterpretI64,
}