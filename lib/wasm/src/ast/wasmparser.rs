use std::convert::TryInto;
use std::{fmt, io, iter};

use ordered_float::OrderedFloat;
use wasmparser::{ImportSectionEntryType, Parser, Payload, TypeDef};

use crate::highlevel::{
    Code, Data, Element, Function, Global, GlobalOp, ImportOrPresent, Instr, LoadOp, Local,
    LocalOp, Memory, Module, StoreOp, Table,
};
use crate::lowlevel::Offsets;
use crate::{
    BlockType, ElemType, FunctionType, GlobalType, Idx, Label, Limits, Memarg, MemoryType,
    Mutability, TableType, Val, ValType,
};

/// 64 KiB, the minimum amount of bytes read in one chunk from the input reader.
const MIN_READ_SIZE: usize = 64 * 1024;

pub fn parse_module_with_offsets<R: io::Read>(
    mut reader: R,
    // TODO once all "benign"/correct cases work, implement proper typed error.
) -> Result<(Module, Offsets), Box<dyn std::error::Error>> {
    // TODO Streaming reading: read only 8-64 KiB chunks of the reader, use `Parser::parser()`.
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let mut module = Module::default();

    // State during module parsing.
    let mut types = Types::none();
    let mut imported_function_count = 0;
    let mut current_code_idx = 0;

    let offset = 0;
    for payload in Parser::new(offset).parse_all(&buf) {
        match payload? {
            Payload::Version { .. } => {
                // The version number is checked by wasmparser to always be 1.
            }
            Payload::TypeSection(mut reader) => {
                let count = reader.get_count();
                types.set_capacity(count)?;
                for _ in 0..count {
                    let ty = reader.read()?;
                    match ty {
                        TypeDef::Func(ty) => types.add(ty)?,
                        TypeDef::Instance(_) | TypeDef::Module(_) => {
                            Err(UnsupportedError(WasmExtension::ModuleLinking))?
                        }
                    }
                }
            }
            Payload::ImportSection(mut reader) => {
                let count = reader.get_count();
                for _ in 0..count {
                    let import = reader.read()?;

                    let import_module = import.module.to_string();
                    let import_name = import
                        .field
                        .ok_or(UnsupportedError(WasmExtension::ModuleLinking))?
                        .to_string();

                    match import.ty {
                        ImportSectionEntryType::Function(ty_i) => {
                            imported_function_count += 1;
                            module.functions.push(Function::new_imported(
                                types.get(ty_i)?,
                                import_module,
                                import_name,
                            ))
                        }
                        ImportSectionEntryType::Global(ty) => module.globals.push(
                            Global::new_imported(convert_global_ty(ty), import_module, import_name),
                        ),
                        ImportSectionEntryType::Table(ty) => module.tables.push(
                            Table::new_imported(convert_table_ty(ty)?, import_module, import_name),
                        ),
                        ImportSectionEntryType::Memory(ty) => {
                            module.memories.push(Memory::new_imported(
                                convert_memory_ty(ty)?,
                                import_module,
                                import_name,
                            ))
                        }
                        ImportSectionEntryType::Tag(_) => {
                            Err(UnsupportedError(WasmExtension::ExceptionHandling))?
                        }
                        ImportSectionEntryType::Module(_) | ImportSectionEntryType::Instance(_) => {
                            Err(UnsupportedError(WasmExtension::ModuleLinking))?
                        }
                    }
                }
            }
            Payload::AliasSection(_) => Err(UnsupportedError(WasmExtension::ModuleLinking))?,
            Payload::InstanceSection(_) => Err(UnsupportedError(WasmExtension::ModuleLinking))?,
            Payload::FunctionSection(mut reader) => {
                let count = reader.get_count();
                module.functions.reserve(u32_to_usize(count));
                for _ in 0..count {
                    let ty_i = reader.read()?;
                    let type_ = types.get(ty_i)?;
                    // Fill in the code of the function later with the code section.
                    module.functions.push(Function::new(type_, Code::new()));
                }
            }
            Payload::TableSection(mut reader) => {
                let count = reader.get_count();
                module.tables.reserve(u32_to_usize(count));
                for _ in 0..count {
                    let type_ = reader.read()?;
                    let type_ = convert_table_ty(type_)?;
                    // Fill in the elements of the table later with the elem section.
                    module.tables.push(Table::new(type_));
                }
            }
            Payload::MemorySection(mut reader) => {
                let count = reader.get_count();
                module.memories.reserve(u32_to_usize(count));
                for _ in 0..count {
                    let type_ = reader.read()?;
                    let type_ = convert_memory_ty(type_)?;
                    // Fill in the data of the memory later with the data section.
                    module.memories.push(Memory::new(type_));
                }
            }
            Payload::TagSection(_) => Err(UnsupportedError(WasmExtension::ExceptionHandling))?,
            Payload::GlobalSection(mut reader) => {
                let count = reader.get_count();
                module.globals.reserve(u32_to_usize(count));
                for _ in 0..count {
                    let global = reader.read()?;
                    let type_ = convert_global_ty(global.ty);

                    // Most initialization expressions have just a constant and the end instruction.
                    let mut init = Vec::with_capacity(2);
                    for op in global.init_expr.get_operators_reader() {
                        init.push(convert_instr(op?, &types)?)
                    }

                    module.globals.push(Global::new(type_, init))
                }
            }
            Payload::ExportSection(mut reader) => {
                let count = reader.get_count();
                for _ in 0..count {
                    let export = reader.read()?;
                    let name = export.field.to_string();
                    let idx = u32_to_usize(export.index);
                    use wasmparser::ExternalKind;
                    match export.kind {
                        ExternalKind::Function => module
                            .functions
                            .get_mut(idx)
                            .ok_or(IndexError::<Function>(idx.into()))?
                            .export
                            .push(name),
                        ExternalKind::Table => module
                            .tables
                            .get_mut(idx)
                            .ok_or(IndexError::<Table>(idx.into()))?
                            .export
                            .push(name),
                        ExternalKind::Memory => module
                            .memories
                            .get_mut(idx)
                            .ok_or(IndexError::<Memory>(idx.into()))?
                            .export
                            .push(name),
                        ExternalKind::Global => module
                            .globals
                            .get_mut(idx)
                            .ok_or(IndexError::<Global>(idx.into()))?
                            .export
                            .push(name),
                        ExternalKind::Tag => {
                            Err(UnsupportedError(WasmExtension::ExceptionHandling))?
                        }
                        ExternalKind::Type => Err(UnsupportedError(WasmExtension::TypeImports))?,
                        ExternalKind::Module | ExternalKind::Instance => {
                            Err(UnsupportedError(WasmExtension::ModuleLinking))?
                        }
                    }
                }
            }
            Payload::StartSection { func, range: _ } => module.start = Some(func.into()),
            Payload::ElementSection(mut reader) => {
                let count = reader.get_count();
                for _ in 0..count {
                    let element = reader.read()?;
                    let elem_type = convert_elem_ty(element.ty)?;

                    let items_reader = element.items.get_items_reader()?;
                    let mut items = Vec::with_capacity(u32_to_usize(items_reader.get_count()));
                    for item in items_reader {
                        let item = item?;
                        use wasmparser::ElementItem;
                        items.push(match item {
                            ElementItem::Func(idx) => idx.into(),
                            ElementItem::Expr(_) => todo!(),
                        });
                    }

                    use wasmparser::ElementKind;
                    match element.kind {
                        ElementKind::Active {
                            table_index,
                            init_expr,
                        } => {
                            let table = module
                                .tables
                                .get_mut(u32_to_usize(table_index))
                                .ok_or(IndexError::<Table>(table_index.into()))?;

                            // TODO I am not sure this is correct.
                            if table.type_.0 != elem_type {
                                Err("type error: table and element not fitting together")?
                            }

                            // Most offset expressions are just a constant and the end instruction.
                            let mut offset = Vec::with_capacity(2);
                            for op in init_expr.get_operators_reader() {
                                offset.push(convert_instr(op?, &types)?)
                            }

                            table.elements.push(Element {
                                offset,
                                functions: items,
                            })
                        }
                        ElementKind::Passive => {
                            Err(UnsupportedError(WasmExtension::BulkMemoryOperations))?
                        }
                        ElementKind::Declared => {
                            Err(UnsupportedError(WasmExtension::ReferenceTypes))?
                        }
                    }
                }
            }
            Payload::DataCountSection { count: _, range: _ } => {
                Err(UnsupportedError(WasmExtension::BulkMemoryOperations))?
            }
            Payload::DataSection(mut reader) => {
                let count = reader.get_count();
                for _ in 0..count {
                    let data = reader.read()?;

                    use wasmparser::DataKind;
                    match data.kind {
                        DataKind::Active {
                            memory_index,
                            init_expr,
                        } => {
                            let memory = module
                                .memories
                                .get_mut(u32_to_usize(memory_index))
                                .ok_or(IndexError::<Memory>(memory_index.into()))?;

                            // Most offset expressions are just a constant and the end instruction.
                            let mut offset = Vec::with_capacity(2);
                            for op in init_expr.get_operators_reader() {
                                offset.push(convert_instr(op?, &types)?)
                            }

                            memory.data.push(Data {
                                offset,
                                bytes: data.data.to_vec(),
                            })
                        }
                        DataKind::Passive => {
                            Err(UnsupportedError(WasmExtension::BulkMemoryOperations))?
                        }
                    }
                }
            }
            Payload::CustomSection {
                name,
                data_offset,
                data,
                range,
            } => {
                todo!()
                // TODO name section
                // TODO other sections -> ignore/save as data
            }
            Payload::CodeSectionStart {
                count: _,
                range: _,
                size: _,
            } => {
                // Because the individual code section entries (i.e., function bodies)
                // are parsed in the following, we don't need to do anything with the
                // code section start itself.
            }
            Payload::CodeSectionEntry(body) => {
                // TODO parallelize

                let func_idx = imported_function_count + current_code_idx;
                let function = module
                    .functions
                    .get_mut(func_idx)
                    .ok_or(IndexError::<Function>(func_idx.into()))?;

                function.code = ImportOrPresent::Present(parse_body(body, &types)?);

                current_code_idx += 1;
            }
            Payload::ModuleSectionStart {
                count: _,
                range: _,
                size: _,
            } => Err(UnsupportedError(WasmExtension::ModuleLinking))?,
            Payload::ModuleSectionEntry {
                parser: _,
                range: _,
            } => Err(UnsupportedError(WasmExtension::ModuleLinking))?,
            Payload::UnknownSection {
                id: _,
                contents: _,
                range: _,
            } => Err("unknown section")?,
            Payload::End => {
                // I don't understand what this end marker is for?
                // If the module ended (i.e., the input buffer is exhausted),
                // there isn't any payload following, so this won't reach anyway.
            }
        }
    }

    let offsets = Offsets {
        sections: Vec::new(),
        functions_code: Vec::new(),
    };

    Ok((module, offsets))
}

fn parse_body(
    body: wasmparser::FunctionBody,
    types: &Types,
) -> Result<Code, Box<dyn std::error::Error>> {
    let mut locals = Vec::new();
    for local in body.get_locals_reader()? {
        let (count, type_) = local?;
        for _ in 0..count {
            locals.push(Local::new(convert_ty(type_)));
        }
    }

    // There is roughly one instruction per byte, so reserve space for
    // approximately this many instructions.
    let wasmparser::Range {
        start: body_start,
        end: body_end,
    } = body.range();
    let body_byte_size = body_end - body_start;
    let mut instrs = Vec::with_capacity(body_byte_size);

    for op in body.get_operators_reader()? {
        instrs.push(convert_instr(op?, &types)?);
    }

    Ok(Code {
        locals,
        body: instrs,
    })
}

#[allow(unused)]
fn convert_instr(
    op: wasmparser::Operator,
    types: &Types,
) -> Result<Instr, Box<dyn std::error::Error>> {
    use crate::highlevel::Instr::*;
    use wasmparser::Operator as wp;
    Ok(match op {
        wp::Unreachable => Unreachable,
        wp::Nop => Nop,

        wp::Block { ty } => Block(convert_block_ty(ty)?),
        wp::Loop { ty } => Loop(convert_block_ty(ty)?),
        wp::If { ty } => If(convert_block_ty(ty)?),
        wp::Else => Else,
        wp::End => End,

        wp::Try { ty: _ }
        | wp::Catch { index: _ }
        | wp::CatchAll
        | wp::Throw { index: _ }
        | wp::Rethrow { relative_depth: _ }
        | wp::Delegate { relative_depth: _ } => {
            Err(UnsupportedError(WasmExtension::ExceptionHandling))?
        }

        wp::Br { relative_depth } => Br(Label(relative_depth)),
        wp::BrIf { relative_depth } => BrIf(Label(relative_depth)),
        wp::BrTable { table } => {
            let default = Label(table.default());
            let mut targets = Vec::with_capacity(u32_to_usize(table.len()));
            for target in table.targets() {
                targets.push(Label(target?))
            }
            BrTable {
                table: targets,
                default,
            }
        }

        wp::Return => Return,
        wp::Call { function_index } => Call(function_index.into()),
        wp::CallIndirect { index, table_index } => {
            CallIndirect(types.get(index)?, table_index.into())
        }

        wp::ReturnCall { function_index: _ }
        | wp::ReturnCallIndirect {
            index: _,
            table_index: _,
        } => Err(UnsupportedError(WasmExtension::TailCall))?,

        wp::Drop => Drop,
        wp::Select => Select,

        wp::TypedSelect { ty } => Err(UnsupportedError(WasmExtension::ReferenceTypes))?,

        wp::LocalGet { local_index } => Local(LocalOp::Get, local_index.into()),
        wp::LocalSet { local_index } => Local(LocalOp::Set, local_index.into()),
        wp::LocalTee { local_index } => Local(LocalOp::Tee, local_index.into()),
        wp::GlobalGet { global_index } => Global(GlobalOp::Get, global_index.into()),
        wp::GlobalSet { global_index } => Global(GlobalOp::Set, global_index.into()),

        wp::I32Load { memarg } => Load(LoadOp::I32Load, convert_memarg(memarg)?),
        wp::I64Load { memarg } => Load(LoadOp::I64Load, convert_memarg(memarg)?),
        wp::F32Load { memarg } => Load(LoadOp::F32Load, convert_memarg(memarg)?),
        wp::F64Load { memarg } => Load(LoadOp::F64Load, convert_memarg(memarg)?),
        wp::I32Load8S { memarg } => Load(LoadOp::I32Load8S, convert_memarg(memarg)?),
        wp::I32Load8U { memarg } => Load(LoadOp::I32Load8U, convert_memarg(memarg)?),
        wp::I32Load16S { memarg } => Load(LoadOp::I32Load16S, convert_memarg(memarg)?),
        wp::I32Load16U { memarg } => Load(LoadOp::I32Load16U, convert_memarg(memarg)?),
        wp::I64Load8S { memarg } => Load(LoadOp::I64Load8S, convert_memarg(memarg)?),
        wp::I64Load8U { memarg } => Load(LoadOp::I64Load8U, convert_memarg(memarg)?),
        wp::I64Load16S { memarg } => Load(LoadOp::I64Load16S, convert_memarg(memarg)?),
        wp::I64Load16U { memarg } => Load(LoadOp::I64Load16U, convert_memarg(memarg)?),
        wp::I64Load32S { memarg } => Load(LoadOp::I64Load32S, convert_memarg(memarg)?),
        wp::I64Load32U { memarg } => Load(LoadOp::I64Load32U, convert_memarg(memarg)?),

        wp::I32Store { memarg } => Store(StoreOp::I32Store, convert_memarg(memarg)?),
        wp::I64Store { memarg } => Store(StoreOp::I64Store, convert_memarg(memarg)?),
        wp::F32Store { memarg } => Store(StoreOp::F32Store, convert_memarg(memarg)?),
        wp::F64Store { memarg } => Store(StoreOp::F64Store, convert_memarg(memarg)?),
        wp::I32Store8 { memarg } => Store(StoreOp::I32Store8, convert_memarg(memarg)?),
        wp::I32Store16 { memarg } => Store(StoreOp::I32Store16, convert_memarg(memarg)?),
        wp::I64Store8 { memarg } => Store(StoreOp::I64Store8, convert_memarg(memarg)?),
        wp::I64Store16 { memarg } => Store(StoreOp::I64Store16, convert_memarg(memarg)?),
        wp::I64Store32 { memarg } => Store(StoreOp::I64Store32, convert_memarg(memarg)?),

        // This is not well documented in wasmparser: `mem_byte` and `mem` essentially contain
        // the same information, it's just that mem_byte is the original (single) byte that was
        // read from the instruction stream, and mem is it if parsed as a LEB128.
        // I think the variable-length parser is more robust, as it can handle memory indices
        // above 255, so ignore `mem_byte` here.
        wp::MemorySize { mem, mem_byte: _ } => {
            if mem != 0 {
                Err(UnsupportedError(WasmExtension::MultiMemory))?
            }
            MemorySize(0u32.into())
        }
        wp::MemoryGrow { mem, mem_byte: _ } => {
            if mem != 0 {
                Err(UnsupportedError(WasmExtension::MultiMemory))?
            }
            MemoryGrow(0u32.into())
        }

        wp::I32Const { value } => Const(Val::I32(value)),
        wp::I64Const { value } => Const(Val::I64(value)),
        wp::F32Const { value } => Const(Val::F32(OrderedFloat(f32::from_bits(value.bits())))),
        wp::F64Const { value } => Const(Val::F64(OrderedFloat(f64::from_bits(value.bits())))),

        wp::RefNull { ty: _ } | wp::RefIsNull | wp::RefFunc { function_index: _ } => {
            Err(UnsupportedError(WasmExtension::ReferenceTypes))?
        }

        wp::I32Eqz => todo!(),
        wp::I32Eq => todo!(),
        wp::I32Ne => todo!(),
        wp::I32LtS => todo!(),
        wp::I32LtU => todo!(),
        wp::I32GtS => todo!(),
        wp::I32GtU => todo!(),
        wp::I32LeS => todo!(),
        wp::I32LeU => todo!(),
        wp::I32GeS => todo!(),
        wp::I32GeU => todo!(),
        wp::I64Eqz => todo!(),
        wp::I64Eq => todo!(),
        wp::I64Ne => todo!(),
        wp::I64LtS => todo!(),
        wp::I64LtU => todo!(),
        wp::I64GtS => todo!(),
        wp::I64GtU => todo!(),
        wp::I64LeS => todo!(),
        wp::I64LeU => todo!(),
        wp::I64GeS => todo!(),
        wp::I64GeU => todo!(),
        wp::F32Eq => todo!(),
        wp::F32Ne => todo!(),
        wp::F32Lt => todo!(),
        wp::F32Gt => todo!(),
        wp::F32Le => todo!(),
        wp::F32Ge => todo!(),
        wp::F64Eq => todo!(),
        wp::F64Ne => todo!(),
        wp::F64Lt => todo!(),
        wp::F64Gt => todo!(),
        wp::F64Le => todo!(),
        wp::F64Ge => todo!(),
        wp::I32Clz => todo!(),
        wp::I32Ctz => todo!(),
        wp::I32Popcnt => todo!(),
        wp::I32Add => todo!(),
        wp::I32Sub => todo!(),
        wp::I32Mul => todo!(),
        wp::I32DivS => todo!(),
        wp::I32DivU => todo!(),
        wp::I32RemS => todo!(),
        wp::I32RemU => todo!(),
        wp::I32And => todo!(),
        wp::I32Or => todo!(),
        wp::I32Xor => todo!(),
        wp::I32Shl => todo!(),
        wp::I32ShrS => todo!(),
        wp::I32ShrU => todo!(),
        wp::I32Rotl => todo!(),
        wp::I32Rotr => todo!(),
        wp::I64Clz => todo!(),
        wp::I64Ctz => todo!(),
        wp::I64Popcnt => todo!(),
        wp::I64Add => todo!(),
        wp::I64Sub => todo!(),
        wp::I64Mul => todo!(),
        wp::I64DivS => todo!(),
        wp::I64DivU => todo!(),
        wp::I64RemS => todo!(),
        wp::I64RemU => todo!(),
        wp::I64And => todo!(),
        wp::I64Or => todo!(),
        wp::I64Xor => todo!(),
        wp::I64Shl => todo!(),
        wp::I64ShrS => todo!(),
        wp::I64ShrU => todo!(),
        wp::I64Rotl => todo!(),
        wp::I64Rotr => todo!(),
        wp::F32Abs => todo!(),
        wp::F32Neg => todo!(),
        wp::F32Ceil => todo!(),
        wp::F32Floor => todo!(),
        wp::F32Trunc => todo!(),
        wp::F32Nearest => todo!(),
        wp::F32Sqrt => todo!(),
        wp::F32Add => todo!(),
        wp::F32Sub => todo!(),
        wp::F32Mul => todo!(),
        wp::F32Div => todo!(),
        wp::F32Min => todo!(),
        wp::F32Max => todo!(),
        wp::F32Copysign => todo!(),
        wp::F64Abs => todo!(),
        wp::F64Neg => todo!(),
        wp::F64Ceil => todo!(),
        wp::F64Floor => todo!(),
        wp::F64Trunc => todo!(),
        wp::F64Nearest => todo!(),
        wp::F64Sqrt => todo!(),
        wp::F64Add => todo!(),
        wp::F64Sub => todo!(),
        wp::F64Mul => todo!(),
        wp::F64Div => todo!(),
        wp::F64Min => todo!(),
        wp::F64Max => todo!(),
        wp::F64Copysign => todo!(),
        wp::I32WrapI64 => todo!(),
        wp::I32TruncF32S => todo!(),
        wp::I32TruncF32U => todo!(),
        wp::I32TruncF64S => todo!(),
        wp::I32TruncF64U => todo!(),
        wp::I64ExtendI32S => todo!(),
        wp::I64ExtendI32U => todo!(),
        wp::I64TruncF32S => todo!(),
        wp::I64TruncF32U => todo!(),
        wp::I64TruncF64S => todo!(),
        wp::I64TruncF64U => todo!(),
        wp::F32ConvertI32S => todo!(),
        wp::F32ConvertI32U => todo!(),
        wp::F32ConvertI64S => todo!(),
        wp::F32ConvertI64U => todo!(),
        wp::F32DemoteF64 => todo!(),
        wp::F64ConvertI32S => todo!(),
        wp::F64ConvertI32U => todo!(),
        wp::F64ConvertI64S => todo!(),
        wp::F64ConvertI64U => todo!(),
        wp::F64PromoteF32 => todo!(),
        wp::I32ReinterpretF32 => todo!(),
        wp::I64ReinterpretF64 => todo!(),
        wp::F32ReinterpretI32 => todo!(),
        wp::F64ReinterpretI64 => todo!(),
        wp::I32Extend8S => todo!(),
        wp::I32Extend16S => todo!(),
        wp::I64Extend8S => todo!(),
        wp::I64Extend16S => todo!(),
        wp::I64Extend32S => todo!(),
        wp::I32TruncSatF32S => todo!(),
        wp::I32TruncSatF32U => todo!(),
        wp::I32TruncSatF64S => todo!(),
        wp::I32TruncSatF64U => todo!(),
        wp::I64TruncSatF32S => todo!(),
        wp::I64TruncSatF32U => todo!(),
        wp::I64TruncSatF64S => todo!(),
        wp::I64TruncSatF64U => todo!(),
        wp::MemoryInit { segment, mem } => todo!(),
        wp::DataDrop { segment } => todo!(),
        wp::MemoryCopy { src, dst } => todo!(),
        wp::MemoryFill { mem } => todo!(),
        wp::TableInit { segment, table } => todo!(),
        wp::ElemDrop { segment } => todo!(),
        wp::TableCopy {
            dst_table,
            src_table,
        } => todo!(),
        wp::TableFill { table } => todo!(),
        wp::TableGet { table } => todo!(),
        wp::TableSet { table } => todo!(),
        wp::TableGrow { table } => todo!(),
        wp::TableSize { table } => todo!(),
        wp::MemoryAtomicNotify { memarg } => todo!(),
        wp::MemoryAtomicWait32 { memarg } => todo!(),
        wp::MemoryAtomicWait64 { memarg } => todo!(),
        wp::AtomicFence { flags } => todo!(),
        wp::I32AtomicLoad { memarg } => todo!(),
        wp::I64AtomicLoad { memarg } => todo!(),
        wp::I32AtomicLoad8U { memarg } => todo!(),
        wp::I32AtomicLoad16U { memarg } => todo!(),
        wp::I64AtomicLoad8U { memarg } => todo!(),
        wp::I64AtomicLoad16U { memarg } => todo!(),
        wp::I64AtomicLoad32U { memarg } => todo!(),
        wp::I32AtomicStore { memarg } => todo!(),
        wp::I64AtomicStore { memarg } => todo!(),
        wp::I32AtomicStore8 { memarg } => todo!(),
        wp::I32AtomicStore16 { memarg } => todo!(),
        wp::I64AtomicStore8 { memarg } => todo!(),
        wp::I64AtomicStore16 { memarg } => todo!(),
        wp::I64AtomicStore32 { memarg } => todo!(),
        wp::I32AtomicRmwAdd { memarg } => todo!(),
        wp::I64AtomicRmwAdd { memarg } => todo!(),
        wp::I32AtomicRmw8AddU { memarg } => todo!(),
        wp::I32AtomicRmw16AddU { memarg } => todo!(),
        wp::I64AtomicRmw8AddU { memarg } => todo!(),
        wp::I64AtomicRmw16AddU { memarg } => todo!(),
        wp::I64AtomicRmw32AddU { memarg } => todo!(),
        wp::I32AtomicRmwSub { memarg } => todo!(),
        wp::I64AtomicRmwSub { memarg } => todo!(),
        wp::I32AtomicRmw8SubU { memarg } => todo!(),
        wp::I32AtomicRmw16SubU { memarg } => todo!(),
        wp::I64AtomicRmw8SubU { memarg } => todo!(),
        wp::I64AtomicRmw16SubU { memarg } => todo!(),
        wp::I64AtomicRmw32SubU { memarg } => todo!(),
        wp::I32AtomicRmwAnd { memarg } => todo!(),
        wp::I64AtomicRmwAnd { memarg } => todo!(),
        wp::I32AtomicRmw8AndU { memarg } => todo!(),
        wp::I32AtomicRmw16AndU { memarg } => todo!(),
        wp::I64AtomicRmw8AndU { memarg } => todo!(),
        wp::I64AtomicRmw16AndU { memarg } => todo!(),
        wp::I64AtomicRmw32AndU { memarg } => todo!(),
        wp::I32AtomicRmwOr { memarg } => todo!(),
        wp::I64AtomicRmwOr { memarg } => todo!(),
        wp::I32AtomicRmw8OrU { memarg } => todo!(),
        wp::I32AtomicRmw16OrU { memarg } => todo!(),
        wp::I64AtomicRmw8OrU { memarg } => todo!(),
        wp::I64AtomicRmw16OrU { memarg } => todo!(),
        wp::I64AtomicRmw32OrU { memarg } => todo!(),
        wp::I32AtomicRmwXor { memarg } => todo!(),
        wp::I64AtomicRmwXor { memarg } => todo!(),
        wp::I32AtomicRmw8XorU { memarg } => todo!(),
        wp::I32AtomicRmw16XorU { memarg } => todo!(),
        wp::I64AtomicRmw8XorU { memarg } => todo!(),
        wp::I64AtomicRmw16XorU { memarg } => todo!(),
        wp::I64AtomicRmw32XorU { memarg } => todo!(),
        wp::I32AtomicRmwXchg { memarg } => todo!(),
        wp::I64AtomicRmwXchg { memarg } => todo!(),
        wp::I32AtomicRmw8XchgU { memarg } => todo!(),
        wp::I32AtomicRmw16XchgU { memarg } => todo!(),
        wp::I64AtomicRmw8XchgU { memarg } => todo!(),
        wp::I64AtomicRmw16XchgU { memarg } => todo!(),
        wp::I64AtomicRmw32XchgU { memarg } => todo!(),
        wp::I32AtomicRmwCmpxchg { memarg } => todo!(),
        wp::I64AtomicRmwCmpxchg { memarg } => todo!(),
        wp::I32AtomicRmw8CmpxchgU { memarg } => todo!(),
        wp::I32AtomicRmw16CmpxchgU { memarg } => todo!(),
        wp::I64AtomicRmw8CmpxchgU { memarg } => todo!(),
        wp::I64AtomicRmw16CmpxchgU { memarg } => todo!(),
        wp::I64AtomicRmw32CmpxchgU { memarg } => todo!(),
        wp::V128Load { memarg } => todo!(),
        wp::V128Load8x8S { memarg } => todo!(),
        wp::V128Load8x8U { memarg } => todo!(),
        wp::V128Load16x4S { memarg } => todo!(),
        wp::V128Load16x4U { memarg } => todo!(),
        wp::V128Load32x2S { memarg } => todo!(),
        wp::V128Load32x2U { memarg } => todo!(),
        wp::V128Load8Splat { memarg } => todo!(),
        wp::V128Load16Splat { memarg } => todo!(),
        wp::V128Load32Splat { memarg } => todo!(),
        wp::V128Load64Splat { memarg } => todo!(),
        wp::V128Load32Zero { memarg } => todo!(),
        wp::V128Load64Zero { memarg } => todo!(),
        wp::V128Store { memarg } => todo!(),
        wp::V128Load8Lane { memarg, lane } => todo!(),
        wp::V128Load16Lane { memarg, lane } => todo!(),
        wp::V128Load32Lane { memarg, lane } => todo!(),
        wp::V128Load64Lane { memarg, lane } => todo!(),
        wp::V128Store8Lane { memarg, lane } => todo!(),
        wp::V128Store16Lane { memarg, lane } => todo!(),
        wp::V128Store32Lane { memarg, lane } => todo!(),
        wp::V128Store64Lane { memarg, lane } => todo!(),
        wp::V128Const { value } => todo!(),
        wp::I8x16Shuffle { lanes } => todo!(),
        wp::I8x16ExtractLaneS { lane } => todo!(),
        wp::I8x16ExtractLaneU { lane } => todo!(),
        wp::I8x16ReplaceLane { lane } => todo!(),
        wp::I16x8ExtractLaneS { lane } => todo!(),
        wp::I16x8ExtractLaneU { lane } => todo!(),
        wp::I16x8ReplaceLane { lane } => todo!(),
        wp::I32x4ExtractLane { lane } => todo!(),
        wp::I32x4ReplaceLane { lane } => todo!(),
        wp::I64x2ExtractLane { lane } => todo!(),
        wp::I64x2ReplaceLane { lane } => todo!(),
        wp::F32x4ExtractLane { lane } => todo!(),
        wp::F32x4ReplaceLane { lane } => todo!(),
        wp::F64x2ExtractLane { lane } => todo!(),
        wp::F64x2ReplaceLane { lane } => todo!(),
        wp::I8x16Swizzle => todo!(),
        wp::I8x16Splat => todo!(),
        wp::I16x8Splat => todo!(),
        wp::I32x4Splat => todo!(),
        wp::I64x2Splat => todo!(),
        wp::F32x4Splat => todo!(),
        wp::F64x2Splat => todo!(),
        wp::I8x16Eq => todo!(),
        wp::I8x16Ne => todo!(),
        wp::I8x16LtS => todo!(),
        wp::I8x16LtU => todo!(),
        wp::I8x16GtS => todo!(),
        wp::I8x16GtU => todo!(),
        wp::I8x16LeS => todo!(),
        wp::I8x16LeU => todo!(),
        wp::I8x16GeS => todo!(),
        wp::I8x16GeU => todo!(),
        wp::I16x8Eq => todo!(),
        wp::I16x8Ne => todo!(),
        wp::I16x8LtS => todo!(),
        wp::I16x8LtU => todo!(),
        wp::I16x8GtS => todo!(),
        wp::I16x8GtU => todo!(),
        wp::I16x8LeS => todo!(),
        wp::I16x8LeU => todo!(),
        wp::I16x8GeS => todo!(),
        wp::I16x8GeU => todo!(),
        wp::I32x4Eq => todo!(),
        wp::I32x4Ne => todo!(),
        wp::I32x4LtS => todo!(),
        wp::I32x4LtU => todo!(),
        wp::I32x4GtS => todo!(),
        wp::I32x4GtU => todo!(),
        wp::I32x4LeS => todo!(),
        wp::I32x4LeU => todo!(),
        wp::I32x4GeS => todo!(),
        wp::I32x4GeU => todo!(),
        wp::I64x2Eq => todo!(),
        wp::I64x2Ne => todo!(),
        wp::I64x2LtS => todo!(),
        wp::I64x2GtS => todo!(),
        wp::I64x2LeS => todo!(),
        wp::I64x2GeS => todo!(),
        wp::F32x4Eq => todo!(),
        wp::F32x4Ne => todo!(),
        wp::F32x4Lt => todo!(),
        wp::F32x4Gt => todo!(),
        wp::F32x4Le => todo!(),
        wp::F32x4Ge => todo!(),
        wp::F64x2Eq => todo!(),
        wp::F64x2Ne => todo!(),
        wp::F64x2Lt => todo!(),
        wp::F64x2Gt => todo!(),
        wp::F64x2Le => todo!(),
        wp::F64x2Ge => todo!(),
        wp::V128Not => todo!(),
        wp::V128And => todo!(),
        wp::V128AndNot => todo!(),
        wp::V128Or => todo!(),
        wp::V128Xor => todo!(),
        wp::V128Bitselect => todo!(),
        wp::V128AnyTrue => todo!(),
        wp::I8x16Abs => todo!(),
        wp::I8x16Neg => todo!(),
        wp::I8x16Popcnt => todo!(),
        wp::I8x16AllTrue => todo!(),
        wp::I8x16Bitmask => todo!(),
        wp::I8x16NarrowI16x8S => todo!(),
        wp::I8x16NarrowI16x8U => todo!(),
        wp::I8x16Shl => todo!(),
        wp::I8x16ShrS => todo!(),
        wp::I8x16ShrU => todo!(),
        wp::I8x16Add => todo!(),
        wp::I8x16AddSatS => todo!(),
        wp::I8x16AddSatU => todo!(),
        wp::I8x16Sub => todo!(),
        wp::I8x16SubSatS => todo!(),
        wp::I8x16SubSatU => todo!(),
        wp::I8x16MinS => todo!(),
        wp::I8x16MinU => todo!(),
        wp::I8x16MaxS => todo!(),
        wp::I8x16MaxU => todo!(),
        wp::I8x16RoundingAverageU => todo!(),
        wp::I16x8ExtAddPairwiseI8x16S => todo!(),
        wp::I16x8ExtAddPairwiseI8x16U => todo!(),
        wp::I16x8Abs => todo!(),
        wp::I16x8Neg => todo!(),
        wp::I16x8Q15MulrSatS => todo!(),
        wp::I16x8AllTrue => todo!(),
        wp::I16x8Bitmask => todo!(),
        wp::I16x8NarrowI32x4S => todo!(),
        wp::I16x8NarrowI32x4U => todo!(),
        wp::I16x8ExtendLowI8x16S => todo!(),
        wp::I16x8ExtendHighI8x16S => todo!(),
        wp::I16x8ExtendLowI8x16U => todo!(),
        wp::I16x8ExtendHighI8x16U => todo!(),
        wp::I16x8Shl => todo!(),
        wp::I16x8ShrS => todo!(),
        wp::I16x8ShrU => todo!(),
        wp::I16x8Add => todo!(),
        wp::I16x8AddSatS => todo!(),
        wp::I16x8AddSatU => todo!(),
        wp::I16x8Sub => todo!(),
        wp::I16x8SubSatS => todo!(),
        wp::I16x8SubSatU => todo!(),
        wp::I16x8Mul => todo!(),
        wp::I16x8MinS => todo!(),
        wp::I16x8MinU => todo!(),
        wp::I16x8MaxS => todo!(),
        wp::I16x8MaxU => todo!(),
        wp::I16x8RoundingAverageU => todo!(),
        wp::I16x8ExtMulLowI8x16S => todo!(),
        wp::I16x8ExtMulHighI8x16S => todo!(),
        wp::I16x8ExtMulLowI8x16U => todo!(),
        wp::I16x8ExtMulHighI8x16U => todo!(),
        wp::I32x4ExtAddPairwiseI16x8S => todo!(),
        wp::I32x4ExtAddPairwiseI16x8U => todo!(),
        wp::I32x4Abs => todo!(),
        wp::I32x4Neg => todo!(),
        wp::I32x4AllTrue => todo!(),
        wp::I32x4Bitmask => todo!(),
        wp::I32x4ExtendLowI16x8S => todo!(),
        wp::I32x4ExtendHighI16x8S => todo!(),
        wp::I32x4ExtendLowI16x8U => todo!(),
        wp::I32x4ExtendHighI16x8U => todo!(),
        wp::I32x4Shl => todo!(),
        wp::I32x4ShrS => todo!(),
        wp::I32x4ShrU => todo!(),
        wp::I32x4Add => todo!(),
        wp::I32x4Sub => todo!(),
        wp::I32x4Mul => todo!(),
        wp::I32x4MinS => todo!(),
        wp::I32x4MinU => todo!(),
        wp::I32x4MaxS => todo!(),
        wp::I32x4MaxU => todo!(),
        wp::I32x4DotI16x8S => todo!(),
        wp::I32x4ExtMulLowI16x8S => todo!(),
        wp::I32x4ExtMulHighI16x8S => todo!(),
        wp::I32x4ExtMulLowI16x8U => todo!(),
        wp::I32x4ExtMulHighI16x8U => todo!(),
        wp::I64x2Abs => todo!(),
        wp::I64x2Neg => todo!(),
        wp::I64x2AllTrue => todo!(),
        wp::I64x2Bitmask => todo!(),
        wp::I64x2ExtendLowI32x4S => todo!(),
        wp::I64x2ExtendHighI32x4S => todo!(),
        wp::I64x2ExtendLowI32x4U => todo!(),
        wp::I64x2ExtendHighI32x4U => todo!(),
        wp::I64x2Shl => todo!(),
        wp::I64x2ShrS => todo!(),
        wp::I64x2ShrU => todo!(),
        wp::I64x2Add => todo!(),
        wp::I64x2Sub => todo!(),
        wp::I64x2Mul => todo!(),
        wp::I64x2ExtMulLowI32x4S => todo!(),
        wp::I64x2ExtMulHighI32x4S => todo!(),
        wp::I64x2ExtMulLowI32x4U => todo!(),
        wp::I64x2ExtMulHighI32x4U => todo!(),
        wp::F32x4Ceil => todo!(),
        wp::F32x4Floor => todo!(),
        wp::F32x4Trunc => todo!(),
        wp::F32x4Nearest => todo!(),
        wp::F32x4Abs => todo!(),
        wp::F32x4Neg => todo!(),
        wp::F32x4Sqrt => todo!(),
        wp::F32x4Add => todo!(),
        wp::F32x4Sub => todo!(),
        wp::F32x4Mul => todo!(),
        wp::F32x4Div => todo!(),
        wp::F32x4Min => todo!(),
        wp::F32x4Max => todo!(),
        wp::F32x4PMin => todo!(),
        wp::F32x4PMax => todo!(),
        wp::F64x2Ceil => todo!(),
        wp::F64x2Floor => todo!(),
        wp::F64x2Trunc => todo!(),
        wp::F64x2Nearest => todo!(),
        wp::F64x2Abs => todo!(),
        wp::F64x2Neg => todo!(),
        wp::F64x2Sqrt => todo!(),
        wp::F64x2Add => todo!(),
        wp::F64x2Sub => todo!(),
        wp::F64x2Mul => todo!(),
        wp::F64x2Div => todo!(),
        wp::F64x2Min => todo!(),
        wp::F64x2Max => todo!(),
        wp::F64x2PMin => todo!(),
        wp::F64x2PMax => todo!(),
        wp::I32x4TruncSatF32x4S => todo!(),
        wp::I32x4TruncSatF32x4U => todo!(),
        wp::F32x4ConvertI32x4S => todo!(),
        wp::F32x4ConvertI32x4U => todo!(),
        wp::I32x4TruncSatF64x2SZero => todo!(),
        wp::I32x4TruncSatF64x2UZero => todo!(),
        wp::F64x2ConvertLowI32x4S => todo!(),
        wp::F64x2ConvertLowI32x4U => todo!(),
        wp::F32x4DemoteF64x2Zero => todo!(),
        wp::F64x2PromoteLowF32x4 => todo!(),
    })
}

fn convert_memarg(memarg: wasmparser::MemoryImmediate) -> Result<Memarg, UnsupportedError> {
    let offset: u32 = memarg
        .offset
        .try_into()
        .map_err(|_| UnsupportedError(WasmExtension::Memory64))?;
    if memarg.memory != 0 {
        Err(UnsupportedError(WasmExtension::MultiMemory))?
    }
    Ok(Memarg {
        alignment_exp: memarg.align,
        offset,
    })
}

fn convert_memory_ty(ty: wasmparser::MemoryType) -> Result<MemoryType, UnsupportedError> {
    if ty.memory64 {
        Err(UnsupportedError(WasmExtension::Memory64))?
    }
    Ok(MemoryType(Limits {
        initial_size: ty
            .initial
            .try_into()
            .expect("guaranteed by wasmparser if !memory64"),
        max_size: ty
            .maximum
            .map(|u| u.try_into().expect("guaranteed by wasmparser if !memory64")),
    }))
}

fn convert_table_ty(ty: wasmparser::TableType) -> Result<TableType, UnsupportedError> {
    Ok(TableType(
        convert_elem_ty(ty.element_type)?,
        Limits {
            initial_size: ty.initial,
            max_size: ty.maximum,
        },
    ))
}

fn convert_elem_ty(ty: wasmparser::Type) -> Result<ElemType, UnsupportedError> {
    use wasmparser::Type::*;
    match ty {
        I32 => todo!(),
        I64 => todo!(),
        F32 => todo!(),
        F64 => todo!(),
        V128 => todo!(),
        FuncRef => Ok(ElemType::Anyfunc),
        ExternRef => todo!(),
        ExnRef => todo!(),
        Func => todo!(),
        EmptyBlockType => unreachable!("table elements should have no block type"),
    }
}

fn convert_block_ty(ty: wasmparser::TypeOrFuncType) -> Result<BlockType, UnsupportedError> {
    use wasmparser::TypeOrFuncType::*;
    match ty {
        Type(wasmparser::Type::EmptyBlockType) => Ok(BlockType(None)),
        Type(ty) => Ok(BlockType(Some(convert_ty(ty)))),
        FuncType(_) => todo!(),
    }
}

fn convert_func_ty(ty: wasmparser::FuncType) -> FunctionType {
    FunctionType {
        // TODO Optimize, no intermediate collection to Vec.
        params: ty
            .params
            .iter()
            .cloned()
            .map(convert_ty)
            .collect::<Vec<_>>()
            .into(),
        results: ty
            .returns
            .iter()
            .cloned()
            .map(convert_ty)
            .collect::<Vec<_>>()
            .into(),
    }
}

fn convert_global_ty(ty: wasmparser::GlobalType) -> GlobalType {
    GlobalType(
        convert_ty(ty.content_type),
        if ty.mutable {
            Mutability::Mut
        } else {
            Mutability::Const
        },
    )
}

fn convert_ty(ty: wasmparser::Type) -> ValType {
    use wasmparser::Type;
    match ty {
        Type::I32 => ValType::I32,
        Type::I64 => ValType::I64,
        Type::F32 => ValType::F32,
        Type::F64 => ValType::F64,
        Type::V128 => todo!(),
        Type::FuncRef => todo!(),
        Type::ExternRef => todo!(),
        Type::ExnRef => todo!(),
        Type::Func => todo!(),
        Type::EmptyBlockType => unreachable!("this function should only be called with generic types, for block types this should already be covered"),
    }
}

// impl<T> AddErrInfo<T> for Result<T, BinaryReaderError> {
//     fn add_err_info<GrammarElement>(self: Result<T, BinaryReaderError>, offset: usize) -> Result<T, Error> {
//         self.map_err(|err|
//             Error::with_source::<GrammarElement, _>(offset, ErrorKind::Leb128, err))
//     }
// }

#[derive(Debug)]
struct IndexError<T>(Idx<T>);

impl<T: fmt::Debug> std::error::Error for IndexError<T> {}

impl<T> fmt::Display for IndexError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_name = std::any::type_name::<T>().split("::").last().unwrap();
        writeln!(
            f,
            "{} index out of bounds: {}",
            type_name,
            self.0.into_inner()
        )
    }
}

// TODO higher level error type that contains:
//     offset: usize,

#[derive(Debug)]
struct UnsupportedError(WasmExtension);

impl std::error::Error for UnsupportedError {}

impl fmt::Display for UnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "This module uses a WebAssembly extension we don't support yet: {}",
            self.0.name()
        )?;
        writeln!(
            f,
            "See {} for more information about the extension.",
            self.0.url()
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum WasmExtension {
    ModuleLinking,
    Memory64,
    ExceptionHandling,
    TailCall,
    ReferenceTypes,
    MultiMemory,
    TypeImports,
    BulkMemoryOperations,
}

impl WasmExtension {
    pub fn name(self) -> &'static str {
        use WasmExtension::*;
        match self {
            ModuleLinking => "module linking",
            Memory64 => "64-bit memory",
            ExceptionHandling => "exception handling",
            TailCall => "tail calls",
            ReferenceTypes => "reference types",
            MultiMemory => "multiple memories",
            TypeImports => "type imports",
            BulkMemoryOperations => "bulk memory operations",
        }
    }

    pub fn url(self) -> &'static str {
        use WasmExtension::*;
        match self {
            ModuleLinking => r"https://github.com/WebAssembly/module-linking",
            Memory64 => r"https://github.com/WebAssembly/memory64",
            ExceptionHandling => r"https://github.com/WebAssembly/exception-handling",
            TailCall => r"https://github.com/WebAssembly/tail-call",
            ReferenceTypes => r"https://github.com/WebAssembly/reference-types",
            MultiMemory => r"https://github.com/WebAssembly/multi-memory",
            TypeImports => r"https://github.com/WebAssembly/proposal-type-imports",
            BulkMemoryOperations => r"https://github.com/WebAssembly/bulk-memory-operations",
        }
    }
}

// Wrapper for type map, to offer some convenience like:
// - u32 indices (which we get from wasmparser) instead of usize (which Vec expects)
// - checking that type section was present and type index is occupied
struct Types(Option<Vec<FunctionType>>);

impl Types {
    /// Initial state, where the type section has not been parsed yet.
    pub fn none() -> Self {
        Types(None)
    }

    /// Next state, where the number of type entries is known, but nothing filled yet.
    pub fn set_capacity(&mut self, count: u32) -> Result<(), &'static str> {
        let prev_state = self.0.replace(Vec::with_capacity(u32_to_usize(count)));
        match prev_state {
            Some(_) => Err("duplicate type section"),
            None => Ok(()),
        }
    }

    pub fn add(&mut self, ty: wasmparser::FuncType) -> Result<(), &'static str> {
        self.0
            .as_mut()
            .ok_or("missing type section")?
            .push(convert_func_ty(ty));
        Ok(())
    }

    pub fn get(&self, idx: u32) -> Result<FunctionType, Box<dyn std::error::Error>> {
        Ok(self
            .0
            .as_ref()
            // TODO typed error
            .ok_or("missing type section")?
            .get(u32_to_usize(idx))
            .cloned()
            .ok_or(IndexError::<FunctionType>(idx.into()))?)
    }
}

fn u32_to_usize(u: u32) -> usize {
    u.try_into().expect("u32 to usize should always succeed")
}
