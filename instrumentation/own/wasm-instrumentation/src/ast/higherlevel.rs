use ast::{BlockType, FunctionType, Idx, Label, Local, Memarg, Val, ValType, ValType::*};
use ast::highlevel::{Function, Global, Memory, Table};

// TODO replace highlevel::Instr with this and InstrGroup with InstrType

#[derive(Debug, Clone, PartialEq)]
enum Instr {
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
enum LocalOp { Get, Set, Tee }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum GlobalOp { Get, Set }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum LoadOp {
    I32,
    I64,
    F32,
    F64,

    I328S,
    I328U,
    I3216S,
    I3216U,

    I648S,
    I648U,
    I6416S,
    I6416U,
    I6432S,
    I6432U,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum StoreOp {
    I32,
    I64,
    F32,
    F64,

    I328,
    I3216,

    I648,
    I6416,
    I6432,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum NumericOp {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Default, new)]
struct InstrType {
    inputs: Vec<ValType>,
    results: Vec<ValType>,
}

impl NumericOp {
    fn to_type(&self) -> InstrType {
        use self::NumericOp::*;
        match *self {
            /* Unary */

            I32Eqz => InstrType::new(vec![I32], vec![I32]),
            I64Eqz => InstrType::new(vec![I64], vec![I32]),

            I32Clz | I32Ctz | I32Popcnt => InstrType::new(vec![I32], vec![I32]),
            I64Clz | I64Ctz | I64Popcnt => InstrType::new(vec![I64], vec![I64]),

            F32Abs | F32Neg | F32Ceil | F32Floor | F32Trunc | F32Nearest | F32Sqrt => InstrType::new(vec![F32], vec![F32]),
            F64Abs | F64Neg | F64Ceil | F64Floor | F64Trunc | F64Nearest | F64Sqrt => InstrType::new(vec![F64], vec![F64]),

            // conversions
            I32WrapI64 => InstrType::new(vec![I64], vec![I32]),
            I32TruncSF32 | I32TruncUF32 => InstrType::new(vec![I32], vec![F32]),
            I32TruncSF64 | I32TruncUF64 => InstrType::new(vec![F64], vec![I32]),
            I64ExtendSI32 | I64ExtendUI32 => InstrType::new(vec![I32], vec![I64]),
            I64TruncSF32 | I64TruncUF32 => InstrType::new(vec![F32], vec![I64]),
            I64TruncSF64 | I64TruncUF64 => InstrType::new(vec![F64], vec![I64]),
            F32ConvertSI32 | F32ConvertUI32 => InstrType::new(vec![I32], vec![F32]),
            F32ConvertSI64 | F32ConvertUI64 => InstrType::new(vec![I64], vec![F32]),
            F32DemoteF64 => InstrType::new(vec![F64], vec![F32]),
            F64ConvertSI32 | F64ConvertUI32 => InstrType::new(vec![I32], vec![F64]),
            F64ConvertSI64 | F64ConvertUI64 => InstrType::new(vec![I64], vec![F64]),
            F64PromoteF32 => InstrType::new(vec![F32], vec![F64]),
            I32ReinterpretF32 => InstrType::new(vec![F32], vec![I32]),
            I64ReinterpretF64 => InstrType::new(vec![F64], vec![I64]),
            F32ReinterpretI32 => InstrType::new(vec![I32], vec![F32]),
            F64ReinterpretI64 => InstrType::new(vec![I64], vec![F64]),

            /* Binary */

            I32Eq | I32Ne | I32LtS | I32LtU | I32GtS | I32GtU | I32LeS | I32LeU | I32GeS | I32GeU => InstrType::new(vec![I32, I32], vec![I32]),
            I64Eq | I64Ne | I64LtS | I64LtU | I64GtS | I64GtU | I64LeS | I64LeU | I64GeS | I64GeU => InstrType::new(vec![I64, I64], vec![I32]),

            F32Eq | F32Ne | F32Lt | F32Gt | F32Le | F32Ge => InstrType::new(vec![F32, F32], vec![I32]),
            F64Eq | F64Ne | F64Lt | F64Gt | F64Le | F64Ge => InstrType::new(vec![F64, F64], vec![I32]),

            I32Add | I32Sub | I32Mul | I32DivS | I32DivU | I32RemS | I32RemU | I32And | I32Or | I32Xor | I32Shl | I32ShrS | I32ShrU | I32Rotl | I32Rotr => InstrType::new(vec![I32, I32], vec![I32]),
            I64Add | I64Sub | I64Mul | I64DivS | I64DivU | I64RemS | I64RemU | I64And | I64Or | I64Xor | I64Shl | I64ShrS | I64ShrU | I64Rotl | I64Rotr => InstrType::new(vec![I64, I64], vec![I64]),
            F32Add | F32Sub | F32Mul | F32Div | F32Min | F32Max | F32Copysign => InstrType::new(vec![F32, F32], vec![F32]),
            F64Add | F64Sub | F64Mul | F64Div | F64Min | F64Max | F64Copysign => InstrType::new(vec![F64, F64], vec![F64]),
        }
    }
}

impl LoadOp {
    fn to_type(&self) -> InstrType {
        match *self {
            LoadOp::I32 => InstrType::new(vec![], vec![I32]),
            LoadOp::I64 => InstrType::new(vec![], vec![I64]),
            LoadOp::F32 => InstrType::new(vec![], vec![F32]),
            LoadOp::F64 => InstrType::new(vec![], vec![F64]),

            LoadOp::I328S => InstrType::new(vec![], vec![I32]),
            LoadOp::I328U => InstrType::new(vec![], vec![I32]),
            LoadOp::I3216S => InstrType::new(vec![], vec![I32]),
            LoadOp::I3216U => InstrType::new(vec![], vec![I32]),
            LoadOp::I648S => InstrType::new(vec![], vec![I64]),
            LoadOp::I648U => InstrType::new(vec![], vec![I64]),
            LoadOp::I6416S => InstrType::new(vec![], vec![I64]),
            LoadOp::I6416U => InstrType::new(vec![], vec![I64]),
            LoadOp::I6432S => InstrType::new(vec![], vec![I64]),
            LoadOp::I6432U => InstrType::new(vec![], vec![I64]),
        }
    }
}

impl StoreOp {
    fn to_type(&self) -> InstrType {
        match *self {
            StoreOp::I32 => InstrType::new(vec![I32], vec![]),
            StoreOp::I64 => InstrType::new(vec![I64], vec![]),
            StoreOp::F32 => InstrType::new(vec![F32], vec![]),
            StoreOp::F64 => InstrType::new(vec![F64], vec![]),

            StoreOp::I328 => InstrType::new(vec![I32], vec![]),
            StoreOp::I3216 => InstrType::new(vec![I32], vec![]),
            StoreOp::I648 => InstrType::new(vec![I64], vec![]),
            StoreOp::I6416 => InstrType::new(vec![I64], vec![]),
            StoreOp::I6432 => InstrType::new(vec![I64], vec![]),
        }
    }
}

impl Instr {
    /// for all where the type can be determined by just looking at the instruction, not additional
    /// information like the function or module etc.
    fn to_type(&self) -> Option<InstrType> {
        use self::Instr::*;
        match *self {
            Unreachable | Nop => Some(InstrType::default()),
            Load(ref op, _) => Some(op.to_type()),
            Store(ref op, _) => Some(op.to_type()),
            MemorySize(_) => Some(InstrType::new(vec![], vec![I32])),
            MemoryGrow(_) => Some(InstrType::new(vec![I32], vec![I32])),
            Const(ref val) => Some(InstrType::new(vec![], vec![val.to_type()])),
            Numeric(ref op) => Some(op.to_type()),

            // nesting...
            Block(_) | Loop(_) | If(_) | Else | End => None,
            // depends on branch target?
            Br(_) | BrIf(_) | BrTable(_, _) => None,
            // need to inspect function type
            Return | Call(_) | CallIndirect(_, _) => None,
            // need abstract type stack "evaluation"
            Drop | Select => None,
            // need lookup in locals/globals
            Local(_, _) | Global(_, _) => None,
        }
    }

    fn to_name(&self) -> &'static str {
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
            Local(LocalOp::Get, _) => "get_local",
            Local(LocalOp::Set, _) => "set_local",
            Local(LocalOp::Tee, _) => "tee_local",
            Global(GlobalOp::Get, _) => "get_global",
            Global(GlobalOp::Set, _) => "set_global",
            MemorySize(_) => "memory.size",
            MemoryGrow(_) => "memory.grow",
            Const(Val::I32(_)) => "i32.const",
            Const(Val::I64(_)) => "i64.const",
            Const(Val::F32(_)) => "f32.const",
            Const(Val::F64(_)) => "f64.const",
            Load(LoadOp::I32, _) => "i32.load",
            Load(LoadOp::I64, _) => "i64.load",
            Load(LoadOp::F32, _) => "f32.load",
            Load(LoadOp::F64, _) => "f64.load",
            Load(LoadOp::I328S, _) => "i32.load8_s",
            Load(LoadOp::I328U, _) => "i32.load8_u",
            Load(LoadOp::I3216S, _) => "i32.load16_s",
            Load(LoadOp::I3216U, _) => "i32.load16_u",
            Load(LoadOp::I648S, _) => "i64.load8_s",
            Load(LoadOp::I648U, _) => "i64.load8_u",
            Load(LoadOp::I6416S, _) => "i64.load16_s",
            Load(LoadOp::I6416U, _) => "i64.load16_u",
            Load(LoadOp::I6432S, _) => "i64.load32_s",
            Load(LoadOp::I6432U, _) => "i64.load32_u",
            Store(StoreOp::I32, _) => "i32.store",
            Store(StoreOp::I64, _) => "i64.store",
            Store(StoreOp::F32, _) => "f32.store",
            Store(StoreOp::F64, _) => "f64.store",
            Store(StoreOp::I328, _) => "i32.store8",
            Store(StoreOp::I3216, _) => "i32.store16",
            Store(StoreOp::I648, _) => "i64.store8",
            Store(StoreOp::I6416, _) => "i64.store16",
            Store(StoreOp::I6432, _) => "i64.store32",
            // TODO
            Numeric(I32Eqz) => "",
            Numeric(I64Eqz) => "",
            Numeric(I32Clz) => "",
            Numeric(I32Ctz) => "",
            Numeric(I32Popcnt) => "",
            Numeric(I64Clz) => "",
            Numeric(I64Ctz) => "",
            Numeric(I64Popcnt) => "",
            Numeric(F32Abs) => "",
            Numeric(F32Neg) => "",
            Numeric(F32Ceil) => "",
            Numeric(F32Floor) => "",
            Numeric(F32Trunc) => "",
            Numeric(F32Nearest) => "",
            Numeric(F32Sqrt) => "",
            Numeric(F64Abs) => "",
            Numeric(F64Neg) => "",
            Numeric(F64Ceil) => "",
            Numeric(F64Floor) => "",
            Numeric(F64Trunc) => "",
            Numeric(F64Nearest) => "",
            Numeric(F64Sqrt) => "",
            Numeric(I32WrapI64) => "",
            Numeric(I32TruncSF32) => "",
            Numeric(I32TruncUF32) => "",
            Numeric(I32TruncSF64) => "",
            Numeric(I32TruncUF64) => "",
            Numeric(I64ExtendSI32) => "",
            Numeric(I64ExtendUI32) => "",
            Numeric(I64TruncSF32) => "",
            Numeric(I64TruncUF32) => "",
            Numeric(I64TruncSF64) => "",
            Numeric(I64TruncUF64) => "",
            Numeric(F32ConvertSI32) => "",
            Numeric(F32ConvertUI32) => "",
            Numeric(F32ConvertSI64) => "",
            Numeric(F32ConvertUI64) => "",
            Numeric(F32DemoteF64) => "",
            Numeric(F64ConvertSI32) => "",
            Numeric(F64ConvertUI32) => "",
            Numeric(F64ConvertSI64) => "",
            Numeric(F64ConvertUI64) => "",
            Numeric(F64PromoteF32) => "",
            Numeric(I32ReinterpretF32) => "",
            Numeric(I64ReinterpretF64) => "",
            Numeric(F32ReinterpretI32) => "",
            Numeric(F64ReinterpretI64) => "",
            Numeric(I32Eq) => "",
            Numeric(I32Ne) => "",
            Numeric(I32LtS) => "",
            Numeric(I32LtU) => "",
            Numeric(I32GtS) => "",
            Numeric(I32GtU) => "",
            Numeric(I32LeS) => "",
            Numeric(I32LeU) => "",
            Numeric(I32GeS) => "",
            Numeric(I32GeU) => "",
            Numeric(I64Eq) => "",
            Numeric(I64Ne) => "",
            Numeric(I64LtS) => "",
            Numeric(I64LtU) => "",
            Numeric(I64GtS) => "",
            Numeric(I64GtU) => "",
            Numeric(I64LeS) => "",
            Numeric(I64LeU) => "",
            Numeric(I64GeS) => "",
            Numeric(I64GeU) => "",
            Numeric(F32Eq) => "",
            Numeric(F32Ne) => "",
            Numeric(F32Lt) => "",
            Numeric(F32Gt) => "",
            Numeric(F32Le) => "",
            Numeric(F32Ge) => "",
            Numeric(F64Eq) => "",
            Numeric(F64Ne) => "",
            Numeric(F64Lt) => "",
            Numeric(F64Gt) => "",
            Numeric(F64Le) => "",
            Numeric(F64Ge) => "",
            Numeric(I32Add) => "",
            Numeric(I32Sub) => "",
            Numeric(I32Mul) => "",
            Numeric(I32DivS) => "",
            Numeric(I32DivU) => "",
            Numeric(I32RemS) => "",
            Numeric(I32RemU) => "",
            Numeric(I32And) => "",
            Numeric(I32Or) => "",
            Numeric(I32Xor) => "",
            Numeric(I32Shl) => "",
            Numeric(I32ShrS) => "",
            Numeric(I32ShrU) => "",
            Numeric(I32Rotl) => "",
            Numeric(I32Rotr) => "",
            Numeric(I64Add) => "",
            Numeric(I64Sub) => "",
            Numeric(I64Mul) => "",
            Numeric(I64DivS) => "",
            Numeric(I64DivU) => "",
            Numeric(I64RemS) => "",
            Numeric(I64RemU) => "",
            Numeric(I64And) => "",
            Numeric(I64Or) => "",
            Numeric(I64Xor) => "",
            Numeric(I64Shl) => "",
            Numeric(I64ShrS) => "",
            Numeric(I64ShrU) => "",
            Numeric(I64Rotl) => "",
            Numeric(I64Rotr) => "",
            Numeric(F32Add) => "",
            Numeric(F32Sub) => "",
            Numeric(F32Mul) => "",
            Numeric(F32Div) => "",
            Numeric(F32Min) => "",
            Numeric(F32Max) => "",
            Numeric(F32Copysign) => "",
            Numeric(F64Add) => "",
            Numeric(F64Sub) => "",
            Numeric(F64Mul) => "",
            Numeric(F64Div) => "",
            Numeric(F64Min) => "",
            Numeric(F64Max) => "",
            Numeric(F64Copysign) => "",
        }
    }
}

