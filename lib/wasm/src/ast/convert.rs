use std::collections::HashMap;

use rayon::prelude::*;

use crate::{FunctionType, Idx, MemoryType, TableType, Val};
use crate::highlevel as hl;
use crate::lowlevel as ll;

/* Conversions between high-level and low-level AST. */

// TODO Convert all the panics in the conversion functions into errors, i.e., return Result<_, wasm::Error>.
// NOTE A lot of the panics are implicit, e.g., in indexing operations of Vecs etc.

/* From low-level to high-level. */

impl From<ll::Module> for hl::Module {
    fn from(ll::Module { sections }: ll::Module) -> Self {
        let mut module = hl::Module::default();
        let mut types: Vec<FunctionType> = Vec::new();

        for section in sections {
            match section {
                ll::Section::Custom(ll::CustomSection::Name(ll::NameSection { subsections })) => {
                    // Each name subsection should appear at most once and in the correct order.
                    let mut function_names = Vec::new();
                    let mut local_names = Vec::new();
                    for sec in subsections {
                        match sec {
                            ll::NameSubSection::Module(ll::WithSize(name)) => {
                                if module.name.is_some() {
                                    panic!("name section: more than one module name subsection");
                                }
                                if !function_names.is_empty() || !local_names.is_empty() {
                                    panic!("name section: out-of-order module name subsection");
                                }
                                module.name = Some(name);
                            }
                            ll::NameSubSection::Function(ll::WithSize(names)) => {
                                if !function_names.is_empty() {
                                    panic!("name section: more than one function names subsection");
                                }
                                if !local_names.is_empty() {
                                    panic!("name section: out-of-order function names subsection");
                                }
                                function_names = names;
                            }
                            ll::NameSubSection::Local(ll::WithSize(names)) => {
                                if !local_names.is_empty() {
                                    panic!("name section: more than one local names subsection");
                                }
                                local_names = names;
                            }
                            _ => {
                                // FIXME: This just ignores other low-level name sections!
                                // TODO: Add `.name` fields to `hl::Global` etc.
                            }
                        }
                    }

                    // Inline the name information for functions and locals into our high-level AST.
                    for ll::NameAssoc { idx, name } in function_names {
                        module.functions
                            .get_mut(idx.into_inner())
                            .expect("invalid function index")
                            .name = Some(name);
                    }
                    for ll::IndirectNameAssoc { idx: func_idx, name_map } in local_names {
                        let function = module.functions
                            .get_mut(func_idx.into_inner())
                            .expect("invalid function index");
                        for ll::NameAssoc { idx, name } in name_map {
                            *function.param_or_local_name_mut(idx.into_inner().into()) = Some(name);
                        }
                    }
                }

                // Pass through unknown custom sections unmodified.
                ll::Section::Custom(ll::CustomSection::Raw(sec)) => module.custom_sections.push(sec),

                ll::Section::Type(ll::WithSize(ll::SectionOffset(types_))) => types = types_,

                /* Imported functions, tables, memories, and globals are first added to the respective index spaces... */

                ll::Section::Import(ll::WithSize(ll::SectionOffset(imports))) => {
                    for import_ in imports {
                        let export = Vec::new();
                        match import_.type_ {
                            ll::ImportType::Function(type_idx) => module.functions.push(
                                hl::Function::new_imported(
                                    types[type_idx.into_inner()].clone(),
                                    import_.module, import_.name,
                                    export)
                            ),
                            ll::ImportType::Global(type_) => module.globals.push(hl::Global {
                                type_,
                                init: hl::ImportOrPresent::Import(import_.module, import_.name),
                                export,
                            }),
                            ll::ImportType::Table(type_) => module.tables.push(hl::Table {
                                type_,
                                import: Some((import_.module, import_.name)),
                                elements: Vec::new(),
                                export,
                            }),
                            ll::ImportType::Memory(type_) => module.memories.push(hl::Memory {
                                type_,
                                import: Some((import_.module, import_.name)),
                                data: Vec::new(),
                                export,
                            }),
                        }
                    }
                }

                /* Then all "local" (i.e., non-imported) functions/tables/memories/globals are added. */

                ll::Section::Function(ll::WithSize(ll::SectionOffset(function_signatures))) => {
                    for type_idx in function_signatures {
                        module.functions.push(
                            hl::Function::new(
                                types[type_idx.into_inner()].clone(),
                                // Use an empty body/locals for now, code is only converted later.
                                hl::Code { locals: vec![], body: vec![] },
                                Vec::new()
                            )
                        );
                    }
                }
                ll::Section::Global(ll::WithSize(ll::SectionOffset(globals))) => {
                    for ll::Global { type_, init } in globals {
                        module.globals.push(hl::Global {
                            type_,
                            init: hl::ImportOrPresent::Present(from_lowlevel_expr(init, &types)),
                            export: Vec::new(),
                        });
                    }
                }
                ll::Section::Table(ll::WithSize(ll::SectionOffset(tables))) => {
                    for type_ in tables {
                        module.tables.push(hl::Table {
                            type_,
                            import: None,
                            elements: Vec::new(),
                            export: Vec::new(),
                        });
                    }
                }
                ll::Section::Memory(ll::WithSize(ll::SectionOffset(memories))) => {
                    for type_ in memories {
                        module.memories.push(hl::Memory {
                            type_,
                            import: None,
                            data: Vec::new(),
                            export: Vec::new(),
                        });
                    }
                }

                /* Other metadata sections: Export, Start */

                ll::Section::Export(ll::WithSize(ll::SectionOffset(exports))) => {
                    for ll::Export { name, type_ } in exports {
                        match type_ {
                            ll::ExportType::Function(idx) => module.functions[idx.into_inner()].export.push(name),
                            ll::ExportType::Table(idx) => module.tables[idx.into_inner()].export.push(name),
                            ll::ExportType::Memory(idx) => module.memories[idx.into_inner()].export.push(name),
                            ll::ExportType::Global(idx) => module.globals[idx.into_inner()].export.push(name),
                        }
                    }
                }
                ll::Section::Start(ll::WithSize(ll::SectionOffset(function_idx))) => module.start = Some(function_idx.into_inner().into()),

                /* Finally, all "contents" of the already declared functions/tables/memories. */

                ll::Section::Element(ll::WithSize(ll::SectionOffset(elements))) => {
                    for element in elements {
                        module.tables[element.table_idx.into_inner()].elements.push(hl::Element {
                            offset: from_lowlevel_expr(element.offset, &types),
                            functions: element.init.into_iter().map(|idx| idx.into_inner().into()).collect(),
                        })
                    }
                }
                ll::Section::Code(ll::WithSize(ll::SectionOffset(code))) => {
                    let imported_function_count = module.functions.iter()
                        .filter(|f| f.import().is_some())
                        .count();
                    let code_hl: Vec<_> = code.0.into_par_iter().map(|ll::WithSize(code)| {
                        from_lowlevel_code(code, &types)
                    }).collect();
                    for (i, code) in code_hl.into_iter().enumerate() {
                        module.functions[imported_function_count + i].code = hl::ImportOrPresent::Present(code);
                    }
                }
                ll::Section::Data(ll::WithSize(ll::SectionOffset(data))) => {
                    for data in data {
                        module.memories[data.memory_idx.into_inner()].data.push(hl::Data {
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
    for ll::Locals { type_, count } in code.locals {
        for _ in 0..count {
            locals.push(hl::Local::new(type_));
        }
    }
    hl::Code {
        locals,
        body: from_lowlevel_expr(code.body, types),
    }
}

fn from_lowlevel_expr(expr: ll::Expr, types: &[FunctionType]) -> hl::Expr {
    expr.0.into_iter().map(|instr| from_lowlevel_instr(instr, types)).collect()
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
        ll::Instr::BrTable { table, default } => hl::Instr::BrTable { table, default },

        ll::Instr::Return => hl::Instr::Return,
        ll::Instr::Call(function_idx) => hl::Instr::Call(function_idx.into_inner().into()),
        ll::Instr::CallIndirect(type_idx, table_idx) => hl::Instr::CallIndirect(types[type_idx.into_inner()].clone(), table_idx.into_inner().into()),

        ll::Instr::Drop => hl::Instr::Drop,
        ll::Instr::Select => hl::Instr::Select,

        ll::Instr::LocalGet(local_idx) => hl::Instr::Local(hl::LocalOp::Get, local_idx.into_inner().into()),
        ll::Instr::LocalSet(local_idx) => hl::Instr::Local(hl::LocalOp::Set, local_idx.into_inner().into()),
        ll::Instr::LocalTee(local_idx) => hl::Instr::Local(hl::LocalOp::Tee, local_idx.into_inner().into()),
        ll::Instr::GlobalGet(global_idx) => hl::Instr::Global(hl::GlobalOp::Get, global_idx.into_inner().into()),
        ll::Instr::GlobalSet(global_idx) => hl::Instr::Global(hl::GlobalOp::Set, global_idx.into_inner().into()),

        ll::Instr::I32Load(memarg) => hl::Instr::Load(hl::LoadOp::I32Load, memarg),
        ll::Instr::I64Load(memarg) => hl::Instr::Load(hl::LoadOp::I64Load, memarg),
        ll::Instr::F32Load(memarg) => hl::Instr::Load(hl::LoadOp::F32Load, memarg),
        ll::Instr::F64Load(memarg) => hl::Instr::Load(hl::LoadOp::F64Load, memarg),
        ll::Instr::I32Load8S(memarg) => hl::Instr::Load(hl::LoadOp::I32Load8S, memarg),
        ll::Instr::I32Load8U(memarg) => hl::Instr::Load(hl::LoadOp::I32Load8U, memarg),
        ll::Instr::I32Load16S(memarg) => hl::Instr::Load(hl::LoadOp::I32Load16S, memarg),
        ll::Instr::I32Load16U(memarg) => hl::Instr::Load(hl::LoadOp::I32Load16U, memarg),
        ll::Instr::I64Load8S(memarg) => hl::Instr::Load(hl::LoadOp::I64Load8S, memarg),
        ll::Instr::I64Load8U(memarg) => hl::Instr::Load(hl::LoadOp::I64Load8U, memarg),
        ll::Instr::I64Load16S(memarg) => hl::Instr::Load(hl::LoadOp::I64Load16S, memarg),
        ll::Instr::I64Load16U(memarg) => hl::Instr::Load(hl::LoadOp::I64Load16U, memarg),
        ll::Instr::I64Load32S(memarg) => hl::Instr::Load(hl::LoadOp::I64Load32S, memarg),
        ll::Instr::I64Load32U(memarg) => hl::Instr::Load(hl::LoadOp::I64Load32U, memarg),
        ll::Instr::I32Store(memarg) => hl::Instr::Store(hl::StoreOp::I32Store, memarg),
        ll::Instr::I64Store(memarg) => hl::Instr::Store(hl::StoreOp::I64Store, memarg),
        ll::Instr::F32Store(memarg) => hl::Instr::Store(hl::StoreOp::F32Store, memarg),
        ll::Instr::F64Store(memarg) => hl::Instr::Store(hl::StoreOp::F64Store, memarg),
        ll::Instr::I32Store8(memarg) => hl::Instr::Store(hl::StoreOp::I32Store8, memarg),
        ll::Instr::I32Store16(memarg) => hl::Instr::Store(hl::StoreOp::I32Store16, memarg),
        ll::Instr::I64Store8(memarg) => hl::Instr::Store(hl::StoreOp::I64Store8, memarg),
        ll::Instr::I64Store16(memarg) => hl::Instr::Store(hl::StoreOp::I64Store16, memarg),
        ll::Instr::I64Store32(memarg) => hl::Instr::Store(hl::StoreOp::I64Store32, memarg),

        ll::Instr::MemorySize(memory_idx) => hl::Instr::MemorySize(memory_idx.into_inner().into()),
        ll::Instr::MemoryGrow(memory_idx) => hl::Instr::MemoryGrow(memory_idx.into_inner().into()),

        ll::Instr::I32Const(immediate) => hl::Instr::Const(Val::I32(immediate)),
        ll::Instr::I64Const(immediate) => hl::Instr::Const(Val::I64(immediate)),
        ll::Instr::F32Const(immediate) => hl::Instr::Const(Val::F32(immediate)),
        ll::Instr::F64Const(immediate) => hl::Instr::Const(Val::F64(immediate)),

        ll::Instr::I32Eqz => hl::Instr::Numeric(hl::NumericOp::I32Eqz),
        ll::Instr::I32Eq => hl::Instr::Numeric(hl::NumericOp::I32Eq),
        ll::Instr::I32Ne => hl::Instr::Numeric(hl::NumericOp::I32Ne),
        ll::Instr::I32LtS => hl::Instr::Numeric(hl::NumericOp::I32LtS),
        ll::Instr::I32LtU => hl::Instr::Numeric(hl::NumericOp::I32LtU),
        ll::Instr::I32GtS => hl::Instr::Numeric(hl::NumericOp::I32GtS),
        ll::Instr::I32GtU => hl::Instr::Numeric(hl::NumericOp::I32GtU),
        ll::Instr::I32LeS => hl::Instr::Numeric(hl::NumericOp::I32LeS),
        ll::Instr::I32LeU => hl::Instr::Numeric(hl::NumericOp::I32LeU),
        ll::Instr::I32GeS => hl::Instr::Numeric(hl::NumericOp::I32GeS),
        ll::Instr::I32GeU => hl::Instr::Numeric(hl::NumericOp::I32GeU),
        ll::Instr::I64Eqz => hl::Instr::Numeric(hl::NumericOp::I64Eqz),
        ll::Instr::I64Eq => hl::Instr::Numeric(hl::NumericOp::I64Eq),
        ll::Instr::I64Ne => hl::Instr::Numeric(hl::NumericOp::I64Ne),
        ll::Instr::I64LtS => hl::Instr::Numeric(hl::NumericOp::I64LtS),
        ll::Instr::I64LtU => hl::Instr::Numeric(hl::NumericOp::I64LtU),
        ll::Instr::I64GtS => hl::Instr::Numeric(hl::NumericOp::I64GtS),
        ll::Instr::I64GtU => hl::Instr::Numeric(hl::NumericOp::I64GtU),
        ll::Instr::I64LeS => hl::Instr::Numeric(hl::NumericOp::I64LeS),
        ll::Instr::I64LeU => hl::Instr::Numeric(hl::NumericOp::I64LeU),
        ll::Instr::I64GeS => hl::Instr::Numeric(hl::NumericOp::I64GeS),
        ll::Instr::I64GeU => hl::Instr::Numeric(hl::NumericOp::I64GeU),
        ll::Instr::F32Eq => hl::Instr::Numeric(hl::NumericOp::F32Eq),
        ll::Instr::F32Ne => hl::Instr::Numeric(hl::NumericOp::F32Ne),
        ll::Instr::F32Lt => hl::Instr::Numeric(hl::NumericOp::F32Lt),
        ll::Instr::F32Gt => hl::Instr::Numeric(hl::NumericOp::F32Gt),
        ll::Instr::F32Le => hl::Instr::Numeric(hl::NumericOp::F32Le),
        ll::Instr::F32Ge => hl::Instr::Numeric(hl::NumericOp::F32Ge),
        ll::Instr::F64Eq => hl::Instr::Numeric(hl::NumericOp::F64Eq),
        ll::Instr::F64Ne => hl::Instr::Numeric(hl::NumericOp::F64Ne),
        ll::Instr::F64Lt => hl::Instr::Numeric(hl::NumericOp::F64Lt),
        ll::Instr::F64Gt => hl::Instr::Numeric(hl::NumericOp::F64Gt),
        ll::Instr::F64Le => hl::Instr::Numeric(hl::NumericOp::F64Le),
        ll::Instr::F64Ge => hl::Instr::Numeric(hl::NumericOp::F64Ge),
        ll::Instr::I32Clz => hl::Instr::Numeric(hl::NumericOp::I32Clz),
        ll::Instr::I32Ctz => hl::Instr::Numeric(hl::NumericOp::I32Ctz),
        ll::Instr::I32Popcnt => hl::Instr::Numeric(hl::NumericOp::I32Popcnt),
        ll::Instr::I32Add => hl::Instr::Numeric(hl::NumericOp::I32Add),
        ll::Instr::I32Sub => hl::Instr::Numeric(hl::NumericOp::I32Sub),
        ll::Instr::I32Mul => hl::Instr::Numeric(hl::NumericOp::I32Mul),
        ll::Instr::I32DivS => hl::Instr::Numeric(hl::NumericOp::I32DivS),
        ll::Instr::I32DivU => hl::Instr::Numeric(hl::NumericOp::I32DivU),
        ll::Instr::I32RemS => hl::Instr::Numeric(hl::NumericOp::I32RemS),
        ll::Instr::I32RemU => hl::Instr::Numeric(hl::NumericOp::I32RemU),
        ll::Instr::I32And => hl::Instr::Numeric(hl::NumericOp::I32And),
        ll::Instr::I32Or => hl::Instr::Numeric(hl::NumericOp::I32Or),
        ll::Instr::I32Xor => hl::Instr::Numeric(hl::NumericOp::I32Xor),
        ll::Instr::I32Shl => hl::Instr::Numeric(hl::NumericOp::I32Shl),
        ll::Instr::I32ShrS => hl::Instr::Numeric(hl::NumericOp::I32ShrS),
        ll::Instr::I32ShrU => hl::Instr::Numeric(hl::NumericOp::I32ShrU),
        ll::Instr::I32Rotl => hl::Instr::Numeric(hl::NumericOp::I32Rotl),
        ll::Instr::I32Rotr => hl::Instr::Numeric(hl::NumericOp::I32Rotr),
        ll::Instr::I64Clz => hl::Instr::Numeric(hl::NumericOp::I64Clz),
        ll::Instr::I64Ctz => hl::Instr::Numeric(hl::NumericOp::I64Ctz),
        ll::Instr::I64Popcnt => hl::Instr::Numeric(hl::NumericOp::I64Popcnt),
        ll::Instr::I64Add => hl::Instr::Numeric(hl::NumericOp::I64Add),
        ll::Instr::I64Sub => hl::Instr::Numeric(hl::NumericOp::I64Sub),
        ll::Instr::I64Mul => hl::Instr::Numeric(hl::NumericOp::I64Mul),
        ll::Instr::I64DivS => hl::Instr::Numeric(hl::NumericOp::I64DivS),
        ll::Instr::I64DivU => hl::Instr::Numeric(hl::NumericOp::I64DivU),
        ll::Instr::I64RemS => hl::Instr::Numeric(hl::NumericOp::I64RemS),
        ll::Instr::I64RemU => hl::Instr::Numeric(hl::NumericOp::I64RemU),
        ll::Instr::I64And => hl::Instr::Numeric(hl::NumericOp::I64And),
        ll::Instr::I64Or => hl::Instr::Numeric(hl::NumericOp::I64Or),
        ll::Instr::I64Xor => hl::Instr::Numeric(hl::NumericOp::I64Xor),
        ll::Instr::I64Shl => hl::Instr::Numeric(hl::NumericOp::I64Shl),
        ll::Instr::I64ShrS => hl::Instr::Numeric(hl::NumericOp::I64ShrS),
        ll::Instr::I64ShrU => hl::Instr::Numeric(hl::NumericOp::I64ShrU),
        ll::Instr::I64Rotl => hl::Instr::Numeric(hl::NumericOp::I64Rotl),
        ll::Instr::I64Rotr => hl::Instr::Numeric(hl::NumericOp::I64Rotr),
        ll::Instr::F32Abs => hl::Instr::Numeric(hl::NumericOp::F32Abs),
        ll::Instr::F32Neg => hl::Instr::Numeric(hl::NumericOp::F32Neg),
        ll::Instr::F32Ceil => hl::Instr::Numeric(hl::NumericOp::F32Ceil),
        ll::Instr::F32Floor => hl::Instr::Numeric(hl::NumericOp::F32Floor),
        ll::Instr::F32Trunc => hl::Instr::Numeric(hl::NumericOp::F32Trunc),
        ll::Instr::F32Nearest => hl::Instr::Numeric(hl::NumericOp::F32Nearest),
        ll::Instr::F32Sqrt => hl::Instr::Numeric(hl::NumericOp::F32Sqrt),
        ll::Instr::F32Add => hl::Instr::Numeric(hl::NumericOp::F32Add),
        ll::Instr::F32Sub => hl::Instr::Numeric(hl::NumericOp::F32Sub),
        ll::Instr::F32Mul => hl::Instr::Numeric(hl::NumericOp::F32Mul),
        ll::Instr::F32Div => hl::Instr::Numeric(hl::NumericOp::F32Div),
        ll::Instr::F32Min => hl::Instr::Numeric(hl::NumericOp::F32Min),
        ll::Instr::F32Max => hl::Instr::Numeric(hl::NumericOp::F32Max),
        ll::Instr::F32Copysign => hl::Instr::Numeric(hl::NumericOp::F32Copysign),
        ll::Instr::F64Abs => hl::Instr::Numeric(hl::NumericOp::F64Abs),
        ll::Instr::F64Neg => hl::Instr::Numeric(hl::NumericOp::F64Neg),
        ll::Instr::F64Ceil => hl::Instr::Numeric(hl::NumericOp::F64Ceil),
        ll::Instr::F64Floor => hl::Instr::Numeric(hl::NumericOp::F64Floor),
        ll::Instr::F64Trunc => hl::Instr::Numeric(hl::NumericOp::F64Trunc),
        ll::Instr::F64Nearest => hl::Instr::Numeric(hl::NumericOp::F64Nearest),
        ll::Instr::F64Sqrt => hl::Instr::Numeric(hl::NumericOp::F64Sqrt),
        ll::Instr::F64Add => hl::Instr::Numeric(hl::NumericOp::F64Add),
        ll::Instr::F64Sub => hl::Instr::Numeric(hl::NumericOp::F64Sub),
        ll::Instr::F64Mul => hl::Instr::Numeric(hl::NumericOp::F64Mul),
        ll::Instr::F64Div => hl::Instr::Numeric(hl::NumericOp::F64Div),
        ll::Instr::F64Min => hl::Instr::Numeric(hl::NumericOp::F64Min),
        ll::Instr::F64Max => hl::Instr::Numeric(hl::NumericOp::F64Max),
        ll::Instr::F64Copysign => hl::Instr::Numeric(hl::NumericOp::F64Copysign),
        ll::Instr::I32WrapI64 => hl::Instr::Numeric(hl::NumericOp::I32WrapI64),
        ll::Instr::I32TruncF32S => hl::Instr::Numeric(hl::NumericOp::I32TruncF32S),
        ll::Instr::I32TruncF32U => hl::Instr::Numeric(hl::NumericOp::I32TruncF32U),
        ll::Instr::I32TruncF64S => hl::Instr::Numeric(hl::NumericOp::I32TruncF64S),
        ll::Instr::I32TruncF64U => hl::Instr::Numeric(hl::NumericOp::I32TruncF64U),
        ll::Instr::I64ExtendI32S => hl::Instr::Numeric(hl::NumericOp::I64ExtendI32S),
        ll::Instr::I64ExtendI32U => hl::Instr::Numeric(hl::NumericOp::I64ExtendI32U),
        ll::Instr::I64TruncF32S => hl::Instr::Numeric(hl::NumericOp::I64TruncF32S),
        ll::Instr::I64TruncF32U => hl::Instr::Numeric(hl::NumericOp::I64TruncF32U),
        ll::Instr::I64TruncF64S => hl::Instr::Numeric(hl::NumericOp::I64TruncF64S),
        ll::Instr::I64TruncF64U => hl::Instr::Numeric(hl::NumericOp::I64TruncF64U),
        ll::Instr::F32ConvertI32S => hl::Instr::Numeric(hl::NumericOp::F32ConvertI32S),
        ll::Instr::F32ConvertI32U => hl::Instr::Numeric(hl::NumericOp::F32ConvertI32U),
        ll::Instr::F32ConvertI64S => hl::Instr::Numeric(hl::NumericOp::F32ConvertI64S),
        ll::Instr::F32ConvertI64U => hl::Instr::Numeric(hl::NumericOp::F32ConvertI64U),
        ll::Instr::F32DemoteF64 => hl::Instr::Numeric(hl::NumericOp::F32DemoteF64),
        ll::Instr::F64ConvertI32S => hl::Instr::Numeric(hl::NumericOp::F64ConvertI32S),
        ll::Instr::F64ConvertI32U => hl::Instr::Numeric(hl::NumericOp::F64ConvertI32U),
        ll::Instr::F64ConvertI64S => hl::Instr::Numeric(hl::NumericOp::F64ConvertI64S),
        ll::Instr::F64ConvertI64U => hl::Instr::Numeric(hl::NumericOp::F64ConvertI64U),
        ll::Instr::F64PromoteF32 => hl::Instr::Numeric(hl::NumericOp::F64PromoteF32),
        ll::Instr::I32ReinterpretF32 => hl::Instr::Numeric(hl::NumericOp::I32ReinterpretF32),
        ll::Instr::I64ReinterpretF64 => hl::Instr::Numeric(hl::NumericOp::I64ReinterpretF64),
        ll::Instr::F32ReinterpretI32 => hl::Instr::Numeric(hl::NumericOp::F32ReinterpretI32),
        ll::Instr::F64ReinterpretI64 => hl::Instr::Numeric(hl::NumericOp::F64ReinterpretI64),
    }
}


/* From high-level to low-level. */

struct EncodeState {
    types: HashMap<FunctionType, usize>,
    // Mapping of indices from the high-level AST to the low-level AST.
    // Necessary, because in the low-level AST, imported functions/globals etc. must come before
    // "local" ones.
    function_idx: HashMap<usize, usize>,
    table_idx: HashMap<usize, usize>,
    memory_idx: HashMap<usize, usize>,
    global_idx: HashMap<usize, usize>,
}

macro_rules! element_idx_fns {
    ($insert_fn: ident, $map_fn: ident, $field: ident, $hl_ty: ty, $ll_ty: ty) => {
        fn $insert_fn(&mut self, old_idx: usize) {
            let new_idx = self.$field.len();
            self.$field.insert(old_idx, new_idx);
        }
        fn $map_fn(&self, old_idx: Idx<$hl_ty>) -> Idx<$ll_ty> {
            self.$field[&old_idx.into_inner()].into()
        }
    };
}

impl EncodeState {
    fn get_or_insert_type(&mut self, type_: FunctionType) -> Idx<FunctionType> {
        let new_idx = self.types.len();
        (*self.types.entry(type_).or_insert(new_idx)).into()
    }
    fn get_type_idx(&self, type_: &FunctionType) -> Idx<FunctionType> {
        (*self.types.get(type_).expect("call_indirect with unknown type")).into()
    }

    element_idx_fns!(insert_function_idx, map_function_idx, function_idx, hl::Function, ll::Function);
    element_idx_fns!(insert_table_idx, map_table_idx, table_idx, hl::Table, ll::Table);
    element_idx_fns!(insert_memory_idx, map_memory_idx, memory_idx, hl::Memory, ll::Memory);
    element_idx_fns!(insert_global_idx, map_global_idx, global_idx, hl::Global, ll::Global);
}

impl From<&hl::Module> for ll::Module {
    fn from(module: &hl::Module) -> Self {
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

        // also collect and insert types in all call_indirect instructions, maybe they are calling
        // with a signature that not function mentions (which would be unpractical, because it could
        // never be valid at runtime, but is done in the spec tests)
        for function in &module.functions {
            for instr in function.instrs() {
                if let hl::Instr::CallIndirect(ty, _) = instr {
                    state.get_or_insert_type(ty.clone());
                }
            }
        }

        /* All types and indices are now determined, so we can start writing out sections... */

        // Type
        let mut types = state.types.iter().collect::<Vec<_>>();
        types.sort_unstable_by_key(|&(_, idx)| idx);
        let types = types.into_iter()
            .map(|(type_, _)| type_.clone())
            .collect::<Vec<FunctionType>>();
        if !types.is_empty() {
            sections.push(ll::Section::Type(ll::WithSize(ll::SectionOffset(types))));
        }

        // Import
        if !imports.is_empty() {
            sections.push(ll::Section::Import(ll::WithSize(ll::SectionOffset(imports))));
        }

        // Function
        if !functions.is_empty() {
            sections.push(ll::Section::Function(ll::WithSize(ll::SectionOffset(functions))));
        }

        // Table
        if !tables.is_empty() {
            sections.push(ll::Section::Table(ll::WithSize(ll::SectionOffset(tables))));
        }

        // Memory
        if !memories.is_empty() {
            sections.push(ll::Section::Memory(ll::WithSize(ll::SectionOffset(memories))));
        }

        // Global
        if !globals.is_empty() {
            sections.push(ll::Section::Global(ll::WithSize(ll::SectionOffset(globals))));
        }

        // Export
        let exports = to_lowlevel_exports(&module, &state);
        if !exports.is_empty() {
            sections.push(ll::Section::Export(ll::WithSize(ll::SectionOffset(exports))));
        }

        // Start
        if let Some(start_func_idx) = module.start {
            sections.push(ll::Section::Start(ll::WithSize(ll::SectionOffset(state.map_function_idx(start_func_idx)))));
        }

        // Element
        let elements: Vec<ll::Element> = module.tables()
            .flat_map(|(idx, table)| table.elements.iter()
                .map(|element| ll::Element {
                    table_idx: state.map_table_idx(idx),
                    offset: to_lowlevel_expr(&element.offset, &state),
                    init: element.functions.iter().map(|&fn_idx| state.map_function_idx(fn_idx)).collect(),
                })
                .collect::<Vec<_>>())
            .collect();
        if !elements.is_empty() {
            sections.push(ll::Section::Element(ll::WithSize(ll::SectionOffset(elements))));
        }

        // Code
        let code: Vec<ll::WithSize<ll::Code>> = module.functions.par_iter()
            // Only non-imported functions.
            .filter_map(|func| func.code())
            .map(|code| to_lowlevel_code(code, &state))
            .collect();
        if !code.is_empty() {
            sections.push(ll::Section::Code(ll::WithSize(ll::SectionOffset(ll::Parallel(code)))));
        }

        // Data
        let data: Vec<ll::Data> = module.memories()
            .flat_map(|(idx, memory)| memory.data.iter()
                .map(|data| ll::Data {
                    memory_idx: state.map_memory_idx(idx),
                    offset: to_lowlevel_expr(&data.offset, &state),
                    init: data.bytes.clone(),
                })
                .collect::<Vec<_>>())
            .collect();
        if !data.is_empty() {
            sections.push(ll::Section::Data(ll::WithSize(ll::SectionOffset(data))));
        }

        // Name section, always after Data section.
        let mut name_subsections = Vec::new();
        if let Some(name) = &module.name {
            name_subsections.push(ll::NameSubSection::Module(ll::WithSize(name.clone())));
        }
        let function_names = to_lowlevel_function_names(&module, &state);
        if !function_names.is_empty() {
            name_subsections.push(ll::NameSubSection::Function(ll::WithSize(function_names)));
        }
        let local_names = to_lowlevel_local_names(&module, &state);
        if !local_names.is_empty() {
            name_subsections.push(ll::NameSubSection::Local(ll::WithSize(local_names)));
        }
        if !name_subsections.is_empty() {
            sections.push(ll::Section::Custom(ll::CustomSection::Name(ll::NameSection { subsections: name_subsections })));
        }

        // Other custom sections, that we do not have inlined into the high-level AST.
        // They are inserted after the correct non-custom section. They relative order of custom
        // sections is the same in this array as when they were parsed originally, so no need
        // to handle that specifically.
        for section in &module.custom_sections {
            let after = section.after;
            let section = ll::Section::Custom(ll::CustomSection::Raw(section.clone()));

            // Find the reference section after which this custom section came in the original binary.
            let after_position = if let Some(after) = after {
                sections.iter()
                    .position(|sec| std::mem::discriminant(sec) == after)
                    .expect("cannot find the reference section for inserting custom section anymore")
                    + 1
            } else {
                // In the original binary the custom section came at the beginning, so insert at beginning.
                0
            };

            // Additionally skip all custom sections, as to not change the relative order between custom sections.
            let custom_section_discriminant = std::mem::discriminant(&section);
            let custom_skipped = sections.iter()
                .skip(after_position)
                .position(|sec| std::mem::discriminant(sec) != custom_section_discriminant)
                // If all remaining sections are custom, skip all of them.
                .unwrap_or(sections.len() - after_position);

            let position = after_position + custom_skipped;
            sections.insert(position, section);
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
                    element.import().map(|(module, name)| {
                        state.$insert_idx_fn(i);
                        ll::Import {
                            module: module.to_string(),
                            name: name.to_string(),
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
            .filter(|&(_, element)| element.import().is_none())
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
        init: to_lowlevel_expr(&global.init().unwrap(), state),
    })
}

fn to_lowlevel_exports(module: &hl::Module, state: &EncodeState) -> Vec<ll::Export> {
    let mut exports = Vec::new();

    macro_rules! add_exports {
        ($elems: ident, $map_idx_fn: ident, $export_ty_variant: ident) => {
            for (idx, element) in module.$elems() {
                for name in &element.export {
                    exports.push(ll::Export {
                        name: name.clone(),
                        type_: ll::ExportType::$export_ty_variant(state.$map_idx_fn(idx)),
                    });
                }
            }
        };
    }
    add_exports!(functions, map_function_idx, Function);
    add_exports!(tables, map_table_idx, Table);
    add_exports!(memories, map_memory_idx, Memory);
    add_exports!(globals, map_global_idx, Global);

    exports
}

fn to_lowlevel_function_names(module: &hl::Module, state: &EncodeState) -> ll::NameMap<ll::Function> {
    let mut names = Vec::new();
    for (idx, func) in module.functions() {
        if let Some(name) = &func.name {
            names.push(ll::NameAssoc {
                idx: state.map_function_idx(idx),
                name: name.clone(),
            });
        }
    }
    names
}

fn to_lowlevel_local_names(module: &hl::Module, state: &EncodeState) -> ll::IndirectNameMap<ll::Function, ll::Local> {
    let mut names = Vec::new();
    for (func_idx, func) in module.functions() {
        let mut name_map = Vec::new();

        for (idx, param_or_local) in func.param_or_locals() {
            if let Some(name) = param_or_local.name() {
                name_map.push(ll::NameAssoc { idx: idx.into_inner().into(), name: name.to_string() });
            }
        }

        if !name_map.is_empty() {
            names.push(ll::IndirectNameAssoc {
                idx: state.map_function_idx(func_idx),
                name_map,
            })
        }
    }
    names
}

fn to_lowlevel_code(code: &hl::Code, state: &EncodeState) -> ll::WithSize<ll::Code> {
    let mut locals: Vec<ll::Locals> = Vec::new();
    for &hl::Local { type_, .. } in code.locals.iter() {
        // Convert the type to the low-level "run-length encoded" format.
        match locals.last_mut() {
            // Increase the count of the last local, if it has the same type as the next one.
            Some(last_local) if last_local.type_ == type_ => last_local.count += 1,
            // If there either was no previous local at all, or none with a matching type, create a
            // new Locals entry.
            Some(_) | None => locals.push(ll::Locals {
                count: 1,
                type_,
            })
        }
    }

    let body = to_lowlevel_expr(&code.body, state);

    ll::WithSize(ll::Code { locals, body })
}

fn to_lowlevel_expr(expr: &[hl::Instr], state: &EncodeState) -> ll::Expr {
    ll::Expr(expr.iter().map(|instr| to_lowlevel_instr(instr, state)).collect())
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
        hl::Instr::BrTable { ref table, default } => ll::Instr::BrTable { table: table.clone(), default },

        hl::Instr::Return => ll::Instr::Return,
        hl::Instr::Call(function_idx) => ll::Instr::Call(state.map_function_idx(function_idx)),
        hl::Instr::CallIndirect(ref type_, table_idx) => ll::Instr::CallIndirect(state.get_type_idx(&type_), state.map_table_idx(table_idx)),

        hl::Instr::Drop => ll::Instr::Drop,
        hl::Instr::Select => ll::Instr::Select,

        hl::Instr::Local(hl::LocalOp::Get, local_idx) => ll::Instr::LocalGet(local_idx.into_inner().into()),
        hl::Instr::Local(hl::LocalOp::Set, local_idx) => ll::Instr::LocalSet(local_idx.into_inner().into()),
        hl::Instr::Local(hl::LocalOp::Tee, local_idx) => ll::Instr::LocalTee(local_idx.into_inner().into()),
        hl::Instr::Global(hl::GlobalOp::Get, global_idx) => ll::Instr::GlobalGet(state.map_global_idx(global_idx)),
        hl::Instr::Global(hl::GlobalOp::Set, global_idx) => ll::Instr::GlobalSet(state.map_global_idx(global_idx)),

        hl::Instr::Load(hl::LoadOp::I32Load, memarg) => ll::Instr::I32Load(memarg),
        hl::Instr::Load(hl::LoadOp::I64Load, memarg) => ll::Instr::I64Load(memarg),
        hl::Instr::Load(hl::LoadOp::F32Load, memarg) => ll::Instr::F32Load(memarg),
        hl::Instr::Load(hl::LoadOp::F64Load, memarg) => ll::Instr::F64Load(memarg),
        hl::Instr::Load(hl::LoadOp::I32Load8S, memarg) => ll::Instr::I32Load8S(memarg),
        hl::Instr::Load(hl::LoadOp::I32Load8U, memarg) => ll::Instr::I32Load8U(memarg),
        hl::Instr::Load(hl::LoadOp::I32Load16S, memarg) => ll::Instr::I32Load16S(memarg),
        hl::Instr::Load(hl::LoadOp::I32Load16U, memarg) => ll::Instr::I32Load16U(memarg),
        hl::Instr::Load(hl::LoadOp::I64Load8S, memarg) => ll::Instr::I64Load8S(memarg),
        hl::Instr::Load(hl::LoadOp::I64Load8U, memarg) => ll::Instr::I64Load8U(memarg),
        hl::Instr::Load(hl::LoadOp::I64Load16S, memarg) => ll::Instr::I64Load16S(memarg),
        hl::Instr::Load(hl::LoadOp::I64Load16U, memarg) => ll::Instr::I64Load16U(memarg),
        hl::Instr::Load(hl::LoadOp::I64Load32S, memarg) => ll::Instr::I64Load32S(memarg),
        hl::Instr::Load(hl::LoadOp::I64Load32U, memarg) => ll::Instr::I64Load32U(memarg),
        hl::Instr::Store(hl::StoreOp::I32Store, memarg) => ll::Instr::I32Store(memarg),
        hl::Instr::Store(hl::StoreOp::I64Store, memarg) => ll::Instr::I64Store(memarg),
        hl::Instr::Store(hl::StoreOp::F32Store, memarg) => ll::Instr::F32Store(memarg),
        hl::Instr::Store(hl::StoreOp::F64Store, memarg) => ll::Instr::F64Store(memarg),
        hl::Instr::Store(hl::StoreOp::I32Store8, memarg) => ll::Instr::I32Store8(memarg),
        hl::Instr::Store(hl::StoreOp::I32Store16, memarg) => ll::Instr::I32Store16(memarg),
        hl::Instr::Store(hl::StoreOp::I64Store8, memarg) => ll::Instr::I64Store8(memarg),
        hl::Instr::Store(hl::StoreOp::I64Store16, memarg) => ll::Instr::I64Store16(memarg),
        hl::Instr::Store(hl::StoreOp::I64Store32, memarg) => ll::Instr::I64Store32(memarg),

        hl::Instr::MemorySize(memory_idx) => ll::Instr::MemorySize(state.map_memory_idx(memory_idx)),
        hl::Instr::MemoryGrow(memory_idx) => ll::Instr::MemoryGrow(state.map_memory_idx(memory_idx)),

        hl::Instr::Const(Val::I32(immediate)) => ll::Instr::I32Const(immediate),
        hl::Instr::Const(Val::I64(immediate)) => ll::Instr::I64Const(immediate),
        hl::Instr::Const(Val::F32(immediate)) => ll::Instr::F32Const(immediate),
        hl::Instr::Const(Val::F64(immediate)) => ll::Instr::F64Const(immediate),

        hl::Instr::Numeric(hl::NumericOp::I32Eqz) => ll::Instr::I32Eqz,
        hl::Instr::Numeric(hl::NumericOp::I32Eq) => ll::Instr::I32Eq,
        hl::Instr::Numeric(hl::NumericOp::I32Ne) => ll::Instr::I32Ne,
        hl::Instr::Numeric(hl::NumericOp::I32LtS) => ll::Instr::I32LtS,
        hl::Instr::Numeric(hl::NumericOp::I32LtU) => ll::Instr::I32LtU,
        hl::Instr::Numeric(hl::NumericOp::I32GtS) => ll::Instr::I32GtS,
        hl::Instr::Numeric(hl::NumericOp::I32GtU) => ll::Instr::I32GtU,
        hl::Instr::Numeric(hl::NumericOp::I32LeS) => ll::Instr::I32LeS,
        hl::Instr::Numeric(hl::NumericOp::I32LeU) => ll::Instr::I32LeU,
        hl::Instr::Numeric(hl::NumericOp::I32GeS) => ll::Instr::I32GeS,
        hl::Instr::Numeric(hl::NumericOp::I32GeU) => ll::Instr::I32GeU,
        hl::Instr::Numeric(hl::NumericOp::I64Eqz) => ll::Instr::I64Eqz,
        hl::Instr::Numeric(hl::NumericOp::I64Eq) => ll::Instr::I64Eq,
        hl::Instr::Numeric(hl::NumericOp::I64Ne) => ll::Instr::I64Ne,
        hl::Instr::Numeric(hl::NumericOp::I64LtS) => ll::Instr::I64LtS,
        hl::Instr::Numeric(hl::NumericOp::I64LtU) => ll::Instr::I64LtU,
        hl::Instr::Numeric(hl::NumericOp::I64GtS) => ll::Instr::I64GtS,
        hl::Instr::Numeric(hl::NumericOp::I64GtU) => ll::Instr::I64GtU,
        hl::Instr::Numeric(hl::NumericOp::I64LeS) => ll::Instr::I64LeS,
        hl::Instr::Numeric(hl::NumericOp::I64LeU) => ll::Instr::I64LeU,
        hl::Instr::Numeric(hl::NumericOp::I64GeS) => ll::Instr::I64GeS,
        hl::Instr::Numeric(hl::NumericOp::I64GeU) => ll::Instr::I64GeU,
        hl::Instr::Numeric(hl::NumericOp::F32Eq) => ll::Instr::F32Eq,
        hl::Instr::Numeric(hl::NumericOp::F32Ne) => ll::Instr::F32Ne,
        hl::Instr::Numeric(hl::NumericOp::F32Lt) => ll::Instr::F32Lt,
        hl::Instr::Numeric(hl::NumericOp::F32Gt) => ll::Instr::F32Gt,
        hl::Instr::Numeric(hl::NumericOp::F32Le) => ll::Instr::F32Le,
        hl::Instr::Numeric(hl::NumericOp::F32Ge) => ll::Instr::F32Ge,
        hl::Instr::Numeric(hl::NumericOp::F64Eq) => ll::Instr::F64Eq,
        hl::Instr::Numeric(hl::NumericOp::F64Ne) => ll::Instr::F64Ne,
        hl::Instr::Numeric(hl::NumericOp::F64Lt) => ll::Instr::F64Lt,
        hl::Instr::Numeric(hl::NumericOp::F64Gt) => ll::Instr::F64Gt,
        hl::Instr::Numeric(hl::NumericOp::F64Le) => ll::Instr::F64Le,
        hl::Instr::Numeric(hl::NumericOp::F64Ge) => ll::Instr::F64Ge,
        hl::Instr::Numeric(hl::NumericOp::I32Clz) => ll::Instr::I32Clz,
        hl::Instr::Numeric(hl::NumericOp::I32Ctz) => ll::Instr::I32Ctz,
        hl::Instr::Numeric(hl::NumericOp::I32Popcnt) => ll::Instr::I32Popcnt,
        hl::Instr::Numeric(hl::NumericOp::I32Add) => ll::Instr::I32Add,
        hl::Instr::Numeric(hl::NumericOp::I32Sub) => ll::Instr::I32Sub,
        hl::Instr::Numeric(hl::NumericOp::I32Mul) => ll::Instr::I32Mul,
        hl::Instr::Numeric(hl::NumericOp::I32DivS) => ll::Instr::I32DivS,
        hl::Instr::Numeric(hl::NumericOp::I32DivU) => ll::Instr::I32DivU,
        hl::Instr::Numeric(hl::NumericOp::I32RemS) => ll::Instr::I32RemS,
        hl::Instr::Numeric(hl::NumericOp::I32RemU) => ll::Instr::I32RemU,
        hl::Instr::Numeric(hl::NumericOp::I32And) => ll::Instr::I32And,
        hl::Instr::Numeric(hl::NumericOp::I32Or) => ll::Instr::I32Or,
        hl::Instr::Numeric(hl::NumericOp::I32Xor) => ll::Instr::I32Xor,
        hl::Instr::Numeric(hl::NumericOp::I32Shl) => ll::Instr::I32Shl,
        hl::Instr::Numeric(hl::NumericOp::I32ShrS) => ll::Instr::I32ShrS,
        hl::Instr::Numeric(hl::NumericOp::I32ShrU) => ll::Instr::I32ShrU,
        hl::Instr::Numeric(hl::NumericOp::I32Rotl) => ll::Instr::I32Rotl,
        hl::Instr::Numeric(hl::NumericOp::I32Rotr) => ll::Instr::I32Rotr,
        hl::Instr::Numeric(hl::NumericOp::I64Clz) => ll::Instr::I64Clz,
        hl::Instr::Numeric(hl::NumericOp::I64Ctz) => ll::Instr::I64Ctz,
        hl::Instr::Numeric(hl::NumericOp::I64Popcnt) => ll::Instr::I64Popcnt,
        hl::Instr::Numeric(hl::NumericOp::I64Add) => ll::Instr::I64Add,
        hl::Instr::Numeric(hl::NumericOp::I64Sub) => ll::Instr::I64Sub,
        hl::Instr::Numeric(hl::NumericOp::I64Mul) => ll::Instr::I64Mul,
        hl::Instr::Numeric(hl::NumericOp::I64DivS) => ll::Instr::I64DivS,
        hl::Instr::Numeric(hl::NumericOp::I64DivU) => ll::Instr::I64DivU,
        hl::Instr::Numeric(hl::NumericOp::I64RemS) => ll::Instr::I64RemS,
        hl::Instr::Numeric(hl::NumericOp::I64RemU) => ll::Instr::I64RemU,
        hl::Instr::Numeric(hl::NumericOp::I64And) => ll::Instr::I64And,
        hl::Instr::Numeric(hl::NumericOp::I64Or) => ll::Instr::I64Or,
        hl::Instr::Numeric(hl::NumericOp::I64Xor) => ll::Instr::I64Xor,
        hl::Instr::Numeric(hl::NumericOp::I64Shl) => ll::Instr::I64Shl,
        hl::Instr::Numeric(hl::NumericOp::I64ShrS) => ll::Instr::I64ShrS,
        hl::Instr::Numeric(hl::NumericOp::I64ShrU) => ll::Instr::I64ShrU,
        hl::Instr::Numeric(hl::NumericOp::I64Rotl) => ll::Instr::I64Rotl,
        hl::Instr::Numeric(hl::NumericOp::I64Rotr) => ll::Instr::I64Rotr,
        hl::Instr::Numeric(hl::NumericOp::F32Abs) => ll::Instr::F32Abs,
        hl::Instr::Numeric(hl::NumericOp::F32Neg) => ll::Instr::F32Neg,
        hl::Instr::Numeric(hl::NumericOp::F32Ceil) => ll::Instr::F32Ceil,
        hl::Instr::Numeric(hl::NumericOp::F32Floor) => ll::Instr::F32Floor,
        hl::Instr::Numeric(hl::NumericOp::F32Trunc) => ll::Instr::F32Trunc,
        hl::Instr::Numeric(hl::NumericOp::F32Nearest) => ll::Instr::F32Nearest,
        hl::Instr::Numeric(hl::NumericOp::F32Sqrt) => ll::Instr::F32Sqrt,
        hl::Instr::Numeric(hl::NumericOp::F32Add) => ll::Instr::F32Add,
        hl::Instr::Numeric(hl::NumericOp::F32Sub) => ll::Instr::F32Sub,
        hl::Instr::Numeric(hl::NumericOp::F32Mul) => ll::Instr::F32Mul,
        hl::Instr::Numeric(hl::NumericOp::F32Div) => ll::Instr::F32Div,
        hl::Instr::Numeric(hl::NumericOp::F32Min) => ll::Instr::F32Min,
        hl::Instr::Numeric(hl::NumericOp::F32Max) => ll::Instr::F32Max,
        hl::Instr::Numeric(hl::NumericOp::F32Copysign) => ll::Instr::F32Copysign,
        hl::Instr::Numeric(hl::NumericOp::F64Abs) => ll::Instr::F64Abs,
        hl::Instr::Numeric(hl::NumericOp::F64Neg) => ll::Instr::F64Neg,
        hl::Instr::Numeric(hl::NumericOp::F64Ceil) => ll::Instr::F64Ceil,
        hl::Instr::Numeric(hl::NumericOp::F64Floor) => ll::Instr::F64Floor,
        hl::Instr::Numeric(hl::NumericOp::F64Trunc) => ll::Instr::F64Trunc,
        hl::Instr::Numeric(hl::NumericOp::F64Nearest) => ll::Instr::F64Nearest,
        hl::Instr::Numeric(hl::NumericOp::F64Sqrt) => ll::Instr::F64Sqrt,
        hl::Instr::Numeric(hl::NumericOp::F64Add) => ll::Instr::F64Add,
        hl::Instr::Numeric(hl::NumericOp::F64Sub) => ll::Instr::F64Sub,
        hl::Instr::Numeric(hl::NumericOp::F64Mul) => ll::Instr::F64Mul,
        hl::Instr::Numeric(hl::NumericOp::F64Div) => ll::Instr::F64Div,
        hl::Instr::Numeric(hl::NumericOp::F64Min) => ll::Instr::F64Min,
        hl::Instr::Numeric(hl::NumericOp::F64Max) => ll::Instr::F64Max,
        hl::Instr::Numeric(hl::NumericOp::F64Copysign) => ll::Instr::F64Copysign,
        hl::Instr::Numeric(hl::NumericOp::I32WrapI64) => ll::Instr::I32WrapI64,
        hl::Instr::Numeric(hl::NumericOp::I32TruncF32S) => ll::Instr::I32TruncF32S,
        hl::Instr::Numeric(hl::NumericOp::I32TruncF32U) => ll::Instr::I32TruncF32U,
        hl::Instr::Numeric(hl::NumericOp::I32TruncF64S) => ll::Instr::I32TruncF64S,
        hl::Instr::Numeric(hl::NumericOp::I32TruncF64U) => ll::Instr::I32TruncF64U,
        hl::Instr::Numeric(hl::NumericOp::I64ExtendI32S) => ll::Instr::I64ExtendI32S,
        hl::Instr::Numeric(hl::NumericOp::I64ExtendI32U) => ll::Instr::I64ExtendI32U,
        hl::Instr::Numeric(hl::NumericOp::I64TruncF32S) => ll::Instr::I64TruncF32S,
        hl::Instr::Numeric(hl::NumericOp::I64TruncF32U) => ll::Instr::I64TruncF32U,
        hl::Instr::Numeric(hl::NumericOp::I64TruncF64S) => ll::Instr::I64TruncF64S,
        hl::Instr::Numeric(hl::NumericOp::I64TruncF64U) => ll::Instr::I64TruncF64U,
        hl::Instr::Numeric(hl::NumericOp::F32ConvertI32S) => ll::Instr::F32ConvertI32S,
        hl::Instr::Numeric(hl::NumericOp::F32ConvertI32U) => ll::Instr::F32ConvertI32U,
        hl::Instr::Numeric(hl::NumericOp::F32ConvertI64S) => ll::Instr::F32ConvertI64S,
        hl::Instr::Numeric(hl::NumericOp::F32ConvertI64U) => ll::Instr::F32ConvertI64U,
        hl::Instr::Numeric(hl::NumericOp::F32DemoteF64) => ll::Instr::F32DemoteF64,
        hl::Instr::Numeric(hl::NumericOp::F64ConvertI32S) => ll::Instr::F64ConvertI32S,
        hl::Instr::Numeric(hl::NumericOp::F64ConvertI32U) => ll::Instr::F64ConvertI32U,
        hl::Instr::Numeric(hl::NumericOp::F64ConvertI64S) => ll::Instr::F64ConvertI64S,
        hl::Instr::Numeric(hl::NumericOp::F64ConvertI64U) => ll::Instr::F64ConvertI64U,
        hl::Instr::Numeric(hl::NumericOp::F64PromoteF32) => ll::Instr::F64PromoteF32,
        hl::Instr::Numeric(hl::NumericOp::I32ReinterpretF32) => ll::Instr::I32ReinterpretF32,
        hl::Instr::Numeric(hl::NumericOp::I64ReinterpretF64) => ll::Instr::I64ReinterpretF64,
        hl::Instr::Numeric(hl::NumericOp::F32ReinterpretI32) => ll::Instr::F32ReinterpretI32,
        hl::Instr::Numeric(hl::NumericOp::F64ReinterpretI64) => ll::Instr::F64ReinterpretI64,
    }
}