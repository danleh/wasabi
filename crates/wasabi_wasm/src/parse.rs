//! Code for parsing the WebAssembly binary format to our AST.
//! Uses `wasmparser` crate for the actual low-level work.

use std::convert::TryInto;
use std::sync::RwLock;

use ordered_float::OrderedFloat;
use rayon::prelude::*;

use smallvec::SmallVec;
use wasmparser as wp;

use crate::extensions::WasmExtension;
use crate::*;

// The streaming API of wasmparser is a bit cumbersome, so implement reading
// from bytes fully resident in memory first.
// TODO Add a second API from streaming sources, i.e., `io::Read` like here:
// https://docs.rs/wasmparser/latest/wasmparser/struct.Parser.html#examples
pub fn parse_module(bytes: &[u8]) -> Result<(Module, Offsets, ParseWarnings), ParseError> {
    let mut warnings = Vec::new();

    // The final module to return.
    let mut module = Module::default();

    // State during module parsing.
    let mut types = Types::none();
    let mut imported_function_count = 0;
    let mut current_code_index = 0;
    let mut section_offsets = Vec::with_capacity(16);
    let mut function_offsets = Vec::new();
    // Put the function bodies in their own vector, such that parallel processing of the
    // code section doesn't require synchronization on the shared `module` variable.
    let mut function_bodies = Vec::new();
    let mut code_entries_count = 0;
    let metadata = RwLock::new(ModuleMetadata::default());

    for payload in wp::Parser::new(0).parse_all(bytes) {
        match payload? {
            wp::Payload::Version {
                num: _,
                encoding,
                range: _,
            } => {
                // The version number is checked by wasmparser to always be 1.
                match encoding {
                    wp::Encoding::Module => {
                        // That's what we are here for :)
                    }
                    wp::Encoding::Component => {
                        Err(ParseIssue::unsupported(0, WasmExtension::ComponentModel))?
                    }
                }
            }
            wp::Payload::TypeSection(reader) => {
                // This is the offset AFTER the section tag and size in bytes,
                // but BEFORE the number of elements in the section.
                let type_offset = reader.range().start;
                section_offsets.push((SectionId::Type, type_offset));

                types.new_type_section(reader.count(), type_offset)?;

                for elem in reader.into_iter_with_offsets() {
                    let (offset, wp::Type::Func(type_)) = elem?;
                    let type_ = parse_func_ty(type_, offset)?;
                    types.add(type_);
                }
            }
            wp::Payload::ImportSection(reader) => {
                section_offsets.push((SectionId::Import, reader.range().start));

                for elem in reader.into_iter_with_offsets() {
                    let (import_offset, import) = elem?;

                    let import_module = import.module.to_string();
                    let import_name = import.name.to_string();

                    match import.ty {
                        wp::TypeRef::Func(ty_index) => {
                            imported_function_count += 1;
                            module.functions.push(Function::new_imported(
                                // The `import_offset` is not actually the offset of the type index,
                                // but wasmparser doesn't offer a way to get the latter.
                                // This slightly misattributes potential errors, namely to the beginning of the import.
                                types.get(ty_index, import_offset)?,
                                import_module,
                                import_name,
                                Vec::new(),
                            ))
                        }
                        wp::TypeRef::Global(ty) => module.globals.push(
                            // Same issue regarding `import_offset`.
                            Global::new_imported(
                                parse_global_ty(ty, import_offset)?,
                                import_module,
                                import_name,
                            ),
                        ),
                        wp::TypeRef::Table(ty) => module.tables.push(
                            // Same issue regarding `import_offset`.
                            Table::new_imported(
                                parse_table_ty(ty, import_offset)?,
                                import_module,
                                import_name,
                            ),
                        ),
                        wp::TypeRef::Memory(ty) => {
                            // Same issue regarding `import_offset`.
                            module.memories.push(Memory::new_imported(
                                parse_memory_ty(ty, import_offset)?,
                                import_module,
                                import_name,
                            ))
                        }
                        wp::TypeRef::Tag(_) => {
                            // Same issue regarding `import_offset`.
                            Err(ParseIssue::unsupported(
                                import_offset,
                                WasmExtension::ExceptionHandling,
                            ))?
                        }
                    }
                }
            }
            wp::Payload::FunctionSection(reader) => {
                section_offsets.push((SectionId::Function, reader.range().start));

                let function_count = reader.count();
                module.functions.reserve(u32_to_usize(function_count));

                for elem in reader.into_iter_with_offsets() {
                    let (offset, type_index) = elem?;
                    let type_ = types.get(type_index, offset)?;
                    // Fill in the code of the function later with the code section.
                    module
                        .functions
                        .push(Function::new(type_, Code::new(), Vec::new()));
                }
            }
            wp::Payload::TableSection(reader) => {
                section_offsets.push((SectionId::Table, reader.range().start));

                let table_count = reader.count();
                module.tables.reserve(u32_to_usize(table_count));

                for elem in reader.into_iter_with_offsets() {
                    let (offset, table_ty) = elem?;
                    let table_ty = parse_table_ty(table_ty, offset)?;
                    // Fill in the elements of the table later with the element section.
                    module.tables.push(Table::new(table_ty));
                }
            }
            wp::Payload::MemorySection(reader) => {
                section_offsets.push((SectionId::Memory, reader.range().start));

                let memory_count = reader.count();
                module.memories.reserve(u32_to_usize(memory_count));

                for elem in reader.into_iter_with_offsets() {
                    let (offset, memory_ty) = elem?;
                    let memory_ty = parse_memory_ty(memory_ty, offset)?;
                    // Fill in the data of the memory later with the data section.
                    module.memories.push(Memory::new(memory_ty));
                }
            }
            wp::Payload::TagSection(reader) => Err(ParseIssue::unsupported(
                reader.range().start,
                WasmExtension::ExceptionHandling,
            ))?,
            wp::Payload::GlobalSection(reader) => {
                section_offsets.push((SectionId::Global, reader.range().start));

                let global_count = reader.count();
                module.globals.reserve(u32_to_usize(global_count));

                for elem in reader.into_iter_with_offsets() {
                    let (offset, global) = elem?;
                    let type_ = parse_global_ty(global.ty, offset)?;

                    // Most initialization expressions have just a constant and the end instruction.
                    let mut init = Vec::with_capacity(2);
                    for op in global.init_expr.get_operators_reader() {
                        // The `offset` will be slightly off, because it points to the beginning of the
                        // whole global entry, not the initialization expression.
                        init.push(parse_instr(op?, offset, &types, &metadata)?)
                    }

                    module.globals.push(Global::new(type_, init));
                }
            }
            wp::Payload::ExportSection(reader) => {
                section_offsets.push((SectionId::Export, reader.range().start));

                for elem in reader.into_iter_with_offsets() {
                    let (export_offset, export) = elem?;

                    let name = export.name.to_string();
                    let index_u32 = export.index;
                    let index = u32_to_usize(export.index);

                    use wp::ExternalKind;
                    match export.kind {
                        ExternalKind::Func => module
                            .functions
                            .get_mut(index)
                            // The `export_offset` is not actually the offset of the function index,
                            // but wasmparser doesn't offer a way to get the latter.
                            // This slightly misattributes potential errors, namely to the beginning of the export.
                            .ok_or_else(|| ParseIssue::index(export_offset, index_u32, "function"))?
                            .export
                            .push(name),
                        ExternalKind::Table => module
                            .tables
                            .get_mut(index)
                            // Same issue regarding `export_offset`.
                            .ok_or_else(|| ParseIssue::index(export_offset, index_u32, "table"))?
                            .export
                            .push(name),
                        ExternalKind::Memory => module
                            .memories
                            .get_mut(index)
                            // Same issue regarding `export_offset`.
                            .ok_or_else(|| ParseIssue::index(export_offset, index_u32, "memory"))?
                            .export
                            .push(name),
                        ExternalKind::Global => module
                            .globals
                            .get_mut(index)
                            // Same issue regarding `export_offset`.
                            .ok_or_else(|| ParseIssue::index(export_offset, index_u32, "global"))?
                            .export
                            .push(name),
                        ExternalKind::Tag => {
                            // Same issue regarding `export_offset`.
                            Err(ParseIssue::unsupported(
                                export_offset,
                                WasmExtension::ExceptionHandling,
                            ))?
                        }
                    };
                }
            }
            wp::Payload::StartSection { func, range } => {
                section_offsets.push((SectionId::Start, range.start));

                let prev_start = std::mem::replace(&mut module.start, Some(func.into()));
                if prev_start.is_some() {
                    Err(ParseIssue::message(
                        range.start,
                        "duplicate start section",
                        None,
                    ))?
                }
            }
            wp::Payload::ElementSection(reader) => {
                section_offsets.push((SectionId::Element, reader.range().start));

                for elem in reader.into_iter_with_offsets() {
                    let (element_offset, element) = elem?;
                    parse_elem_ty(element.ty, element_offset)?;

                    let items = match element.items {
                        wp::ElementItems::Functions(items_reader) => items_reader
                            .into_iter()
                            .map(|func_idx| func_idx.map(|func_idx| u32_to_usize(func_idx).into()))
                            .collect::<Result<Vec<Idx<Function>>, _>>()?,
                        wp::ElementItems::Expressions(reader) => Err(ParseIssue::unsupported(
                            reader.original_position(),
                            WasmExtension::ReferenceTypes,
                        ))?,
                    };

                    match element.kind {
                        wp::ElementKind::Active {
                            table_index,
                            offset_expr,
                        } => {
                            let table = module
                                .tables
                                .get_mut(u32_to_usize(table_index))
                                .ok_or_else(|| {
                                    ParseIssue::index(element_offset, table_index, "table")
                                })?;

                            // Most offset expressions are just a constant and the end instruction.
                            let mut offset_instrs = Vec::with_capacity(2);
                            for op_offset in
                                offset_expr.get_operators_reader().into_iter_with_offsets()
                            {
                                let (op, offset) = op_offset?;
                                offset_instrs.push(parse_instr(op, offset, &types, &metadata)?)
                            }

                            table.elements.push(Element {
                                offset: offset_instrs,
                                functions: items,
                            })
                        }
                        wp::ElementKind::Passive => Err(ParseIssue::unsupported(
                            element_offset,
                            WasmExtension::BulkMemoryOperations,
                        ))?,
                        wp::ElementKind::Declared => Err(ParseIssue::unsupported(
                            element_offset,
                            WasmExtension::ReferenceTypes,
                        ))?,
                    }
                }
            }
            wp::Payload::DataCountSection { count: _, range } => Err(ParseIssue::unsupported(
                range.start,
                WasmExtension::BulkMemoryOperations,
            ))?,
            wp::Payload::DataSection(reader) => {
                section_offsets.push((SectionId::Data, reader.range().start));

                for elem in reader.into_iter_with_offsets() {
                    let (data_offset, data) = elem?;

                    match data.kind {
                        wp::DataKind::Active {
                            memory_index,
                            offset_expr,
                        } => {
                            let memory = module
                                .memories
                                .get_mut(u32_to_usize(memory_index))
                                .ok_or_else(|| {
                                    ParseIssue::index(data_offset, memory_index, "memory")
                                })?;

                            // Most offset expressions are just a constant and the end instruction.
                            let mut offset_instrs = Vec::with_capacity(2);
                            for op_offset in
                                offset_expr.get_operators_reader().into_iter_with_offsets()
                            {
                                let (op, offset) = op_offset?;
                                offset_instrs.push(parse_instr(op, offset, &types, &metadata)?)
                            }

                            memory.data.push(Data {
                                offset: offset_instrs,
                                bytes: data.data.to_vec(),
                            })
                        }
                        wp::DataKind::Passive => Err(ParseIssue::unsupported(
                            data_offset,
                            WasmExtension::BulkMemoryOperations,
                        ))?,
                    }
                }
            }
            wp::Payload::CodeSectionStart {
                count,
                range,
                size: _,
            } => {
                section_offsets.push((SectionId::Code, range.start));

                function_offsets.reserve_exact(u32_to_usize(count));
                function_bodies.reserve_exact(u32_to_usize(count));

                code_entries_count = count;
            }
            wp::Payload::CodeSectionEntry(body) => {
                let func_index = imported_function_count + current_code_index;

                function_offsets.push((func_index.into(), body.range().start));
                function_bodies.push((func_index, body));

                current_code_index += 1;

                let last_code_entry = current_code_index == code_entries_count;
                if last_code_entry {
                    // Parse and convert to high-level instructions in parallel.
                    let function_bodies = function_bodies
                        .par_drain(..)
                        .map(|(func_idx, body)| {
                            (
                                func_idx,
                                body.range().start,
                                parse_body(body, &types, &metadata),
                            )
                        })
                        .collect::<Vec<_>>();
                    // Attach the converted function bodies to the function definitions (not parallel).
                    for (func_idx, offset, code) in function_bodies {
                        let function = module
                            .functions
                            .get_mut(u32_to_usize(func_idx))
                            .ok_or_else(|| ParseIssue::index(offset, func_idx, "function"))?;
                        function.code = ImportOrPresent::Present(code?);
                    }
                }
            }
            wp::Payload::CustomSection(reader) => {
                let name = reader.name().to_string();
                let previous_section_id = section_offsets
                    .last()
                    .map(|(section, _offset)| section)
                    .cloned();
                let custom_section_start_offset = reader.range().start;
                section_offsets
                    .push((SectionId::Custom(name.clone()), custom_section_start_offset));

                // Name custom section.
                if name == "name" {
                    // If parts of the name section cannot be parsed, collect the issue as a warning and abort parsing the
                    // name section, but produce an AST for the rest of the module.
                    match parse_name_custom_section(
                        reader.data(),
                        reader.data_offset(),
                        &mut warnings,
                        &mut module,
                    ) {
                        Ok(()) => {
                            // All the names got inserted into the AST, so no need to add a custom section.
                            continue;
                        }
                        Err(name_parsing_aborted) => {
                            warnings.push(ParseIssue::Message {
                                offset: custom_section_start_offset,
                                message: "could not parse name section, adding it as a raw (unparsed) custom section...",
                                source: Some(Box::new(name_parsing_aborted)),
                            });
                        }
                    }
                }

                // If the custom section is NOT a name section, or if its parsing was not successful:
                let raw_custom_section = RawCustomSection {
                    name,
                    content: reader.data().to_vec(),
                    previous_section: previous_section_id,
                };
                module.custom_sections.push(raw_custom_section);
            }
            wp::Payload::ModuleSection { parser: _, range }
            | wp::Payload::ComponentSection { parser: _, range } => Err(ParseIssue::unsupported(
                range.start,
                WasmExtension::ComponentModel,
            ))?,
            wp::Payload::InstanceSection(reader) => Err(ParseIssue::unsupported(
                reader.range().start,
                WasmExtension::ComponentModel,
            ))?,
            wp::Payload::CoreTypeSection(reader) => Err(ParseIssue::unsupported(
                reader.range().start,
                WasmExtension::ComponentModel,
            ))?,
            wp::Payload::ComponentInstanceSection(reader) => Err(ParseIssue::unsupported(
                reader.range().start,
                WasmExtension::ComponentModel,
            ))?,
            wp::Payload::ComponentAliasSection(reader) => Err(ParseIssue::unsupported(
                reader.range().start,
                WasmExtension::ComponentModel,
            ))?,
            wp::Payload::ComponentTypeSection(reader) => Err(ParseIssue::unsupported(
                reader.range().start,
                WasmExtension::ComponentModel,
            ))?,
            wp::Payload::ComponentCanonicalSection(reader) => Err(ParseIssue::unsupported(
                reader.range().start,
                WasmExtension::ComponentModel,
            ))?,
            wp::Payload::ComponentStartSection { start: _, range } => Err(
                ParseIssue::unsupported(range.start, WasmExtension::ComponentModel),
            )?,
            wp::Payload::ComponentImportSection(reader) => Err(ParseIssue::unsupported(
                reader.range().start,
                WasmExtension::ComponentModel,
            ))?,
            wp::Payload::ComponentExportSection(reader) => Err(ParseIssue::unsupported(
                reader.range().start,
                WasmExtension::ComponentModel,
            ))?,
            wp::Payload::UnknownSection {
                id: _,
                contents: _,
                range,
            } => Err(ParseIssue::message(range.start, "unknown section", None))?,
            wp::Payload::End(_offset_bytes) => {
                // I don't understand what this end marker is for?
                // If the module ended (i.e., the input buffer is exhausted),
                // there is just no more payload following, isn't there?
            }
        }
    }

    let offsets = Offsets {
        sections: section_offsets,
        functions_code: function_offsets,
    };

    module.metadata = metadata.into_inner().unwrap();

    Ok((module, offsets, warnings))
}

fn parse_body(
    body: wp::FunctionBody,
    types: &Types,
    metadata: &RwLock<ModuleMetadata>,
) -> Result<Code, ParseError> {
    let mut locals_reader = body.get_locals_reader()?;
    let mut offset = locals_reader.original_position();
    // Pre-allocate: There are at least as many locals as there are _unique_ local types.
    let mut locals = Vec::with_capacity(u32_to_usize(locals_reader.get_count()));
    for _ in 0..locals_reader.get_count() {
        let (count, type_) = locals_reader.read()?;
        let count = u32_to_usize(count);
        let type_ = parse_val_ty(type_, offset)?;
        locals.extend(std::iter::repeat(Local::new(type_)).take(count));
        offset = locals_reader.original_position();
    }

    // Pre-allocate: We don't know the exact number of instructions yet,
    // but there are typically one or two bytes per instruction.
    // So conservatively, bytes / 2 should be a good starting point.
    // It will almost never over-reserve memory and has to grow / re-allocate
    // at most once in case there really is on avg. 1 instruction per byte.
    // UNFORTUNATELY, on my Windows 10 installation, this pre-allocation is
    // causing a massive slowdown during parallel parsing (Ryzen 5950X, 16 cores, 32 threads)
    // of >200% (!!) vs. just using Vec::new(), i.e., no pre-allocation at all.
    // It seems the combination of pre-allocation + from multiple threads
    // sucks big time with the Windows default allocator.
    // I also tried whether the amount of pre-allocation makes a difference (e.g.,
    // bytes / 16, or limiting to at most 1024 instructions), but there is a floor.
    // Even with just Vec::new() it is still >200% slower than Linux.
    // Speculating, I think the problem is that all threads start parsing functions
    // at the same time and then compete for some global lock in the allocator.
    // I considered a couple of options:
    // a) Get rid of parallel parsing, but I don't want that since for big
    //    binaries its a huge speedup and embarrassingly parallel.
    // b) Don't pre-allocate at all, but this hurts Linux performance (overall
    //    parsing without this single pre-allocation, i.e., Vec::new() is ~5% slower!)
    //    and Windows is still >200% slower than Linux, which it shouldn't be.
    // c) Use a different allocator on Windows.
    // Even though I don't like the complexity of d), I think it's the best option
    // since it fixes the underlying problem, and also improves overall performance
    // as we are quite allocation-heavy.
    // For reference, benchmarking a single medium-size binary (bananabread),
    // parallel parsing is 700% slower with the Windows 10 system allocator, and
    // encoding is 50% slower vs. mimalloc.
    // Since the performance is not improved on Linux, just enable it on Windows.
    let body_byte_size = body.range().end - body.range().start;
    let approx_instr_count = body_byte_size / 2;
    let mut instrs = Vec::with_capacity(approx_instr_count);

    for op_offset in body.get_operators_reader()?.into_iter_with_offsets() {
        let (op, offset) = op_offset?;
        instrs.push(parse_instr(op, offset, types, metadata)?);
    }

    Ok(Code {
        locals,
        body: instrs,
    })
}

fn parse_instr(
    op: wp::Operator,
    offset: usize,
    types: &Types,
    metadata: &RwLock<ModuleMetadata>,
) -> Result<Instr, ParseError> {
    use crate::Instr::*;
    use wp::Operator as wp;
    Ok(match op {
        wp::Unreachable => Unreachable,
        wp::Nop => Nop,

        wp::Block { blockty } => Block(parse_block_ty(blockty, offset + 1, types, metadata)?),
        wp::Loop { blockty } => Loop(parse_block_ty(blockty, offset + 1, types, metadata)?),
        wp::If { blockty } => If(parse_block_ty(blockty, offset + 1, types, metadata)?),
        wp::Else => Else,
        wp::End => End,

        wp::Try { blockty: _ }
        | wp::Catch { tag_index: _ }
        | wp::CatchAll
        | wp::Throw { tag_index: _ }
        | wp::Rethrow { relative_depth: _ }
        | wp::Delegate { relative_depth: _ } => Err(ParseIssue::unsupported(
            offset,
            WasmExtension::ExceptionHandling,
        ))?,

        wp::Br { relative_depth } => Br(Label::from(relative_depth)),
        wp::BrIf { relative_depth } => BrIf(Label::from(relative_depth)),
        wp::BrTable { targets } => {
            let default = Label::from(targets.default());
            let mut table = Vec::with_capacity(u32_to_usize(targets.len()));
            for target in targets.targets() {
                table.push(Label::from(target?))
            }
            BrTable {
                table: table.into_boxed_slice(),
                default,
            }
        }

        wp::Return => Return,
        wp::Call { function_index } => Call(function_index.into()),
        wp::CallIndirect {
            type_index,
            table_index,
            table_byte,
        } => {
            if table_index != 0 {
                Err(ParseIssue::unsupported(
                    offset,
                    WasmExtension::ReferenceTypes,
                ))?
            }
            assert!(table_byte == 0, "not sure which extension this is");
            CallIndirect(types.get(type_index, offset + 1)?, 0usize.into())
        }

        wp::ReturnCall { function_index: _ }
        | wp::ReturnCallIndirect {
            type_index: _,
            table_index: _,
        } => Err(ParseIssue::unsupported(offset, WasmExtension::TailCalls))?,

        wp::Drop => Drop,
        wp::Select => Select,

        wp::TypedSelect { ty: _ } => Err(ParseIssue::unsupported(
            offset,
            WasmExtension::ReferenceTypes,
        ))?,

        wp::LocalGet { local_index } => Local(LocalOp::Get, local_index.into()),
        wp::LocalSet { local_index } => Local(LocalOp::Set, local_index.into()),
        wp::LocalTee { local_index } => Local(LocalOp::Tee, local_index.into()),
        wp::GlobalGet { global_index } => Global(GlobalOp::Get, global_index.into()),
        wp::GlobalSet { global_index } => Global(GlobalOp::Set, global_index.into()),

        wp::I32Load { memarg } => Load(LoadOp::I32Load, parse_memarg(memarg, offset + 1)?),
        wp::I64Load { memarg } => Load(LoadOp::I64Load, parse_memarg(memarg, offset + 1)?),
        wp::F32Load { memarg } => Load(LoadOp::F32Load, parse_memarg(memarg, offset + 1)?),
        wp::F64Load { memarg } => Load(LoadOp::F64Load, parse_memarg(memarg, offset + 1)?),
        wp::I32Load8S { memarg } => Load(LoadOp::I32Load8S, parse_memarg(memarg, offset + 1)?),
        wp::I32Load8U { memarg } => Load(LoadOp::I32Load8U, parse_memarg(memarg, offset + 1)?),
        wp::I32Load16S { memarg } => Load(LoadOp::I32Load16S, parse_memarg(memarg, offset + 1)?),
        wp::I32Load16U { memarg } => Load(LoadOp::I32Load16U, parse_memarg(memarg, offset + 1)?),
        wp::I64Load8S { memarg } => Load(LoadOp::I64Load8S, parse_memarg(memarg, offset + 1)?),
        wp::I64Load8U { memarg } => Load(LoadOp::I64Load8U, parse_memarg(memarg, offset + 1)?),
        wp::I64Load16S { memarg } => Load(LoadOp::I64Load16S, parse_memarg(memarg, offset + 1)?),
        wp::I64Load16U { memarg } => Load(LoadOp::I64Load16U, parse_memarg(memarg, offset + 1)?),
        wp::I64Load32S { memarg } => Load(LoadOp::I64Load32S, parse_memarg(memarg, offset + 1)?),
        wp::I64Load32U { memarg } => Load(LoadOp::I64Load32U, parse_memarg(memarg, offset + 1)?),

        wp::I32Store { memarg } => Store(StoreOp::I32Store, parse_memarg(memarg, offset + 1)?),
        wp::I64Store { memarg } => Store(StoreOp::I64Store, parse_memarg(memarg, offset + 1)?),
        wp::F32Store { memarg } => Store(StoreOp::F32Store, parse_memarg(memarg, offset + 1)?),
        wp::F64Store { memarg } => Store(StoreOp::F64Store, parse_memarg(memarg, offset + 1)?),
        wp::I32Store8 { memarg } => Store(StoreOp::I32Store8, parse_memarg(memarg, offset + 1)?),
        wp::I32Store16 { memarg } => Store(StoreOp::I32Store16, parse_memarg(memarg, offset + 1)?),
        wp::I64Store8 { memarg } => Store(StoreOp::I64Store8, parse_memarg(memarg, offset + 1)?),
        wp::I64Store16 { memarg } => Store(StoreOp::I64Store16, parse_memarg(memarg, offset + 1)?),
        wp::I64Store32 { memarg } => Store(StoreOp::I64Store32, parse_memarg(memarg, offset + 1)?),

        // This is not well documented in wasmparser: `mem_byte` and `mem` essentially contain
        // the same information, it's just that mem_byte is the original (single) byte that was
        // read from the instruction stream, and mem is it if parsed as a LEB128.
        // I think the variable-length parser is more robust, as it can handle memory indices
        // above 255, so ignore `mem_byte` here.
        wp::MemorySize { mem, mem_byte: _ } => {
            if mem != 0 {
                Err(ParseIssue::unsupported(offset, WasmExtension::MultiMemory))?
            }
            MemorySize(0u32.into())
        }
        wp::MemoryGrow { mem, mem_byte: _ } => {
            if mem != 0 {
                Err(ParseIssue::unsupported(offset, WasmExtension::MultiMemory))?
            }
            MemoryGrow(0u32.into())
        }

        wp::I32Const { value } => Const(Val::I32(value)),
        wp::I64Const { value } => Const(Val::I64(value)),
        wp::F32Const { value } => Const(Val::F32(OrderedFloat(f32::from_bits(value.bits())))),
        wp::F64Const { value } => Const(Val::F64(OrderedFloat(f64::from_bits(value.bits())))),

        wp::RefNull { ty: _ } | wp::RefIsNull | wp::RefFunc { function_index: _ } => Err(
            ParseIssue::unsupported(offset, WasmExtension::ReferenceTypes),
        )?,

        wp::I32Eqz => Unary(UnaryOp::I32Eqz),
        wp::I64Eqz => Unary(UnaryOp::I64Eqz),
        wp::I32Clz => Unary(UnaryOp::I32Clz),
        wp::I32Ctz => Unary(UnaryOp::I32Ctz),
        wp::I32Popcnt => Unary(UnaryOp::I32Popcnt),
        wp::I64Clz => Unary(UnaryOp::I64Clz),
        wp::I64Ctz => Unary(UnaryOp::I64Ctz),
        wp::I64Popcnt => Unary(UnaryOp::I64Popcnt),
        wp::F32Abs => Unary(UnaryOp::F32Abs),
        wp::F32Neg => Unary(UnaryOp::F32Neg),
        wp::F32Ceil => Unary(UnaryOp::F32Ceil),
        wp::F32Floor => Unary(UnaryOp::F32Floor),
        wp::F32Trunc => Unary(UnaryOp::F32Trunc),
        wp::F32Nearest => Unary(UnaryOp::F32Nearest),
        wp::F32Sqrt => Unary(UnaryOp::F32Sqrt),
        wp::F64Abs => Unary(UnaryOp::F64Abs),
        wp::F64Neg => Unary(UnaryOp::F64Neg),
        wp::F64Ceil => Unary(UnaryOp::F64Ceil),
        wp::F64Floor => Unary(UnaryOp::F64Floor),
        wp::F64Trunc => Unary(UnaryOp::F64Trunc),
        wp::F64Nearest => Unary(UnaryOp::F64Nearest),
        wp::F64Sqrt => Unary(UnaryOp::F64Sqrt),
        wp::I32WrapI64 => Unary(UnaryOp::I32WrapI64),
        wp::I32TruncF32S => Unary(UnaryOp::I32TruncF32S),
        wp::I32TruncF32U => Unary(UnaryOp::I32TruncF32U),
        wp::I32TruncF64S => Unary(UnaryOp::I32TruncF64S),
        wp::I32TruncF64U => Unary(UnaryOp::I32TruncF64U),
        wp::I64ExtendI32S => Unary(UnaryOp::I64ExtendI32S),
        wp::I64ExtendI32U => Unary(UnaryOp::I64ExtendI32U),
        wp::I64TruncF32S => Unary(UnaryOp::I64TruncF32S),
        wp::I64TruncF32U => Unary(UnaryOp::I64TruncF32U),
        wp::I64TruncF64S => Unary(UnaryOp::I64TruncF64S),
        wp::I64TruncF64U => Unary(UnaryOp::I64TruncF64U),
        wp::F32ConvertI32S => Unary(UnaryOp::F32ConvertI32S),
        wp::F32ConvertI32U => Unary(UnaryOp::F32ConvertI32U),
        wp::F32ConvertI64S => Unary(UnaryOp::F32ConvertI64S),
        wp::F32ConvertI64U => Unary(UnaryOp::F32ConvertI64U),
        wp::F32DemoteF64 => Unary(UnaryOp::F32DemoteF64),
        wp::F64ConvertI32S => Unary(UnaryOp::F64ConvertI32S),
        wp::F64ConvertI32U => Unary(UnaryOp::F64ConvertI32U),
        wp::F64ConvertI64S => Unary(UnaryOp::F64ConvertI64S),
        wp::F64ConvertI64U => Unary(UnaryOp::F64ConvertI64U),
        wp::F64PromoteF32 => Unary(UnaryOp::F64PromoteF32),
        wp::I32ReinterpretF32 => Unary(UnaryOp::I32ReinterpretF32),
        wp::I64ReinterpretF64 => Unary(UnaryOp::I64ReinterpretF64),
        wp::F32ReinterpretI32 => Unary(UnaryOp::F32ReinterpretI32),
        wp::F64ReinterpretI64 => Unary(UnaryOp::F64ReinterpretI64),

        wp::I32Eq => Binary(BinaryOp::I32Eq),
        wp::I32Ne => Binary(BinaryOp::I32Ne),
        wp::I32LtS => Binary(BinaryOp::I32LtS),
        wp::I32LtU => Binary(BinaryOp::I32LtU),
        wp::I32GtS => Binary(BinaryOp::I32GtS),
        wp::I32GtU => Binary(BinaryOp::I32GtU),
        wp::I32LeS => Binary(BinaryOp::I32LeS),
        wp::I32LeU => Binary(BinaryOp::I32LeU),
        wp::I32GeS => Binary(BinaryOp::I32GeS),
        wp::I32GeU => Binary(BinaryOp::I32GeU),
        wp::I64Eq => Binary(BinaryOp::I64Eq),
        wp::I64Ne => Binary(BinaryOp::I64Ne),
        wp::I64LtS => Binary(BinaryOp::I64LtS),
        wp::I64LtU => Binary(BinaryOp::I64LtU),
        wp::I64GtS => Binary(BinaryOp::I64GtS),
        wp::I64GtU => Binary(BinaryOp::I64GtU),
        wp::I64LeS => Binary(BinaryOp::I64LeS),
        wp::I64LeU => Binary(BinaryOp::I64LeU),
        wp::I64GeS => Binary(BinaryOp::I64GeS),
        wp::I64GeU => Binary(BinaryOp::I64GeU),
        wp::F32Eq => Binary(BinaryOp::F32Eq),
        wp::F32Ne => Binary(BinaryOp::F32Ne),
        wp::F32Lt => Binary(BinaryOp::F32Lt),
        wp::F32Gt => Binary(BinaryOp::F32Gt),
        wp::F32Le => Binary(BinaryOp::F32Le),
        wp::F32Ge => Binary(BinaryOp::F32Ge),
        wp::F64Eq => Binary(BinaryOp::F64Eq),
        wp::F64Ne => Binary(BinaryOp::F64Ne),
        wp::F64Lt => Binary(BinaryOp::F64Lt),
        wp::F64Gt => Binary(BinaryOp::F64Gt),
        wp::F64Le => Binary(BinaryOp::F64Le),
        wp::F64Ge => Binary(BinaryOp::F64Ge),
        wp::I32Add => Binary(BinaryOp::I32Add),
        wp::I32Sub => Binary(BinaryOp::I32Sub),
        wp::I32Mul => Binary(BinaryOp::I32Mul),
        wp::I32DivS => Binary(BinaryOp::I32DivS),
        wp::I32DivU => Binary(BinaryOp::I32DivU),
        wp::I32RemS => Binary(BinaryOp::I32RemS),
        wp::I32RemU => Binary(BinaryOp::I32RemU),
        wp::I32And => Binary(BinaryOp::I32And),
        wp::I32Or => Binary(BinaryOp::I32Or),
        wp::I32Xor => Binary(BinaryOp::I32Xor),
        wp::I32Shl => Binary(BinaryOp::I32Shl),
        wp::I32ShrS => Binary(BinaryOp::I32ShrS),
        wp::I32ShrU => Binary(BinaryOp::I32ShrU),
        wp::I32Rotl => Binary(BinaryOp::I32Rotl),
        wp::I32Rotr => Binary(BinaryOp::I32Rotr),
        wp::I64Add => Binary(BinaryOp::I64Add),
        wp::I64Sub => Binary(BinaryOp::I64Sub),
        wp::I64Mul => Binary(BinaryOp::I64Mul),
        wp::I64DivS => Binary(BinaryOp::I64DivS),
        wp::I64DivU => Binary(BinaryOp::I64DivU),
        wp::I64RemS => Binary(BinaryOp::I64RemS),
        wp::I64RemU => Binary(BinaryOp::I64RemU),
        wp::I64And => Binary(BinaryOp::I64And),
        wp::I64Or => Binary(BinaryOp::I64Or),
        wp::I64Xor => Binary(BinaryOp::I64Xor),
        wp::I64Shl => Binary(BinaryOp::I64Shl),
        wp::I64ShrS => Binary(BinaryOp::I64ShrS),
        wp::I64ShrU => Binary(BinaryOp::I64ShrU),
        wp::I64Rotl => Binary(BinaryOp::I64Rotl),
        wp::I64Rotr => Binary(BinaryOp::I64Rotr),
        wp::F32Add => Binary(BinaryOp::F32Add),
        wp::F32Sub => Binary(BinaryOp::F32Sub),
        wp::F32Mul => Binary(BinaryOp::F32Mul),
        wp::F32Div => Binary(BinaryOp::F32Div),
        wp::F32Min => Binary(BinaryOp::F32Min),
        wp::F32Max => Binary(BinaryOp::F32Max),
        wp::F32Copysign => Binary(BinaryOp::F32Copysign),
        wp::F64Add => Binary(BinaryOp::F64Add),
        wp::F64Sub => Binary(BinaryOp::F64Sub),
        wp::F64Mul => Binary(BinaryOp::F64Mul),
        wp::F64Div => Binary(BinaryOp::F64Div),
        wp::F64Min => Binary(BinaryOp::F64Min),
        wp::F64Max => Binary(BinaryOp::F64Max),
        wp::F64Copysign => Binary(BinaryOp::F64Copysign),

        wp::I32Extend8S => Unary(UnaryOp::I32Extend8S),
        wp::I32Extend16S => Unary(UnaryOp::I32Extend16S),
        wp::I64Extend8S => Unary(UnaryOp::I64Extend8S),
        wp::I64Extend16S => Unary(UnaryOp::I64Extend16S),
        wp::I64Extend32S => Unary(UnaryOp::I64Extend32S),

        wp::I32TruncSatF32S
        | wp::I32TruncSatF32U
        | wp::I32TruncSatF64S
        | wp::I32TruncSatF64U
        | wp::I64TruncSatF32S
        | wp::I64TruncSatF32U
        | wp::I64TruncSatF64S
        | wp::I64TruncSatF64U => Err(ParseIssue::unsupported(
            offset,
            WasmExtension::NontrappingFloatToInt,
        ))?,

        wp::MemoryInit {
            data_index: _,
            mem: _,
        }
        | wp::DataDrop { data_index: _ }
        | wp::MemoryCopy {
            dst_mem: _,
            src_mem: _,
        }
        | wp::MemoryFill { mem: _ }
        | wp::TableInit {
            elem_index: _,
            table: _,
        }
        | wp::ElemDrop { elem_index: _ }
        | wp::TableCopy {
            dst_table: _,
            src_table: _,
        } => Err(ParseIssue::unsupported(
            offset,
            WasmExtension::BulkMemoryOperations,
        ))?,

        wp::TableFill { table: _ } => Err(ParseIssue::unsupported(
            offset,
            WasmExtension::ReferenceTypes,
        ))?,

        wp::TableGet { table: _ }
        | wp::TableSet { table: _ }
        | wp::TableGrow { table: _ }
        | wp::TableSize { table: _ } => Err(ParseIssue::unsupported(
            offset,
            WasmExtension::ReferenceTypes,
        ))?,

        wp::MemoryAtomicNotify { memarg: _ }
        | wp::MemoryAtomicWait32 { memarg: _ }
        | wp::MemoryAtomicWait64 { memarg: _ }
        | wp::AtomicFence
        | wp::I32AtomicLoad { memarg: _ }
        | wp::I64AtomicLoad { memarg: _ }
        | wp::I32AtomicLoad8U { memarg: _ }
        | wp::I32AtomicLoad16U { memarg: _ }
        | wp::I64AtomicLoad8U { memarg: _ }
        | wp::I64AtomicLoad16U { memarg: _ }
        | wp::I64AtomicLoad32U { memarg: _ }
        | wp::I32AtomicStore { memarg: _ }
        | wp::I64AtomicStore { memarg: _ }
        | wp::I32AtomicStore8 { memarg: _ }
        | wp::I32AtomicStore16 { memarg: _ }
        | wp::I64AtomicStore8 { memarg: _ }
        | wp::I64AtomicStore16 { memarg: _ }
        | wp::I64AtomicStore32 { memarg: _ }
        | wp::I32AtomicRmwAdd { memarg: _ }
        | wp::I64AtomicRmwAdd { memarg: _ }
        | wp::I32AtomicRmw8AddU { memarg: _ }
        | wp::I32AtomicRmw16AddU { memarg: _ }
        | wp::I64AtomicRmw8AddU { memarg: _ }
        | wp::I64AtomicRmw16AddU { memarg: _ }
        | wp::I64AtomicRmw32AddU { memarg: _ }
        | wp::I32AtomicRmwSub { memarg: _ }
        | wp::I64AtomicRmwSub { memarg: _ }
        | wp::I32AtomicRmw8SubU { memarg: _ }
        | wp::I32AtomicRmw16SubU { memarg: _ }
        | wp::I64AtomicRmw8SubU { memarg: _ }
        | wp::I64AtomicRmw16SubU { memarg: _ }
        | wp::I64AtomicRmw32SubU { memarg: _ }
        | wp::I32AtomicRmwAnd { memarg: _ }
        | wp::I64AtomicRmwAnd { memarg: _ }
        | wp::I32AtomicRmw8AndU { memarg: _ }
        | wp::I32AtomicRmw16AndU { memarg: _ }
        | wp::I64AtomicRmw8AndU { memarg: _ }
        | wp::I64AtomicRmw16AndU { memarg: _ }
        | wp::I64AtomicRmw32AndU { memarg: _ }
        | wp::I32AtomicRmwOr { memarg: _ }
        | wp::I64AtomicRmwOr { memarg: _ }
        | wp::I32AtomicRmw8OrU { memarg: _ }
        | wp::I32AtomicRmw16OrU { memarg: _ }
        | wp::I64AtomicRmw8OrU { memarg: _ }
        | wp::I64AtomicRmw16OrU { memarg: _ }
        | wp::I64AtomicRmw32OrU { memarg: _ }
        | wp::I32AtomicRmwXor { memarg: _ }
        | wp::I64AtomicRmwXor { memarg: _ }
        | wp::I32AtomicRmw8XorU { memarg: _ }
        | wp::I32AtomicRmw16XorU { memarg: _ }
        | wp::I64AtomicRmw8XorU { memarg: _ }
        | wp::I64AtomicRmw16XorU { memarg: _ }
        | wp::I64AtomicRmw32XorU { memarg: _ }
        | wp::I32AtomicRmwXchg { memarg: _ }
        | wp::I64AtomicRmwXchg { memarg: _ }
        | wp::I32AtomicRmw8XchgU { memarg: _ }
        | wp::I32AtomicRmw16XchgU { memarg: _ }
        | wp::I64AtomicRmw8XchgU { memarg: _ }
        | wp::I64AtomicRmw16XchgU { memarg: _ }
        | wp::I64AtomicRmw32XchgU { memarg: _ }
        | wp::I32AtomicRmwCmpxchg { memarg: _ }
        | wp::I64AtomicRmwCmpxchg { memarg: _ }
        | wp::I32AtomicRmw8CmpxchgU { memarg: _ }
        | wp::I32AtomicRmw16CmpxchgU { memarg: _ }
        | wp::I64AtomicRmw8CmpxchgU { memarg: _ }
        | wp::I64AtomicRmw16CmpxchgU { memarg: _ }
        | wp::I64AtomicRmw32CmpxchgU { memarg: _ } => Err(ParseIssue::unsupported(
            offset,
            WasmExtension::ThreadsAtomics,
        ))?,

        wp::V128Load { memarg: _ }
        | wp::V128Load8x8S { memarg: _ }
        | wp::V128Load8x8U { memarg: _ }
        | wp::V128Load16x4S { memarg: _ }
        | wp::V128Load16x4U { memarg: _ }
        | wp::V128Load32x2S { memarg: _ }
        | wp::V128Load32x2U { memarg: _ }
        | wp::V128Load8Splat { memarg: _ }
        | wp::V128Load16Splat { memarg: _ }
        | wp::V128Load32Splat { memarg: _ }
        | wp::V128Load64Splat { memarg: _ }
        | wp::V128Load32Zero { memarg: _ }
        | wp::V128Load64Zero { memarg: _ }
        | wp::V128Store { memarg: _ }
        | wp::V128Load8Lane { memarg: _, lane: _ }
        | wp::V128Load16Lane { memarg: _, lane: _ }
        | wp::V128Load32Lane { memarg: _, lane: _ }
        | wp::V128Load64Lane { memarg: _, lane: _ }
        | wp::V128Store8Lane { memarg: _, lane: _ }
        | wp::V128Store16Lane { memarg: _, lane: _ }
        | wp::V128Store32Lane { memarg: _, lane: _ }
        | wp::V128Store64Lane { memarg: _, lane: _ }
        | wp::V128Const { value: _ }
        | wp::I8x16Shuffle { lanes: _ }
        | wp::I8x16ExtractLaneS { lane: _ }
        | wp::I8x16ExtractLaneU { lane: _ }
        | wp::I8x16ReplaceLane { lane: _ }
        | wp::I16x8ExtractLaneS { lane: _ }
        | wp::I16x8ExtractLaneU { lane: _ }
        | wp::I16x8ReplaceLane { lane: _ }
        | wp::I32x4ExtractLane { lane: _ }
        | wp::I32x4ReplaceLane { lane: _ }
        | wp::I64x2ExtractLane { lane: _ }
        | wp::I64x2ReplaceLane { lane: _ }
        | wp::F32x4ExtractLane { lane: _ }
        | wp::F32x4ReplaceLane { lane: _ }
        | wp::F64x2ExtractLane { lane: _ }
        | wp::F64x2ReplaceLane { lane: _ }
        | wp::I8x16Swizzle
        | wp::I8x16Splat
        | wp::I16x8Splat
        | wp::I32x4Splat
        | wp::I64x2Splat
        | wp::F32x4Splat
        | wp::F64x2Splat
        | wp::I8x16Eq
        | wp::I8x16Ne
        | wp::I8x16LtS
        | wp::I8x16LtU
        | wp::I8x16GtS
        | wp::I8x16GtU
        | wp::I8x16LeS
        | wp::I8x16LeU
        | wp::I8x16GeS
        | wp::I8x16GeU
        | wp::I16x8Eq
        | wp::I16x8Ne
        | wp::I16x8LtS
        | wp::I16x8LtU
        | wp::I16x8GtS
        | wp::I16x8GtU
        | wp::I16x8LeS
        | wp::I16x8LeU
        | wp::I16x8GeS
        | wp::I16x8GeU
        | wp::I32x4Eq
        | wp::I32x4Ne
        | wp::I32x4LtS
        | wp::I32x4LtU
        | wp::I32x4GtS
        | wp::I32x4GtU
        | wp::I32x4LeS
        | wp::I32x4LeU
        | wp::I32x4GeS
        | wp::I32x4GeU
        | wp::I64x2Eq
        | wp::I64x2Ne
        | wp::I64x2LtS
        | wp::I64x2GtS
        | wp::I64x2LeS
        | wp::I64x2GeS
        | wp::F32x4Eq
        | wp::F32x4Ne
        | wp::F32x4Lt
        | wp::F32x4Gt
        | wp::F32x4Le
        | wp::F32x4Ge
        | wp::F64x2Eq
        | wp::F64x2Ne
        | wp::F64x2Lt
        | wp::F64x2Gt
        | wp::F64x2Le
        | wp::F64x2Ge
        | wp::V128Not
        | wp::V128And
        | wp::V128AndNot
        | wp::V128Or
        | wp::V128Xor
        | wp::V128Bitselect
        | wp::V128AnyTrue
        | wp::I8x16Abs
        | wp::I8x16Neg
        | wp::I8x16Popcnt
        | wp::I8x16AllTrue
        | wp::I8x16Bitmask
        | wp::I8x16NarrowI16x8S
        | wp::I8x16NarrowI16x8U
        | wp::I8x16Shl
        | wp::I8x16ShrS
        | wp::I8x16ShrU
        | wp::I8x16Add
        | wp::I8x16AddSatS
        | wp::I8x16AddSatU
        | wp::I8x16Sub
        | wp::I8x16SubSatS
        | wp::I8x16SubSatU
        | wp::I8x16MinS
        | wp::I8x16MinU
        | wp::I8x16MaxS
        | wp::I8x16MaxU
        | wp::I8x16AvgrU
        | wp::I16x8ExtAddPairwiseI8x16S
        | wp::I16x8ExtAddPairwiseI8x16U
        | wp::I16x8Abs
        | wp::I16x8Neg
        | wp::I16x8Q15MulrSatS
        | wp::I16x8AllTrue
        | wp::I16x8Bitmask
        | wp::I16x8NarrowI32x4S
        | wp::I16x8NarrowI32x4U
        | wp::I16x8ExtendLowI8x16S
        | wp::I16x8ExtendHighI8x16S
        | wp::I16x8ExtendLowI8x16U
        | wp::I16x8ExtendHighI8x16U
        | wp::I16x8Shl
        | wp::I16x8ShrS
        | wp::I16x8ShrU
        | wp::I16x8Add
        | wp::I16x8AddSatS
        | wp::I16x8AddSatU
        | wp::I16x8Sub
        | wp::I16x8SubSatS
        | wp::I16x8SubSatU
        | wp::I16x8Mul
        | wp::I16x8MinS
        | wp::I16x8MinU
        | wp::I16x8MaxS
        | wp::I16x8MaxU
        | wp::I16x8AvgrU
        | wp::I16x8ExtMulLowI8x16S
        | wp::I16x8ExtMulHighI8x16S
        | wp::I16x8ExtMulLowI8x16U
        | wp::I16x8ExtMulHighI8x16U
        | wp::I32x4ExtAddPairwiseI16x8S
        | wp::I32x4ExtAddPairwiseI16x8U
        | wp::I32x4Abs
        | wp::I32x4Neg
        | wp::I32x4AllTrue
        | wp::I32x4Bitmask
        | wp::I32x4ExtendLowI16x8S
        | wp::I32x4ExtendHighI16x8S
        | wp::I32x4ExtendLowI16x8U
        | wp::I32x4ExtendHighI16x8U
        | wp::I32x4Shl
        | wp::I32x4ShrS
        | wp::I32x4ShrU
        | wp::I32x4Add
        | wp::I32x4Sub
        | wp::I32x4Mul
        | wp::I32x4MinS
        | wp::I32x4MinU
        | wp::I32x4MaxS
        | wp::I32x4MaxU
        | wp::I32x4DotI16x8S
        | wp::I32x4ExtMulLowI16x8S
        | wp::I32x4ExtMulHighI16x8S
        | wp::I32x4ExtMulLowI16x8U
        | wp::I32x4ExtMulHighI16x8U
        | wp::I64x2Abs
        | wp::I64x2Neg
        | wp::I64x2AllTrue
        | wp::I64x2Bitmask
        | wp::I64x2ExtendLowI32x4S
        | wp::I64x2ExtendHighI32x4S
        | wp::I64x2ExtendLowI32x4U
        | wp::I64x2ExtendHighI32x4U
        | wp::I64x2Shl
        | wp::I64x2ShrS
        | wp::I64x2ShrU
        | wp::I64x2Add
        | wp::I64x2Sub
        | wp::I64x2Mul
        | wp::I64x2ExtMulLowI32x4S
        | wp::I64x2ExtMulHighI32x4S
        | wp::I64x2ExtMulLowI32x4U
        | wp::I64x2ExtMulHighI32x4U
        | wp::F32x4Ceil
        | wp::F32x4Floor
        | wp::F32x4Trunc
        | wp::F32x4Nearest
        | wp::F32x4Abs
        | wp::F32x4Neg
        | wp::F32x4Sqrt
        | wp::F32x4Add
        | wp::F32x4Sub
        | wp::F32x4Mul
        | wp::F32x4Div
        | wp::F32x4Min
        | wp::F32x4Max
        | wp::F32x4PMin
        | wp::F32x4PMax
        | wp::F64x2Ceil
        | wp::F64x2Floor
        | wp::F64x2Trunc
        | wp::F64x2Nearest
        | wp::F64x2Abs
        | wp::F64x2Neg
        | wp::F64x2Sqrt
        | wp::F64x2Add
        | wp::F64x2Sub
        | wp::F64x2Mul
        | wp::F64x2Div
        | wp::F64x2Min
        | wp::F64x2Max
        | wp::F64x2PMin
        | wp::F64x2PMax
        | wp::I32x4TruncSatF32x4S
        | wp::I32x4TruncSatF32x4U
        | wp::F32x4ConvertI32x4S
        | wp::F32x4ConvertI32x4U
        | wp::I32x4TruncSatF64x2SZero
        | wp::I32x4TruncSatF64x2UZero
        | wp::F64x2ConvertLowI32x4S
        | wp::F64x2ConvertLowI32x4U
        | wp::F32x4DemoteF64x2Zero
        | wp::F64x2PromoteLowF32x4 => Err(ParseIssue::unsupported(offset, WasmExtension::Simd))?,

        wp::I8x16RelaxedSwizzle
        | wp::I32x4RelaxedTruncSatF32x4S
        | wp::I32x4RelaxedTruncSatF32x4U
        | wp::I32x4RelaxedTruncSatF64x2SZero
        | wp::I32x4RelaxedTruncSatF64x2UZero
        | wp::F32x4RelaxedFma
        | wp::F32x4RelaxedFnma
        | wp::F64x2RelaxedFma
        | wp::F64x2RelaxedFnma
        | wp::I8x16RelaxedLaneselect
        | wp::I16x8RelaxedLaneselect
        | wp::I32x4RelaxedLaneselect
        | wp::I64x2RelaxedLaneselect
        | wp::F32x4RelaxedMin
        | wp::F32x4RelaxedMax
        | wp::F64x2RelaxedMin
        | wp::F64x2RelaxedMax
        | wp::I16x8RelaxedQ15mulrS
        | wp::I16x8DotI8x16I7x16S
        | wp::I32x4DotI8x16I7x16AddS
        | wp::F32x4RelaxedDotBf16x8AddF32x4 => {
            Err(ParseIssue::unsupported(offset, WasmExtension::RelaxedSimd))?
        }
    })
}

fn parse_memarg(memarg: wp::MemArg, parser_offset: usize) -> Result<Memarg, ParseError> {
    if memarg.memory != 0 {
        Err(ParseIssue::unsupported(
            parser_offset,
            WasmExtension::MultiMemory,
        ))?
    }
    let offset: u32 = memarg
        .offset
        .try_into()
        .map_err(|_| ParseIssue::unsupported(parser_offset, WasmExtension::Memory64))?;
    Ok(Memarg {
        alignment_exp: memarg.align,
        offset,
    })
}

fn parse_memory_ty(ty: wp::MemoryType, offset: usize) -> Result<Limits, ParseError> {
    if ty.memory64 {
        Err(ParseIssue::unsupported(offset, WasmExtension::Memory64))?
    }
    if ty.shared {
        Err(ParseIssue::unsupported(
            offset,
            WasmExtension::ThreadsAtomics,
        ))?
    }
    Ok(Limits {
        initial_size: ty
            .initial
            .try_into()
            .expect("guaranteed u32 by wasmparser if !memory64"),
        max_size: ty.maximum.map(|u| {
            u.try_into()
                .expect("guaranteed u32 by wasmparser if !memory64")
        }),
    })
}

fn parse_table_ty(ty: wp::TableType, offset: usize) -> Result<Limits, ParseError> {
    parse_elem_ty(ty.element_type, offset)?;
    Ok(Limits {
        initial_size: ty.initial,
        max_size: ty.maximum,
    })
}

fn parse_elem_ty(ty: wp::ValType, offset: usize) -> Result<(), ParseError> {
    use wp::ValType::*;
    match ty {
        I32 | I64 | F32 | F64 => Err(ParseIssue::message(
            offset,
            "only reftypes, not value types are allowed as table elements",
            None,
        ))?,
        V128 => Err(ParseIssue::message(
            offset,
            "only reftypes, not value types are allowed as table elements",
            None,
        ))?,
        FuncRef => Ok(()),
        ExternRef => Err(ParseIssue::unsupported(
            offset,
            WasmExtension::ReferenceTypes,
        ))?,
    }
}

fn parse_block_ty(
    ty: wp::BlockType,
    offset: usize,
    types: &Types,
    metadata: &RwLock<ModuleMetadata>,
) -> Result<FunctionType, ParseError> {
    use wp::BlockType::*;
    match ty {
        Empty => Ok(FunctionType::empty()),
        Type(ty) => Ok(FunctionType::new(&[], &[parse_val_ty(ty, offset)?])),
        FuncType(type_idx) => {
            metadata
                .write()
                .unwrap()
                .add_used_extension(WasmExtension::MultiValue);
            types.get(type_idx, offset)
        }
    }
}

fn parse_func_ty(ty: wp::FuncType, offset: usize) -> Result<FunctionType, ParseError> {
    let convert_tys = |tys: &[wp::ValType]| -> Result<_, ParseError> {
        let mut smallvec: SmallVec<[ValType; 8]> = SmallVec::new();
        for ty in tys {
            // The `offset` for error reporting is not exactly correct, because it marks the start
            // of the function type, not the individual wrong parameter/result type.
            smallvec.push(parse_val_ty(*ty, offset)?);
        }
        Ok(smallvec)
    };

    Ok(FunctionType::new(
        &convert_tys(ty.params())?,
        &convert_tys(ty.results())?,
    ))
}

fn parse_global_ty(ty: wp::GlobalType, offset: usize) -> Result<GlobalType, ParseError> {
    Ok(GlobalType(
        parse_val_ty(ty.content_type, offset)?,
        if ty.mutable {
            Mutability::Mut
        } else {
            Mutability::Const
        },
    ))
}

fn parse_val_ty(ty: wp::ValType, offset: usize) -> Result<ValType, ParseError> {
    match ty {
        wp::ValType::I32 => Ok(ValType::I32),
        wp::ValType::I64 => Ok(ValType::I64),
        wp::ValType::F32 => Ok(ValType::F32),
        wp::ValType::F64 => Ok(ValType::F64),
        wp::ValType::V128 => Err(ParseIssue::unsupported(offset, WasmExtension::Simd))?,
        wp::ValType::FuncRef => Err(ParseIssue::unsupported(
            offset,
            WasmExtension::ReferenceTypes,
        ))?,
        wp::ValType::ExternRef => Err(ParseIssue::unsupported(
            offset,
            WasmExtension::ReferenceTypes,
        ))?,
    }
}

// The difference between `warnings` and returning a `Err(ParseIssue)` is that the latter will abort
// further parsing of the name section.
fn parse_name_custom_section(
    data: &[u8],
    data_offset: usize,
    warnings: &mut Vec<ParseIssue>,
    module: &mut Module,
) -> Result<(), ParseIssue> {
    for name_subsection in wp::NameSectionReader::new(data, data_offset) {
        use wp::Name;
        match name_subsection? {
            Name::Module { name, name_range } => {
                let prev = module.name.replace(name.to_string());
                if prev.is_some() {
                    warnings.push(ParseIssue::message(
                        name_range.start,
                        "name section: duplicate module name",
                        None,
                    ))
                }
            }
            Name::Function(name_map) => {
                for elem in name_map.into_iter_with_offsets() {
                    let (
                        offset,
                        wp::Naming {
                            index: function_index,
                            name,
                        },
                    ) = elem?;
                    module
                        .functions
                        .get_mut(u32_to_usize(function_index))
                        .ok_or_else(|| ParseIssue::index(offset, function_index, "function"))?
                        .name = Some(name.to_string());
                }
            }
            Name::Local(indirect_name_map) => {
                for elem in indirect_name_map.into_iter_with_offsets() {
                    let (
                        offset,
                        wp::IndirectNaming {
                            index: function_index,
                            names,
                        },
                    ) = elem?;
                    let function = module
                        .functions
                        .get_mut(u32_to_usize(function_index))
                        .ok_or_else(|| ParseIssue::index(offset, function_index, "function"))?;

                    for elem in names.into_iter_with_offsets() {
                        let (
                            offset,
                            wp::Naming {
                                index: local_index,
                                name,
                            },
                        ) = elem?;

                        // TODO Refactor param_or_local_name to return a `Result`
                        // instead of checking the index beforehand.
                        if local_index as usize >= (function.param_count() + function.local_count())
                        {
                            warnings.push(ParseIssue::index(offset, local_index, "local"));
                        } else {
                            *function.param_or_local_name_mut(local_index.into()) =
                                Some(name.to_string());
                        }
                    }
                }
            }
            Name::Label(name_map) => warnings.push(ParseIssue::unsupported(
                name_map.range().start,
                WasmExtension::ExtendedNameSection,
            )),
            Name::Type(name_map)
            | Name::Table(name_map)
            | Name::Memory(name_map)
            | Name::Global(name_map)
            | Name::Element(name_map)
            | Name::Data(name_map) => warnings.push(ParseIssue::unsupported(
                name_map.range().start,
                WasmExtension::ExtendedNameSection,
            )),
            Name::Unknown {
                ty: _,
                data: _,
                range,
            } => warnings.push(ParseIssue::message(
                range.start,
                "name section: unknown name subsection",
                None,
            )),
        }
    }

    Ok(())
}

// Wrapper for type map, to offer some convenience like:
// - u32 indices (which we get from wasmparser) instead of usize (which Vec expects)
// - checking that type section exists only a single time and type index is valid
struct Types(Option<Vec<FunctionType>>);

impl Types {
    /// Initial state, where the type section has not been parsed yet.
    pub fn none() -> Self {
        Types(None)
    }

    /// Next state, where the number of type entries is known, but nothing filled yet.
    pub fn new_type_section(
        &mut self,
        count: u32,
        type_section_offset: usize,
    ) -> Result<(), ParseError> {
        let prev_state = self.0.replace(Vec::with_capacity(u32_to_usize(count)));
        match prev_state {
            Some(_) => Err(ParseIssue::message(
                type_section_offset,
                "duplicate type section",
                None,
            ))?,
            None => Ok(()),
        }
    }

    pub fn add(&mut self, ty: FunctionType) {
        self.0
            .as_mut()
            .expect("type section should be present, we are in the process of parsing it")
            .push(ty);
    }

    pub fn get(&self, index: u32, index_offset: usize) -> Result<FunctionType, ParseError> {
        Ok(self
            .0
            .as_deref()
            // No type section == empty type vector.
            .unwrap_or(&[])
            .get(u32_to_usize(index))
            .cloned()
            .ok_or_else(|| ParseIssue::index(index_offset, index, "type"))?)
    }
}

fn u32_to_usize(u: u32) -> usize {
    u.try_into().expect("u32 to usize should always succeed")
}
