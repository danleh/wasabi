// Right now the structure in module ast::* is extremely low-level, i.e., faithful to the original
// encoding (e.g. order of sections, order of types in Type section, width of LEB128 numbers etc.)
// This allows decoding-encoding to round-trip, but is tedious to work with for instrumentation.
// TODO Is round-trip/this "faithfulness" to the exact original representation necessary?
// Or should we only provide a high-level AST that logically captures everything but may be
// serialized differently than the original module?

// TODO Would this higher level Module/AST format be more convenient to work with?
// - no WithSize<T> or Leb128<T>
// - no explicit TypeIdx, all types are inlined and the Type section is built upon serialization
//   with a HashMap to still avoid type duplication, then all inlined types are replaced with idx
//   into the "HashMap".
// -> TODO We cannot get completely rid of *Idx, because globals, locals, functions can and must still
//    be referenced from code. Maybe we should thus still have Type section and TypeIdx explicitly available?
// - functions combines Function and Code section
// - table combines Table and Element (initialization of tables) section
// - memory combines Memory and Data (initialization of memory) section

// TODO "streaming AST" API: return Module {} after reading only the first 8 bytes, implement
// Iterator<Item = Section> for Module -> Module must somehow retain the reader to do so...

use super::lowlevel as ll;
// these are either plain enums or structs that do not contain Leb128 or indices
use super::lowlevel::BlockType;
use super::lowlevel::ElemType;
use super::lowlevel::GlobalType;
use super::lowlevel::Mutability;
use super::lowlevel::ValType;
use typed_arena::Arena;

fn test() {
    let mut empty_module = Module {
        start: None,
        imports: Vec::new(),
        exports: Vec::new(),

        functions: Arena::new(),
        tables: Arena::new(),
        memories: Arena::new(),
        globals: Arena::new(),

        custom_sections: Vec::new(),
    };

    let function = Function {
        type_: FunctionType(vec![ValType::I32], vec![]),
        locals: Vec::new(),
        body: vec![
//            Instr::Call(None), // will be replaced by an index to this function itself during encoding
Instr::End
        ],
    };

    let function = empty_module.functions.alloc(function);

    empty_module.exports.push(Export {
        name: "newfunc".to_string(),
        type_: ExportType::Function(function),
    })
}

impl<'a> From<ll::Module> for Module<'a> {
    fn from(module: ll::Module) -> Self {
        let mut start = None;

        let mut imports = Vec::new();
        let mut exports = Vec::new();

        let mut functions = Arena::new();
        let mut tables = Arena::new();
        let mut memories = Arena::new();
        let mut globals = Arena::new();

        let mut custom_sections = Vec::new();

        let mut types: Vec<FunctionType> = Vec::new();

        for section in module.sections {
            match section {
                ll::Section::Custom(vec) => custom_sections.push(vec.value),
                ll::Section::Type(vec) => types = vec.content.value.into_iter().map(Into::into).collect(),
                ll::Section::Import(vec) => {
                    vec.content.value.into_iter().map(|import| {
                        Import {
                            module: import.module.value,
                            name: import.name.value,
                            type_: match import.type_ {
                                ll::ImportType::Function(type_idx) => ImportType::Function(types[type_idx.0.value]),
                                ll::ImportType::Table(table_type) => ImportType::Table(table_type.into()),
                                ll::ImportType::Memory(_) => unimplemented!(),
                                ll::ImportType::Global(_) => unimplemented!(),
                            },
                        }
                    })
                }
                ll::Section::Function(_) => {}
                ll::Section::Table(_) => {}
                ll::Section::Memory(_) => {}
                ll::Section::Global(_) => {}
                ll::Section::Export(_) => {}
                ll::Section::Start(_) => {}
                ll::Section::Element(_) => {}
                ll::Section::Code(_) => {}
                ll::Section::Data(_) => {}
            }
        }

        Module {
            start,
            imports,
            exports,
            functions,
            tables,
            memories,
            globals,
            custom_sections,
        }
    }
}

//impl<'a> From<ll::Import> for Import {
//    fn from(import: ll::Import) -> Self {
//        Import {
//            module: import.module.value,
//            name: import.name.value,
//            type_: match import.type_ {
//                ll::ImportType::Function(type_idx) => {},
//                ll::ImportType::Table(table_type) => {},
//                ll::ImportType::Memory(memory_type) => {},
//                ll::ImportType::Global(global_type) => {},
//            },
//        }
//    }
//}

struct Module<'a> {
    start: Option<Idx<'a, Function<'a>>>,

    imports: Vec<Import>,
    exports: Vec<Export<'a>>,

    functions: Arena<Function<'a>>,
    tables: Arena<Table<'a>>,
    memories: Arena<Memory<'a>>,
    globals: Arena<Global<'a>>,

    custom_sections: Vec<Vec<u8>>,
}

pub struct Function<'a> {
    type_: FunctionType,
    locals: Vec<Local>,
    body: Expr<'a>,
}

type Local = ValType;

pub struct Import {
    module: String,
    name: String,
    type_: ImportType,
}

pub struct Export<'a> {
    name: String,
    type_: ExportType<'a>,
}

pub enum ImportType {
    Function(FunctionType),
    Table(TableType),
    Memory(MemoryType),
    Global(GlobalType),
}

pub enum ExportType<'a> {
    Function(Idx<'a, Function<'a>>),
    Table(Idx<'a, Table<'a>>),
    Memory(Idx<'a, Memory<'a>>),
    Global(Idx<'a, Global<'a>>),
}

pub struct Table<'a> {
    type_: TableType,
    inits: Vec<Element<'a>>,
}

pub struct Memory<'a> {
    type_: MemoryType,
    inits: Vec<Data<'a>>,
}

// == TableInit
pub struct Element<'a> {
    offset: ConstExpr<'a>,
    functions: Vec<Idx<'a, Function<'a>>>,
}

// == MemoryInit
pub struct Data<'a> {
    offset: ConstExpr<'a>,
    bytes: Vec<u8>,
}


pub struct FunctionType(Vec<ValType>, Vec<ValType>);

impl From<ll::FuncType> for FunctionType {
    fn from(func_type: ll::FuncType) -> Self {
        FunctionType(func_type.params.value, func_type.results.value)
    }
}

pub struct TableType(ElemType, Limits);


pub struct MemoryType(Limits);


pub struct Limits {
    pub initial_size: u32,
    pub max_size: Option<u32>,
}

//impl From<ll::Limits> for Limits {
//    fn from(limits: ll::Limits) -> Self {
//        unimplemented!()
//    }
//}


pub struct Global<'a> {
    type_: GlobalType,
    init: ConstExpr<'a>,
}

pub struct Label(usize);


pub struct Memarg {
    pub alignment: u32,
    pub offset: u32,
}

pub type Expr<'a> = Vec<Instr<'a>>;
pub type ConstExpr<'a> = Vec<Instr<'a>>;

/// None indicates a self referential index which will be patched during serialization
//pub type RecursiveIdx<T> = Option<Rc<T>>;
pub type Idx<'a, T> = &'a T;


pub enum Instr<'a> {
    Unreachable,
    Nop,

    Block(BlockType, Expr<'a>),
    Loop(BlockType, Expr<'a>),
    If(BlockType, Expr<'a>),
    Else(Expr<'a>),
    End,

    Br(Label),
    BrIf(Label),
    BrTable(Vec<Label>, Label),

    Return,
    Call(Idx<'a, Function<'a>>),
    CallIndirect(FunctionType, /* unused, always 0x00 in WASM version 1 */ Idx<'a, Table<'a>>),

    Drop,
    Select,

    GetLocal(Idx<'a, Local>),
    SetLocal(Idx<'a, Local>),
    TeeLocal(Idx<'a, Local>),
    GetGlobal(Idx<'a, Global<'a>>),
    SetGlobal(Idx<'a, Global<'a>>),

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

    CurrentMemory(/* unused, always 0x00 in WASM version 1 */ Idx<'a, Memory<'a>>),
    GrowMemory(/* unused, always 0x00 in WASM version 1 */ Idx<'a, Memory<'a>>),

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

//trait SelfIndices {
//    fn patch_self_indices(&self, replacement: &Rc<Function>);
//}
//
//impl SelfIndices for Expr {
//    fn patch_self_indices(&self, replacement: &Rc<Function>) {
//        for instr in self.iter() {
//            instr.patch_self_indices(replacement);
//        }
//    }
//}
//
//impl SelfIndices for Instr {
//    fn patch_self_indices(&self, replacement: &Rc<Function>) {
//        match *self {
//            Instr::Block(_, ref expr) => expr.patch_self_indices(replacement),
//            Instr::Loop(_, ref expr) => expr.patch_self_indices(replacement),
//            Instr::If(_, ref expr) => expr.patch_self_indices(replacement),
//            Instr::Else(ref expr) => expr.patch_self_indices(replacement),
//
////            Instr::Br(_) => {},
////            Instr::BrIf(_) => {},
////            Instr::BrTable(_, _) => {},
//
//            Instr::Call(ref idx) if idx.borrow().is_none() => *idx.borrow_mut() = Some(replacement.clone()),
////            Instr::CallIndirect(_, _) => {},
//
////            Instr::GetLocal(_) => {},
////            Instr::SetLocal(_) => {},
////            Instr::TeeLocal(_) => {},
////            Instr::GetGlobal(_) => {},
////            Instr::SetGlobal(_) => {},
//
////            Instr::CurrentMemory(_) => {},
////            Instr::GrowMemory(_) => {},
//
//            _ => {}
//        }
//    }
//}