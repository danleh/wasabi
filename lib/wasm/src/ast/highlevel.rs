use self::{GlobalOp::*, LoadOp::*, LocalOp::*, StoreOp::*};
use std::collections::HashSet;
use super::{*, ValType::*};

/* High-level AST:
    - types are inlined instead of referenced by type idx (i.e., no manual handling of Type "pool")
    - Function + Code sections are merged into one list of functions,
      same for tables: Table + Element sections and memories: Memory + Data sections.
    - imports and exports are part of the respective item, not stored externally and referring to
      their item by index.
    - similar instructions are grouped together, for easier uniform handling, e.g., T.const
      instructions, loads, stores, and numeric instructions.
*/

#[derive(Debug, Clone, Default)]
pub struct Module {
    pub functions: Vec<Function>,
    pub tables: Vec<Table>,
    pub memories: Vec<Memory>,
    pub globals: Vec<Global>,

    pub start: Option<Idx<Function>>,

    pub custom_sections: Vec<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Function {
    // type is inlined here compared to low-level/binary/spec representation
    pub type_: FunctionType,
    // import and code are mutually exclusive, i.e., exactly one of both must be Some(...)
    pub import: Option<(String, String)>,
    pub code: Option<Code>,
    pub export: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Global {
    pub type_: GlobalType,
    // import and init are mutually exclusive, i.e., exactly one of both must be Some(...)
    pub import: Option<(String, String)>,
    pub init: Option<Expr>,
    pub export: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub type_: TableType,
    pub import: Option<(String, String)>,
    pub elements: Vec<Element>,
    pub export: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub type_: MemoryType,
    pub import: Option<(String, String)>,
    pub data: Vec<Data>,
    pub export: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Code {
    pub locals: Vec<ValType>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub offset: Expr,
    pub functions: Vec<Idx<Function>>,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub offset: Expr,
    pub bytes: Vec<u8>,
}

pub type Expr = Vec<Instr>;

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    Unreachable,
    Nop,

    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    Else,
    End,

    Br(Idx<Label>),
    BrIf(Idx<Label>),
    BrTable(Vec<Idx<Label>>, Idx<Label>),

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LocalOp { GetLocal, SetLocal, TeeLocal }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GlobalOp { GetGlobal, SetGlobal }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
    I32TruncSF32,
    I32TruncUF32,
    I32TruncSF64,
    I32TruncUF64,

    I64ExtendSI32,
    I64ExtendUI32,
    I64TruncSF32,
    I64TruncUF32,
    I64TruncSF64,
    I64TruncUF64,

    F32ConvertSI32,
    F32ConvertUI32,
    F32ConvertSI64,
    F32ConvertUI64,
    F32DemoteF64,

    F64ConvertSI32,
    F64ConvertUI32,
    F64ConvertSI64,
    F64ConvertUI64,
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
    pub fn to_type(&self, local_ty: ValType) -> InstrType {
        match *self {
            LocalOp::GetLocal => InstrType::new(&[], &[local_ty]),
            LocalOp::SetLocal => InstrType::new(&[local_ty], &[]),
            LocalOp::TeeLocal => InstrType::new(&[local_ty], &[local_ty]),
        }
    }
}

impl GlobalOp {
    pub fn to_type(&self, global_ty: ValType) -> InstrType {
        match *self {
            GlobalOp::GetGlobal => InstrType::new(&[], &[global_ty]),
            GlobalOp::SetGlobal => InstrType::new(&[global_ty], &[]),
        }
    }
}

impl NumericOp {
    pub fn to_type(&self) -> InstrType {
        use self::NumericOp::*;
        match *self {
            /* Unary */

            I32Eqz => InstrType::new(&[I32], &[I32]),
            I64Eqz => InstrType::new(&[I64], &[I32]),

            I32Clz | I32Ctz | I32Popcnt => InstrType::new(&[I32], &[I32]),
            I64Clz | I64Ctz | I64Popcnt => InstrType::new(&[I64], &[I64]),

            F32Abs | F32Neg | F32Ceil | F32Floor | F32Trunc | F32Nearest | F32Sqrt => InstrType::new(&[F32], &[F32]),
            F64Abs | F64Neg | F64Ceil | F64Floor | F64Trunc | F64Nearest | F64Sqrt => InstrType::new(&[F64], &[F64]),

            // conversions
            I32WrapI64 => InstrType::new(&[I64], &[I32]),
            I32TruncSF32 | I32TruncUF32 => InstrType::new(&[I32], &[F32]),
            I32TruncSF64 | I32TruncUF64 => InstrType::new(&[F64], &[I32]),
            I64ExtendSI32 | I64ExtendUI32 => InstrType::new(&[I32], &[I64]),
            I64TruncSF32 | I64TruncUF32 => InstrType::new(&[F32], &[I64]),
            I64TruncSF64 | I64TruncUF64 => InstrType::new(&[F64], &[I64]),
            F32ConvertSI32 | F32ConvertUI32 => InstrType::new(&[I32], &[F32]),
            F32ConvertSI64 | F32ConvertUI64 => InstrType::new(&[I64], &[F32]),
            F32DemoteF64 => InstrType::new(&[F64], &[F32]),
            F64ConvertSI32 | F64ConvertUI32 => InstrType::new(&[I32], &[F64]),
            F64ConvertSI64 | F64ConvertUI64 => InstrType::new(&[I64], &[F64]),
            F64PromoteF32 => InstrType::new(&[F32], &[F64]),
            I32ReinterpretF32 => InstrType::new(&[F32], &[I32]),
            I64ReinterpretF64 => InstrType::new(&[F64], &[I64]),
            F32ReinterpretI32 => InstrType::new(&[I32], &[F32]),
            F64ReinterpretI64 => InstrType::new(&[I64], &[F64]),

            /* Binary */

            I32Eq | I32Ne | I32LtS | I32LtU | I32GtS | I32GtU | I32LeS | I32LeU | I32GeS | I32GeU => InstrType::new(&[I32, I32], &[I32]),
            I64Eq | I64Ne | I64LtS | I64LtU | I64GtS | I64GtU | I64LeS | I64LeU | I64GeS | I64GeU => InstrType::new(&[I64, I64], &[I32]),

            F32Eq | F32Ne | F32Lt | F32Gt | F32Le | F32Ge => InstrType::new(&[F32, F32], &[I32]),
            F64Eq | F64Ne | F64Lt | F64Gt | F64Le | F64Ge => InstrType::new(&[F64, F64], &[I32]),

            I32Add | I32Sub | I32Mul | I32DivS | I32DivU | I32RemS | I32RemU | I32And | I32Or | I32Xor | I32Shl | I32ShrS | I32ShrU | I32Rotl | I32Rotr => InstrType::new(&[I32, I32], &[I32]),
            I64Add | I64Sub | I64Mul | I64DivS | I64DivU | I64RemS | I64RemU | I64And | I64Or | I64Xor | I64Shl | I64ShrS | I64ShrU | I64Rotl | I64Rotr => InstrType::new(&[I64, I64], &[I64]),
            F32Add | F32Sub | F32Mul | F32Div | F32Min | F32Max | F32Copysign => InstrType::new(&[F32, F32], &[F32]),
            F64Add | F64Sub | F64Mul | F64Div | F64Min | F64Max | F64Copysign => InstrType::new(&[F64, F64], &[F64]),
        }
    }
}

impl LoadOp {
    pub fn to_type(&self) -> InstrType {
        match *self {
            I32Load => InstrType::new(&[], &[I32]),
            I64Load => InstrType::new(&[], &[I64]),
            F32Load => InstrType::new(&[], &[F32]),
            F64Load => InstrType::new(&[], &[F64]),

            I32Load8S => InstrType::new(&[], &[I32]),
            I32Load8U => InstrType::new(&[], &[I32]),
            I32Load16S => InstrType::new(&[], &[I32]),
            I32Load16U => InstrType::new(&[], &[I32]),
            I64Load8S => InstrType::new(&[], &[I64]),
            I64Load8U => InstrType::new(&[], &[I64]),
            I64Load16S => InstrType::new(&[], &[I64]),
            I64Load16U => InstrType::new(&[], &[I64]),
            I64Load32S => InstrType::new(&[], &[I64]),
            I64Load32U => InstrType::new(&[], &[I64]),
        }
    }
}

impl StoreOp {
    pub fn to_type(&self) -> InstrType {
        match *self {
            I32Store => InstrType::new(&[I32], &[]),
            I64Store => InstrType::new(&[I64], &[]),
            F32Store => InstrType::new(&[F32], &[]),
            F64Store => InstrType::new(&[F64], &[]),

            I32Store8 => InstrType::new(&[I32], &[]),
            I32Store16 => InstrType::new(&[I32], &[]),
            I64Store8 => InstrType::new(&[I64], &[]),
            I64Store16 => InstrType::new(&[I64], &[]),
            I64Store32 => InstrType::new(&[I64], &[]),
        }
    }
}

impl Instr {
    /// for all where the type can be determined by just looking at the instruction, not additional
    /// information like the function or module etc.
    pub fn to_type(&self) -> Option<InstrType> {
        use self::Instr::*;
        match *self {
            Unreachable | Nop => Some(InstrType::default()),
            Load(ref op, _) => Some(op.to_type()),
            Store(ref op, _) => Some(op.to_type()),
            MemorySize(_) => Some(InstrType::new(&[], &[I32])),
            MemoryGrow(_) => Some(InstrType::new(&[I32], &[I32])),
            Const(ref val) => Some(InstrType::new(&[], &[val.to_type()])),
            Numeric(ref op) => Some(op.to_type()),
            CallIndirect(ref func_ty, _) => Some(InstrType::new(&[&func_ty.params[..], &[I32]].concat(), &func_ty.results)),

            // nesting...
            Block(_) | Loop(_) | If(_) | Else | End => None,
            // depends on branch target?
            Br(_) | BrIf(_) | BrTable(_, _) => None,
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
        use self::Instr::*;
        use self::NumericOp::*;
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
            BrTable(_, _) => "br_table",
            Return => "return",
            Call(_) => "call",
            CallIndirect(_, _) => "call_indirect",
            Drop => "drop",
            Select => "select",
            Local(GetLocal, _) => "get_local",
            Local(SetLocal, _) => "set_local",
            Local(TeeLocal, _) => "tee_local",
            Global(GetGlobal, _) => "get_global",
            Global(SetGlobal, _) => "set_global",
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
            Numeric(I32WrapI64) => "i32.wrap/i64",
            Numeric(I32TruncSF32) => "i32.trunc_s/f32",
            Numeric(I32TruncUF32) => "i32.trunc_u/f32",
            Numeric(I32TruncSF64) => "i32.trunc_s/f64",
            Numeric(I32TruncUF64) => "i32.trunc_u/f64",
            Numeric(I64ExtendSI32) => "i64.extend_s/i32",
            Numeric(I64ExtendUI32) => "i64.extend_u/i32",
            Numeric(I64TruncSF32) => "i64.trunc_s/f32",
            Numeric(I64TruncUF32) => "i64.trunc_u/f32",
            Numeric(I64TruncSF64) => "i64.trunc_s/f64",
            Numeric(I64TruncUF64) => "i64.trunc_u/f64",
            Numeric(F32ConvertSI32) => "f32.convert_s/i32",
            Numeric(F32ConvertUI32) => "f32.convert_u/i32",
            Numeric(F32ConvertSI64) => "f32.convert_s/i64",
            Numeric(F32ConvertUI64) => "f32.convert_u/i64",
            Numeric(F32DemoteF64) => "f32.demote/f64",
            Numeric(F64ConvertSI32) => "f64.convert_s/i32",
            Numeric(F64ConvertUI32) => "f64.convert_u/i32",
            Numeric(F64ConvertSI64) => "f64.convert_s/i64",
            Numeric(F64ConvertUI64) => "f64.convert_u/i64",
            Numeric(F64PromoteF32) => "f64.promote/f32",
            Numeric(I32ReinterpretF32) => "i32.reinterpret/f32",
            Numeric(I64ReinterpretF64) => "i64.reinterpret/f64",
            Numeric(F32ReinterpretI32) => "f32.reinterpret/i32",
            Numeric(F64ReinterpretI64) => "f64.reinterpret/i64",
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


/* Impls/functions for typical use cases on WASM modules. */

impl Module {
    pub fn add_function(&mut self, type_: FunctionType, locals: Vec<ValType>, body: Vec<Instr>) -> Idx<Function> {
        self.functions.push(Function {
            type_,
            import: None,
            code: Some(Code {
                locals,
                body,
            }),
            export: None,
        });
        (self.functions.len() - 1).into()
    }

    pub fn add_function_import(&mut self, type_: FunctionType, module: String, name: String) -> Idx<Function> {
        self.functions.push(Function {
            type_,
            import: Some((module, name)),
            code: None,
            export: None,
        });
        (self.functions.len() - 1).into()
    }

    pub fn add_global(&mut self, type_: ValType, mut_: Mutability, init: Vec<Instr>) -> Idx<Global> {
        self.globals.push(Global {
            type_: GlobalType(type_, mut_),
            import: None,
            init: Some(init),
            export: None,
        });
        (self.globals.len() - 1).into()
    }

    pub fn function(&mut self, idx: Idx<Function>) -> &mut Function { &mut self.functions[idx.0] }
    pub fn functions(&mut self) -> impl Iterator<Item=(Idx<Function>, &mut Function)> {
        self.functions.iter_mut().enumerate().map(|(i, f)| (i.into(), f))
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
    pub fn instructions(&mut self) -> impl Iterator<Item=(Idx<Instr>, &mut Instr)> {
        self.code.iter_mut().flat_map(|code| code.body.iter_mut().enumerate().map(|(i, f)| (i.into(), f)))
    }

    pub fn modify_instr(&mut self, f: impl Fn(Instr) -> Vec<Instr>) {
        if let Some(Code { ref mut body, .. }) = self.code {
            let new_body = Vec::with_capacity(body.len());
            let old_body = ::std::mem::replace(body, new_body);
            for instr in old_body.into_iter() {
                body.append(&mut f(instr));
            }
        }
    }

    /// add a new local with type ty and return its index
    pub fn add_fresh_local(&mut self, ty: ValType) -> Idx<Local> {
        let locals = &mut self.code.as_mut()
            .expect("cannot add local to imported function")
            .locals;
        let idx = locals.len() + self.type_.params.len();
        locals.push(ty);
        idx.into()
    }

    pub fn add_fresh_locals(&mut self, tys: &[ValType]) -> Vec<Idx<Local>> {
        tys.iter()
            .map(|ty| self.add_fresh_local(*ty))
            .collect()
    }

    /// get type of the local with index idx
    pub fn local_type(&self, idx: Idx<Local>) -> ValType {
        let param_count = self.type_.params.len();
        if (idx.0) < param_count {
            self.type_.params[idx.0]
        } else {
            let locals = &self.code.as_ref()
                .expect("cannot get type of a local in an imported function")
                .locals;
            *locals.get(idx.0 - param_count)
                .expect(&format!("invalid local index {}, function has {} parameters and {} locals", idx.0, param_count, locals.len()))
        }
    }
}