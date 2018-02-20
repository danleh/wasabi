use std::collections::HashMap;
use super::lowlevel as ll;
// reuse as much as possible from low-level AST
use super::lowlevel::{BlockType, ElemType, FunctionType, GlobalType, Idx, Label, Limits, Local, Memarg, MemoryType, Mutability, Section, TableType, ValType, WithSize};

// TODO parallelize conversion from/to low-level AST with rayon
// TODO streaming AST: replace Vec's with iterators, where possible, in particular: Expr
// TODO avoid high-level/low-level split, read to high-level directly

/// convenience for adding elements
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

/* High-level AST:
    - types are inlined instead of referenced by type idx (i.e., no manual handling of Type "pool")
    - Function + Code sections are merged into one list of functions,
      same for tables: Table + Element sections and memories: Memory + Data sections.
    - imports and exports are part of the respective item, not stored externally and referring to
      their item by index.
*/

#[derive(Debug, Clone)]
pub struct Module {
    functions: Vec<Function>,
    tables: Vec<Table>,
    memories: Vec<Memory>,
    globals: Vec<Global>,

    pub start: Option<Idx<Function>>,

    custom_sections: Vec<Vec<u8>>,
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


/* Visitors for transforming instructions without having to recurse manually over the tree.
   bottom_up: goes first into the leaves (i.e., instrs or exprs) and applies f,
              and then applies f to the current level again (== post-order).
   TODO top_down
   TODO down_up: does first top_down, then bottom_up (i.e., taking two closures)
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


/* Other helpful functions on highlevel AST */

impl Instr {
    pub fn is_call(&self) -> bool {
        match *self {
            Instr::Call(_) => true,
            Instr::CallIndirect(_, _) => true,
            _ => false
        }
    }
}


// TODO move conversions between high and low-level into own convert module

/* Convert from low-level to high-level AST. */

impl From<ll::Module> for Module {
    fn from(ll::Module { sections }: ll::Module) -> Self {
        let mut module = Module {
            functions: Vec::new(),
            tables: Vec::new(),
            memories: Vec::new(),
            globals: Vec::new(),

            start: None,

            custom_sections: Vec::new(),
        };

        let mut types: Vec<FunctionType> = Vec::new();

        for section in sections {
            match section {
                Section::Custom(bytes) => module.custom_sections.push(bytes),
                Section::Type(WithSize(types_)) => types = types_,

                /* Imported functions, tables, memories, and globals are first added to the respective index spaces... */

                Section::Import(vec) => {
                    for import_ in vec.0 {
                        let import = Some((import_.module, import_.name));
                        let export = None;
                        match import_.type_ {
                            ll::ImportType::Function(type_idx) => module.functions.push(Function {
                                type_: types[type_idx.0].clone(),
                                import,
                                code: None,
                                export,
                            }),
                            ll::ImportType::Table(type_) => module.tables.push(Table {
                                type_,
                                import,
                                elements: Vec::new(),
                                export,
                            }),
                            ll::ImportType::Memory(type_) => module.memories.push(Memory {
                                type_,
                                import,
                                data: Vec::new(),
                                export,
                            }),
                            ll::ImportType::Global(type_) => module.globals.push(Global {
                                type_,
                                import,
                                init: None,
                                export,
                            }),
                        }
                    }
                }

                /* Then all "local" (i.e., non-imported) functions/tables/memories/globals are added. */

                Section::Function(WithSize(function_signatures)) => {
                    for type_idx in function_signatures {
                        module.functions.push(Function {
                            type_: types[type_idx.0].clone(),
                            import: None,
                            code: None,
                            export: None,
                        });
                    }
                }
                Section::Table(WithSize(tables)) => {
                    for type_ in tables {
                        module.tables.push(Table {
                            type_,
                            import: None,
                            elements: Vec::new(),
                            export: None,
                        });
                    }
                }
                Section::Memory(WithSize(memories)) => {
                    for type_ in memories {
                        module.memories.push(Memory {
                            type_,
                            import: None,
                            data: Vec::new(),
                            export: None,
                        });
                    }
                }
                Section::Global(WithSize(globals)) => {
                    for ll::Global { type_, init } in globals {
                        module.globals.push(Global {
                            type_,
                            import: None,
                            init: Some(from_lowlevel_expr(init, &types)),
                            export: None,
                        });
                    }
                }

                /* Other metadata sections: Export, Start */

                Section::Export(WithSize(exports)) => {
                    for ll::Export { name, type_ } in exports {
                        let export = Some(name);
                        match type_ {
                            ll::ExportType::Function(idx) => module.functions[idx.0].export = export,
                            ll::ExportType::Table(idx) => module.tables[idx.0].export = export,
                            ll::ExportType::Memory(idx) => module.memories[idx.0].export = export,
                            ll::ExportType::Global(idx) => module.globals[idx.0].export = export,
                        }
                    }
                }
                Section::Start(WithSize(function_idx)) => module.start = Some(function_idx.0.into()),

                /* Finally, all "contents" of the already declared functions/tables/memories. */

                Section::Element(WithSize(elements)) => {
                    for element in elements {
                        module.tables[element.table_idx.0].elements.push(Element {
                            offset: from_lowlevel_expr(element.offset, &types),
                            functions: element.init.into_iter().map(|idx| idx.0.into()).collect(),
                        })
                    }
                }
                Section::Code(WithSize(code)) => {
                    let imported_function_count = module.functions.iter()
                        .filter(|f| f.import.is_some())
                        .count();
                    for (i, WithSize(code)) in code.into_iter().enumerate() {
                        module.functions[imported_function_count + i].code =
                            Some(from_lowlevel_code(code, &types))
                    }
                }
                Section::Data(WithSize(data)) => {
                    for data in data {
                        module.memories[data.memory_idx.0].data.push(Data {
                            offset: from_lowlevel_expr(data.offset, &types),
                            bytes: data.init,
                        })
                    }
                }
            }
        }

        module
    }
}

fn from_lowlevel_code(code: ll::Code, types: &[FunctionType]) -> Code {
    let mut locals = Vec::new();
    for local in code.locals {
        for _ in 0..local.count {
            locals.push(local.type_);
        }
    }
    Code {
        locals,
        body: from_lowlevel_expr(code.body, types),
    }
}

fn from_lowlevel_expr(expr: ll::Expr, types: &[FunctionType]) -> Expr {
    expr.0.into_iter().map(|instr| from_lowlevel_instr(instr, types)).collect()
}


fn from_lowlevel_instr(instr: ll::Instr, types: &[FunctionType]) -> Instr {
    match instr {
        ll::Instr::Unreachable => Instr::Unreachable,
        ll::Instr::Nop => Instr::Nop,

        ll::Instr::Block(block_type, expr) => Instr::Block(block_type, from_lowlevel_expr(expr, &types)),
        ll::Instr::Loop(block_type, expr) => Instr::Loop(block_type, from_lowlevel_expr(expr, &types)),
        ll::Instr::If(block_type, expr) => Instr::If(block_type, from_lowlevel_expr(expr, &types)),
        ll::Instr::Else(expr) => Instr::Else(from_lowlevel_expr(expr, &types)),
        ll::Instr::End => Instr::End,

        ll::Instr::Br(label_idx) => Instr::Br(label_idx),
        ll::Instr::BrIf(label_idx) => Instr::BrIf(label_idx),
        ll::Instr::BrTable(label_idx_table, default) => Instr::BrTable(label_idx_table, default),

        ll::Instr::Return => Instr::Return,
        ll::Instr::Call(function_idx) => Instr::Call(function_idx.0.into()),
        ll::Instr::CallIndirect(type_idx, table_idx) => Instr::CallIndirect(types[type_idx.0].clone(), table_idx.0.into()),

        ll::Instr::Drop => Instr::Drop,
        ll::Instr::Select => Instr::Select,

        ll::Instr::GetLocal(local_idx) => Instr::GetLocal(local_idx.0.into()),
        ll::Instr::SetLocal(local_idx) => Instr::SetLocal(local_idx.0.into()),
        ll::Instr::TeeLocal(local_idx) => Instr::TeeLocal(local_idx.0.into()),
        ll::Instr::GetGlobal(global_idx) => Instr::GetGlobal(global_idx.0.into()),
        ll::Instr::SetGlobal(global_idx) => Instr::SetGlobal(global_idx.0.into()),

        ll::Instr::I32Load(memarg) => Instr::I32Load(memarg),
        ll::Instr::I64Load(memarg) => Instr::I64Load(memarg),
        ll::Instr::F32Load(memarg) => Instr::F32Load(memarg),
        ll::Instr::F64Load(memarg) => Instr::F64Load(memarg),
        ll::Instr::I32Load8S(memarg) => Instr::I32Load8S(memarg),
        ll::Instr::I32Load8U(memarg) => Instr::I32Load8U(memarg),
        ll::Instr::I32Load16S(memarg) => Instr::I32Load16S(memarg),
        ll::Instr::I32Load16U(memarg) => Instr::I32Load16U(memarg),
        ll::Instr::I64Load8S(memarg) => Instr::I64Load8S(memarg),
        ll::Instr::I64Load8U(memarg) => Instr::I64Load8U(memarg),
        ll::Instr::I64Load16S(memarg) => Instr::I64Load16S(memarg),
        ll::Instr::I64Load16U(memarg) => Instr::I64Load16U(memarg),
        ll::Instr::I64Load32S(memarg) => Instr::I64Load32S(memarg),
        ll::Instr::I64Load32U(memarg) => Instr::I64Load32U(memarg),
        ll::Instr::I32Store(memarg) => Instr::I32Store(memarg),
        ll::Instr::I64Store(memarg) => Instr::I64Store(memarg),
        ll::Instr::F32Store(memarg) => Instr::F32Store(memarg),
        ll::Instr::F64Store(memarg) => Instr::F64Store(memarg),
        ll::Instr::I32Store8(memarg) => Instr::I32Store8(memarg),
        ll::Instr::I32Store16(memarg) => Instr::I32Store16(memarg),
        ll::Instr::I64Store8(memarg) => Instr::I64Store8(memarg),
        ll::Instr::I64Store16(memarg) => Instr::I64Store16(memarg),
        ll::Instr::I64Store32(memarg) => Instr::I64Store32(memarg),

        ll::Instr::CurrentMemory(memory_idx) => Instr::CurrentMemory(memory_idx.0.into()),
        ll::Instr::GrowMemory(memory_idx) => Instr::GrowMemory(memory_idx.0.into()),

        ll::Instr::I32Const(immediate) => Instr::I32Const(immediate),
        ll::Instr::I64Const(immediate) => Instr::I64Const(immediate),
        ll::Instr::F32Const(immediate) => Instr::F32Const(immediate),
        ll::Instr::F64Const(immediate) => Instr::F64Const(immediate),

        ll::Instr::I32Eqz => Instr::I32Eqz,
        ll::Instr::I32Eq => Instr::I32Eq,
        ll::Instr::I32Ne => Instr::I32Ne,
        ll::Instr::I32LtS => Instr::I32LtS,
        ll::Instr::I32LtU => Instr::I32LtU,
        ll::Instr::I32GtS => Instr::I32GtS,
        ll::Instr::I32GtU => Instr::I32GtU,
        ll::Instr::I32LeS => Instr::I32LeS,
        ll::Instr::I32LeU => Instr::I32LeU,
        ll::Instr::I32GeS => Instr::I32GeS,
        ll::Instr::I32GeU => Instr::I32GeU,
        ll::Instr::I64Eqz => Instr::I64Eqz,
        ll::Instr::I64Eq => Instr::I64Eq,
        ll::Instr::I64Ne => Instr::I64Ne,
        ll::Instr::I64LtS => Instr::I64LtS,
        ll::Instr::I64LtU => Instr::I64LtU,
        ll::Instr::I64GtS => Instr::I64GtS,
        ll::Instr::I64GtU => Instr::I64GtU,
        ll::Instr::I64LeS => Instr::I64LeS,
        ll::Instr::I64LeU => Instr::I64LeU,
        ll::Instr::I64GeS => Instr::I64GeS,
        ll::Instr::I64GeU => Instr::I64GeU,
        ll::Instr::F32Eq => Instr::F32Eq,
        ll::Instr::F32Ne => Instr::F32Ne,
        ll::Instr::F32Lt => Instr::F32Lt,
        ll::Instr::F32Gt => Instr::F32Gt,
        ll::Instr::F32Le => Instr::F32Le,
        ll::Instr::F32Ge => Instr::F32Ge,
        ll::Instr::F64Eq => Instr::F64Eq,
        ll::Instr::F64Ne => Instr::F64Ne,
        ll::Instr::F64Lt => Instr::F64Lt,
        ll::Instr::F64Gt => Instr::F64Gt,
        ll::Instr::F64Le => Instr::F64Le,
        ll::Instr::F64Ge => Instr::F64Ge,
        ll::Instr::I32Clz => Instr::I32Clz,
        ll::Instr::I32Ctz => Instr::I32Ctz,
        ll::Instr::I32Popcnt => Instr::I32Popcnt,
        ll::Instr::I32Add => Instr::I32Add,
        ll::Instr::I32Sub => Instr::I32Sub,
        ll::Instr::I32Mul => Instr::I32Mul,
        ll::Instr::I32DivS => Instr::I32DivS,
        ll::Instr::I32DivU => Instr::I32DivU,
        ll::Instr::I32RemS => Instr::I32RemS,
        ll::Instr::I32RemU => Instr::I32RemU,
        ll::Instr::I32And => Instr::I32And,
        ll::Instr::I32Or => Instr::I32Or,
        ll::Instr::I32Xor => Instr::I32Xor,
        ll::Instr::I32Shl => Instr::I32Shl,
        ll::Instr::I32ShrS => Instr::I32ShrS,
        ll::Instr::I32ShrU => Instr::I32ShrU,
        ll::Instr::I32Rotl => Instr::I32Rotl,
        ll::Instr::I32Rotr => Instr::I32Rotr,
        ll::Instr::I64Clz => Instr::I64Clz,
        ll::Instr::I64Ctz => Instr::I64Ctz,
        ll::Instr::I64Popcnt => Instr::I64Popcnt,
        ll::Instr::I64Add => Instr::I64Add,
        ll::Instr::I64Sub => Instr::I64Sub,
        ll::Instr::I64Mul => Instr::I64Mul,
        ll::Instr::I64DivS => Instr::I64DivS,
        ll::Instr::I64DivU => Instr::I64DivU,
        ll::Instr::I64RemS => Instr::I64RemS,
        ll::Instr::I64RemU => Instr::I64RemU,
        ll::Instr::I64And => Instr::I64And,
        ll::Instr::I64Or => Instr::I64Or,
        ll::Instr::I64Xor => Instr::I64Xor,
        ll::Instr::I64Shl => Instr::I64Shl,
        ll::Instr::I64ShrS => Instr::I64ShrS,
        ll::Instr::I64ShrU => Instr::I64ShrU,
        ll::Instr::I64Rotl => Instr::I64Rotl,
        ll::Instr::I64Rotr => Instr::I64Rotr,
        ll::Instr::F32Abs => Instr::F32Abs,
        ll::Instr::F32Neg => Instr::F32Neg,
        ll::Instr::F32Ceil => Instr::F32Ceil,
        ll::Instr::F32Floor => Instr::F32Floor,
        ll::Instr::F32Trunc => Instr::F32Trunc,
        ll::Instr::F32Nearest => Instr::F32Nearest,
        ll::Instr::F32Sqrt => Instr::F32Sqrt,
        ll::Instr::F32Add => Instr::F32Add,
        ll::Instr::F32Sub => Instr::F32Sub,
        ll::Instr::F32Mul => Instr::F32Mul,
        ll::Instr::F32Div => Instr::F32Div,
        ll::Instr::F32Min => Instr::F32Min,
        ll::Instr::F32Max => Instr::F32Max,
        ll::Instr::F32Copysign => Instr::F32Copysign,
        ll::Instr::F64Abs => Instr::F64Abs,
        ll::Instr::F64Neg => Instr::F64Neg,
        ll::Instr::F64Ceil => Instr::F64Ceil,
        ll::Instr::F64Floor => Instr::F64Floor,
        ll::Instr::F64Trunc => Instr::F64Trunc,
        ll::Instr::F64Nearest => Instr::F64Nearest,
        ll::Instr::F64Sqrt => Instr::F64Sqrt,
        ll::Instr::F64Add => Instr::F64Add,
        ll::Instr::F64Sub => Instr::F64Sub,
        ll::Instr::F64Mul => Instr::F64Mul,
        ll::Instr::F64Div => Instr::F64Div,
        ll::Instr::F64Min => Instr::F64Min,
        ll::Instr::F64Max => Instr::F64Max,
        ll::Instr::F64Copysign => Instr::F64Copysign,
        ll::Instr::I32WrapI64 => Instr::I32WrapI64,
        ll::Instr::I32TruncSF32 => Instr::I32TruncSF32,
        ll::Instr::I32TruncUF32 => Instr::I32TruncUF32,
        ll::Instr::I32TruncSF64 => Instr::I32TruncSF64,
        ll::Instr::I32TruncUF64 => Instr::I32TruncUF64,
        ll::Instr::I64ExtendSI32 => Instr::I64ExtendSI32,
        ll::Instr::I64ExtendUI32 => Instr::I64ExtendUI32,
        ll::Instr::I64TruncSF32 => Instr::I64TruncSF32,
        ll::Instr::I64TruncUF32 => Instr::I64TruncUF32,
        ll::Instr::I64TruncSF64 => Instr::I64TruncSF64,
        ll::Instr::I64TruncUF64 => Instr::I64TruncUF64,
        ll::Instr::F32ConvertSI32 => Instr::F32ConvertSI32,
        ll::Instr::F32ConvertUI32 => Instr::F32ConvertUI32,
        ll::Instr::F32ConvertSI64 => Instr::F32ConvertSI64,
        ll::Instr::F32ConvertUI64 => Instr::F32ConvertUI64,
        ll::Instr::F32DemoteF64 => Instr::F32DemoteF64,
        ll::Instr::F64ConvertSI32 => Instr::F64ConvertSI32,
        ll::Instr::F64ConvertUI32 => Instr::F64ConvertUI32,
        ll::Instr::F64ConvertSI64 => Instr::F64ConvertSI64,
        ll::Instr::F64ConvertUI64 => Instr::F64ConvertUI64,
        ll::Instr::F64PromoteF32 => Instr::F64PromoteF32,
        ll::Instr::I32ReinterpretF32 => Instr::I32ReinterpretF32,
        ll::Instr::I64ReinterpretF64 => Instr::I64ReinterpretF64,
        ll::Instr::F32ReinterpretI32 => Instr::F32ReinterpretI32,
        ll::Instr::F64ReinterpretI64 => Instr::F64ReinterpretI64,
    }
}


/* Convert from high-level to low-level AST. */

struct EncodeState {
    types: HashMap<FunctionType, usize>,
    function_idx: HashMap<usize, usize>,
    table_idx: HashMap<usize, usize>,
    memory_idx: HashMap<usize, usize>,
    global_idx: HashMap<usize, usize>,
}

macro_rules! element_idx_fns {
    ($insert_fn: ident, $map_fn: ident, $field: ident, $ll_ty: ty) => {
        fn $insert_fn(&mut self, old_idx: usize) {
            let new_idx = self.$field.len();
            self.$field.insert(old_idx, new_idx);
        }
        fn $map_fn(&self, old_idx: usize) -> Idx<$ll_ty> {
            self.$field[&old_idx].into()
        }
    };
}

impl EncodeState {
    fn get_or_insert_type(&mut self, type_: FunctionType) -> Idx<FunctionType> {
        let new_idx = self.types.len();
        (*self.types.entry(type_).or_insert(new_idx)).into()
    }
    fn get_type_idx(&self, type_: &FunctionType) -> Idx<FunctionType> {
        (*self.types.get(type_).unwrap()).into()
    }

    element_idx_fns!(insert_function_idx, map_function_idx, function_idx, ll::Function);
    element_idx_fns!(insert_table_idx, map_table_idx, table_idx, ll::Table);
    element_idx_fns!(insert_memory_idx, map_memory_idx, memory_idx, ll::Memory);
    element_idx_fns!(insert_global_idx, map_global_idx, global_idx, ll::Global);
}

impl From<Module> for ll::Module {
    fn from(module: Module) -> Self {
        let mut sections = Vec::new();

        let mut state = EncodeState {
            types: HashMap::new(),
            function_idx: HashMap::new(),
            table_idx: HashMap::new(),
            memory_idx: HashMap::new(),
            global_idx: HashMap::new(),
        };

        let imports = to_lowlevel_imports(&module, &mut state);
        let functions = to_lowlevel_functions(&module.functions, &mut state);
        let tables = to_lowlevel_tables(&module.tables, &mut state);
        let memories = to_lowlevel_memories(&module.memories, &mut state);
        let globals = to_lowlevel_globals(&module.globals, &mut state);

        /* All types and indices are now determined, so we can start writing out sections... */

        // Type
        let mut types = state.types.iter().collect::<Vec<_>>();
        types.sort_unstable_by_key(|&(_, idx)| idx);
        let types = types.into_iter()
            .map(|(type_, _)| type_.clone())
            .collect::<Vec<FunctionType>>();
        sections.push(Section::Type(WithSize(types)));

        // Import
        sections.push(Section::Import(WithSize(imports)));

        // Function
        sections.push(Section::Function(WithSize(functions)));

        // Table
        sections.push(Section::Table(WithSize(tables)));

        // Memory
        sections.push(Section::Memory(WithSize(memories)));

        // Global
        sections.push(Section::Global(WithSize(globals)));

        // Export
        let exports = to_lowlevel_exports(&module, &state);
        sections.push(Section::Export(WithSize(exports)));

        // Start
        for start in module.start {
            sections.push(Section::Start(WithSize(state.map_function_idx(start.0))));
        }

        // Element
        let elements = module.tables.into_iter()
            .enumerate()
            .flat_map(|(i, table)| table.elements.into_iter()
                .map(|element| ll::Element {
                    table_idx: state.map_table_idx(i),
                    offset: to_lowlevel_expr(&element.offset, &state),
                    init: element.functions.iter().map(|fn_idx| state.map_function_idx(fn_idx.0)).collect(),
                })
                .collect::<Vec<_>>())
            .collect();
        sections.push(Section::Element(WithSize(elements)));

        // Code
        let code = module.functions.into_iter()
            .filter_map(|function|
                function.code.map(|code| WithSize(to_lowlevel_code(code, &state))))
            .collect();
        sections.push(Section::Code(WithSize(code)));

        // Data
        let data = module.memories.into_iter()
            .enumerate()
            .flat_map(|(i, memory)| memory.data.into_iter()
                .map(|data| ll::Data {
                    memory_idx: state.map_memory_idx(i),
                    offset: to_lowlevel_expr(&data.offset, &state),
                    init: data.bytes,
                })
                .collect::<Vec<_>>())
            .collect();
        sections.push(Section::Data(WithSize(data)));

        // Custom
        for custom in module.custom_sections {
            sections.push(Section::Custom(custom));
        }

        ll::Module { sections }
    }
}

fn to_lowlevel_imports(module: &Module, state: &mut EncodeState) -> Vec<ll::Import> {
    let mut imports = Vec::new();

    macro_rules! add_imports {
        ($elems: ident, $insert_idx_fn: ident, $import_ty_variant: ident, $ty_transform: expr) => {
            imports.extend(module.$elems.iter()
                .enumerate()
                .filter_map(|(i, element)|
                    element.import.as_ref().map(|&(ref module, ref name)| {
                        state.$insert_idx_fn(i);
                        ll::Import {
                            module: module.clone(),
                            name: name.clone(),
                            type_: ll::ImportType::$import_ty_variant($ty_transform(element.type_.clone())),
                        }
                    })));
        };
    }
    add_imports!(functions, insert_function_idx, Function, |ty| state.get_or_insert_type(ty));
    add_imports!(tables, insert_table_idx, Table, |ty| ty);
    add_imports!(memories, insert_memory_idx, Memory, |ty| ty);
    add_imports!(globals, insert_global_idx, Global, |ty| ty);

    imports
}

macro_rules! to_lowlevel_elements {
    ($elems: expr, $state: ident, $insert_idx_fn: ident, $elem_transform: expr) => {
        $elems.iter()
            .enumerate()
            .filter(|&(_, element)| element.import.is_none())
            .map(|(i, element)| {
                $state.$insert_idx_fn(i);
                $elem_transform(&element)
            })
            .collect()
    };
}

fn to_lowlevel_functions(functions: &[Function], state: &mut EncodeState) -> Vec<Idx<FunctionType>> {
    to_lowlevel_elements!(functions, state, insert_function_idx, |func: &Function| state.get_or_insert_type(func.type_.clone()))
}

fn to_lowlevel_tables(tables: &[Table], state: &mut EncodeState) -> Vec<TableType> {
    to_lowlevel_elements!(tables, state, insert_table_idx, |table: &Table| table.type_.clone())
}

fn to_lowlevel_memories(memories: &[Memory], state: &mut EncodeState) -> Vec<MemoryType> {
    to_lowlevel_elements!(memories, state, insert_memory_idx, |memory: &Memory| memory.type_.clone())
}

fn to_lowlevel_globals(globals: &[Global], state: &mut EncodeState) -> Vec<ll::Global> {
    to_lowlevel_elements!(globals, state, insert_global_idx, |global: &Global| ll::Global {
        type_: global.type_,
        init: to_lowlevel_expr(&global.init.as_ref().unwrap(), state),
    })
}

fn to_lowlevel_exports(module: &Module, state: &EncodeState) -> Vec<ll::Export> {
    let mut exports = Vec::new();

    macro_rules! add_exports {
        ($elems: ident, $map_idx_fn: ident, $export_ty_variant: ident) => {
            exports.extend(module.$elems.iter()
                .enumerate()
                .filter_map(|(i, element)|
                    element.export.as_ref().map(|name| ll::Export {
                        name: name.clone(),
                        type_: ll::ExportType::$export_ty_variant(state.$map_idx_fn(i)),
                    })));
        };
    }
    add_exports!(functions, map_function_idx, Function);
    add_exports!(tables, map_table_idx, Table);
    add_exports!(memories, map_memory_idx, Memory);
    add_exports!(globals, map_global_idx, Global);

    exports
}

fn to_lowlevel_code(code: Code, state: &EncodeState) -> ll::Code {
    let mut locals = Vec::new();
    for type_ in code.locals {
        if locals.first().map(|locals: &ll::Locals| locals.type_ == type_).unwrap_or(false) {
            locals[0].count += 1;
        } else {
            locals.push(ll::Locals {
                count: 1,
                type_,
            })
        }
    }

    ll::Code {
        locals,
        body: to_lowlevel_expr(&code.body, state),
    }
}

fn to_lowlevel_expr(expr: &[Instr], state: &EncodeState) -> ll::Expr {
    ll::Expr(expr.iter().map(|instr| to_lowlevel_instr(instr, state)).collect())
}

fn to_lowlevel_instr(instr: &Instr, state: &EncodeState) -> ll::Instr {
    match *instr {
        Instr::Unreachable => ll::Instr::Unreachable,
        Instr::Nop => ll::Instr::Nop,

        Instr::Block(block_type, ref expr) => ll::Instr::Block(block_type, to_lowlevel_expr(&expr, state)),
        Instr::Loop(block_type, ref expr) => ll::Instr::Loop(block_type, to_lowlevel_expr(&expr, state)),
        Instr::If(block_type, ref expr) => ll::Instr::If(block_type, to_lowlevel_expr(&expr, state)),
        Instr::Else(ref expr) => ll::Instr::Else(to_lowlevel_expr(&expr, state)),
        Instr::End => ll::Instr::End,

        Instr::Br(label_idx) => ll::Instr::Br(label_idx),
        Instr::BrIf(label_idx) => ll::Instr::BrIf(label_idx),
        Instr::BrTable(ref label_idx_table, default) => ll::Instr::BrTable(label_idx_table.clone(), default),

        Instr::Return => ll::Instr::Return,
        Instr::Call(function_idx) => ll::Instr::Call(state.map_function_idx(function_idx.0)),
        Instr::CallIndirect(ref type_, table_idx) => ll::Instr::CallIndirect(state.get_type_idx(&type_), state.map_table_idx(table_idx.0)),

        Instr::Drop => ll::Instr::Drop,
        Instr::Select => ll::Instr::Select,

        Instr::GetLocal(local_idx) => ll::Instr::GetLocal(local_idx.0.into()),
        Instr::SetLocal(local_idx) => ll::Instr::SetLocal(local_idx.0.into()),
        Instr::TeeLocal(local_idx) => ll::Instr::TeeLocal(local_idx.0.into()),
        Instr::GetGlobal(global_idx) => ll::Instr::GetGlobal(state.map_global_idx(global_idx.0)),
        Instr::SetGlobal(global_idx) => ll::Instr::SetGlobal(state.map_global_idx(global_idx.0)),

        Instr::I32Load(memarg) => ll::Instr::I32Load(memarg),
        Instr::I64Load(memarg) => ll::Instr::I64Load(memarg),
        Instr::F32Load(memarg) => ll::Instr::F32Load(memarg),
        Instr::F64Load(memarg) => ll::Instr::F64Load(memarg),
        Instr::I32Load8S(memarg) => ll::Instr::I32Load8S(memarg),
        Instr::I32Load8U(memarg) => ll::Instr::I32Load8U(memarg),
        Instr::I32Load16S(memarg) => ll::Instr::I32Load16S(memarg),
        Instr::I32Load16U(memarg) => ll::Instr::I32Load16U(memarg),
        Instr::I64Load8S(memarg) => ll::Instr::I64Load8S(memarg),
        Instr::I64Load8U(memarg) => ll::Instr::I64Load8U(memarg),
        Instr::I64Load16S(memarg) => ll::Instr::I64Load16S(memarg),
        Instr::I64Load16U(memarg) => ll::Instr::I64Load16U(memarg),
        Instr::I64Load32S(memarg) => ll::Instr::I64Load32S(memarg),
        Instr::I64Load32U(memarg) => ll::Instr::I64Load32U(memarg),
        Instr::I32Store(memarg) => ll::Instr::I32Store(memarg),
        Instr::I64Store(memarg) => ll::Instr::I64Store(memarg),
        Instr::F32Store(memarg) => ll::Instr::F32Store(memarg),
        Instr::F64Store(memarg) => ll::Instr::F64Store(memarg),
        Instr::I32Store8(memarg) => ll::Instr::I32Store8(memarg),
        Instr::I32Store16(memarg) => ll::Instr::I32Store16(memarg),
        Instr::I64Store8(memarg) => ll::Instr::I64Store8(memarg),
        Instr::I64Store16(memarg) => ll::Instr::I64Store16(memarg),
        Instr::I64Store32(memarg) => ll::Instr::I64Store32(memarg),

        Instr::CurrentMemory(memory_idx) => ll::Instr::CurrentMemory(state.map_memory_idx(memory_idx.0)),
        Instr::GrowMemory(memory_idx) => ll::Instr::GrowMemory(state.map_memory_idx(memory_idx.0)),

        Instr::I32Const(immediate) => ll::Instr::I32Const(immediate),
        Instr::I64Const(immediate) => ll::Instr::I64Const(immediate),
        Instr::F32Const(immediate) => ll::Instr::F32Const(immediate),
        Instr::F64Const(immediate) => ll::Instr::F64Const(immediate),

        Instr::I32Eqz => ll::Instr::I32Eqz,
        Instr::I32Eq => ll::Instr::I32Eq,
        Instr::I32Ne => ll::Instr::I32Ne,
        Instr::I32LtS => ll::Instr::I32LtS,
        Instr::I32LtU => ll::Instr::I32LtU,
        Instr::I32GtS => ll::Instr::I32GtS,
        Instr::I32GtU => ll::Instr::I32GtU,
        Instr::I32LeS => ll::Instr::I32LeS,
        Instr::I32LeU => ll::Instr::I32LeU,
        Instr::I32GeS => ll::Instr::I32GeS,
        Instr::I32GeU => ll::Instr::I32GeU,
        Instr::I64Eqz => ll::Instr::I64Eqz,
        Instr::I64Eq => ll::Instr::I64Eq,
        Instr::I64Ne => ll::Instr::I64Ne,
        Instr::I64LtS => ll::Instr::I64LtS,
        Instr::I64LtU => ll::Instr::I64LtU,
        Instr::I64GtS => ll::Instr::I64GtS,
        Instr::I64GtU => ll::Instr::I64GtU,
        Instr::I64LeS => ll::Instr::I64LeS,
        Instr::I64LeU => ll::Instr::I64LeU,
        Instr::I64GeS => ll::Instr::I64GeS,
        Instr::I64GeU => ll::Instr::I64GeU,
        Instr::F32Eq => ll::Instr::F32Eq,
        Instr::F32Ne => ll::Instr::F32Ne,
        Instr::F32Lt => ll::Instr::F32Lt,
        Instr::F32Gt => ll::Instr::F32Gt,
        Instr::F32Le => ll::Instr::F32Le,
        Instr::F32Ge => ll::Instr::F32Ge,
        Instr::F64Eq => ll::Instr::F64Eq,
        Instr::F64Ne => ll::Instr::F64Ne,
        Instr::F64Lt => ll::Instr::F64Lt,
        Instr::F64Gt => ll::Instr::F64Gt,
        Instr::F64Le => ll::Instr::F64Le,
        Instr::F64Ge => ll::Instr::F64Ge,
        Instr::I32Clz => ll::Instr::I32Clz,
        Instr::I32Ctz => ll::Instr::I32Ctz,
        Instr::I32Popcnt => ll::Instr::I32Popcnt,
        Instr::I32Add => ll::Instr::I32Add,
        Instr::I32Sub => ll::Instr::I32Sub,
        Instr::I32Mul => ll::Instr::I32Mul,
        Instr::I32DivS => ll::Instr::I32DivS,
        Instr::I32DivU => ll::Instr::I32DivU,
        Instr::I32RemS => ll::Instr::I32RemS,
        Instr::I32RemU => ll::Instr::I32RemU,
        Instr::I32And => ll::Instr::I32And,
        Instr::I32Or => ll::Instr::I32Or,
        Instr::I32Xor => ll::Instr::I32Xor,
        Instr::I32Shl => ll::Instr::I32Shl,
        Instr::I32ShrS => ll::Instr::I32ShrS,
        Instr::I32ShrU => ll::Instr::I32ShrU,
        Instr::I32Rotl => ll::Instr::I32Rotl,
        Instr::I32Rotr => ll::Instr::I32Rotr,
        Instr::I64Clz => ll::Instr::I64Clz,
        Instr::I64Ctz => ll::Instr::I64Ctz,
        Instr::I64Popcnt => ll::Instr::I64Popcnt,
        Instr::I64Add => ll::Instr::I64Add,
        Instr::I64Sub => ll::Instr::I64Sub,
        Instr::I64Mul => ll::Instr::I64Mul,
        Instr::I64DivS => ll::Instr::I64DivS,
        Instr::I64DivU => ll::Instr::I64DivU,
        Instr::I64RemS => ll::Instr::I64RemS,
        Instr::I64RemU => ll::Instr::I64RemU,
        Instr::I64And => ll::Instr::I64And,
        Instr::I64Or => ll::Instr::I64Or,
        Instr::I64Xor => ll::Instr::I64Xor,
        Instr::I64Shl => ll::Instr::I64Shl,
        Instr::I64ShrS => ll::Instr::I64ShrS,
        Instr::I64ShrU => ll::Instr::I64ShrU,
        Instr::I64Rotl => ll::Instr::I64Rotl,
        Instr::I64Rotr => ll::Instr::I64Rotr,
        Instr::F32Abs => ll::Instr::F32Abs,
        Instr::F32Neg => ll::Instr::F32Neg,
        Instr::F32Ceil => ll::Instr::F32Ceil,
        Instr::F32Floor => ll::Instr::F32Floor,
        Instr::F32Trunc => ll::Instr::F32Trunc,
        Instr::F32Nearest => ll::Instr::F32Nearest,
        Instr::F32Sqrt => ll::Instr::F32Sqrt,
        Instr::F32Add => ll::Instr::F32Add,
        Instr::F32Sub => ll::Instr::F32Sub,
        Instr::F32Mul => ll::Instr::F32Mul,
        Instr::F32Div => ll::Instr::F32Div,
        Instr::F32Min => ll::Instr::F32Min,
        Instr::F32Max => ll::Instr::F32Max,
        Instr::F32Copysign => ll::Instr::F32Copysign,
        Instr::F64Abs => ll::Instr::F64Abs,
        Instr::F64Neg => ll::Instr::F64Neg,
        Instr::F64Ceil => ll::Instr::F64Ceil,
        Instr::F64Floor => ll::Instr::F64Floor,
        Instr::F64Trunc => ll::Instr::F64Trunc,
        Instr::F64Nearest => ll::Instr::F64Nearest,
        Instr::F64Sqrt => ll::Instr::F64Sqrt,
        Instr::F64Add => ll::Instr::F64Add,
        Instr::F64Sub => ll::Instr::F64Sub,
        Instr::F64Mul => ll::Instr::F64Mul,
        Instr::F64Div => ll::Instr::F64Div,
        Instr::F64Min => ll::Instr::F64Min,
        Instr::F64Max => ll::Instr::F64Max,
        Instr::F64Copysign => ll::Instr::F64Copysign,
        Instr::I32WrapI64 => ll::Instr::I32WrapI64,
        Instr::I32TruncSF32 => ll::Instr::I32TruncSF32,
        Instr::I32TruncUF32 => ll::Instr::I32TruncUF32,
        Instr::I32TruncSF64 => ll::Instr::I32TruncSF64,
        Instr::I32TruncUF64 => ll::Instr::I32TruncUF64,
        Instr::I64ExtendSI32 => ll::Instr::I64ExtendSI32,
        Instr::I64ExtendUI32 => ll::Instr::I64ExtendUI32,
        Instr::I64TruncSF32 => ll::Instr::I64TruncSF32,
        Instr::I64TruncUF32 => ll::Instr::I64TruncUF32,
        Instr::I64TruncSF64 => ll::Instr::I64TruncSF64,
        Instr::I64TruncUF64 => ll::Instr::I64TruncUF64,
        Instr::F32ConvertSI32 => ll::Instr::F32ConvertSI32,
        Instr::F32ConvertUI32 => ll::Instr::F32ConvertUI32,
        Instr::F32ConvertSI64 => ll::Instr::F32ConvertSI64,
        Instr::F32ConvertUI64 => ll::Instr::F32ConvertUI64,
        Instr::F32DemoteF64 => ll::Instr::F32DemoteF64,
        Instr::F64ConvertSI32 => ll::Instr::F64ConvertSI32,
        Instr::F64ConvertUI32 => ll::Instr::F64ConvertUI32,
        Instr::F64ConvertSI64 => ll::Instr::F64ConvertSI64,
        Instr::F64ConvertUI64 => ll::Instr::F64ConvertUI64,
        Instr::F64PromoteF32 => ll::Instr::F64PromoteF32,
        Instr::I32ReinterpretF32 => ll::Instr::I32ReinterpretF32,
        Instr::I64ReinterpretF64 => ll::Instr::I64ReinterpretF64,
        Instr::F32ReinterpretI32 => ll::Instr::F32ReinterpretI32,
        Instr::F64ReinterpretI64 => ll::Instr::F64ReinterpretI64,
    }
}