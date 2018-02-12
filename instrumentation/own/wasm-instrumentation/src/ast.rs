use leb128::Leb128;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use WasmBinary;

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub version: u32,
    // the number of sections is not encoded
    pub sections: ::std::vec::Vec<Section>,
}


/* Generic "AST combinators" */

// redefine Vec and String so that their length is encoded/decoded as LEB128 by default,
// use ::std::vec::Vec<T> / ::std::string::String for unaltered behavior
type Vec<T> = Leb128<::std::vec::Vec<T>>;
type String = Leb128<::std::string::String>;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct WithSize<T> {
    /// Do not save the size of the contents, since it might change through AST transformations anyway.
    /// But DO save the byte_count that was used to save the size (in Leb128), so that decoding and
    /// encoding the size will round-trip.
    pub size: Leb128<()>,
    pub content: T,
}

/// convenience
impl<T> From<T> for WithSize<T> {
    fn from(content: T) -> Self {
        WithSize {
            size: ().into(),
            content,
        }
    }
}

/// convenience
impl<T> From<T> for WithSize<Leb128<T>> {
    fn from(content: T) -> Self {
        WithSize {
            size: ().into(),
            content: content.into(),
        }
    }
}

impl<T: Serialize> Serialize for WithSize<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        self.content.serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for WithSize<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        Ok(T::deserialize(deserializer)?.into())
    }
}


/* Sections */

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
// FIXME manual impl of PartialOrt where Custom < any == None, so that own custom sections are always appended to the end
pub enum Section {
    // untested
    #[tag = 0] Custom(Vec<u8>),
    #[tag = 1] Type(WithSize<Vec<FuncType>>),
    #[tag = 2] Import(WithSize<Vec<Import>>),
    #[tag = 3] Function(WithSize<Vec<TypeIdx>>),
    // untested
    #[tag = 4] Table(WithSize<Vec<TableType>>),
    // untested
    #[tag = 5] Memory(WithSize<Vec<Limits>>),
    #[tag = 6] Global(WithSize<Vec<Global>>),
    #[tag = 7] Export(WithSize<Vec<Export>>),
    #[tag = 8] Start(WithSize<FuncIdx>),
    #[tag = 9] Element(WithSize<Vec<Element>>),
    #[tag = 10] Code(WithSize<Vec<WithSize<Func>>>),
    // to benchmark how much faster it gets without instruction decoding
//    #[tag = 10] Code(WithSize<Vec<Vec<u8>>>),
    #[tag = 11] Data(WithSize<Vec<Data>>),
}

//impl PartialOrd for Section {
//    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
//        match
//    }
//}

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Global {
    pub type_: GlobalType,
    pub init: Expr,
}

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Element {
    // always 0x00 in WASM version 1
    pub _unused: TableIdx,
    pub offset: Expr,
    pub init: Vec<FuncIdx>,
}

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Data {
    // always 0x00 in WASM version 1
    pub _unused: MemoryIdx,
    pub offset: Expr,
    pub init: Vec<u8>,
}

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub type_: ImportType,
}

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Export {
    pub name: String,
    pub type_: ExportType,
}


/* Types */

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum ValType {
    #[tag = 0x7f] I32,
    #[tag = 0x7e] I64,
    #[tag = 0x7d] F32,
    #[tag = 0x7c] F64,
}

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[tag = 0x60]
pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct BlockType(pub Option<ValType>);

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Limits {
    #[tag = 0x00] Min(Leb128<u32>),
    #[tag = 0x01] MinMax(Leb128<u32>, Leb128<u32>),
}

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
#[tag = 0x70]
pub struct TableType(pub Limits);

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct GlobalType(pub ValType, pub Mut);

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Mut {
    #[tag = 0x00] Const,
    #[tag = 0x01] Var,
}

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum ImportType {
    #[tag = 0x0] Function(TypeIdx),
    #[tag = 0x1] Table(TableType),
    #[tag = 0x2] Memory(Limits),
    #[tag = 0x3] Global(GlobalType),
}

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum ExportType {
    #[tag = 0x0] Function(TypeIdx),
    #[tag = 0x1] Table(TableIdx),
    #[tag = 0x2] Memory(MemoryIdx),
    #[tag = 0x3] Global(GlobalIdx),
}


/* Indices */

#[derive(WasmBinary, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TypeIdx(pub Leb128<u32>);

#[derive(WasmBinary, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct FuncIdx(pub Leb128<u32>);

#[derive(WasmBinary, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TableIdx(pub Leb128<u32>);

#[derive(WasmBinary, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MemoryIdx(pub Leb128<u32>);

#[derive(WasmBinary, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GlobalIdx(pub Leb128<u32>);

#[derive(WasmBinary, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LocalIdx(pub Leb128<u32>);

#[derive(WasmBinary, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LabelIdx(pub Leb128<u32>);


/* Code */

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Func(pub Vec<Locals>, pub Expr);

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Locals {
    pub count: Leb128<u32>,
    pub type_: ValType,
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
// the number of instructions is not encoded
pub struct Expr(pub ::std::vec::Vec<Instr>);

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Instr {
    #[tag = 0x00] Unreachable,
    #[tag = 0x01] Nop,

    // NOTE block nesting is handled by Expr::decode which returns as soon as an Else or End is found
    #[tag = 0x02] Block(BlockType, Expr),
    #[tag = 0x03] Loop(BlockType, Expr),
    #[tag = 0x04] If(BlockType, Expr),
    #[tag = 0x05] Else(Expr),
    #[tag = 0x0b] End,

    #[tag = 0x0c] Br(LabelIdx),
    #[tag = 0x0d] BrIf(LabelIdx),
    #[tag = 0x0e] BrTable(Vec<LabelIdx>, LabelIdx),

    #[tag = 0x0f] Return,
    #[tag = 0x10] Call(FuncIdx),
    #[tag = 0x11] CallIndirect(TypeIdx, /* unused, always 0x00 in WASM version 1 */ TableIdx),

    #[tag = 0x1a] Drop,
    #[tag = 0x1b] Select,

    #[tag = 0x20] GetLocal(LocalIdx),
    #[tag = 0x21] SetLocal(LocalIdx),
    #[tag = 0x22] TeeLocal(LocalIdx),
    #[tag = 0x23] GetGlobal(GlobalIdx),
    #[tag = 0x24] SetGlobal(GlobalIdx),

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
    #[tag = 0x3f] CurrentMemory(/* unused, always 0x00 in WASM version 1 */ MemoryIdx),
    #[tag = 0x40] GrowMemory(/* unused, always 0x00 in WASM version 1 */ MemoryIdx),

    #[tag = 0x41] I32Const(Leb128<i32>),
    #[tag = 0x42] I64Const(Leb128<i64>),
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
    #[tag = 0xa8] I32TruncSF32,
    #[tag = 0xa9] I32TruncUF32,
    #[tag = 0xaa] I32TruncSF64,
    #[tag = 0xab] I32TruncUF64,
    #[tag = 0xac] I64ExtendSI32,
    #[tag = 0xad] I64ExtendUI32,
    #[tag = 0xae] I64TruncSF32,
    #[tag = 0xaf] I64TruncUF32,
    #[tag = 0xb0] I64TruncSF64,
    #[tag = 0xb1] I64TruncUF64,
    #[tag = 0xb2] F32ConvertSI32,
    #[tag = 0xb3] F32ConvertUI32,
    #[tag = 0xb4] F32ConvertSI64,
    #[tag = 0xb5] F32ConvertUI64,
    #[tag = 0xb6] F32DemoteF64,
    #[tag = 0xb7] F64ConvertSI32,
    #[tag = 0xb8] F64ConvertUI32,
    #[tag = 0xb9] F64ConvertSI64,
    #[tag = 0xba] F64ConvertUI64,
    #[tag = 0xbb] F64PromoteF32,
    #[tag = 0xbc] I32ReinterpretF32,
    #[tag = 0xbd] I64ReinterpretF64,
    #[tag = 0xbe] F32ReinterpretI32,
    #[tag = 0xbf] F64ReinterpretI64,
}

#[derive(WasmBinary, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Memarg {
    pub alignment: Leb128<u32>,
    pub offset: Leb128<u32>,
}
