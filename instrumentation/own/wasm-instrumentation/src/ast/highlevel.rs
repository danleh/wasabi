// TODO "streaming AST" API: return Module {} after reading only the first 8 bytes, implement
// Iterator<Item = Section> for Module -> Module must somehow retain the reader to do so...

// used, but not re-exported
use binary::WasmBinary;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::marker::PhantomData;
// reuse as much as possible from the low-level AST
use super::lowlevel::{BlockType, ElemType, FunctionType, GlobalType, Idx, Label, Limits, Local, Memarg, MemoryType, Mutability, Section, TableType, ValType, WithSize};
use super::lowlevel as ll;

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

/* High-level AST:
    - types are inlined instead of referenced by type idx (i.e., no manual handling of Type "pool")
    - Function + Code sections are merged into one list of functions,
      same for tables: Table + Element sections and memories: Memory + Data sections.
    - imports and exports are part of the respective item, not stored externally and referring to
      their item by index.
*/

#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
    pub tables: Vec<Table>,
    pub memories: Vec<Memory>,
    pub globals: Vec<Global>,

    pub start: Option<Idx<Function>>,

    custom_sections: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct Function {
    // type is inlined here compared to low-level/binary/spec representation
    pub type_: FunctionType,
    // import and code are mutually exclusive, i.e., exactly one of both must be Some(...)
    pub import: Option<(String, String)>,
    pub code: Option<Code>,
    pub export: Option<String>,
}

#[derive(Debug)]
pub struct Global {
    pub type_: GlobalType,
    // import and init are mutually exclusive, i.e., exactly one of both must be Some(...)
    pub import: Option<(String, String)>,
    pub init: Option<Expr>,
    pub export: Option<String>,
}

#[derive(Debug)]
pub struct Table {
    pub type_: TableType,
    pub import: Option<(String, String)>,
    pub elements: Vec<Element>,
    pub export: Option<String>,
}

#[derive(Debug)]
pub struct Memory {
    pub type_: MemoryType,
    pub import: Option<(String, String)>,
    pub data: Vec<Data>,
    pub export: Option<String>,
}

#[derive(Debug)]
pub struct Code {
    pub locals: Vec<ValType>,
    pub body: Expr,
}

#[derive(Debug)]
pub struct Element {
    pub offset: Expr,
    pub functions: Vec<Idx<Function>>,
}

#[derive(Debug)]
pub struct Data {
    pub offset: Expr,
    pub bytes: Vec<u8>,
}

// TODO if Expr is an iterator instead of a Vec, we could lazily parse instructions
pub type Expr = Vec<Instr>;

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

impl From<Module> for ll::Module {
    fn from(module: Module) -> Self {
        let mut sections = Vec::new();

        {
            let mut types = HashMap::new();

            let mut function_idx_patch = HashMap::new();
            let mut tables_idx_patch = HashMap::new();
            let mut memories_idx_patch = HashMap::new();
            let mut globals_idx_patch = HashMap::new();

            let imports = to_lowlevel_imports(&module, &mut types,
                                              &mut function_idx_patch, &mut tables_idx_patch, &mut memories_idx_patch, &mut globals_idx_patch);
            let functions = to_lowlevel_functions(&module.functions, &mut types, &mut function_idx_patch);
            let tables = to_lowlevel_tables(&module.tables, &mut tables_idx_patch);
            let memories = to_lowlevel_memories(&module.memories, &mut memories_idx_patch);
            let globals = to_lowlevel_globals(&module.globals, &mut globals_idx_patch);

            /* All types and indices are now determined, so we can start writing out sections... */

            // Type
            let mut types = types.into_iter().collect::<Vec<_>>();
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
            let exports = to_lowlevel_exports(&module, &function_idx_patch, &tables_idx_patch, &memories_idx_patch, &globals_idx_patch);
            sections.push(Section::Export(WithSize(exports)));

            // Start
            for start in &module.start {
                sections.push(Section::Start(WithSize(function_idx_patch[&start.0].into())));
            }

            // Element, Code, Data
            // TODO

        }

        for custom in module.custom_sections {
            sections.push(Section::Custom(custom));
        }

        ll::Module { sections }
    }
}

fn to_lowlevel_type_idx<'a>(function_type: &'a FunctionType, types: &mut HashMap<&'a FunctionType, usize>) -> Idx<FunctionType> {
    let new_idx = types.len();
    let &mut idx = types.entry(function_type).or_insert(new_idx);
    idx.into()
}

fn to_lowlevel_imports<'a>(module: &'a Module,
                           types: &mut HashMap<&'a FunctionType, usize>,
                           function_idx_patch: &mut HashMap<usize, usize>,
                           tables_idx_patch: &mut HashMap<usize, usize>,
                           memories_idx_patch: &mut HashMap<usize, usize>,
                           globals_idx_patch: &mut HashMap<usize, usize>,
) -> Vec<ll::Import> {
    let mut imports = Vec::new();

    imports.extend(module.functions.iter()
        .enumerate()
        .filter_map(|(i, func)|
            func.import.as_ref().map(|&(ref module, ref name)| {
                let new_idx = function_idx_patch.len();
                function_idx_patch.insert(i, new_idx);
                ll::Import {
                    module: module.clone(),
                    name: name.clone(),
                    type_: ll::ImportType::Function(to_lowlevel_type_idx(&func.type_, types)),
                }
            })));

    imports.extend(module.tables.iter()
        .enumerate()
        .filter_map(|(i, table)|
            table.import.as_ref().map(|&(ref module, ref name)| {
                let new_idx = tables_idx_patch.len();
                tables_idx_patch.insert(i, new_idx);
                ll::Import {
                    module: module.clone(),
                    name: name.clone(),
                    type_: ll::ImportType::Table(table.type_.clone()),
                }
            })));

    imports.extend(module.memories.iter()
        .enumerate()
        .filter_map(|(i, memory)|
            memory.import.as_ref().map(|&(ref module, ref name)| {
                let new_idx = memories_idx_patch.len();
                memories_idx_patch.insert(i, new_idx);
                ll::Import {
                    module: module.clone(),
                    name: name.clone(),
                    type_: ll::ImportType::Memory(memory.type_.clone()),
                }
            })));

    imports.extend(module.globals.iter()
        .enumerate()
        .filter_map(|(i, global)|
            global.import.as_ref().map(|&(ref module, ref name)| {
                let new_idx = globals_idx_patch.len();
                globals_idx_patch.insert(i, new_idx);
                ll::Import {
                    module: module.clone(),
                    name: name.clone(),
                    type_: ll::ImportType::Global(global.type_.clone()),
                }
            })));

    imports
}

fn to_lowlevel_functions<'a>(functions: &'a [Function],
                             types: &mut HashMap<&'a FunctionType, usize>,
                             function_idx_patch: &mut HashMap<usize, usize>,
) -> Vec<Idx<FunctionType>> {
    functions.iter()
        .enumerate()
        .filter(|&(_, func)| func.import.is_none())
        .map(|(i, func)| {
            let new_idx = function_idx_patch.len();
            function_idx_patch.insert(i, new_idx);
            to_lowlevel_type_idx(&func.type_, types)
        })
        .collect()
}

fn to_lowlevel_tables<'a>(tables: &'a [Table],
                          tables_idx_patch: &mut HashMap<usize, usize>) -> Vec<TableType> {
    tables.iter()
        .enumerate()
        .filter(|&(_, table)| table.import.is_none())
        .map(|(i, table)| {
            let new_idx = tables_idx_patch.len();
            tables_idx_patch.insert(i, new_idx);
            table.type_.clone()
        })
        .collect()
}

fn to_lowlevel_memories<'a>(memories: &'a [Memory],
                            memories_idx_patch: &mut HashMap<usize, usize>) -> Vec<MemoryType> {
    memories.iter()
        .enumerate()
        .filter(|&(_, memory)| memory.import.is_none())
        .map(|(i, memory)| {
            let new_idx = memories_idx_patch.len();
            memories_idx_patch.insert(i, new_idx);
            memory.type_.clone()
        })
        .collect()
}


fn to_lowlevel_globals<'a>(globals: &'a [Global],
                           globals_idx_patch: &mut HashMap<usize, usize>) -> Vec<ll::Global> {
    globals.iter()
        .enumerate()
        .filter_map(|(i, global)| {
            let type_ = global.type_;
            global.init.as_ref().map(|init| {
                let new_idx = globals_idx_patch.len();
                globals_idx_patch.insert(i, new_idx);
                ll::Global {
                    type_,
                    init: to_lowlevel_expr(init),
                }
            })
        })
        .collect()
}

fn to_lowlevel_expr(expr: &[Instr]) -> ll::Expr {
    ll::Expr(vec![]) // FIXME
}

fn to_lowlevel_exports<'a>(module: &'a Module,
                           // TODO move all of this into a helper struct, then simplify all fn signatures above
                           // TODO singular for tables memories and globals
                           // TODO add API /struct/impl that looks up or inserts a mapping in a hash map, use for all patch maps
                           function_idx_patch: &HashMap<usize, usize>,
                           tables_idx_patch: &HashMap<usize, usize>,
                           memories_idx_patch: &HashMap<usize, usize>,
                           globals_idx_patch: &HashMap<usize, usize>,
) -> Vec<ll::Export> {
    let mut exports = Vec::new();

    exports.extend(module.functions.iter()
        .enumerate()
        .filter_map(|(i, func)|
            func.export.as_ref().map(|name | ll::Export {
                name: name.clone(),
                type_: ll::ExportType::Function(function_idx_patch[&i].into()),
            })));

    exports.extend(module.tables.iter()
        .enumerate()
        .filter_map(|(i, table)|
            table.export.as_ref().map(|name | ll::Export {
                name: name.clone(),
                type_: ll::ExportType::Table(tables_idx_patch[&i].into()),
            })));

    exports.extend(module.memories.iter()
        .enumerate()
        .filter_map(|(i, memory)|
            memory.export.as_ref().map(|name | ll::Export {
                name: name.clone(),
                type_: ll::ExportType::Memory(memories_idx_patch[&i].into()),
            })));

    exports.extend(module.globals.iter()
        .enumerate()
        .filter_map(|(i, global)|
            global.export.as_ref().map(|name | ll::Export {
                name: name.clone(),
                type_: ll::ExportType::Global(globals_idx_patch[&i].into()),
            })));

    exports
}