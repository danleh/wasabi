// TODO prototype of even higher level AST format :P
// could replace highlevel::Instr::group() function

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

    Load(ValType, Option<LoadSize>, Memarg),
    Store(ValType, Option<StoreSize>, Memarg),

    CurrentMemory(Idx<Memory>),
    GrowMemory(Idx<Memory>),

    Const(Val),
    Unary(UnaryOp),
    Binary(BinaryOp),

}

enum Signedness { Signed, Unsigned }

enum MemSize { _8, _16, _32 }

struct LoadSize(MemSize, Signedness);

struct StoreSize(MemSize);

enum LocalOp { Get, Set, Tee }

enum GlobalOp { Get, Set }

enum UnaryOp {
    // integer
    Clz,
    Ctz,
    Popcnt,
    // float
    Abs,
    Neg,
    Sqrt,
    Ceil,
    Floor,
    Trunc,
    Nearest,
}

enum Val {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Val {
    fn to_type(&self) -> ValType {}
}

type ValType = Discriminant<Val>;

static I32: ValType = discriminant(Val::I32(0));
static I64: ValType = discriminant(Val::I64(0));
static F32: ValType = discriminant(Val::F32(0.0));
static F64: ValType = discriminant(Val::F64(0.0));