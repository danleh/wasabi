use std::collections::HashSet;
use std::fmt;

use crate::{BlockType, FunctionType, GlobalType, Idx, Label, Memarg, MemoryType, Mutability, RawCustomSection, TableType, Val, ValType};

/* High-level AST:
    - Types are inlined instead of referenced by type idx (i.e., no manual handling of Type "pool")
    - Function + Code sections are merged into one list of functions,
      same for tables: Table + Element sections and memories: Memory + Data sections.
    - Imports and exports are part of the respective item, not stored externally and referring to
      their item by index.
    - Similar instructions are grouped together, for easier uniform handling, e.g., T.const
      instructions, loads, stores, and numeric instructions.
    - Debug names (from the name custom section) are attached to their respective items (module,
      functions, and locals).
*/

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Module {
    // From the name section, if present, e.g., compiler-generated debug info.
    pub name: Option<String>,

    pub functions: Vec<Function>,
    pub globals: Vec<Global>,
    pub tables: Vec<Table>,
    pub memories: Vec<Memory>,

    pub start: Option<Idx<Function>>,

    pub custom_sections: Vec<RawCustomSection>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ImportOrPresent<T> {
    Import(String, String),
    Present(T),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Function {
    // Type is inlined here compared to low-level/binary/spec representation.
    pub type_: FunctionType,
    pub code: ImportOrPresent<Code>,
    // Functions/globals/memories/tables can be exported multiple times under different names.
    // But the export names must be unique (not ensured in this representation!).
    pub export: Vec<String>,
    // From the name section, if present, e.g., compiler-generated debug info.
    pub name: Option<String>,
    // Invariant: param_names.len() == type_.params.len(), i.e., one optional name per type.
    // TODO Since you cannot access param_names mutably without checking the invariant before,
    // it is currently impossible to change the number of function parameters without breaking the
    // invariant.
    // However, so far it was never necessary to change the type signature of an existing function.
    param_names: Vec<Option<String>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Global {
    pub type_: GlobalType,
    pub init: ImportOrPresent<Expr>,
    pub export: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Table {
    pub type_: TableType,
    // Unlike functions and globals, an imported table can still be initialized with elements.
    pub import: Option<(String, String)>,
    pub elements: Vec<Element>,
    pub export: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Memory {
    pub type_: MemoryType,
    // Unlike functions and globals, an imported memory can still be initialized with data elements.
    pub import: Option<(String, String)>,
    pub data: Vec<Data>,
    pub export: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Code {
    pub locals: Vec<Local>,
    pub body: Expr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Local {
    pub type_: ValType,
    // From the name section, if present, e.g., compiler-generated debug info.
    pub name: Option<String>,
}

// For function parameters, the name and type are spread over the Function.type_ and
// Function.param_names fields, so we cannot hand out a single reference like for &Local.
// Instead we have two reference types (one mutable, one not) for the (non-explicit) Param that
// encapsulate the two separate references to type_ and name.
// As a common "supertype" to both Param and Local (mutable) references, we also have
// ParamOrLocalRef/ParamOrLocalMut with type and name accessor functions.

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ParamRef<'a> {
    // ValType is a Copy-type and smaller than a pointer, so store as value instead of reference.
    pub type_: ValType,
    pub name: Option<&'a str>,
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ParamMut<'a> {
    pub type_: &'a mut ValType,
    pub name: &'a mut Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ParamOrLocalRef<'a> {
    Param(ParamRef<'a>),
    Local(&'a Local),
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ParamOrLocalMut<'a> {
    Param(ParamMut<'a>),
    Local(&'a mut Local),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Element {
    pub offset: Expr,
    pub functions: Vec<Idx<Function>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Data {
    pub offset: Expr,
    pub bytes: Vec<u8>,
}

pub type Expr = Vec<Instr>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Instr {
    Unreachable,
    Nop,

    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    Else,
    End,

    Br(Label),
    BrIf(Label),
    BrTable { table: Vec<Label>, default: Label },

    Return,
    Call(Idx<Function>),
    CallIndirect(FunctionType, Idx<Table>),

    Drop,
    Select,

    Local(LocalOp, Idx<Local>),
    Global(GlobalOp, Idx<Global>),

    Load(LoadOp, Memarg),
    Store(StoreOp, Memarg),

    MemorySize(Idx<Memory>),
    MemoryGrow(Idx<Memory>),

    Const(Val),
    Numeric(NumericOp),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum LocalOp { Get, Set, Tee }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GlobalOp { Get, Set }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum LoadOp {
    I32Load,
    I64Load,
    F32Load,
    F64Load,

    I32Load8S,
    I32Load8U,
    I32Load16S,
    I32Load16U,

    I64Load8S,
    I64Load8U,
    I64Load16S,
    I64Load16U,
    I64Load32S,
    I64Load32U,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum StoreOp {
    I32Store,
    I64Store,
    F32Store,
    F64Store,

    I32Store8,
    I32Store16,

    I64Store8,
    I64Store16,
    I64Store32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum NumericOp {
    /* Unary */
    I32Eqz,
    I64Eqz,

    I32Clz,
    I32Ctz,
    I32Popcnt,

    I64Clz,
    I64Ctz,
    I64Popcnt,

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,

    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,

    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,

    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,

    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,

    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,

    /* Binary */
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,

    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
}


/* Type information for each instruction */

impl LocalOp {
    pub fn to_type(self, local_ty: ValType) -> FunctionType {
        match self {
            LocalOp::Get => FunctionType::new(&[], &[local_ty]),
            LocalOp::Set => FunctionType::new(&[local_ty], &[]),
            LocalOp::Tee => FunctionType::new(&[local_ty], &[local_ty]),
        }
    }
}

impl GlobalOp {
    pub fn to_type(self, global_ty: ValType) -> FunctionType {
        match self {
            GlobalOp::Get => FunctionType::new(&[], &[global_ty]),
            GlobalOp::Set => FunctionType::new(&[global_ty], &[]),
        }
    }
}

impl NumericOp {
    pub fn to_type(self) -> FunctionType {
        use NumericOp::*;
        use ValType::*;
        match self {
            /* Unary */

            I32Eqz => FunctionType::new(&[I32], &[I32]),
            I64Eqz => FunctionType::new(&[I64], &[I32]),

            I32Clz | I32Ctz | I32Popcnt => FunctionType::new(&[I32], &[I32]),
            I64Clz | I64Ctz | I64Popcnt => FunctionType::new(&[I64], &[I64]),

            F32Abs | F32Neg | F32Ceil | F32Floor | F32Trunc | F32Nearest | F32Sqrt => FunctionType::new(&[F32], &[F32]),
            F64Abs | F64Neg | F64Ceil | F64Floor | F64Trunc | F64Nearest | F64Sqrt => FunctionType::new(&[F64], &[F64]),

            // conversions
            I32WrapI64 => FunctionType::new(&[I64], &[I32]),
            I32TruncF32S | I32TruncF32U => FunctionType::new(&[F32], &[I32]),
            I32TruncF64S | I32TruncF64U => FunctionType::new(&[F64], &[I32]),
            I64ExtendI32S | I64ExtendI32U => FunctionType::new(&[I32], &[I64]),
            I64TruncF32S | I64TruncF32U => FunctionType::new(&[F32], &[I64]),
            I64TruncF64S | I64TruncF64U => FunctionType::new(&[F64], &[I64]),
            F32ConvertI32S | F32ConvertI32U => FunctionType::new(&[I32], &[F32]),
            F32ConvertI64S | F32ConvertI64U => FunctionType::new(&[I64], &[F32]),
            F32DemoteF64 => FunctionType::new(&[F64], &[F32]),
            F64ConvertI32S | F64ConvertI32U => FunctionType::new(&[I32], &[F64]),
            F64ConvertI64S | F64ConvertI64U => FunctionType::new(&[I64], &[F64]),
            F64PromoteF32 => FunctionType::new(&[F32], &[F64]),
            I32ReinterpretF32 => FunctionType::new(&[F32], &[I32]),
            I64ReinterpretF64 => FunctionType::new(&[F64], &[I64]),
            F32ReinterpretI32 => FunctionType::new(&[I32], &[F32]),
            F64ReinterpretI64 => FunctionType::new(&[I64], &[F64]),

            /* Binary */

            I32Eq | I32Ne | I32LtS | I32LtU | I32GtS | I32GtU | I32LeS | I32LeU | I32GeS | I32GeU => FunctionType::new(&[I32, I32], &[I32]),
            I64Eq | I64Ne | I64LtS | I64LtU | I64GtS | I64GtU | I64LeS | I64LeU | I64GeS | I64GeU => FunctionType::new(&[I64, I64], &[I32]),

            F32Eq | F32Ne | F32Lt | F32Gt | F32Le | F32Ge => FunctionType::new(&[F32, F32], &[I32]),
            F64Eq | F64Ne | F64Lt | F64Gt | F64Le | F64Ge => FunctionType::new(&[F64, F64], &[I32]),

            I32Add | I32Sub | I32Mul | I32DivS | I32DivU | I32RemS | I32RemU | I32And | I32Or | I32Xor | I32Shl | I32ShrS | I32ShrU | I32Rotl | I32Rotr => FunctionType::new(&[I32, I32], &[I32]),
            I64Add | I64Sub | I64Mul | I64DivS | I64DivU | I64RemS | I64RemU | I64And | I64Or | I64Xor | I64Shl | I64ShrS | I64ShrU | I64Rotl | I64Rotr => FunctionType::new(&[I64, I64], &[I64]),
            F32Add | F32Sub | F32Mul | F32Div | F32Min | F32Max | F32Copysign => FunctionType::new(&[F32, F32], &[F32]),
            F64Add | F64Sub | F64Mul | F64Div | F64Min | F64Max | F64Copysign => FunctionType::new(&[F64, F64], &[F64]),
        }
    }
}

pub trait MemoryOp : Sized {
    fn to_type(self) -> FunctionType;

    // See comments on Memarg type for more information on the alignment hint and natural alignment.
    fn natural_alignment_exp(self) -> u8;
    fn natural_alignment(self) -> u32 {
        2u32.pow(self.natural_alignment_exp() as u32)
    }
}

impl MemoryOp for LoadOp {
    fn to_type(self) -> FunctionType {
        use LoadOp::*;
        use ValType::*;
        match self {
            I32Load => FunctionType::new(&[I32], &[I32]),
            I64Load => FunctionType::new(&[I32], &[I64]),
            F32Load => FunctionType::new(&[I32], &[F32]),
            F64Load => FunctionType::new(&[I32], &[F64]),

            I32Load8S => FunctionType::new(&[I32], &[I32]),
            I32Load8U => FunctionType::new(&[I32], &[I32]),
            I32Load16S => FunctionType::new(&[I32], &[I32]),
            I32Load16U => FunctionType::new(&[I32], &[I32]),
            I64Load8S => FunctionType::new(&[I32], &[I64]),
            I64Load8U => FunctionType::new(&[I32], &[I64]),
            I64Load16S => FunctionType::new(&[I32], &[I64]),
            I64Load16U => FunctionType::new(&[I32], &[I64]),
            I64Load32S => FunctionType::new(&[I32], &[I64]),
            I64Load32U => FunctionType::new(&[I32], &[I64]),
        }
    }

    fn natural_alignment_exp(self) -> u8 {
        use LoadOp::*;
        match self {
            I32Load => 2,
            I64Load => 3,
            F32Load => 2,
            F64Load => 3,

            I32Load8S => 0,
            I32Load8U => 0,
            I32Load16S => 1,
            I32Load16U => 1,
            I64Load8S => 0,
            I64Load8U => 0,
            I64Load16S => 1,
            I64Load16U => 1,
            I64Load32S => 2,
            I64Load32U => 2,
        }
    }
}

impl MemoryOp for StoreOp {
    fn to_type(self) -> FunctionType {
        use StoreOp::*;
        use ValType::*;
        match self {
            I32Store => FunctionType::new(&[I32, I32], &[]),
            I64Store => FunctionType::new(&[I32, I64], &[]),
            F32Store => FunctionType::new(&[I32, F32], &[]),
            F64Store => FunctionType::new(&[I32, F64], &[]),

            I32Store8 => FunctionType::new(&[I32, I32], &[]),
            I32Store16 => FunctionType::new(&[I32, I32], &[]),
            I64Store8 => FunctionType::new(&[I32, I64], &[]),
            I64Store16 => FunctionType::new(&[I32, I64], &[]),
            I64Store32 => FunctionType::new(&[I32, I64], &[]),
        }
    }

    fn natural_alignment_exp(self) -> u8 {
        use StoreOp::*;
        match self {
            I32Store => 2,
            I64Store => 3,
            F32Store => 2,
            F64Store => 3,

            I32Store8 => 0,
            I32Store16 => 1,
            I64Store8 => 0,
            I64Store16 => 1,
            I64Store32 => 2,
        }
    }
}

impl Instr {
    /// for all where the type can be determined by just looking at the instruction, not additional
    /// information like the function or module etc.
    pub fn to_type(&self) -> Option<FunctionType> {
        use Instr::*;
        use ValType::*;
        match *self {
            Unreachable | Nop => Some(FunctionType::new(&[], &[])),
            Load(ref op, _) => Some(op.to_type()),
            Store(ref op, _) => Some(op.to_type()),
            MemorySize(_) => Some(FunctionType::new(&[], &[I32])),
            MemoryGrow(_) => Some(FunctionType::new(&[I32], &[I32])),
            Const(ref val) => Some(FunctionType::new(&[], &[val.to_type()])),
            Numeric(ref op) => Some(op.to_type()),
            CallIndirect(ref func_ty, _) => Some(FunctionType::new(&[&func_ty.params[..], &[I32]].concat(), &func_ty.results)),

            // nesting...
            Block(_) | Loop(_) | If(_) | Else | End => None,
            // depends on branch target?
            Br(_) | BrIf(_) | BrTable { .. } => None,
            // need to inspect function type
            Return | Call(_) => None,
            // need abstract type stack "evaluation"
            Drop | Select => None,
            // need lookup in locals/globals
            Local(_, _) | Global(_, _) => None,
        }
    }

    /// returns instruction name as in Wasm spec
    pub fn to_name(&self) -> &'static str {
        use Instr::*;
        use NumericOp::*;
        use LoadOp::*;
        use StoreOp::*;
        match *self {
            Unreachable => "unreachable",
            Nop => "nop",
            Block(_) => "block",
            Loop(_) => "loop",
            If(_) => "if",
            Else => "else",
            End => "end",
            Br(_) => "br",
            BrIf(_) => "br_if",
            BrTable { .. } => "br_table",
            Return => "return",
            Call(_) => "call",
            CallIndirect(_, _) => "call_indirect",
            Drop => "drop",
            Select => "select",
            Local(LocalOp::Get, _) => "local.get",
            Local(LocalOp::Set, _) => "local.set",
            Local(LocalOp::Tee, _) => "local.tee",
            Global(GlobalOp::Get, _) => "global.get",
            Global(GlobalOp::Set, _) => "global.set",
            MemorySize(_) => "memory.size",
            MemoryGrow(_) => "memory.grow",
            Const(Val::I32(_)) => "i32.const",
            Const(Val::I64(_)) => "i64.const",
            Const(Val::F32(_)) => "f32.const",
            Const(Val::F64(_)) => "f64.const",
            Load(I32Load, _) => "i32.load",
            Load(I64Load, _) => "i64.load",
            Load(F32Load, _) => "f32.load",
            Load(F64Load, _) => "f64.load",
            Load(I32Load8S, _) => "i32.load8_s",
            Load(I32Load8U, _) => "i32.load8_u",
            Load(I32Load16S, _) => "i32.load16_s",
            Load(I32Load16U, _) => "i32.load16_u",
            Load(I64Load8S, _) => "i64.load8_s",
            Load(I64Load8U, _) => "i64.load8_u",
            Load(I64Load16S, _) => "i64.load16_s",
            Load(I64Load16U, _) => "i64.load16_u",
            Load(I64Load32S, _) => "i64.load32_s",
            Load(I64Load32U, _) => "i64.load32_u",
            Store(I32Store, _) => "i32.store",
            Store(I64Store, _) => "i64.store",
            Store(F32Store, _) => "f32.store",
            Store(F64Store, _) => "f64.store",
            Store(I32Store8, _) => "i32.store8",
            Store(I32Store16, _) => "i32.store16",
            Store(I64Store8, _) => "i64.store8",
            Store(I64Store16, _) => "i64.store16",
            Store(I64Store32, _) => "i64.store32",
            Numeric(I32Eqz) => "i32.eqz",
            Numeric(I64Eqz) => "i64.eqz",
            Numeric(I32Clz) => "i32.clz",
            Numeric(I32Ctz) => "i32.ctz",
            Numeric(I32Popcnt) => "i32.popcnt",
            Numeric(I64Clz) => "i64.clz",
            Numeric(I64Ctz) => "i64.ctz",
            Numeric(I64Popcnt) => "i64.popcnt",
            Numeric(F32Abs) => "f32.abs",
            Numeric(F32Neg) => "f32.neg",
            Numeric(F32Ceil) => "f32.ceil",
            Numeric(F32Floor) => "f32.floor",
            Numeric(F32Trunc) => "f32.trunc",
            Numeric(F32Nearest) => "f32.nearest",
            Numeric(F32Sqrt) => "f32.sqrt",
            Numeric(F64Abs) => "f64.abs",
            Numeric(F64Neg) => "f64.neg",
            Numeric(F64Ceil) => "f64.ceil",
            Numeric(F64Floor) => "f64.floor",
            Numeric(F64Trunc) => "f64.trunc",
            Numeric(F64Nearest) => "f64.nearest",
            Numeric(F64Sqrt) => "f64.sqrt",
            Numeric(I32WrapI64) => "i32.wrap_i64",
            Numeric(I32TruncF32S) => "i32.trunc_f32_s",
            Numeric(I32TruncF32U) => "i32.trunc_f32_u",
            Numeric(I32TruncF64S) => "i32.trunc_f64_s",
            Numeric(I32TruncF64U) => "i32.trunc_f64_u",
            Numeric(I64ExtendI32S) => "i64.extend_i32_s",
            Numeric(I64ExtendI32U) => "i64.extend_i32_u",
            Numeric(I64TruncF32S) => "i64.trunc_f32_s",
            Numeric(I64TruncF32U) => "i64.trunc_f32_u",
            Numeric(I64TruncF64S) => "i64.trunc_f64_s",
            Numeric(I64TruncF64U) => "i64.trunc_f64_u",
            Numeric(F32ConvertI32S) => "f32.convert_i32_s",
            Numeric(F32ConvertI32U) => "f32.convert_i32_u",
            Numeric(F32ConvertI64S) => "f32.convert_i64_s",
            Numeric(F32ConvertI64U) => "f32.convert_i64_u",
            Numeric(F32DemoteF64) => "f32.demote_f64",
            Numeric(F64ConvertI32S) => "f64.convert_i32_s",
            Numeric(F64ConvertI32U) => "f64.convert_i32_u",
            Numeric(F64ConvertI64S) => "f64.convert_i64_s",
            Numeric(F64ConvertI64U) => "f64.convert_i64_u",
            Numeric(F64PromoteF32) => "f64.promote_f32",
            Numeric(I32ReinterpretF32) => "i32.reinterpret_f32",
            Numeric(I64ReinterpretF64) => "i64.reinterpret_f64",
            Numeric(F32ReinterpretI32) => "f32.reinterpret_i32",
            Numeric(F64ReinterpretI64) => "f64.reinterpret_i64",
            Numeric(I32Eq) => "i32.eq",
            Numeric(I32Ne) => "i32.ne",
            Numeric(I32LtS) => "i32.lt_s",
            Numeric(I32LtU) => "i32.lt_u",
            Numeric(I32GtS) => "i32.gt_s",
            Numeric(I32GtU) => "i32.gt_u",
            Numeric(I32LeS) => "i32.le_s",
            Numeric(I32LeU) => "i32.le_u",
            Numeric(I32GeS) => "i32.ge_s",
            Numeric(I32GeU) => "i32.ge_u",
            Numeric(I64Eq) => "i64.eq",
            Numeric(I64Ne) => "i64.ne",
            Numeric(I64LtS) => "i64.lt_s",
            Numeric(I64LtU) => "i64.lt_u",
            Numeric(I64GtS) => "i64.gt_s",
            Numeric(I64GtU) => "i64.gt_u",
            Numeric(I64LeS) => "i64.le_s",
            Numeric(I64LeU) => "i64.le_u",
            Numeric(I64GeS) => "i64.ge_s",
            Numeric(I64GeU) => "i64.ge_u",
            Numeric(F32Eq) => "f32.eq",
            Numeric(F32Ne) => "f32.ne",
            Numeric(F32Lt) => "f32.lt",
            Numeric(F32Gt) => "f32.gt",
            Numeric(F32Le) => "f32.le",
            Numeric(F32Ge) => "f32.ge",
            Numeric(F64Eq) => "f64.eq",
            Numeric(F64Ne) => "f64.ne",
            Numeric(F64Lt) => "f64.lt",
            Numeric(F64Gt) => "f64.gt",
            Numeric(F64Le) => "f64.le",
            Numeric(F64Ge) => "f64.ge",
            Numeric(I32Add) => "i32.add",
            Numeric(I32Sub) => "i32.sub",
            Numeric(I32Mul) => "i32.mul",
            Numeric(I32DivS) => "i32.div_s",
            Numeric(I32DivU) => "i32.div_u",
            Numeric(I32RemS) => "i32.rem_s",
            Numeric(I32RemU) => "i32.rem_u",
            Numeric(I32And) => "i32.and",
            Numeric(I32Or) => "i32.or",
            Numeric(I32Xor) => "i32.xor",
            Numeric(I32Shl) => "i32.shl",
            Numeric(I32ShrS) => "i32.shr_s",
            Numeric(I32ShrU) => "i32.shr_u",
            Numeric(I32Rotl) => "i32.rotl",
            Numeric(I32Rotr) => "i32.rotr",
            Numeric(I64Add) => "i64.add",
            Numeric(I64Sub) => "i64.sub",
            Numeric(I64Mul) => "i64.mul",
            Numeric(I64DivS) => "i64.div_s",
            Numeric(I64DivU) => "i64.div_u",
            Numeric(I64RemS) => "i64.rem_s",
            Numeric(I64RemU) => "i64.rem_u",
            Numeric(I64And) => "i64.and",
            Numeric(I64Or) => "i64.or",
            Numeric(I64Xor) => "i64.xor",
            Numeric(I64Shl) => "i64.shl",
            Numeric(I64ShrS) => "i64.shr_s",
            Numeric(I64ShrU) => "i64.shr_u",
            Numeric(I64Rotl) => "i64.rotl",
            Numeric(I64Rotr) => "i64.rotr",
            Numeric(F32Add) => "f32.add",
            Numeric(F32Sub) => "f32.sub",
            Numeric(F32Mul) => "f32.mul",
            Numeric(F32Div) => "f32.div",
            Numeric(F32Min) => "f32.min",
            Numeric(F32Max) => "f32.max",
            Numeric(F32Copysign) => "f32.copysign",
            Numeric(F64Add) => "f64.add",
            Numeric(F64Sub) => "f64.sub",
            Numeric(F64Mul) => "f64.mul",
            Numeric(F64Div) => "f64.div",
            Numeric(F64Min) => "f64.min",
            Numeric(F64Max) => "f64.max",
            Numeric(F64Copysign) => "f64.copysign",
        }
    }
}

// FIXME Does not respect the width formatting modifier.
impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_name())?;

        // add arguments if instructions has any
        use self::Instr::*;
        match self {
            // instructions without arguments
            Unreachable | Nop | Drop | Select | Return
            | Else | End
            | MemorySize(_) | MemoryGrow(_)
            | Numeric(_) => Ok(()),

            Block(ty) | Loop(ty) | If(ty) => write!(f, " {}", ty),

            Br(label) => write!(f, " {}", label.0),
            BrIf(label) => write!(f, " {}", label.0),
            BrTable { table, default } => {
                for label in table {
                    write!(f, " {}", label.0)?;
                }
                write!(f, " {}", default.0)
            }

            Call(func_idx) => write!(f, " {}", func_idx.into_inner()),
            // We don't print the table index, because we also don't for memory.size and memory.grow,
            // and because in the MVP the table index is going to be 0 anyway.
            CallIndirect(func_ty, _table_idx) => write!(f, " {}", func_ty),

            Local(_, local_idx) => write!(f, " {}", local_idx.into_inner()),
            Global(_, global_idx) => write!(f, " {}", global_idx.into_inner()),

            Load(_, memarg) | Store(_, memarg) => {
                if memarg.offset != 0 {
                    write!(f, " offset={}", memarg.offset)?;
                }

                let natural_alignment_exp = match self {
                    Load(load_op, _) => load_op.natural_alignment_exp(),
                    Store(store_op, _) => store_op.natural_alignment_exp(),
                    _ => unreachable!()
                };
                if memarg.alignment_exp != natural_alignment_exp {
                    write!(f, " align={}", memarg.alignment())?;
                }

                Ok(())
            }

            Const(val) => write!(f, " {}", val)
        }
    }
}

/* Impls/functions for typical use cases on WASM modules. */

impl Module {

    // Convenient iterators over functions, globals, tables, and memories that include the (typed,
    // high-level) index as well.
    // TODO Add _mut variants for globals, tables, and memories, if needed.

    pub fn functions(&self) -> impl Iterator<Item=(Idx<Function>, &Function)> {
        self.functions.iter().enumerate().map(|(i, f)| (i.into(), f))
    }

    pub fn functions_mut(&mut self) -> impl Iterator<Item=(Idx<Function>, &mut Function)> {
        self.functions.iter_mut().enumerate().map(|(i, f)| (i.into(), f))
    }

    pub fn globals(&self) -> impl Iterator<Item=(Idx<Global>, &Global)> {
        self.globals.iter().enumerate().map(|(i, g)| (i.into(), g))
    }

    pub fn tables(&self) -> impl Iterator<Item=(Idx<Table>, &Table)> {
        self.tables.iter().enumerate().map(|(i, t)| (i.into(), t))
    }

    pub fn memories(&self) -> impl Iterator<Item=(Idx<Memory>, &Memory)> {
        self.memories.iter().enumerate().map(|(i, m)| (i.into(), m))
    }


    // Convenient accessors of functions for the typed, high-level index.
    // TODO Add the same for globals, tables, and memories, if needed.

    pub fn function(&self, idx: Idx<Function>) -> &Function {
        &self.functions[idx.into_inner()]
    }

    pub fn function_mut(&mut self, idx: Idx<Function>) -> &mut Function {
        &mut self.functions[idx.into_inner()]
    }

    pub fn global(&self, idx: Idx<Global>) -> &Global {
        &self.globals[idx.into_inner()]
    }

    pub fn global_mut(&mut self, idx: Idx<Global>) -> &mut Global {
        &mut self.globals[idx.into_inner()]
    }


    pub fn add_function(&mut self, type_: FunctionType, locals: Vec<ValType>, body: Vec<Instr>) -> Idx<Function> {
        self.functions.push(Function::new(
            type_,
            Code {
                locals: locals.into_iter().map(Local::new).collect(),
                body,
            },
            Vec::new(),
        ));
        (self.functions.len() - 1).into()
    }

    pub fn add_function_import(&mut self, type_: FunctionType, module: String, name: String) -> Idx<Function> {
        self.functions.push(Function::new_imported(
            type_,
            module, name,
            Vec::new(),
        ));
        (self.functions.len() - 1).into()
    }

    pub fn add_global(&mut self, type_: ValType, mut_: Mutability, init: Vec<Instr>) -> Idx<Global> {
        self.globals.push(Global {
            type_: GlobalType(type_, mut_),
            init: ImportOrPresent::Present(init),
            export: Vec::new(),
        });
        (self.globals.len() - 1).into()
    }

    pub fn types(&self) -> HashSet<&FunctionType> {
        let mut types = HashSet::new();
        for function in &self.functions {
            types.insert(&function.type_);
        }
        types
    }
}

impl Function {
    pub fn new(type_: FunctionType, code: Code, export: Vec<String>) -> Self {
        let param_names = vec![None; type_.params.len()];
        Function {
            type_,
            code: ImportOrPresent::Present(code),
            export,
            name: None,
            param_names,
        }
    }

    pub fn new_imported(type_: FunctionType, import_module: String, import_name: String, export: Vec<String>) -> Self {
        let param_names = vec![None; type_.params.len()];
        Function {
            type_,
            code: ImportOrPresent::Import(import_module, import_name),
            export,
            name: None,
            param_names,
        }
    }

    pub fn import(&self) -> Option<(&str, &str)> {
        if let ImportOrPresent::Import(module, name) = &self.code {
            Some((module.as_str(), name.as_str()))
        } else {
            None
        }
    }

    pub fn code(&self) -> Option<&Code> {
        if let ImportOrPresent::Present(t) = &self.code {
            Some(t)
        } else {
            None
        }
    }

    pub fn code_mut(&mut self) -> Option<&mut Code> {
        if let ImportOrPresent::Present(t) = &mut self.code {
            Some(t)
        } else {
            None
        }
    }

    pub fn into_code(self) -> Option<Code> {
        if let ImportOrPresent::Present(t) = self.code {
            Some(t)
        } else {
            None
        }
    }

    pub fn instrs(&self) -> &[Instr] {
        self.code().map(|code| code.body.as_slice()).unwrap_or(&[])
    }

    pub fn instrs_mut(&mut self) -> Option<&mut Vec<Instr>> {
        self.code_mut().map(|code| &mut code.body)
    }

    pub fn instr_count(&self) -> usize {
        self.code().map(|code| code.body.len()).unwrap_or(0)
    }

    pub fn modify_instrs(&mut self, f: impl Fn(Instr) -> Vec<Instr>) {
        if let Some(body) = self.instrs_mut() {
            let new_body = Vec::with_capacity(body.len());
            let old_body = ::std::mem::replace(body, new_body);
            for instr in old_body.into_iter() {
                body.append(&mut f(instr));
            }
        }
    }

    /// add a new local with type ty and return its index
    pub fn add_fresh_local(&mut self, ty: ValType) -> Idx<Local> {
        let param_count = self.param_count();
        let locals = &mut self.code_mut()
            .expect("cannot add local to imported function")
            .locals;
        let new_idx = param_count + locals.len();
        locals.push(Local::new(ty));
        new_idx.into()
    }

    pub fn add_fresh_locals(&mut self, tys: &[ValType]) -> Vec<Idx<Local>> {
        tys.iter()
            .map(|ty| self.add_fresh_local(*ty))
            .collect()
    }


    // Functions for the number of parameters and non-parameter locals.

    fn assert_param_name_len_valid(&self) {
        assert!(self.param_names.len() == self.type_.params.len());
    }

    pub fn param_count(&self) -> usize {
        self.assert_param_name_len_valid();
        self.type_.params.len()
    }

    pub fn local_count(&self) -> usize {
        self.code().map(|code| code.locals.len()).unwrap_or(0)
    }


    // Accessors and iterators for parameters and locals uniformly.

    pub fn param_or_local(&self, idx: Idx<Local>) -> ParamOrLocalRef {
        self.param_or_locals()
            .nth(idx.into_inner())
            .expect("invalid local index")
            .1
    }

    pub fn param_or_local_mut(&mut self, idx: Idx<Local>) -> ParamOrLocalMut {
        self.param_or_locals_mut()
            .nth(idx.into_inner())
            .expect("invalid local index")
            .1
    }

    pub fn param_or_locals(&self) -> impl Iterator<Item=(Idx<Local>, ParamOrLocalRef)> {
        let params = self.params().map(|(i, p)| (i, ParamOrLocalRef::Param(p)));
        let locals = self.locals().map(|(i, l)| (i, ParamOrLocalRef::Local(l)));
        params.chain(locals)
    }

    pub fn param_or_locals_mut(&mut self) -> impl Iterator<Item=(Idx<Local>, ParamOrLocalMut)> {
        // Unfortunately, we cannot borrow self mutably twice, so we cannot adapt the code from
        // param_or_locals() here. (We would have to call self.params_mut() and self.locals_mut()).
        // Dirty hack: copy code from params_mut()/locals_mut()/code_mut().
        // TODO If there is a smarter way of re-using params_mut() and locals_mut(), I'd be happy to.

        self.assert_param_name_len_valid();

        let params = self.type_.params.iter_mut()
            .zip(self.param_names.iter_mut())
            .map(|(type_, name)| ParamMut { type_, name })
            .map(ParamOrLocalMut::Param);

        let code = if let ImportOrPresent::Present(t) = &mut self.code {
            Some(t)
        } else {
            None
        };

        let locals = code.into_iter()
            .flat_map(|code| code.locals.iter_mut())
            .map(ParamOrLocalMut::Local);

        params.chain(locals)
            .enumerate()
            .map(|(idx, element)| (idx.into(), element))
    }

    /// Returns the parameters (type and debug name, if any) together with their index.
    pub fn params(&self) -> impl Iterator<Item=(Idx<Local>, ParamRef)> {
        self.assert_param_name_len_valid();

        self.type_.params.iter().cloned()
            .zip(self.param_names.iter().map(|s| s.as_ref().map(String::as_str)))
            .enumerate()
            .map(|(idx, (type_, name))|
                (idx.into(), ParamRef { type_, name })
            )
    }

    pub fn params_mut(&mut self) -> impl Iterator<Item=(Idx<Local>, ParamMut)> {
        self.assert_param_name_len_valid();

        self.type_.params.iter_mut()
            .zip(self.param_names.iter_mut())
            .enumerate()
            .map(|(idx, (type_, name))|
                (idx.into(), ParamMut { type_, name })
            )
    }

    /// Returns the non-parameter locals together with their index.
    /// Returns an empty iterator for imported functions (which don't have non-param locals).
    ///
    /// Since function parameters and locals share the same index space, the returned indices will
    /// start at N, if N is the number of function parameters.
    pub fn locals(&self) -> impl Iterator<Item=(Idx<Local>, &Local)> {
        // The index of the non-parameter locals starts after the parameters.
        // This value needs to be moved (not borrowed) into the innermost closure, hence the two
        // nested move annotations on the closures below.
        let param_count = self.param_count();
        self.code().into_iter()
            .flat_map(move |code|
                code.locals.iter()
                    .enumerate()
                    .map(move |(idx, local)|
                        ((param_count + idx).into(), local))
            )
    }

    pub fn locals_mut(&mut self) -> impl Iterator<Item=(Idx<Local>, &mut Local)> {
        // Only changed mutability compared with self.locals(), see comments there.
        let param_count = self.param_count();
        self.code_mut().into_iter()
            .flat_map(move |code|
                code.locals.iter_mut()
                    .enumerate()
                    .map(move |(idx, local)|
                        ((param_count + idx).into(), local))
            )
    }


    // Accessors for a specific parameter/local type/name.

    /// Return the type of the function parameter or non-parameter local with index idx.
    pub fn param_or_local_type(&self, idx: Idx<Local>) -> ValType {
        self.param_or_local(idx).type_()
    }

    /// Return the (optional) debug name of the function parameter or non-parameter local with
    /// index idx.
    pub fn param_or_local_name(&self, idx: Idx<Local>) -> Option<&str> {
        self.param_or_local(idx).name()
    }

    /// Return a mutable reference to the (optional) debug name of the function parameter or
    /// non-parameter local with index idx.
    pub fn param_or_local_name_mut(&mut self, idx: Idx<Local>) -> &mut Option<String> {
        self.param_or_local_mut(idx).name()
    }
}

impl Local {
    pub fn new(type_: ValType) -> Self {
        Local { type_, name: None }
    }
}

// See description on enum type above.
impl<'a> ParamOrLocalRef<'a> {
    pub fn type_(self) -> ValType {
        match self {
            ParamOrLocalRef::Param(param) => param.type_,
            ParamOrLocalRef::Local(local) => local.type_,
        }
    }
    pub fn name(self) -> Option<&'a str> {
        match self {
            ParamOrLocalRef::Param(param) => param.name,
            ParamOrLocalRef::Local(local) => local.name.as_ref().map(String::as_str),
        }
    }
}

// See description on enum type above.
impl<'a> ParamOrLocalMut<'a> {
    pub fn type_(self) -> &'a mut ValType {
        match self {
            ParamOrLocalMut::Param(param) => param.type_,
            ParamOrLocalMut::Local(local) => &mut local.type_,
        }
    }
    pub fn name(self) -> &'a mut Option<String> {
        match self {
            ParamOrLocalMut::Param(param) => param.name,
            ParamOrLocalMut::Local(local) => &mut local.name,
        }
    }
}

impl Global {
    pub fn import(&self) -> Option<(&str, &str)> {
        if let ImportOrPresent::Import(module, name) = &self.init {
            Some((module.as_str(), name.as_str()))
        } else {
            None
        }
    }

    pub fn init(&self) -> Option<&Expr> {
        if let ImportOrPresent::Present(t) = &self.init {
            Some(t)
        } else {
            None
        }
    }
}

impl Table {
    pub fn import(&self) -> Option<(&str, &str)> {
        self.import.as_ref().map(|(module, name)| (module.as_str(), name.as_str()))
    }
}

impl Memory {
    pub fn import(&self) -> Option<(&str, &str)> {
        self.import.as_ref().map(|(module, name)| (module.as_str(), name.as_str()))
    }
}
