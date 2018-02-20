use super::*;

/* High-level AST:
    - types are inlined instead of referenced by type idx (i.e., no manual handling of Type "pool")
    - Function + Code sections are merged into one list of functions,
      same for tables: Table + Element sections and memories: Memory + Data sections.
    - imports and exports are part of the respective item, not stored externally and referring to
      their item by index.
*/

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Instr {
    Unreachable,
    Nop,

    Block(BlockType, Expr),
    Loop(BlockType, Expr),
    If(BlockType, Expr),
    Else(Expr),
    End,

    Br(Idx<Label>),
    BrIf(Idx<Label>),
    BrTable(Vec<Idx<Label>>, Idx<Label>),

    Return,
    Call(Idx<Function>),
    CallIndirect(FunctionType, Idx<Table>),

    Drop,
    Select,

    GetLocal(Idx<Local>),
    SetLocal(Idx<Local>),
    TeeLocal(Idx<Local>),
    GetGlobal(Idx<Global>),
    SetGlobal(Idx<Global>),

    I32Load(Memarg),
    I64Load(Memarg),
    F32Load(Memarg),
    F64Load(Memarg),
    I32Load8S(Memarg),
    I32Load8U(Memarg),
    I32Load16S(Memarg),
    I32Load16U(Memarg),
    I64Load8S(Memarg),
    I64Load8U(Memarg),
    I64Load16S(Memarg),
    I64Load16U(Memarg),
    I64Load32S(Memarg),
    I64Load32U(Memarg),
    I32Store(Memarg),
    I64Store(Memarg),
    F32Store(Memarg),
    F64Store(Memarg),
    I32Store8(Memarg),
    I32Store16(Memarg),
    I64Store8(Memarg),
    I64Store16(Memarg),
    I64Store32(Memarg),

    CurrentMemory(Idx<Memory>),
    GrowMemory(Idx<Memory>),

    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    I32Eqz,
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
    I64Eqz,
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
    I32Clz,
    I32Ctz,
    I32Popcnt,
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
    I64Clz,
    I64Ctz,
    I64Popcnt,
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
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
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

    pub fn add_global(&mut self, type_: GlobalType, init: Vec<Instr>) -> Idx<Global> {
        self.globals.push(Global {
            type_,
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
}

impl Instr {
    pub fn is_call(&self) -> bool {
        match *self {
            Instr::Call(_) => true,
            Instr::CallIndirect(_, _) => true,
            _ => false
        }
    }
}


/* Visitors for transforming instructions without having to recurse manually over the tree.
   bottom_up: goes first into the leaves (i.e., instrs or exprs) and applies f,
              and then applies f to the current level again (== post-order).
*/

// FIXME I don't understand why we need f to be behind a reference, with impl Fn... I seem to get an inifinite type!?
pub trait VisitExpr {
    fn bottom_up(&mut self, f: &Fn(&mut Expr));
}

impl VisitExpr for Expr {
    fn bottom_up(&mut self, f: &Fn(&mut Expr)) {
        for instr in self.iter_mut() {
            match *instr {
                Instr::Block(_, ref mut expr) => VisitExpr::bottom_up(expr, f),
                Instr::Loop(_, ref mut expr) => VisitExpr::bottom_up(expr, f),
                Instr::If(_, ref mut expr) => VisitExpr::bottom_up(expr, f),
                Instr::Else(ref mut expr) => VisitExpr::bottom_up(expr, f),
                _ => {}
            }
        }
        f(self)
    }
}

pub trait VisitInstr {
    fn bottom_up(&mut self, f: impl Fn(&mut Instr));
}

impl VisitInstr for Expr {
    fn bottom_up(&mut self, f: impl Fn(&mut Instr)) {
        for instr in self.iter_mut() {
            instr.bottom_up(|instr| f(instr))
        }
    }
}

impl VisitInstr for Instr {
    fn bottom_up(&mut self, f: impl Fn(&mut Instr)) {
        match *self {
            Instr::Block(_, ref mut expr) => VisitInstr::bottom_up(expr, |instr| f(instr)),
            Instr::Loop(_, ref mut expr) => VisitInstr::bottom_up(expr, |instr| f(instr)),
            Instr::If(_, ref mut expr) => VisitInstr::bottom_up(expr, |instr| f(instr)),
            Instr::Else(ref mut expr) => VisitInstr::bottom_up(expr, |instr| f(instr)),
            _ => {}
        }
        f(self);
    }
}