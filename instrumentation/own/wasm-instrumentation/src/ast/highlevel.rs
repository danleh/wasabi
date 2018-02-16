// TODO "streaming AST" API: return Module {} after reading only the first 8 bytes, implement
// Iterator<Item = Section> for Module -> Module must somehow retain the reader to do so...

use ast::lowlevel::WithSize;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::mem::transmute;
use std::rc::Rc;
use super::lowlevel as ll;
// reuse as much as possible from the low-level AST
pub use super::lowlevel::{BlockType, ElemType, FunctionType, GlobalType, Idx, Label, Limits, Local, Memarg, MemoryType, Mutability, TableType, ValType};

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

//impl Into<ll::Module> for Module {
//    fn into(self) -> ll::Module {
//        let mut sections = Vec::new();
//
//        sections.push(ll::Section::Type(WithSize(self.types)));
//        sections.push(ll::Section::Import(WithSize({
//            self.functions.iter()
//        })));
//
//        ll::Module { sections }
//    }
//}

// TODO keep indices as usize, as long as imported items come before local ones, that is fine
// during serialization, path all imported items to the front

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
                    for import_ in vec.0 {
                        let import = Some(ImportDesc {
                            module: import_.module,
                            name: import_.name,
                        });
                        let export = String::new();
                        match import_.type_ {
                            ll::ImportType::Function(type_idx) => functions.push(Function {
//                                type_: types[type_idx.0].clone(), // FIXME should we inline types? Or only compact at serialization!?
                                type_: type_idx.0.into(),
                                import,
                                code: None,
                                export,
                            }),
                            ll::ImportType::Table(table_type) => tables.push(Table {
                                type_: table_type,
                                import,
                                elements: Vec::new(),
                                export,
                            }),
                            ll::ImportType::Memory(memory_type) => memories.push(Memory {
                                type_: memory_type,
                                import,
                                data: Vec::new(),
                                export,
                            }),
                            ll::ImportType::Global(global_type) => globals.push(Global {
                                type_: global_type,
                                import,
                                init: None,
                                export,
                            }),
                        }
                    }
                }
                ll::Section::Function(vec) => {
                    for function_type in vec.0 {
                        functions.push(Function {
                            type_: function_type.0.into(),
                            import: None,
                            code: None,
                            export: String::new(),
                        });
                    }
                }
                ll::Section::Table(vec) => {
                    for table_type in vec.0 {
                        tables.push(Table {
                            type_: table_type,
                            import: None,
                            elements: Vec::new(),
                            export: String::new(),
                        });
                    }
                }
                ll::Section::Memory(vec) => {
                    for memory_type in vec.0 {
                        memories.push(Memory {
                            type_: memory_type,
                            import: None,
                            data: Vec::new(),
                            export: String::new(),
                        });
                    }
                }
                ll::Section::Global(vec) => {
                    for global in vec.0 {
                        globals.push(Global {
                            type_: global.type_,
                            import: None,
                            init: Some(global.init.into()),
                            export: String::new(),
                        });
                    }
                }
                ll::Section::Export(vec) => {
                    for export in vec.0 {
                        match export.type_ {
                            ll::ExportType::Function(idx) => functions[idx.0].export = export.name,
                            ll::ExportType::Table(idx) => tables[idx.0].export = export.name,
                            ll::ExportType::Memory(idx) => memories[idx.0].export = export.name,
                            ll::ExportType::Global(idx) => globals[idx.0].export = export.name,
                        }
                    }
                }
                ll::Section::Start(idx) => start = Some((idx.0).0.into()),
                ll::Section::Element(vec) => {
                    for element in vec.0 {
                        tables[element.table.0].elements.push(Element {
                            offset: element.offset.into(),
                            functions: element.init.into_iter().map(|idx| idx.0.into()).collect(),
                        })
                    }
                }
                ll::Section::Code(vec) => {
                    let imported_function_count = functions.iter().filter(|f| f.import.is_some()).count();
                    for (i, WithSize(code)) in vec.0.into_iter().enumerate() {
                        functions[imported_function_count + i].code = Some(Code(
                            {
                                let mut locals = Vec::new();
                                for local in code.locals {
                                    for _ in 0..local.count {
                                        locals.push(local.type_);
                                    }
                                }
                                locals
                            },
                            code.body.into(),
                        ))
                    }
                }
                ll::Section::Data(vec) => {
                    for data in vec.0 {
                        memories[data.memory.0].data.push(Data {
                            offset: data.offset.into(),
                            bytes: data.init,
                        })
                    }
                }
            }
        }

        Module {
            types,
            functions,
            tables,
            memories,
            globals,
            start,
            custom_sections,
        }
    }
}

// Cell<Option<...>> in order to be able to close the link for cyclic Idx into its own element,
// (only case I can think of right now): recursive Functions
// Rc<RefCell<T>> because Rc gives us shared ownership (all IdxPtrs and the module Vec's share the function/table/memory/global
// and RefCell because we need to be able to mutate the elements (e.g. change body of functions, change global inits etc.)
struct IdxBasedOnPtr<T>(Cell<Option<Rc<RefCell<T>>>>);

// Same as above, only that the top-level module Vec's are now the only owner of function/table/memory/global and
// the Idx contains only a reference. RefCell is still necessary, since Cell works only if no other references exist of
// the contents, which is exactly not true because of the Idx's.
// Then we need an Arena allocator for the elements, instead of Vec<Rc<...>>
struct IdxBasedOnRef<'a, T: 'a>(Cell<Option<&'a RefCell<T>>>);

#[derive(Debug)]
pub struct Module {
    types: Vec<FunctionType>,

    functions: Vec<Function>,
    tables: Vec<Table>,
    memories: Vec<Memory>,
    globals: Vec<Global>,

    start: Option<Idx<Function>>,

    custom_sections: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct Function {
    type_: Idx<FunctionType>,
    import: Import,
    // NOTE must not be present if import.is_some() i.e. import and code are mutually exclusive
    code: Option<Code>,
    export: Export,
}

#[derive(Debug)]
pub struct Global {
    type_: GlobalType,
    import: Import,
    // NOTE must not be present if import.is_some()
    init: Option<ConstExpr>,
    export: Export,
}

#[derive(Debug)]
pub struct Table {
    type_: TableType,
    import: Import,
    elements: Vec<Element>,
    export: Export,
}

#[derive(Debug)]
pub struct Memory {
    type_: MemoryType,
    import: Import,
    data: Vec<Data>,
    export: Export,
}

pub type Import = Option<ImportDesc>;

#[derive(Debug)]
pub struct ImportDesc {
    module: String,
    name: String,
}

type Export = Option<String>;

#[derive(Debug)]
pub struct Code(Vec<ValType>, Expr);

#[derive(Debug)]
pub struct Element {
    offset: ConstExpr,
    functions: Vec<Idx<Function>>,
}

#[derive(Debug)]
pub struct Data {
    offset: ConstExpr,
    bytes: Vec<u8>,
}

pub type Expr = Vec<Instr>;
pub type ConstExpr = Vec<Instr>;

impl From<ll::Expr> for Expr {
    fn from(expr: ll::Expr) -> Self {
        // same structure, just different underlying Function, Table, Memory, and Global
        // but FIXME ll::Expr is a struct, hl::Expr a type alias!?
        unsafe {
            transmute(expr)
        }
    }
}

//fn visit(instr: &ll::Instr) -> Instr {
//    match instr {
//        ll::Instr::Unreachable => Instr::Unreachable,
//        ll::Instr::Nop => Instr::Nop,
//
//        ll::Instr::Block(_, _) => Instr::Block,
//        ll::Instr::Loop(_, _) => Instr::Loop,
//        ll::Instr::If(_, _) => Instr::If,
//        ll::Instr::Else(_) => Instr::Else,
//        ll::Instr::End => Instr::End,
//        ll::Instr::Br(_) => Instr::Br,
//        ll::Instr::BrIf(_) => Instr::BrIf,
//        ll::Instr::BrTable(_, _) => Instr::BrTable,
//        ll::Instr::Return => Instr::Return,
//        ll::Instr::Call(_) => Instr::Call,
//        ll::Instr::CallIndirect(_, _) => Instr::CallIndirect,
//        ll::Instr::Drop => Instr::Drop,
//        ll::Instr::Select => Instr::Select,
//        ll::Instr::GetLocal(_) => Instr::GetLocal,
//        ll::Instr::SetLocal(_) => Instr::SetLocal,
//        ll::Instr::TeeLocal(_) => Instr::TeeLocal,
//        ll::Instr::GetGlobal(_) => Instr::GetGlobal,
//        ll::Instr::SetGlobal(_) => Instr::SetGlobal,
//
//        ll::Instr::I32Load(_) => Instr::I32Load,
//        ll::Instr::I64Load(_) => Instr::I64Load,
//        ll::Instr::F32Load(_) => Instr::F32Load,
//        ll::Instr::F64Load(_) => Instr::F64Load,
//        ll::Instr::I32Load8S(_) => Instr::I32Load8S,
//        ll::Instr::I32Load8U(_) => Instr::I32Load8U,
//        ll::Instr::I32Load16S(_) => Instr::I32Load16S,
//        ll::Instr::I32Load16U(_) => Instr::I32Load16U,
//        ll::Instr::I64Load8S(_) => Instr::I64Load8S,
//        ll::Instr::I64Load8U(_) => Instr::I64Load8U,
//        ll::Instr::I64Load16S(_) => Instr::I64Load16S,
//        ll::Instr::I64Load16U(_) => Instr::I64Load16U,
//        ll::Instr::I64Load32S(_) => Instr::I64Load32S,
//        ll::Instr::I64Load32U(_) => Instr::I64Load32U,
//        ll::Instr::I32Store(_) => Instr::I32Store,
//        ll::Instr::I64Store(_) => Instr::I64Store,
//        ll::Instr::F32Store(_) => Instr::F32Store,
//        ll::Instr::F64Store(_) => Instr::F64Store,
//        ll::Instr::I32Store8(_) => Instr::I32Store8,
//        ll::Instr::I32Store16(_) => Instr::I32Store16,
//        ll::Instr::I64Store8(_) => Instr::I64Store8,
//        ll::Instr::I64Store16(_) => Instr::I64Store16,
//        ll::Instr::I64Store32(_) => Instr::I64Store32,
//
//        ll::Instr::CurrentMemory(ref idx) => Instr::CurrentMemory,
//        ll::Instr::GrowMemory(ref idx) => Instr::GrowMemory,
//
//        ll::Instr::I32Const(ref imm) => Instr::I32Const(imm),
//        ll::Instr::I64Const(ref imm) => Instr::I64Const(imm),
//        ll::Instr::F32Const(ref imm) => Instr::F32Const(imm),
//        ll::Instr::F64Const(ref imm) => Instr::F64Const(imm),
//
//        ll::Instr::I32Eqz => Instr::I32Eqz,
//        ll::Instr::I32Eq => Instr::I32Eq,
//        ll::Instr::I32Ne => Instr::I32Ne,
//        ll::Instr::I32LtS => Instr::I32LtS,
//        ll::Instr::I32LtU => Instr::I32LtU,
//        ll::Instr::I32GtS => Instr::I32GtS,
//        ll::Instr::I32GtU => Instr::I32GtU,
//        ll::Instr::I32LeS => Instr::I32LeS,
//        ll::Instr::I32LeU => Instr::I32LeU,
//        ll::Instr::I32GeS => Instr::I32GeS,
//        ll::Instr::I32GeU => Instr::I32GeU,
//        ll::Instr::I64Eqz => Instr::I64Eqz,
//        ll::Instr::I64Eq => Instr::I64Eq,
//        ll::Instr::I64Ne => Instr::I64Ne,
//        ll::Instr::I64LtS => Instr::I64LtS,
//        ll::Instr::I64LtU => Instr::I64LtU,
//        ll::Instr::I64GtS => Instr::I64GtS,
//        ll::Instr::I64GtU => Instr::I64GtU,
//        ll::Instr::I64LeS => Instr::I64LeS,
//        ll::Instr::I64LeU => Instr::I64LeU,
//        ll::Instr::I64GeS => Instr::I64GeS,
//        ll::Instr::I64GeU => Instr::I64GeU,
//        ll::Instr::F32Eq => Instr::F32Eq,
//        ll::Instr::F32Ne => Instr::F32Ne,
//        ll::Instr::F32Lt => Instr::F32Lt,
//        ll::Instr::F32Gt => Instr::F32Gt,
//        ll::Instr::F32Le => Instr::F32Le,
//        ll::Instr::F32Ge => Instr::F32Ge,
//        ll::Instr::F64Eq => Instr::F64Eq,
//        ll::Instr::F64Ne => Instr::F64Ne,
//        ll::Instr::F64Lt => Instr::F64Lt,
//        ll::Instr::F64Gt => Instr::F64Gt,
//        ll::Instr::F64Le => Instr::F64Le,
//        ll::Instr::F64Ge => Instr::F64Ge,
//        ll::Instr::I32Clz => Instr::I32Clz,
//        ll::Instr::I32Ctz => Instr::I32Ctz,
//        ll::Instr::I32Popcnt => Instr::I32Popcnt,
//        ll::Instr::I32Add => Instr::I32Add,
//        ll::Instr::I32Sub => Instr::I32Sub,
//        ll::Instr::I32Mul => Instr::I32Mul,
//        ll::Instr::I32DivS => Instr::I32DivS,
//        ll::Instr::I32DivU => Instr::I32DivU,
//        ll::Instr::I32RemS => Instr::I32RemS,
//        ll::Instr::I32RemU => Instr::I32RemU,
//        ll::Instr::I32And => Instr::I32And,
//        ll::Instr::I32Or => Instr::I32Or,
//        ll::Instr::I32Xor => Instr::I32Xor,
//        ll::Instr::I32Shl => Instr::I32Shl,
//        ll::Instr::I32ShrS => Instr::I32ShrS,
//        ll::Instr::I32ShrU => Instr::I32ShrU,
//        ll::Instr::I32Rotl => Instr::I32Rotl,
//        ll::Instr::I32Rotr => Instr::I32Rotr,
//        ll::Instr::I64Clz => Instr::I64Clz,
//        ll::Instr::I64Ctz => Instr::I64Ctz,
//        ll::Instr::I64Popcnt => Instr::I64Popcnt,
//        ll::Instr::I64Add => Instr::I64Add,
//        ll::Instr::I64Sub => Instr::I64Sub,
//        ll::Instr::I64Mul => Instr::I64Mul,
//        ll::Instr::I64DivS => Instr::I64DivS,
//        ll::Instr::I64DivU => Instr::I64DivU,
//        ll::Instr::I64RemS => Instr::I64RemS,
//        ll::Instr::I64RemU => Instr::I64RemU,
//        ll::Instr::I64And => Instr::I64And,
//        ll::Instr::I64Or => Instr::I64Or,
//        ll::Instr::I64Xor => Instr::I64Xor,
//        ll::Instr::I64Shl => Instr::I64Shl,
//        ll::Instr::I64ShrS => Instr::I64ShrS,
//        ll::Instr::I64ShrU => Instr::I64ShrU,
//        ll::Instr::I64Rotl => Instr::I64Rotl,
//        ll::Instr::I64Rotr => Instr::I64Rotr,
//        ll::Instr::F32Abs => Instr::F32Abs,
//        ll::Instr::F32Neg => Instr::F32Neg,
//        ll::Instr::F32Ceil => Instr::F32Ceil,
//        ll::Instr::F32Floor => Instr::F32Floor,
//        ll::Instr::F32Trunc => Instr::F32Trunc,
//        ll::Instr::F32Nearest => Instr::F32Nearest,
//        ll::Instr::F32Sqrt => Instr::F32Sqrt,
//        ll::Instr::F32Add => Instr::F32Add,
//        ll::Instr::F32Sub => Instr::F32Sub,
//        ll::Instr::F32Mul => Instr::F32Mul,
//        ll::Instr::F32Div => Instr::F32Div,
//        ll::Instr::F32Min => Instr::F32Min,
//        ll::Instr::F32Max => Instr::F32Max,
//        ll::Instr::F32Copysign => Instr::F32Copysign,
//        ll::Instr::F64Abs => Instr::F64Abs,
//        ll::Instr::F64Neg => Instr::F64Neg,
//        ll::Instr::F64Ceil => Instr::F64Ceil,
//        ll::Instr::F64Floor => Instr::F64Floor,
//        ll::Instr::F64Trunc => Instr::F64Trunc,
//        ll::Instr::F64Nearest => Instr::F64Nearest,
//        ll::Instr::F64Sqrt => Instr::F64Sqrt,
//        ll::Instr::F64Add => Instr::F64Add,
//        ll::Instr::F64Sub => Instr::F64Sub,
//        ll::Instr::F64Mul => Instr::F64Mul,
//        ll::Instr::F64Div => Instr::F64Div,
//        ll::Instr::F64Min => Instr::F64Min,
//        ll::Instr::F64Max => Instr::F64Max,
//        ll::Instr::F64Copysign => Instr::F64Copysign,
//        ll::Instr::I32WrapI64 => Instr::I32WrapI64,
//        ll::Instr::I32TruncSF32 => Instr::I32TruncSF32,
//        ll::Instr::I32TruncUF32 => Instr::I32TruncUF32,
//        ll::Instr::I32TruncSF64 => Instr::I32TruncSF64,
//        ll::Instr::I32TruncUF64 => Instr::I32TruncUF64,
//        ll::Instr::I64ExtendSI32 => Instr::I64ExtendSI32,
//        ll::Instr::I64ExtendUI32 => Instr::I64ExtendUI32,
//        ll::Instr::I64TruncSF32 => Instr::I64TruncSF32,
//        ll::Instr::I64TruncUF32 => Instr::I64TruncUF32,
//        ll::Instr::I64TruncSF64 => Instr::I64TruncSF64,
//        ll::Instr::I64TruncUF64 => Instr::I64TruncUF64,
//        ll::Instr::F32ConvertSI32 => Instr::F32ConvertSI32,
//        ll::Instr::F32ConvertUI32 => Instr::F32ConvertUI32,
//        ll::Instr::F32ConvertSI64 => Instr::F32ConvertSI64,
//        ll::Instr::F32ConvertUI64 => Instr::F32ConvertUI64,
//        ll::Instr::F32DemoteF64 => Instr::F32DemoteF64,
//        ll::Instr::F64ConvertSI32 => Instr::F64ConvertSI32,
//        ll::Instr::F64ConvertUI32 => Instr::F64ConvertUI32,
//        ll::Instr::F64ConvertSI64 => Instr::F64ConvertSI64,
//        ll::Instr::F64ConvertUI64 => Instr::F64ConvertUI64,
//        ll::Instr::F64PromoteF32 => Instr::F64PromoteF32,
//        ll::Instr::I32ReinterpretF32 => Instr::I32ReinterpretF32,
//        ll::Instr::I64ReinterpretF64 => Instr::I64ReinterpretF64,
//        ll::Instr::F32ReinterpretI32 => Instr::F32ReinterpretI32,
//        ll::Instr::F64ReinterpretI64 => Instr::F64ReinterpretI64,
//    }
//}

// TODO if we save types inline, we cannot do transmute() anylonger so
// - implement proper visitor for Instr
// - convert hl::Instr to ll::Instr and back with visitor
#[derive(Debug)]
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
    CallIndirect(Idx<FunctionType>, /* unused, always 0x00 in WASM version 1 */ Idx<Table>),

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