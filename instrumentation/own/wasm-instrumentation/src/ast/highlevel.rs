// TODO "streaming AST" API: return Module {} after reading only the first 8 bytes, implement
// Iterator<Item = Section> for Module -> Module must somehow retain the reader to do so...

use std::cell::Cell;
use std::rc::Rc;
use super::lowlevel as ll;
// reuse as much as possible from the low-level AST
pub use super::lowlevel::{BlockType, ElemType, FunctionType, GlobalType, Label, Limits, Memarg, MemoryType, Mutability, TableType, ValType};

//fn test() {
//    let mut empty_module = Module {
//        start: None,
//        imports: Vec::new(),
//        exports: Vec::new(),
//
//        functions: Arena::new(),
//        tables: Arena::new(),
//        memories: Arena::new(),
//        globals: Arena::new(),
//
//        custom_sections: Vec::new(),
//    };
//
//    let function = Function {
//        type_: FunctionType(vec![ValType::I32], vec![]),
//        locals: Vec::new(),
//        body: vec![
////            Instr::Call(None), // will be replaced by an index to this function itself during encoding
//Instr::End
//        ],
//    };
//
//    let function = empty_module.functions.alloc(function);
//
//    empty_module.exports.push(Export {
//        name: "newfunc".to_string(),
//        type_: ExportType::Function(function),
//    })
//}

impl From<ll::Module> for Module {
    #[allow(unused_mut)]
    fn from(module: ll::Module) -> Self {
        let mut start = None;

        let mut functions = Vec::new();
        let mut tables = Vec::new();
        let mut memories = Vec::new();
        let mut globals = Vec::new();

        let mut custom_sections = Vec::new();

        // only for build-up, will be discarded
        let mut types: Vec<FunctionType> = Vec::new();

        for section in module.sections {
            match section {
                ll::Section::Custom(vec) => custom_sections.push(vec),
                ll::Section::Type(vec) => types = vec.0,
                ll::Section::Import(vec) => {
                    for import in vec.0 {
                        match import.type_ {
                            ll::ImportType::Function(type_idx) => functions.push(Rc::new(
                                Function {
                                    type_: types[type_idx.0].clone(),
                                    code: Import::Imported {
                                        module: import.module,
                                        name: import.name,
                                    },
                                    export: Cell::new(None),
                                }
                            )),
                            ll::ImportType::Table(table_type) => tables.push(Rc::new(
                                Table {
                                    type_: table_type,
                                    inits: Import::Imported {
                                        module: import.module,
                                        name: import.name,
                                    },
                                    export: Cell::new(None),
                                }
                            )),
                            ll::ImportType::Memory(memory_type) => memories.push(Rc::new(
                                Memory {
                                    type_: memory_type,
                                    inits: Import::Imported {
                                        module: import.module,
                                        name: import.name,
                                    },
                                    export: Cell::new(None),
                                }
                            )),
                            ll::ImportType::Global(global_type) => globals.push(Rc::new(
                                Global {
                                    type_: global_type,
                                    init: Import::Imported {
                                        module: import.module,
                                        name: import.name,
                                    },
                                    export: Cell::new(None),
                                }
                            )),
                        }
                    }
                }
                ll::Section::Function(vec) => {
                    for function_type in vec.0 {
                        functions.push(Rc::new(Function {
                            type_: types[function_type.0].clone(),
                            code: Import::Local(Cell::new(None)),
                            export: Cell::new(None),
                        }));
                    }
                }
                ll::Section::Table(vec) => {
                    for table_type in vec.0 {
                        tables.push(Rc::new(Table {
                            type_: table_type,
                            inits: Import::Local(Cell::new(None)),
                            export: Cell::new(None),
                        }));
                    }
                }
                ll::Section::Memory(vec) => {
                    for memory_type in vec.0 {
                        memories.push(Rc::new(Memory {
                            type_: memory_type,
                            inits: Import::Local(Cell::new(None)),
                            export: Cell::new(None),
                        }));
                    }
                }
                ll::Section::Global(vec) => {
                    for global in vec.0 {
                        globals.push(Rc::new(Global {
                            type_: global.type_,
                            init: Import::Local(Cell::new(Some(vec![/* FIXME decode Expr */]))),
                            export: Cell::new(None),
                        }));
                    }
                }
//                ll::Section::Export(vec) => {
//                    for export in vec.0 {
//                        exports.push(Export {
//                            name: export.name,
//                            type_: match export.type_ {
//                                ll::ExportType::Function(idx) => ExportType::Function(functions[idx.0].clone()),
//                                ll::ExportType::Table(idx) => ExportType::Table(tables[idx.0].clone()),
//                                ll::ExportType::Memory(idx) => ExportType::Memory(memories[idx.0].clone()),
//                                ll::ExportType::Global(idx) => ExportType::Global(globals[idx.0].clone()),
//                            },
//                        })
//                    }
//                }
//                ll::Section::Start(idx) => start = Some(functions[(idx.0).0].clone()),
//                ll::Section::Element(vec) => {
//                    for element in vec.0 {}
//                }
//                ll::Section::Code(_) => {}
//                ll::Section::Data(_) => {}
                _ => {}
            }
        }

        // FIXME replace with visiting all nodes and calling Cell.get()
//        use std::mem::transmute;
        // remove Cell from functions, tables, and memories (was just necessary for buildup)
//        unsafe {
        Module {
            start,
            functions,
            tables,
            memories,
            globals,
            custom_sections,
        }
//        }
    }
}

struct Module {
    functions: Vec<Rc<Function>>,
    tables: Vec<Rc<Table>>,
    memories: Vec<Rc<Memory>>,
    globals: Vec<Rc<Global>>,

    start: Option<Idx<Function>>,

    custom_sections: Vec<Vec<u8>>,
}

pub struct Function {
    type_: FunctionType,
    code: Import<Code>,
    export: Export,
}

pub struct Table {
    type_: TableType,
    inits: Import<Vec<Element>>,
    export: Export,
}

pub struct Memory {
    type_: MemoryType,
    inits: Import<Vec<Data>>,
    export: Export,
}

pub struct Global {
    type_: GlobalType,
    init: Import<ConstExpr>,
    export: Export,
}

pub enum Import<T> {
    Imported {
        module: String,
        name: String,
    },
    Local(Cell<Option<T>>),
}

type Export = Cell<Option<String>>;

pub struct Code(Vec<Local>, Expr);

type Local = ValType;

pub struct Element {
    offset: ConstExpr,
    functions: Vec<Idx<Function>>,
}

pub struct Data {
    offset: ConstExpr,
    bytes: Vec<u8>,
}


pub type Expr = Vec<Instr>;
pub type ConstExpr = Vec<Instr>;

/// None indicates a self referential index which will be patched during serialization
//pub type RecursiveIdx<T> = Option<Rc<T>>;
pub type Idx<T> = Rc<T>;

pub enum Instr {
    Unreachable,
    Nop,

    Block(BlockType, Expr),
    Loop(BlockType, Expr),
    If(BlockType, Expr),
    Else(Expr),
    End,

    Br(Label),
    BrIf(Label),
    BrTable(Vec<Label>, Label),

    Return,
    Call(Idx<Function>),
    CallIndirect(FunctionType, /* unused, always 0x00 in WASM version 1 */ Idx<Table>),

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

    CurrentMemory(/* unused, always 0x00 in WASM version 1 */ Idx<Memory>),
    GrowMemory(/* unused, always 0x00 in WASM version 1 */ Idx<Memory>),

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
    F32Clonesign,
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
    F64Clonesign,
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