use std::collections::HashMap;
use super::*;
use super::highlevel as hl;
use super::lowlevel as ll;

/* Conversions between high-level and low-level AST. */


/* From low-level to high-level. */

impl From<ll::Module> for hl::Module {
    fn from(ll::Module { sections }: ll::Module) -> Self {
        let mut module = hl::Module::default();
        let mut types: Vec<FunctionType> = Vec::new();

        for section in sections {
            match section {
                ll::Section::Custom(bytes) => module.custom_sections.push(bytes),
                ll::Section::Type(ll::WithSize(types_)) => types = types_,

                /* Imported functions, tables, memories, and globals are first added to the respective index spaces... */

                ll::Section::Import(vec) => {
                    for import_ in vec.0 {
                        let import = Some((import_.module, import_.name));
                        let export = None;
                        match import_.type_ {
                            ll::ImportType::Function(type_idx) => module.functions.push(hl::Function {
                                type_: types[type_idx.0].clone(),
                                import,
                                code: None,
                                export,
                            }),
                            ll::ImportType::Table(type_) => module.tables.push(hl::Table {
                                type_,
                                import,
                                elements: Vec::new(),
                                export,
                            }),
                            ll::ImportType::Memory(type_) => module.memories.push(hl::Memory {
                                type_,
                                import,
                                data: Vec::new(),
                                export,
                            }),
                            ll::ImportType::Global(type_) => module.globals.push(hl::Global {
                                type_,
                                import,
                                init: None,
                                export,
                            }),
                        }
                    }
                }

                /* Then all "local" (i.e., non-imported) functions/tables/memories/globals are added. */

                ll::Section::Function(ll::WithSize(function_signatures)) => {
                    for type_idx in function_signatures {
                        module.functions.push(hl::Function {
                            type_: types[type_idx.0].clone(),
                            import: None,
                            code: None,
                            export: None,
                        });
                    }
                }
                ll::Section::Table(ll::WithSize(tables)) => {
                    for type_ in tables {
                        module.tables.push(hl::Table {
                            type_,
                            import: None,
                            elements: Vec::new(),
                            export: None,
                        });
                    }
                }
                ll::Section::Memory(ll::WithSize(memories)) => {
                    for type_ in memories {
                        module.memories.push(hl::Memory {
                            type_,
                            import: None,
                            data: Vec::new(),
                            export: None,
                        });
                    }
                }
                ll::Section::Global(ll::WithSize(globals)) => {
                    for ll::Global { type_, init } in globals {
                        module.globals.push(hl::Global {
                            type_,
                            import: None,
                            init: Some(from_lowlevel_expr(init, &types)),
                            export: None,
                        });
                    }
                }

                /* Other metadata sections: Export, Start */

                ll::Section::Export(ll::WithSize(exports)) => {
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
                ll::Section::Start(ll::WithSize(function_idx)) => module.start = Some(function_idx.0.into()),

                /* Finally, all "contents" of the already declared functions/tables/memories. */

                ll::Section::Element(ll::WithSize(elements)) => {
                    for element in elements {
                        module.tables[element.table_idx.0].elements.push(hl::Element {
                            offset: from_lowlevel_expr(element.offset, &types),
                            functions: element.init.into_iter().map(|idx| idx.0.into()).collect(),
                        })
                    }
                }
                ll::Section::Code(ll::WithSize(code)) => {
                    let imported_function_count = module.functions.iter()
                        .filter(|f| f.import.is_some())
                        .count();
                    for (i, ll::WithSize(code)) in code.into_iter().enumerate() {
                        module.functions[imported_function_count + i].code =
                            Some(from_lowlevel_code(code, &types))
                    }
                }
                ll::Section::Data(ll::WithSize(data)) => {
                    for data in data {
                        module.memories[data.memory_idx.0].data.push(hl::Data {
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

fn from_lowlevel_code(code: ll::Code, types: &[FunctionType]) -> hl::Code {
    let mut locals = Vec::new();
    for local in code.locals {
        for _ in 0..local.count {
            locals.push(local.type_);
        }
    }
    hl::Code {
        locals,
        body: from_lowlevel_expr(code.body, types),
    }
}

fn from_lowlevel_expr(expr: ll::Expr, types: &[FunctionType]) -> hl::Expr {
    expr.into_iter().map(|instr| from_lowlevel_instr(instr, types)).collect()
}

fn from_lowlevel_instr(instr: ll::Instr, types: &[FunctionType]) -> hl::Instr {
    match instr {
        ll::Instr::Unreachable => hl::Instr::Unreachable,
        ll::Instr::Nop => hl::Instr::Nop,

        ll::Instr::Block(block_type) => hl::Instr::Block(block_type),
        ll::Instr::Loop(block_type) => hl::Instr::Loop(block_type),
        ll::Instr::If(block_type) => hl::Instr::If(block_type),
        ll::Instr::Else => hl::Instr::Else,
        ll::Instr::End => hl::Instr::End,

        ll::Instr::Br(label_idx) => hl::Instr::Br(label_idx),
        ll::Instr::BrIf(label_idx) => hl::Instr::BrIf(label_idx),
        ll::Instr::BrTable(label_idx_table, default) => hl::Instr::BrTable(label_idx_table, default),

        ll::Instr::Return => hl::Instr::Return,
        ll::Instr::Call(function_idx) => hl::Instr::Call(function_idx.0.into()),
        ll::Instr::CallIndirect(type_idx, table_idx) => hl::Instr::CallIndirect(types[type_idx.0].clone(), table_idx.0.into()),

        ll::Instr::Drop => hl::Instr::Drop,
        ll::Instr::Select => hl::Instr::Select,

        ll::Instr::GetLocal(local_idx) => hl::Instr::GetLocal(local_idx.0.into()),
        ll::Instr::SetLocal(local_idx) => hl::Instr::SetLocal(local_idx.0.into()),
        ll::Instr::TeeLocal(local_idx) => hl::Instr::TeeLocal(local_idx.0.into()),
        ll::Instr::GetGlobal(global_idx) => hl::Instr::GetGlobal(global_idx.0.into()),
        ll::Instr::SetGlobal(global_idx) => hl::Instr::SetGlobal(global_idx.0.into()),

        ll::Instr::I32Load(memarg) => hl::Instr::I32Load(memarg),
        ll::Instr::I64Load(memarg) => hl::Instr::I64Load(memarg),
        ll::Instr::F32Load(memarg) => hl::Instr::F32Load(memarg),
        ll::Instr::F64Load(memarg) => hl::Instr::F64Load(memarg),
        ll::Instr::I32Load8S(memarg) => hl::Instr::I32Load8S(memarg),
        ll::Instr::I32Load8U(memarg) => hl::Instr::I32Load8U(memarg),
        ll::Instr::I32Load16S(memarg) => hl::Instr::I32Load16S(memarg),
        ll::Instr::I32Load16U(memarg) => hl::Instr::I32Load16U(memarg),
        ll::Instr::I64Load8S(memarg) => hl::Instr::I64Load8S(memarg),
        ll::Instr::I64Load8U(memarg) => hl::Instr::I64Load8U(memarg),
        ll::Instr::I64Load16S(memarg) => hl::Instr::I64Load16S(memarg),
        ll::Instr::I64Load16U(memarg) => hl::Instr::I64Load16U(memarg),
        ll::Instr::I64Load32S(memarg) => hl::Instr::I64Load32S(memarg),
        ll::Instr::I64Load32U(memarg) => hl::Instr::I64Load32U(memarg),
        ll::Instr::I32Store(memarg) => hl::Instr::I32Store(memarg),
        ll::Instr::I64Store(memarg) => hl::Instr::I64Store(memarg),
        ll::Instr::F32Store(memarg) => hl::Instr::F32Store(memarg),
        ll::Instr::F64Store(memarg) => hl::Instr::F64Store(memarg),
        ll::Instr::I32Store8(memarg) => hl::Instr::I32Store8(memarg),
        ll::Instr::I32Store16(memarg) => hl::Instr::I32Store16(memarg),
        ll::Instr::I64Store8(memarg) => hl::Instr::I64Store8(memarg),
        ll::Instr::I64Store16(memarg) => hl::Instr::I64Store16(memarg),
        ll::Instr::I64Store32(memarg) => hl::Instr::I64Store32(memarg),

        ll::Instr::MemorySize(memory_idx) => hl::Instr::MemorySize(memory_idx.0.into()),
        ll::Instr::MemoryGrow(memory_idx) => hl::Instr::MemoryGrow(memory_idx.0.into()),

        ll::Instr::I32Const(immediate) => hl::Instr::I32Const(immediate),
        ll::Instr::I64Const(immediate) => hl::Instr::I64Const(immediate),
        ll::Instr::F32Const(immediate) => hl::Instr::F32Const(immediate),
        ll::Instr::F64Const(immediate) => hl::Instr::F64Const(immediate),

        ll::Instr::I32Eqz => hl::Instr::I32Eqz,
        ll::Instr::I32Eq => hl::Instr::I32Eq,
        ll::Instr::I32Ne => hl::Instr::I32Ne,
        ll::Instr::I32LtS => hl::Instr::I32LtS,
        ll::Instr::I32LtU => hl::Instr::I32LtU,
        ll::Instr::I32GtS => hl::Instr::I32GtS,
        ll::Instr::I32GtU => hl::Instr::I32GtU,
        ll::Instr::I32LeS => hl::Instr::I32LeS,
        ll::Instr::I32LeU => hl::Instr::I32LeU,
        ll::Instr::I32GeS => hl::Instr::I32GeS,
        ll::Instr::I32GeU => hl::Instr::I32GeU,
        ll::Instr::I64Eqz => hl::Instr::I64Eqz,
        ll::Instr::I64Eq => hl::Instr::I64Eq,
        ll::Instr::I64Ne => hl::Instr::I64Ne,
        ll::Instr::I64LtS => hl::Instr::I64LtS,
        ll::Instr::I64LtU => hl::Instr::I64LtU,
        ll::Instr::I64GtS => hl::Instr::I64GtS,
        ll::Instr::I64GtU => hl::Instr::I64GtU,
        ll::Instr::I64LeS => hl::Instr::I64LeS,
        ll::Instr::I64LeU => hl::Instr::I64LeU,
        ll::Instr::I64GeS => hl::Instr::I64GeS,
        ll::Instr::I64GeU => hl::Instr::I64GeU,
        ll::Instr::F32Eq => hl::Instr::F32Eq,
        ll::Instr::F32Ne => hl::Instr::F32Ne,
        ll::Instr::F32Lt => hl::Instr::F32Lt,
        ll::Instr::F32Gt => hl::Instr::F32Gt,
        ll::Instr::F32Le => hl::Instr::F32Le,
        ll::Instr::F32Ge => hl::Instr::F32Ge,
        ll::Instr::F64Eq => hl::Instr::F64Eq,
        ll::Instr::F64Ne => hl::Instr::F64Ne,
        ll::Instr::F64Lt => hl::Instr::F64Lt,
        ll::Instr::F64Gt => hl::Instr::F64Gt,
        ll::Instr::F64Le => hl::Instr::F64Le,
        ll::Instr::F64Ge => hl::Instr::F64Ge,
        ll::Instr::I32Clz => hl::Instr::I32Clz,
        ll::Instr::I32Ctz => hl::Instr::I32Ctz,
        ll::Instr::I32Popcnt => hl::Instr::I32Popcnt,
        ll::Instr::I32Add => hl::Instr::I32Add,
        ll::Instr::I32Sub => hl::Instr::I32Sub,
        ll::Instr::I32Mul => hl::Instr::I32Mul,
        ll::Instr::I32DivS => hl::Instr::I32DivS,
        ll::Instr::I32DivU => hl::Instr::I32DivU,
        ll::Instr::I32RemS => hl::Instr::I32RemS,
        ll::Instr::I32RemU => hl::Instr::I32RemU,
        ll::Instr::I32And => hl::Instr::I32And,
        ll::Instr::I32Or => hl::Instr::I32Or,
        ll::Instr::I32Xor => hl::Instr::I32Xor,
        ll::Instr::I32Shl => hl::Instr::I32Shl,
        ll::Instr::I32ShrS => hl::Instr::I32ShrS,
        ll::Instr::I32ShrU => hl::Instr::I32ShrU,
        ll::Instr::I32Rotl => hl::Instr::I32Rotl,
        ll::Instr::I32Rotr => hl::Instr::I32Rotr,
        ll::Instr::I64Clz => hl::Instr::I64Clz,
        ll::Instr::I64Ctz => hl::Instr::I64Ctz,
        ll::Instr::I64Popcnt => hl::Instr::I64Popcnt,
        ll::Instr::I64Add => hl::Instr::I64Add,
        ll::Instr::I64Sub => hl::Instr::I64Sub,
        ll::Instr::I64Mul => hl::Instr::I64Mul,
        ll::Instr::I64DivS => hl::Instr::I64DivS,
        ll::Instr::I64DivU => hl::Instr::I64DivU,
        ll::Instr::I64RemS => hl::Instr::I64RemS,
        ll::Instr::I64RemU => hl::Instr::I64RemU,
        ll::Instr::I64And => hl::Instr::I64And,
        ll::Instr::I64Or => hl::Instr::I64Or,
        ll::Instr::I64Xor => hl::Instr::I64Xor,
        ll::Instr::I64Shl => hl::Instr::I64Shl,
        ll::Instr::I64ShrS => hl::Instr::I64ShrS,
        ll::Instr::I64ShrU => hl::Instr::I64ShrU,
        ll::Instr::I64Rotl => hl::Instr::I64Rotl,
        ll::Instr::I64Rotr => hl::Instr::I64Rotr,
        ll::Instr::F32Abs => hl::Instr::F32Abs,
        ll::Instr::F32Neg => hl::Instr::F32Neg,
        ll::Instr::F32Ceil => hl::Instr::F32Ceil,
        ll::Instr::F32Floor => hl::Instr::F32Floor,
        ll::Instr::F32Trunc => hl::Instr::F32Trunc,
        ll::Instr::F32Nearest => hl::Instr::F32Nearest,
        ll::Instr::F32Sqrt => hl::Instr::F32Sqrt,
        ll::Instr::F32Add => hl::Instr::F32Add,
        ll::Instr::F32Sub => hl::Instr::F32Sub,
        ll::Instr::F32Mul => hl::Instr::F32Mul,
        ll::Instr::F32Div => hl::Instr::F32Div,
        ll::Instr::F32Min => hl::Instr::F32Min,
        ll::Instr::F32Max => hl::Instr::F32Max,
        ll::Instr::F32Copysign => hl::Instr::F32Copysign,
        ll::Instr::F64Abs => hl::Instr::F64Abs,
        ll::Instr::F64Neg => hl::Instr::F64Neg,
        ll::Instr::F64Ceil => hl::Instr::F64Ceil,
        ll::Instr::F64Floor => hl::Instr::F64Floor,
        ll::Instr::F64Trunc => hl::Instr::F64Trunc,
        ll::Instr::F64Nearest => hl::Instr::F64Nearest,
        ll::Instr::F64Sqrt => hl::Instr::F64Sqrt,
        ll::Instr::F64Add => hl::Instr::F64Add,
        ll::Instr::F64Sub => hl::Instr::F64Sub,
        ll::Instr::F64Mul => hl::Instr::F64Mul,
        ll::Instr::F64Div => hl::Instr::F64Div,
        ll::Instr::F64Min => hl::Instr::F64Min,
        ll::Instr::F64Max => hl::Instr::F64Max,
        ll::Instr::F64Copysign => hl::Instr::F64Copysign,
        ll::Instr::I32WrapI64 => hl::Instr::I32WrapI64,
        ll::Instr::I32TruncSF32 => hl::Instr::I32TruncSF32,
        ll::Instr::I32TruncUF32 => hl::Instr::I32TruncUF32,
        ll::Instr::I32TruncSF64 => hl::Instr::I32TruncSF64,
        ll::Instr::I32TruncUF64 => hl::Instr::I32TruncUF64,
        ll::Instr::I64ExtendSI32 => hl::Instr::I64ExtendSI32,
        ll::Instr::I64ExtendUI32 => hl::Instr::I64ExtendUI32,
        ll::Instr::I64TruncSF32 => hl::Instr::I64TruncSF32,
        ll::Instr::I64TruncUF32 => hl::Instr::I64TruncUF32,
        ll::Instr::I64TruncSF64 => hl::Instr::I64TruncSF64,
        ll::Instr::I64TruncUF64 => hl::Instr::I64TruncUF64,
        ll::Instr::F32ConvertSI32 => hl::Instr::F32ConvertSI32,
        ll::Instr::F32ConvertUI32 => hl::Instr::F32ConvertUI32,
        ll::Instr::F32ConvertSI64 => hl::Instr::F32ConvertSI64,
        ll::Instr::F32ConvertUI64 => hl::Instr::F32ConvertUI64,
        ll::Instr::F32DemoteF64 => hl::Instr::F32DemoteF64,
        ll::Instr::F64ConvertSI32 => hl::Instr::F64ConvertSI32,
        ll::Instr::F64ConvertUI32 => hl::Instr::F64ConvertUI32,
        ll::Instr::F64ConvertSI64 => hl::Instr::F64ConvertSI64,
        ll::Instr::F64ConvertUI64 => hl::Instr::F64ConvertUI64,
        ll::Instr::F64PromoteF32 => hl::Instr::F64PromoteF32,
        ll::Instr::I32ReinterpretF32 => hl::Instr::I32ReinterpretF32,
        ll::Instr::I64ReinterpretF64 => hl::Instr::I64ReinterpretF64,
        ll::Instr::F32ReinterpretI32 => hl::Instr::F32ReinterpretI32,
        ll::Instr::F64ReinterpretI64 => hl::Instr::F64ReinterpretI64,
    }
}


/* From high-level to low-level. */

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

impl From<hl::Module> for ll::Module {
    fn from(module: hl::Module) -> Self {
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
        if !types.is_empty() {
            sections.push(ll::Section::Type(ll::WithSize(types)));
        }

        // Import
        if !imports.is_empty() {
            sections.push(ll::Section::Import(ll::WithSize(imports)));
        }

        // Function
        if !functions.is_empty() {
            sections.push(ll::Section::Function(ll::WithSize(functions)));
        }

        // Table
        if !tables.is_empty() {
            sections.push(ll::Section::Table(ll::WithSize(tables)));
        }

        // Memory
        if !memories.is_empty() {
            sections.push(ll::Section::Memory(ll::WithSize(memories)));
        }

        // Global
        if !globals.is_empty() {
            sections.push(ll::Section::Global(ll::WithSize(globals)));
        }

        // Export
        let exports = to_lowlevel_exports(&module, &state);
        if !exports.is_empty() {
            sections.push(ll::Section::Export(ll::WithSize(exports)));
        }

        // Start
        for start in module.start {
            sections.push(ll::Section::Start(ll::WithSize(state.map_function_idx(start.0))));
        }

        // Element
        let elements: Vec<ll::Element> = module.tables.into_iter()
            .enumerate()
            .flat_map(|(i, table)| table.elements.into_iter()
                .map(|element| ll::Element {
                    table_idx: state.map_table_idx(i),
                    offset: to_lowlevel_expr(&element.offset, &state),
                    init: element.functions.iter().map(|fn_idx| state.map_function_idx(fn_idx.0)).collect(),
                })
                .collect::<Vec<_>>())
            .collect();
        if !elements.is_empty() {
            sections.push(ll::Section::Element(ll::WithSize(elements)));
        }

        // Code
        let code: Vec<ll::WithSize<ll::Code>> = module.functions.into_iter()
            .filter_map(|function|
                function.code.map(|code| ll::WithSize(to_lowlevel_code(code, &state))))
            .collect();
        if !code.is_empty() {
            sections.push(ll::Section::Code(ll::WithSize(code)));
        }

        // Data
        let data: Vec<ll::Data> = module.memories.into_iter()
            .enumerate()
            .flat_map(|(i, memory)| memory.data.into_iter()
                .map(|data| ll::Data {
                    memory_idx: state.map_memory_idx(i),
                    offset: to_lowlevel_expr(&data.offset, &state),
                    init: data.bytes,
                })
                .collect::<Vec<_>>())
            .collect();
        if !data.is_empty() {
            sections.push(ll::Section::Data(ll::WithSize(data)));
        }

        // Custom
        for custom in module.custom_sections {
            sections.push(ll::Section::Custom(custom));
        }

        ll::Module { sections }
    }
}

fn to_lowlevel_imports(module: &hl::Module, state: &mut EncodeState) -> Vec<ll::Import> {
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

fn to_lowlevel_functions(functions: &[hl::Function], state: &mut EncodeState) -> Vec<Idx<FunctionType>> {
    to_lowlevel_elements!(functions, state, insert_function_idx, |func: &hl::Function| state.get_or_insert_type(func.type_.clone()))
}

fn to_lowlevel_tables(tables: &[hl::Table], state: &mut EncodeState) -> Vec<TableType> {
    to_lowlevel_elements!(tables, state, insert_table_idx, |table: &hl::Table| table.type_.clone())
}

fn to_lowlevel_memories(memories: &[hl::Memory], state: &mut EncodeState) -> Vec<MemoryType> {
    to_lowlevel_elements!(memories, state, insert_memory_idx, |memory: &hl::Memory| memory.type_.clone())
}

fn to_lowlevel_globals(globals: &[hl::Global], state: &mut EncodeState) -> Vec<ll::Global> {
    to_lowlevel_elements!(globals, state, insert_global_idx, |global: &hl::Global| ll::Global {
        type_: global.type_,
        init: to_lowlevel_expr(&global.init.as_ref().unwrap(), state),
    })
}

fn to_lowlevel_exports(module: &hl::Module, state: &EncodeState) -> Vec<ll::Export> {
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

fn to_lowlevel_code(code: hl::Code, state: &EncodeState) -> ll::Code {
    let mut locals = Vec::new();
    for type_ in code.locals {
        if locals.last().map(|locals: &ll::Locals| locals.type_ == type_).unwrap_or(false) {
            let last = locals.len() - 1;
            locals[last].count += 1;
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

fn to_lowlevel_expr(expr: &[hl::Instr], state: &EncodeState) -> ll::Expr {
    expr.iter().map(|instr| to_lowlevel_instr(instr, state)).collect()
}

fn to_lowlevel_instr(instr: &hl::Instr, state: &EncodeState) -> ll::Instr {
    match *instr {
        hl::Instr::Unreachable => ll::Instr::Unreachable,
        hl::Instr::Nop => ll::Instr::Nop,

        hl::Instr::Block(block_type) => ll::Instr::Block(block_type),
        hl::Instr::Loop(block_type) => ll::Instr::Loop(block_type),
        hl::Instr::If(block_type) => ll::Instr::If(block_type),
        hl::Instr::Else => ll::Instr::Else,
        hl::Instr::End => ll::Instr::End,

        hl::Instr::Br(label_idx) => ll::Instr::Br(label_idx),
        hl::Instr::BrIf(label_idx) => ll::Instr::BrIf(label_idx),
        hl::Instr::BrTable(ref label_idx_table, default) => ll::Instr::BrTable(label_idx_table.clone(), default),

        hl::Instr::Return => ll::Instr::Return,
        hl::Instr::Call(function_idx) => ll::Instr::Call(state.map_function_idx(function_idx.0)),
        hl::Instr::CallIndirect(ref type_, table_idx) => ll::Instr::CallIndirect(state.get_type_idx(&type_), state.map_table_idx(table_idx.0)),

        hl::Instr::Drop => ll::Instr::Drop,
        hl::Instr::Select => ll::Instr::Select,

        hl::Instr::GetLocal(local_idx) => ll::Instr::GetLocal(local_idx.0.into()),
        hl::Instr::SetLocal(local_idx) => ll::Instr::SetLocal(local_idx.0.into()),
        hl::Instr::TeeLocal(local_idx) => ll::Instr::TeeLocal(local_idx.0.into()),
        hl::Instr::GetGlobal(global_idx) => ll::Instr::GetGlobal(state.map_global_idx(global_idx.0)),
        hl::Instr::SetGlobal(global_idx) => ll::Instr::SetGlobal(state.map_global_idx(global_idx.0)),

        hl::Instr::I32Load(memarg) => ll::Instr::I32Load(memarg),
        hl::Instr::I64Load(memarg) => ll::Instr::I64Load(memarg),
        hl::Instr::F32Load(memarg) => ll::Instr::F32Load(memarg),
        hl::Instr::F64Load(memarg) => ll::Instr::F64Load(memarg),
        hl::Instr::I32Load8S(memarg) => ll::Instr::I32Load8S(memarg),
        hl::Instr::I32Load8U(memarg) => ll::Instr::I32Load8U(memarg),
        hl::Instr::I32Load16S(memarg) => ll::Instr::I32Load16S(memarg),
        hl::Instr::I32Load16U(memarg) => ll::Instr::I32Load16U(memarg),
        hl::Instr::I64Load8S(memarg) => ll::Instr::I64Load8S(memarg),
        hl::Instr::I64Load8U(memarg) => ll::Instr::I64Load8U(memarg),
        hl::Instr::I64Load16S(memarg) => ll::Instr::I64Load16S(memarg),
        hl::Instr::I64Load16U(memarg) => ll::Instr::I64Load16U(memarg),
        hl::Instr::I64Load32S(memarg) => ll::Instr::I64Load32S(memarg),
        hl::Instr::I64Load32U(memarg) => ll::Instr::I64Load32U(memarg),
        hl::Instr::I32Store(memarg) => ll::Instr::I32Store(memarg),
        hl::Instr::I64Store(memarg) => ll::Instr::I64Store(memarg),
        hl::Instr::F32Store(memarg) => ll::Instr::F32Store(memarg),
        hl::Instr::F64Store(memarg) => ll::Instr::F64Store(memarg),
        hl::Instr::I32Store8(memarg) => ll::Instr::I32Store8(memarg),
        hl::Instr::I32Store16(memarg) => ll::Instr::I32Store16(memarg),
        hl::Instr::I64Store8(memarg) => ll::Instr::I64Store8(memarg),
        hl::Instr::I64Store16(memarg) => ll::Instr::I64Store16(memarg),
        hl::Instr::I64Store32(memarg) => ll::Instr::I64Store32(memarg),

        hl::Instr::MemorySize(memory_idx) => ll::Instr::MemorySize(state.map_memory_idx(memory_idx.0)),
        hl::Instr::MemoryGrow(memory_idx) => ll::Instr::MemoryGrow(state.map_memory_idx(memory_idx.0)),

        hl::Instr::I32Const(immediate) => ll::Instr::I32Const(immediate),
        hl::Instr::I64Const(immediate) => ll::Instr::I64Const(immediate),
        hl::Instr::F32Const(immediate) => ll::Instr::F32Const(immediate),
        hl::Instr::F64Const(immediate) => ll::Instr::F64Const(immediate),

        hl::Instr::I32Eqz => ll::Instr::I32Eqz,
        hl::Instr::I32Eq => ll::Instr::I32Eq,
        hl::Instr::I32Ne => ll::Instr::I32Ne,
        hl::Instr::I32LtS => ll::Instr::I32LtS,
        hl::Instr::I32LtU => ll::Instr::I32LtU,
        hl::Instr::I32GtS => ll::Instr::I32GtS,
        hl::Instr::I32GtU => ll::Instr::I32GtU,
        hl::Instr::I32LeS => ll::Instr::I32LeS,
        hl::Instr::I32LeU => ll::Instr::I32LeU,
        hl::Instr::I32GeS => ll::Instr::I32GeS,
        hl::Instr::I32GeU => ll::Instr::I32GeU,
        hl::Instr::I64Eqz => ll::Instr::I64Eqz,
        hl::Instr::I64Eq => ll::Instr::I64Eq,
        hl::Instr::I64Ne => ll::Instr::I64Ne,
        hl::Instr::I64LtS => ll::Instr::I64LtS,
        hl::Instr::I64LtU => ll::Instr::I64LtU,
        hl::Instr::I64GtS => ll::Instr::I64GtS,
        hl::Instr::I64GtU => ll::Instr::I64GtU,
        hl::Instr::I64LeS => ll::Instr::I64LeS,
        hl::Instr::I64LeU => ll::Instr::I64LeU,
        hl::Instr::I64GeS => ll::Instr::I64GeS,
        hl::Instr::I64GeU => ll::Instr::I64GeU,
        hl::Instr::F32Eq => ll::Instr::F32Eq,
        hl::Instr::F32Ne => ll::Instr::F32Ne,
        hl::Instr::F32Lt => ll::Instr::F32Lt,
        hl::Instr::F32Gt => ll::Instr::F32Gt,
        hl::Instr::F32Le => ll::Instr::F32Le,
        hl::Instr::F32Ge => ll::Instr::F32Ge,
        hl::Instr::F64Eq => ll::Instr::F64Eq,
        hl::Instr::F64Ne => ll::Instr::F64Ne,
        hl::Instr::F64Lt => ll::Instr::F64Lt,
        hl::Instr::F64Gt => ll::Instr::F64Gt,
        hl::Instr::F64Le => ll::Instr::F64Le,
        hl::Instr::F64Ge => ll::Instr::F64Ge,
        hl::Instr::I32Clz => ll::Instr::I32Clz,
        hl::Instr::I32Ctz => ll::Instr::I32Ctz,
        hl::Instr::I32Popcnt => ll::Instr::I32Popcnt,
        hl::Instr::I32Add => ll::Instr::I32Add,
        hl::Instr::I32Sub => ll::Instr::I32Sub,
        hl::Instr::I32Mul => ll::Instr::I32Mul,
        hl::Instr::I32DivS => ll::Instr::I32DivS,
        hl::Instr::I32DivU => ll::Instr::I32DivU,
        hl::Instr::I32RemS => ll::Instr::I32RemS,
        hl::Instr::I32RemU => ll::Instr::I32RemU,
        hl::Instr::I32And => ll::Instr::I32And,
        hl::Instr::I32Or => ll::Instr::I32Or,
        hl::Instr::I32Xor => ll::Instr::I32Xor,
        hl::Instr::I32Shl => ll::Instr::I32Shl,
        hl::Instr::I32ShrS => ll::Instr::I32ShrS,
        hl::Instr::I32ShrU => ll::Instr::I32ShrU,
        hl::Instr::I32Rotl => ll::Instr::I32Rotl,
        hl::Instr::I32Rotr => ll::Instr::I32Rotr,
        hl::Instr::I64Clz => ll::Instr::I64Clz,
        hl::Instr::I64Ctz => ll::Instr::I64Ctz,
        hl::Instr::I64Popcnt => ll::Instr::I64Popcnt,
        hl::Instr::I64Add => ll::Instr::I64Add,
        hl::Instr::I64Sub => ll::Instr::I64Sub,
        hl::Instr::I64Mul => ll::Instr::I64Mul,
        hl::Instr::I64DivS => ll::Instr::I64DivS,
        hl::Instr::I64DivU => ll::Instr::I64DivU,
        hl::Instr::I64RemS => ll::Instr::I64RemS,
        hl::Instr::I64RemU => ll::Instr::I64RemU,
        hl::Instr::I64And => ll::Instr::I64And,
        hl::Instr::I64Or => ll::Instr::I64Or,
        hl::Instr::I64Xor => ll::Instr::I64Xor,
        hl::Instr::I64Shl => ll::Instr::I64Shl,
        hl::Instr::I64ShrS => ll::Instr::I64ShrS,
        hl::Instr::I64ShrU => ll::Instr::I64ShrU,
        hl::Instr::I64Rotl => ll::Instr::I64Rotl,
        hl::Instr::I64Rotr => ll::Instr::I64Rotr,
        hl::Instr::F32Abs => ll::Instr::F32Abs,
        hl::Instr::F32Neg => ll::Instr::F32Neg,
        hl::Instr::F32Ceil => ll::Instr::F32Ceil,
        hl::Instr::F32Floor => ll::Instr::F32Floor,
        hl::Instr::F32Trunc => ll::Instr::F32Trunc,
        hl::Instr::F32Nearest => ll::Instr::F32Nearest,
        hl::Instr::F32Sqrt => ll::Instr::F32Sqrt,
        hl::Instr::F32Add => ll::Instr::F32Add,
        hl::Instr::F32Sub => ll::Instr::F32Sub,
        hl::Instr::F32Mul => ll::Instr::F32Mul,
        hl::Instr::F32Div => ll::Instr::F32Div,
        hl::Instr::F32Min => ll::Instr::F32Min,
        hl::Instr::F32Max => ll::Instr::F32Max,
        hl::Instr::F32Copysign => ll::Instr::F32Copysign,
        hl::Instr::F64Abs => ll::Instr::F64Abs,
        hl::Instr::F64Neg => ll::Instr::F64Neg,
        hl::Instr::F64Ceil => ll::Instr::F64Ceil,
        hl::Instr::F64Floor => ll::Instr::F64Floor,
        hl::Instr::F64Trunc => ll::Instr::F64Trunc,
        hl::Instr::F64Nearest => ll::Instr::F64Nearest,
        hl::Instr::F64Sqrt => ll::Instr::F64Sqrt,
        hl::Instr::F64Add => ll::Instr::F64Add,
        hl::Instr::F64Sub => ll::Instr::F64Sub,
        hl::Instr::F64Mul => ll::Instr::F64Mul,
        hl::Instr::F64Div => ll::Instr::F64Div,
        hl::Instr::F64Min => ll::Instr::F64Min,
        hl::Instr::F64Max => ll::Instr::F64Max,
        hl::Instr::F64Copysign => ll::Instr::F64Copysign,
        hl::Instr::I32WrapI64 => ll::Instr::I32WrapI64,
        hl::Instr::I32TruncSF32 => ll::Instr::I32TruncSF32,
        hl::Instr::I32TruncUF32 => ll::Instr::I32TruncUF32,
        hl::Instr::I32TruncSF64 => ll::Instr::I32TruncSF64,
        hl::Instr::I32TruncUF64 => ll::Instr::I32TruncUF64,
        hl::Instr::I64ExtendSI32 => ll::Instr::I64ExtendSI32,
        hl::Instr::I64ExtendUI32 => ll::Instr::I64ExtendUI32,
        hl::Instr::I64TruncSF32 => ll::Instr::I64TruncSF32,
        hl::Instr::I64TruncUF32 => ll::Instr::I64TruncUF32,
        hl::Instr::I64TruncSF64 => ll::Instr::I64TruncSF64,
        hl::Instr::I64TruncUF64 => ll::Instr::I64TruncUF64,
        hl::Instr::F32ConvertSI32 => ll::Instr::F32ConvertSI32,
        hl::Instr::F32ConvertUI32 => ll::Instr::F32ConvertUI32,
        hl::Instr::F32ConvertSI64 => ll::Instr::F32ConvertSI64,
        hl::Instr::F32ConvertUI64 => ll::Instr::F32ConvertUI64,
        hl::Instr::F32DemoteF64 => ll::Instr::F32DemoteF64,
        hl::Instr::F64ConvertSI32 => ll::Instr::F64ConvertSI32,
        hl::Instr::F64ConvertUI32 => ll::Instr::F64ConvertUI32,
        hl::Instr::F64ConvertSI64 => ll::Instr::F64ConvertSI64,
        hl::Instr::F64ConvertUI64 => ll::Instr::F64ConvertUI64,
        hl::Instr::F64PromoteF32 => ll::Instr::F64PromoteF32,
        hl::Instr::I32ReinterpretF32 => ll::Instr::I32ReinterpretF32,
        hl::Instr::I64ReinterpretF64 => ll::Instr::I64ReinterpretF64,
        hl::Instr::F32ReinterpretI32 => ll::Instr::F32ReinterpretI32,
        hl::Instr::F64ReinterpretI64 => ll::Instr::F64ReinterpretI64,
    }
}