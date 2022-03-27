use std::convert::TryInto;

mod error {
    use std::io;

    use crate::extensions::WasmExtension;

    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct ParseError(
        // Put the actual error behind a box, to keep the size down to a single pointer.
        Box<ParseIssue>
    );

    /// Used both for warnings (recoverable, i.e., parsing can continue afterwards) and errors
    /// (not recoverable, i.e., parsing stops and does not return an AST).
    #[derive(Debug, thiserror::Error)]
    pub enum ParseIssue {
        #[error("parse error at offset 0x{:x}: {}", .0.offset(), .0.message())]
        Wasmparser(#[from] wasmparser::BinaryReaderError),

        #[error("parse error at offset 0x{:x}: {}", offset, message)]
        Message {
            offset: usize,
            message: &'static str
        },

        #[error("index out of bounds at offset 0x{:x}: invalid {} index {}", offset, index_space, index)]
        Index {
            offset: usize,
            index: u32,
            index_space: &'static str,
        },

        #[error("unsupported WebAssembly extension at offset 0x{:x}: {} (see also the repository at {})", offset, extension.name(), extension.url())]
        Unsupported {
            offset: usize,
            extension: WasmExtension
        },

        #[error(transparent)]
        Io(#[from] io::Error)
    }

    // Allow conversion of everything that can be converted into a `ParseIssue` also into the
    // `ParseError` wrapper directly.
    impl<T> From<T> for ParseError 
    where T : Into<ParseIssue>
    {
        fn from(err: T) -> Self {
            ParseError(Box::new(err.into()))
        }
    }

    // Convenience constructors.
    impl ParseIssue {
        pub fn message(offset: usize, message: &'static str) -> Self {
            ParseIssue::Message { offset, message }
        }

        pub fn index(offset: usize, index: u32, index_space: &'static str) -> Self {
            ParseIssue::Index { offset, index, index_space }
        }

        pub fn unsupported(offset: usize, extension: WasmExtension) -> Self {
            ParseIssue::Unsupported { offset, extension }
        }
    }
}

pub mod parser {

    use std::convert::TryInto;

    use ordered_float::OrderedFloat;
    use rayon::prelude::*;
    
    use wasmparser::{
        ImportSectionEntryType, NameSectionReader, Naming, Parser, Payload, SectionReader, TypeDef,
    };

    use crate::highlevel::{
        Code, Data, Element, Function, Global, GlobalOp, ImportOrPresent, Instr, LoadOp, Local,
        LocalOp, Memory, Module, NumericOp, StoreOp, Table,
    };
    use crate::lowlevel::{CustomSection, NameSection, Offsets, Section, SectionOffset, WithSize};
    use crate::{
        BlockType, ElemType, FunctionType, GlobalType, Label, Limits, Memarg, MemoryType,
        Mutability, RawCustomSection, TableType, Val, ValType,
    };

    use crate::extensions::WasmExtension;
    
    use super::error::{ParseError, ParseIssue};
    
    use super::u32_to_usize;

    // The streaming API of wasmparser is a bit cumbersome, so implement reading 
    // from bytes fully resident in memory first. 
    // TODO Add a second API from streaming sources, i.e., `io::Read` like here:
    // https://docs.rs/wasmparser/latest/wasmparser/struct.Parser.html#examples
    pub fn parse_module_with_offsets(bytes: &[u8]) -> Result<(Module, Offsets, Vec<ParseIssue>), ParseError> {
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

        for payload in Parser::new(0).parse_all(&bytes) {
            match payload? {
                Payload::Version { .. } => {
                    // The version number is checked by wasmparser to always be 1.
                }
                Payload::TypeSection(mut reader) => {
                    // TODO Index the section offsets not by the section's discriminant, but by
                    // a new enum `SectionId`, which is just the section name for "normal" sections,
                    // and CustomSection(name: String) for custom sections (whose name should be unique).
                    let discriminant = std::mem::discriminant(&Section::Type(Default::default()));
                    // This is the offset AFTER the section tag and size in bytes,
                    // but BEFORE the number of elements in the section.
                    let offset = reader.range().start;
                    section_offsets.push((discriminant, offset));

                    types.new_type_section(reader.get_count(), offset)?;

                    let mut type_offset = reader.original_position();
                    for _ in 0..reader.get_count() {
                        let type_ = reader.read()?;
                        match type_ {
                            TypeDef::Func(ty) => {
                                let ty = parse_func_ty(ty, type_offset)?;
                                types.add(ty)
                            },
                            TypeDef::Instance(_) | TypeDef::Module(_) => {
                                Err(ParseIssue::unsupported(type_offset, WasmExtension::ModuleLinking))?
                            }
                        }

                        type_offset = reader.original_position();
                    }
                }
                Payload::ImportSection(mut reader) => {
                    // FIXME section order without discriminant
                    let discriminant = std::mem::discriminant(&Section::Import(Default::default()));
                    section_offsets.push((discriminant, reader.range().start));

                    let mut import_offset = reader.original_position();
                    for _ in 0..reader.get_count() {
                        let import = reader.read()?;

                        let import_module = import.module.to_string();
                        let import_name = import
                            .field
                            // The second import string was made optional in this extension, but we don't support it.
                            .ok_or(ParseIssue::unsupported(import_offset, WasmExtension::ModuleLinking))?
                            .to_string();

                        match import.ty {
                            ImportSectionEntryType::Function(ty_index) => {
                                imported_function_count += 1;
                                module.functions.push(Function::new_imported(
                                    // The `import_offset` is not actually the offset of the type index,
                                    // but wasmparser doesn't offer a way to get the latter.
                                    // This slightly misattributes potential errors, namely to the beginning of the import.
                                    types.get(ty_index, import_offset)?,
                                    import_module,
                                    import_name,
                                ))
                            }
                            ImportSectionEntryType::Global(ty) => module.globals.push(
                                // Same issue regarding `import_offset`.
                                Global::new_imported(parse_global_ty(ty, import_offset)?, import_module, import_name),
                            ),
                            ImportSectionEntryType::Table(ty) => module.tables.push(
                                // Same issue regarding `import_offset`.
                                Table::new_imported(parse_table_ty(ty, import_offset)?, import_module, import_name),
                            ),
                            ImportSectionEntryType::Memory(ty) => {
                                // Same issue regarding `import_offset`.
                                module.memories.push(Memory::new_imported(
                                    parse_memory_ty(ty, import_offset)?,
                                    import_module,
                                    import_name,
                                ))
                            }
                            ImportSectionEntryType::Tag(_) => {
                                // Same issue regarding `import_offset`.
                                Err(ParseIssue::unsupported(import_offset, WasmExtension::ExceptionHandling))?
                            }
                            ImportSectionEntryType::Module(_) | ImportSectionEntryType::Instance(_) => {
                                // Same issue regarding `import_offset`.
                                Err(ParseIssue::unsupported(import_offset, WasmExtension::ModuleLinking))?
                            }
                        }

                        import_offset = reader.original_position();
                    }
                }
                Payload::AliasSection(reader) => Err(ParseIssue::unsupported(reader.range().start, WasmExtension::ModuleLinking))?,
                Payload::InstanceSection(reader) => Err(ParseIssue::unsupported(reader.range().start, WasmExtension::ModuleLinking))?,
                Payload::FunctionSection(mut reader) => {
                    // FIXME section order without discriminant
                    let discriminant = std::mem::discriminant(&Section::Function(Default::default()));
                    section_offsets.push((discriminant, reader.range().start));

                    let count = reader.get_count();
                    module.functions.reserve(u32_to_usize(count));

                    let mut offset = reader.original_position();
                    for _ in 0..reader.get_count() {
                        let type_index = reader.read()?;
                        let type_ = types.get(type_index, offset)?;
                        // Fill in the code of the function later with the code section.
                        module.functions.push(Function::new(type_, Code::new()));

                        offset = reader.original_position();
                    }
                }
                Payload::TableSection(mut reader) => {
                    // FIXME section order without discriminant
                    let discriminant = std::mem::discriminant(&Section::Table(Default::default()));
                    section_offsets.push((discriminant, reader.range().start));

                    let count = reader.get_count();
                    module.tables.reserve(u32_to_usize(count));

                    let mut offset = reader.original_position();
                    for _ in 0..reader.get_count() {
                        let table_ty = reader.read()?;
                        let table_ty = parse_table_ty(table_ty, offset)?;
                        // Fill in the elements of the table later with the element section.
                        module.tables.push(Table::new(table_ty));

                        offset = reader.original_position();
                    }
                }
                Payload::MemorySection(mut reader) => {
                    // FIXME section order without discriminant
                    let discriminant = std::mem::discriminant(&Section::Memory(Default::default()));
                    section_offsets.push((discriminant, reader.range().start));

                    let count = reader.get_count();
                    module.memories.reserve(u32_to_usize(count));

                    let mut offset = reader.original_position();
                    for _ in 0..reader.get_count() {
                        let memory_ty = reader.read()?;
                        let memory_ty = parse_memory_ty(memory_ty, offset)?;
                        // Fill in the data of the memory later with the data section.
                        module.memories.push(Memory::new(memory_ty));

                        offset = reader.original_position();
                    }
                }
                Payload::TagSection(reader) => Err(ParseIssue::unsupported(reader.range().start, WasmExtension::ExceptionHandling))?,
                Payload::GlobalSection(mut reader) => {
                    // FIXME section order without discriminant
                    let discriminant = std::mem::discriminant(&Section::Global(Default::default()));
                    section_offsets.push((discriminant, reader.range().start));

                    let count = reader.get_count();
                    module.globals.reserve(u32_to_usize(count));

                    let mut offset = reader.original_position();
                    for _ in 0..reader.get_count() {
                        let global = reader.read()?;
                        let type_ = parse_global_ty(global.ty, offset)?;

                        // Most initialization expressions have just a constant and the end instruction.
                        let mut init = Vec::with_capacity(2);
                        for op in global.init_expr.get_operators_reader() {
                            // The `offset` will be slightly off, because it points to the beginning of the
                            // whole global entry, not the initialization expression.
                            init.push(parse_instr(op?, &types, offset)?)
                        }

                        module.globals.push(Global::new(type_, init));

                        offset = reader.original_position();
                    }
                }
                Payload::ExportSection(mut reader) => {
                    // FIXME section order without discriminant
                    let discriminant = std::mem::discriminant(&Section::Export(Default::default()));
                    section_offsets.push((discriminant, reader.range().start));

                    let mut export_offset = reader.original_position();
                    for _ in 0..reader.get_count() {
                        let export = reader.read()?;

                        let name = export.field.to_string();
                        let index_u32 = export.index;
                        let index = u32_to_usize(export.index);
                        
                        use wasmparser::ExternalKind;
                        match export.kind {
                            ExternalKind::Function => module
                                .functions
                                .get_mut(index)
                                // The `export_offset` is not actually the offset of the function index,
                                // but wasmparser doesn't offer a way to get the latter.
                                // This slightly misattributes potential errors, namely to the beginning of the export.
                                .ok_or(ParseIssue::index(export_offset, index_u32, "function"))?
                                .export
                                .push(name),
                            ExternalKind::Table => module
                                .tables
                                .get_mut(index)
                                // Same issue regarding `export_offset`.
                                .ok_or(ParseIssue::index(export_offset, index_u32, "table"))?
                                .export
                                .push(name),
                            ExternalKind::Memory => module
                                .memories
                                .get_mut(index)
                                // Same issue regarding `export_offset`.
                                .ok_or(ParseIssue::index(export_offset, index_u32, "memory"))?
                                .export
                                .push(name),
                            ExternalKind::Global => module
                                .globals
                                .get_mut(index)
                                // Same issue regarding `export_offset`.
                                .ok_or(ParseIssue::index(export_offset, index_u32, "global"))?
                                .export
                                .push(name),
                            ExternalKind::Tag => {
                                // Same issue regarding `export_offset`.
                                Err(ParseIssue::unsupported(export_offset, WasmExtension::ExceptionHandling))?
                            }
                            ExternalKind::Type => {
                                // Same issue regarding `export_offset`.
                                Err(ParseIssue::unsupported(export_offset, WasmExtension::TypeImports))?
                            },
                            ExternalKind::Module | ExternalKind::Instance => {
                                // Same issue regarding `export_offset`.
                                Err(ParseIssue::unsupported(export_offset, WasmExtension::ModuleLinking))?
                            }
                        };

                        export_offset = reader.original_position();
                    }
                }
                Payload::StartSection { func, range } => {
                    // FIXME section order without discriminant
                    let discriminant =
                        std::mem::discriminant(&Section::Start(WithSize(SectionOffset(0u32.into()))));
                    section_offsets.push((discriminant, range.start));

                    let prev_start = std::mem::replace(&mut module.start, Some(func.into()));
                    if let Some(_) = prev_start {
                        Err(ParseIssue::message(range.start, "duplicate start section"))?
                    }
                }
                Payload::ElementSection(mut reader) => {
                    // FIXME section order without discriminant
                    let discriminant = std::mem::discriminant(&Section::Element(Default::default()));
                    section_offsets.push((discriminant, reader.range().start));

                    let mut element_offset = reader.original_position();
                    for _ in 0..reader.get_count() {
                        let element = reader.read()?;
                        let element_ty = parse_elem_ty(element.ty, element_offset)?;

                        let mut items_reader = element.items.get_items_reader()?;
                        let items_count = u32_to_usize(items_reader.get_count());
                        let mut items = Vec::with_capacity(items_count);

                        let mut item_offset = items_reader.original_position();
                        for _ in 0..items_count {
                            let item = items_reader.read()?;
                            use wasmparser::ElementItem;
                            items.push(match item {
                                ElementItem::Func(index) => index.into(),
                                ElementItem::Expr(_) => Err(ParseIssue::unsupported(item_offset, WasmExtension::ReferenceTypes))?,
                            });

                            item_offset = items_reader.original_position();
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
                                    .ok_or(ParseIssue::index(element_offset, table_index, "table"))?;

                                if table.type_.0 != element_ty {
                                    Err(ParseIssue::message(element_offset, "table and element type do not match"))?
                                }

                                // Most offset expressions are just a constant and the end instruction.
                                let mut offset_expr = Vec::with_capacity(2);
                                for op_offset in init_expr.get_operators_reader().into_iter_with_offsets() {
                                    let (op, offset) = op_offset?;
                                    offset_expr.push(parse_instr(op, &types, offset)?)
                                }

                                table.elements.push(Element {
                                    offset: offset_expr,
                                    functions: items,
                                })
                            }
                            ElementKind::Passive => {
                                Err(ParseIssue::unsupported(element_offset, WasmExtension::BulkMemoryOperations))?
                            }
                            ElementKind::Declared => {
                                Err(ParseIssue::unsupported(element_offset, WasmExtension::ReferenceTypes))?
                            }
                        }

                        element_offset = reader.original_position();
                    }
                }
                Payload::DataCountSection { count: _, range } => {
                    Err(ParseIssue::unsupported(range.start, WasmExtension::BulkMemoryOperations))?
                }
                Payload::DataSection(mut reader) => {
                    // FIXME section order without discriminant
                    let discriminant = std::mem::discriminant(&Section::Data(Default::default()));
                    section_offsets.push((discriminant, reader.range().start));

                    let mut data_offset = reader.original_position();
                    for _ in 0..reader.get_count() {
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
                                    .ok_or(ParseIssue::index(data_offset, memory_index, "memory"))?;

                                // Most offset expressions are just a constant and the end instruction.
                                let mut offset_expr = Vec::with_capacity(2);
                                for op_offset in init_expr.get_operators_reader().into_iter_with_offsets() {
                                    let (op, offset) = op_offset?;
                                    offset_expr.push(parse_instr(op, &types, offset)?)
                                }

                                memory.data.push(Data {
                                    offset: offset_expr,
                                    bytes: data.data.to_vec(),
                                })
                            }
                            DataKind::Passive => {
                                Err(ParseIssue::unsupported(data_offset, WasmExtension::BulkMemoryOperations))?
                            }
                        }

                        data_offset = reader.original_position();
                    }
                }
                Payload::CustomSection {
                    name: "name",
                    data_offset,
                    data,
                    range,
                } => {
                    // If parts of the name section cannot be parsed, collect the issue as a warning and abort parsing the
                    // name section, but produce an AST for the rest of the module.
                    // To make it possible to use the `?` operator, wrap this into a closure.
                    let mut name_parsing = || -> Result<(), ParseIssue> {
                        let mut reader = NameSectionReader::new(data, data_offset)?;
                        while !reader.eof() {
                            let offset = reader.original_position();
                            let name_subsection = reader.read()?;
                            use wasmparser::Name;
                            match name_subsection {
                                Name::Module(name) => {
                                    let prev = module.name.replace(name.get_name()?.to_string());
                                    if let Some(_) = prev {
                                        warnings.push(ParseIssue::message(offset, "duplicate module name"))
                                    }
                                }
                                Name::Function(name_map) => {
                                    let mut name_map = name_map.get_map()?;
                                    for _ in 0..name_map.get_count() {
                                        let offset = name_map.original_position();
        
                                        let Naming { index: function_index, name } = name_map.read()?;
                                        module
                                            .functions
                                            .get_mut(u32_to_usize(function_index))
                                            .ok_or(ParseIssue::index(offset, function_index, "function"))?
                                            .name = Some(name.to_string());
                                    }
                                }
                                Name::Local(indirect_name_map) => {
                                    let mut indirect_name_map = indirect_name_map.get_indirect_map()?;
                                    for _ in 0..indirect_name_map.get_indirect_count() {
                                        let offset = indirect_name_map.original_position();
                                        
                                        let indirect_naming = indirect_name_map.read()?;
                                        let function_index = indirect_naming.indirect_index;
                                        let function = module
                                            .functions
                                            .get_mut(u32_to_usize(function_index))
                                            .ok_or(ParseIssue::index(offset, function_index, "function"))?;
        
                                        let mut name_map = indirect_naming.get_map()?;
                                        for _ in 0..name_map.get_count() {
                                            // FIXME param_or_local_name_mut might panic due to index error
                                            // TODO refactor param_or_local_name
                                            // let offset = name_map.original_position();
        
                                            let Naming {
                                                index: local_index,
                                                name,
                                            } = name_map.read()?;
                                            *function.param_or_local_name_mut(local_index.into()) =
                                                Some(name.to_string());
                                        }
                                    }
                                }
                                Name::Label(_)
                                | Name::Type(_)
                                | Name::Table(_)
                                | Name::Memory(_)
                                | Name::Global(_)
                                | Name::Element(_)
                                | Name::Data(_) => {
                                    warnings.push(ParseIssue::unsupported(offset, WasmExtension::ExtendedNameSection))
                                }
                                | Name::Unknown {
                                    ty: _,
                                    data: _,
                                    range: _,
                                } => warnings.push(ParseIssue::message(offset, "unknown name subsection")),
                            }
                        }

                        Ok(())
                    };

                    // FIXME section order without discriminant
                    let discriminant = if let Err(name_parsing_aborted) = name_parsing() {
                        // Add the warning that stopped parsing the name section as the final warning.
                        warnings.push(name_parsing_aborted);

                        // Add (unsuccessfully parsed) name section as raw custom section instead.
                        let raw_custom_section = RawCustomSection {
                            name: "name".to_string(),
                            content: data.to_vec(),
                            after: section_offsets
                                .last()
                                .map(|(section, _offset)| section)
                                .cloned(),
                        };
                        let discriminant = std::mem::discriminant(&Section::Custom(CustomSection::Raw(RawCustomSection { name: "".to_string(), content: vec![], after: None})));
                        module.custom_sections.push(raw_custom_section);
                        discriminant
                    } else {
                        std::mem::discriminant(&Section::Custom(CustomSection::Name(NameSection {
                            subsections: Vec::new(),
                        })))
                    };

                    // FIXME section order without discriminant
                    section_offsets.push((discriminant, range.start));
                }
                Payload::CustomSection {
                    name,
                    data_offset: _,
                    data,
                    range,
                } => {
                    let raw_custom_section = RawCustomSection {
                        name: name.to_string(),
                        content: data.to_vec(),
                        after: section_offsets
                            .last()
                            .map(|(section, _offset)| section)
                            .cloned(),
                    };

                    // FIXME section order without discriminant
                    let discriminant = std::mem::discriminant(&Section::Custom(CustomSection::Raw(
                        RawCustomSection {
                            name: "".into(),
                            content: Vec::new(),
                            after: None,
                        },
                    )));
                    section_offsets.push((discriminant, range.start));

                    module.custom_sections.push(raw_custom_section);
                }
                Payload::CodeSectionStart {
                    count,
                    range,
                    size: _,
                } => {
                    // FIXME section order without discriminant
                    let discriminant = std::mem::discriminant(&Section::Code(Default::default()));
                    section_offsets.push((discriminant, range.start));

                    function_offsets = Vec::with_capacity(u32_to_usize(count));

                    code_entries_count = count;
                    function_bodies = Vec::with_capacity(u32_to_usize(count));
                }
                Payload::CodeSectionEntry(body) => {
                    let func_index = imported_function_count + current_code_index;

                    function_offsets.push((func_index.into(), body.range().start));

                    function_bodies.push((func_index, body));

                    current_code_index += 1;

                    let last_code_entry = current_code_index == code_entries_count;
                    if last_code_entry {
                        // Unfortunately, this parallel decoding of function bodies is horrendously slow on Windows 10
                        // with a Ryzen 5950X, i.e., with two threads it's already 30% slower than single threaded (!)
                        // and with 32 threads it is 2.7x (!) the runtime of single threaded. There is some contention
                        // going on, but it's not yet clear why/what:
                        // a) On the very same machine, under Ubuntu in a VM, it runs faster multi-threaded -> some OS specific
                        // issue, e.g., memory allocator, thread creation etc? Note that most of the time in the parallel
                        // version is spent in ntoskrnl.exe under Windows.
                        // b) False sharing, because the Ryzen 5950X has larger L2/L3 cache lines than e.g., my laptop
                        // Intel Core i7-7500U chip, where there is no slowdown!? I tried copying the function body
                        // bytes and the iterators to have share-nothing, but that improved for the Ryzen only from 2.7x to
                        // roughly 2.4x slowdown.
                        // Since it works fine under Linux, I'll leave it like it is right now.

                        // Parse and convert to high-level instructions in parallel.
                        let function_bodies: Vec<_> = function_bodies
                            .par_drain(..)
                            .map(|(i, body)| {
                                (i, body.range().start, parse_body(body, &types))
                            })
                            .collect();
                        // Attach the converted function bodies to the function definitions (not parallel).
                        for (func_index, offset, code) in function_bodies {
                            let function = module
                                .functions
                                .get_mut(u32_to_usize(func_index))
                                .ok_or(ParseIssue::index(offset, func_index, "function"))?;
                            function.code = ImportOrPresent::Present(code?);
                        }
                    }
                }
                Payload::ModuleSectionStart {
                    count: _,
                    range,
                    size: _,
                } => Err(ParseIssue::unsupported(range.start, WasmExtension::ModuleLinking))?,
                Payload::ModuleSectionEntry {
                    parser: _,
                    range,
                } => Err(ParseIssue::unsupported(range.start, WasmExtension::ModuleLinking))?,
                Payload::UnknownSection {
                    id: _,
                    contents: _,
                    range,
                } => Err(ParseIssue::message(range.start, "unknown section"))?,
                Payload::End => {
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

        Ok((module, offsets, warnings))
    }

    fn parse_body(body: wasmparser::FunctionBody, types: &Types) -> Result<Code, ParseError> {
        let mut locals = Vec::new();
        let mut locals_reader = body.get_locals_reader()?;
        let mut offset = locals_reader.original_position();
        for _ in 0..locals_reader.get_count() {
            let (count, type_) = locals_reader.read()?;
            for _ in 0..count {
                locals.push(Local::new(parse_val_ty(type_, offset)?));
            }
            offset = locals_reader.original_position();
        }

        // There is roughly one instruction per byte, so reserve space for
        // approximately this many instructions.
        let body_byte_size = body.range().end - body.range().start;
        let mut instrs = Vec::with_capacity(body_byte_size);

        for op_offset in body.get_operators_reader()?.into_iter_with_offsets() {
            let (op, offset) = op_offset?;
            instrs.push(parse_instr(op, &types, offset)?);
        }

        Ok(Code {
            locals,
            body: instrs,
        })
    }

    fn parse_instr(
        op: wasmparser::Operator,
        types: &Types,
        offset: usize,
    ) -> Result<Instr, ParseError> {
        use crate::highlevel::Instr::*;
        use wasmparser::Operator as wp;
        Ok(match op {
            wp::Unreachable => Unreachable,
            wp::Nop => Nop,

            wp::Block { ty } => Block(parse_block_ty(ty, offset+1)?),
            wp::Loop { ty } => Loop(parse_block_ty(ty, offset+1)?),
            wp::If { ty } => If(parse_block_ty(ty, offset+1)?),
            wp::Else => Else,
            wp::End => End,

            wp::Try { ty: _ }
            | wp::Catch { index: _ }
            | wp::CatchAll
            | wp::Throw { index: _ }
            | wp::Rethrow { relative_depth: _ }
            | wp::Delegate { relative_depth: _ } => {
                Err(ParseIssue::unsupported(offset, WasmExtension::ExceptionHandling))?
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
                if table_index != 0 {
                    Err(ParseIssue::unsupported(offset, WasmExtension::ReferenceTypes))?
                }
                CallIndirect(types.get(index, offset+1)?, 0usize.into())
            }

            wp::ReturnCall { function_index: _ }
            | wp::ReturnCallIndirect {
                index: _,
                table_index: _,
            } => Err(ParseIssue::unsupported(offset, WasmExtension::TailCalls))?,

            wp::Drop => Drop,
            wp::Select => Select,

            wp::TypedSelect { ty: _ } => Err(ParseIssue::unsupported(offset, WasmExtension::ReferenceTypes))?,

            wp::LocalGet { local_index } => Local(LocalOp::Get, local_index.into()),
            wp::LocalSet { local_index } => Local(LocalOp::Set, local_index.into()),
            wp::LocalTee { local_index } => Local(LocalOp::Tee, local_index.into()),
            wp::GlobalGet { global_index } => Global(GlobalOp::Get, global_index.into()),
            wp::GlobalSet { global_index } => Global(GlobalOp::Set, global_index.into()),

            wp::I32Load { memarg } => Load(LoadOp::I32Load, parse_memarg(memarg, offset+1)?),
            wp::I64Load { memarg } => Load(LoadOp::I64Load, parse_memarg(memarg, offset+1)?),
            wp::F32Load { memarg } => Load(LoadOp::F32Load, parse_memarg(memarg, offset+1)?),
            wp::F64Load { memarg } => Load(LoadOp::F64Load, parse_memarg(memarg, offset+1)?),
            wp::I32Load8S { memarg } => Load(LoadOp::I32Load8S, parse_memarg(memarg, offset+1)?),
            wp::I32Load8U { memarg } => Load(LoadOp::I32Load8U, parse_memarg(memarg, offset+1)?),
            wp::I32Load16S { memarg } => Load(LoadOp::I32Load16S, parse_memarg(memarg, offset+1)?),
            wp::I32Load16U { memarg } => Load(LoadOp::I32Load16U, parse_memarg(memarg, offset+1)?),
            wp::I64Load8S { memarg } => Load(LoadOp::I64Load8S, parse_memarg(memarg, offset+1)?),
            wp::I64Load8U { memarg } => Load(LoadOp::I64Load8U, parse_memarg(memarg, offset+1)?),
            wp::I64Load16S { memarg } => Load(LoadOp::I64Load16S, parse_memarg(memarg, offset+1)?),
            wp::I64Load16U { memarg } => Load(LoadOp::I64Load16U, parse_memarg(memarg, offset+1)?),
            wp::I64Load32S { memarg } => Load(LoadOp::I64Load32S, parse_memarg(memarg, offset+1)?),
            wp::I64Load32U { memarg } => Load(LoadOp::I64Load32U, parse_memarg(memarg, offset+1)?),

            wp::I32Store { memarg } => Store(StoreOp::I32Store, parse_memarg(memarg, offset+1)?),
            wp::I64Store { memarg } => Store(StoreOp::I64Store, parse_memarg(memarg, offset+1)?),
            wp::F32Store { memarg } => Store(StoreOp::F32Store, parse_memarg(memarg, offset+1)?),
            wp::F64Store { memarg } => Store(StoreOp::F64Store, parse_memarg(memarg, offset+1)?),
            wp::I32Store8 { memarg } => Store(StoreOp::I32Store8, parse_memarg(memarg, offset+1)?),
            wp::I32Store16 { memarg } => Store(StoreOp::I32Store16, parse_memarg(memarg, offset+1)?),
            wp::I64Store8 { memarg } => Store(StoreOp::I64Store8, parse_memarg(memarg, offset+1)?),
            wp::I64Store16 { memarg } => Store(StoreOp::I64Store16, parse_memarg(memarg, offset+1)?),
            wp::I64Store32 { memarg } => Store(StoreOp::I64Store32, parse_memarg(memarg, offset+1)?),

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

            wp::RefNull { ty: _ } | wp::RefIsNull | wp::RefFunc { function_index: _ } => {
                Err(ParseIssue::unsupported(offset, WasmExtension::ReferenceTypes))?
            }

            wp::I32Eqz => Numeric(NumericOp::I32Eqz),
            wp::I32Eq => Numeric(NumericOp::I32Eq),
            wp::I32Ne => Numeric(NumericOp::I32Ne),
            wp::I32LtS => Numeric(NumericOp::I32LtS),
            wp::I32LtU => Numeric(NumericOp::I32LtU),
            wp::I32GtS => Numeric(NumericOp::I32GtS),
            wp::I32GtU => Numeric(NumericOp::I32GtU),
            wp::I32LeS => Numeric(NumericOp::I32LeS),
            wp::I32LeU => Numeric(NumericOp::I32LeU),
            wp::I32GeS => Numeric(NumericOp::I32GeS),
            wp::I32GeU => Numeric(NumericOp::I32GeU),
            wp::I64Eqz => Numeric(NumericOp::I64Eqz),
            wp::I64Eq => Numeric(NumericOp::I64Eq),
            wp::I64Ne => Numeric(NumericOp::I64Ne),
            wp::I64LtS => Numeric(NumericOp::I64LtS),
            wp::I64LtU => Numeric(NumericOp::I64LtU),
            wp::I64GtS => Numeric(NumericOp::I64GtS),
            wp::I64GtU => Numeric(NumericOp::I64GtU),
            wp::I64LeS => Numeric(NumericOp::I64LeS),
            wp::I64LeU => Numeric(NumericOp::I64LeU),
            wp::I64GeS => Numeric(NumericOp::I64GeS),
            wp::I64GeU => Numeric(NumericOp::I64GeU),
            wp::F32Eq => Numeric(NumericOp::F32Eq),
            wp::F32Ne => Numeric(NumericOp::F32Ne),
            wp::F32Lt => Numeric(NumericOp::F32Lt),
            wp::F32Gt => Numeric(NumericOp::F32Gt),
            wp::F32Le => Numeric(NumericOp::F32Le),
            wp::F32Ge => Numeric(NumericOp::F32Ge),
            wp::F64Eq => Numeric(NumericOp::F64Eq),
            wp::F64Ne => Numeric(NumericOp::F64Ne),
            wp::F64Lt => Numeric(NumericOp::F64Lt),
            wp::F64Gt => Numeric(NumericOp::F64Gt),
            wp::F64Le => Numeric(NumericOp::F64Le),
            wp::F64Ge => Numeric(NumericOp::F64Ge),
            wp::I32Clz => Numeric(NumericOp::I32Clz),
            wp::I32Ctz => Numeric(NumericOp::I32Ctz),
            wp::I32Popcnt => Numeric(NumericOp::I32Popcnt),
            wp::I32Add => Numeric(NumericOp::I32Add),
            wp::I32Sub => Numeric(NumericOp::I32Sub),
            wp::I32Mul => Numeric(NumericOp::I32Mul),
            wp::I32DivS => Numeric(NumericOp::I32DivS),
            wp::I32DivU => Numeric(NumericOp::I32DivU),
            wp::I32RemS => Numeric(NumericOp::I32RemS),
            wp::I32RemU => Numeric(NumericOp::I32RemU),
            wp::I32And => Numeric(NumericOp::I32And),
            wp::I32Or => Numeric(NumericOp::I32Or),
            wp::I32Xor => Numeric(NumericOp::I32Xor),
            wp::I32Shl => Numeric(NumericOp::I32Shl),
            wp::I32ShrS => Numeric(NumericOp::I32ShrS),
            wp::I32ShrU => Numeric(NumericOp::I32ShrU),
            wp::I32Rotl => Numeric(NumericOp::I32Rotl),
            wp::I32Rotr => Numeric(NumericOp::I32Rotr),
            wp::I64Clz => Numeric(NumericOp::I64Clz),
            wp::I64Ctz => Numeric(NumericOp::I64Ctz),
            wp::I64Popcnt => Numeric(NumericOp::I64Popcnt),
            wp::I64Add => Numeric(NumericOp::I64Add),
            wp::I64Sub => Numeric(NumericOp::I64Sub),
            wp::I64Mul => Numeric(NumericOp::I64Mul),
            wp::I64DivS => Numeric(NumericOp::I64DivS),
            wp::I64DivU => Numeric(NumericOp::I64DivU),
            wp::I64RemS => Numeric(NumericOp::I64RemS),
            wp::I64RemU => Numeric(NumericOp::I64RemU),
            wp::I64And => Numeric(NumericOp::I64And),
            wp::I64Or => Numeric(NumericOp::I64Or),
            wp::I64Xor => Numeric(NumericOp::I64Xor),
            wp::I64Shl => Numeric(NumericOp::I64Shl),
            wp::I64ShrS => Numeric(NumericOp::I64ShrS),
            wp::I64ShrU => Numeric(NumericOp::I64ShrU),
            wp::I64Rotl => Numeric(NumericOp::I64Rotl),
            wp::I64Rotr => Numeric(NumericOp::I64Rotr),
            wp::F32Abs => Numeric(NumericOp::F32Abs),
            wp::F32Neg => Numeric(NumericOp::F32Neg),
            wp::F32Ceil => Numeric(NumericOp::F32Ceil),
            wp::F32Floor => Numeric(NumericOp::F32Floor),
            wp::F32Trunc => Numeric(NumericOp::F32Trunc),
            wp::F32Nearest => Numeric(NumericOp::F32Nearest),
            wp::F32Sqrt => Numeric(NumericOp::F32Sqrt),
            wp::F32Add => Numeric(NumericOp::F32Add),
            wp::F32Sub => Numeric(NumericOp::F32Sub),
            wp::F32Mul => Numeric(NumericOp::F32Mul),
            wp::F32Div => Numeric(NumericOp::F32Div),
            wp::F32Min => Numeric(NumericOp::F32Min),
            wp::F32Max => Numeric(NumericOp::F32Max),
            wp::F32Copysign => Numeric(NumericOp::F32Copysign),
            wp::F64Abs => Numeric(NumericOp::F64Abs),
            wp::F64Neg => Numeric(NumericOp::F64Neg),
            wp::F64Ceil => Numeric(NumericOp::F64Ceil),
            wp::F64Floor => Numeric(NumericOp::F64Floor),
            wp::F64Trunc => Numeric(NumericOp::F64Trunc),
            wp::F64Nearest => Numeric(NumericOp::F64Nearest),
            wp::F64Sqrt => Numeric(NumericOp::F64Sqrt),
            wp::F64Add => Numeric(NumericOp::F64Add),
            wp::F64Sub => Numeric(NumericOp::F64Sub),
            wp::F64Mul => Numeric(NumericOp::F64Mul),
            wp::F64Div => Numeric(NumericOp::F64Div),
            wp::F64Min => Numeric(NumericOp::F64Min),
            wp::F64Max => Numeric(NumericOp::F64Max),
            wp::F64Copysign => Numeric(NumericOp::F64Copysign),
            wp::I32WrapI64 => Numeric(NumericOp::I32WrapI64),
            wp::I32TruncF32S => Numeric(NumericOp::I32TruncF32S),
            wp::I32TruncF32U => Numeric(NumericOp::I32TruncF32U),
            wp::I32TruncF64S => Numeric(NumericOp::I32TruncF64S),
            wp::I32TruncF64U => Numeric(NumericOp::I32TruncF64U),
            wp::I64ExtendI32S => Numeric(NumericOp::I64ExtendI32S),
            wp::I64ExtendI32U => Numeric(NumericOp::I64ExtendI32U),
            wp::I64TruncF32S => Numeric(NumericOp::I64TruncF32S),
            wp::I64TruncF32U => Numeric(NumericOp::I64TruncF32U),
            wp::I64TruncF64S => Numeric(NumericOp::I64TruncF64S),
            wp::I64TruncF64U => Numeric(NumericOp::I64TruncF64U),
            wp::F32ConvertI32S => Numeric(NumericOp::F32ConvertI32S),
            wp::F32ConvertI32U => Numeric(NumericOp::F32ConvertI32U),
            wp::F32ConvertI64S => Numeric(NumericOp::F32ConvertI64S),
            wp::F32ConvertI64U => Numeric(NumericOp::F32ConvertI64U),
            wp::F32DemoteF64 => Numeric(NumericOp::F32DemoteF64),
            wp::F64ConvertI32S => Numeric(NumericOp::F64ConvertI32S),
            wp::F64ConvertI32U => Numeric(NumericOp::F64ConvertI32U),
            wp::F64ConvertI64S => Numeric(NumericOp::F64ConvertI64S),
            wp::F64ConvertI64U => Numeric(NumericOp::F64ConvertI64U),
            wp::F64PromoteF32 => Numeric(NumericOp::F64PromoteF32),
            wp::I32ReinterpretF32 => Numeric(NumericOp::I32ReinterpretF32),
            wp::I64ReinterpretF64 => Numeric(NumericOp::I64ReinterpretF64),
            wp::F32ReinterpretI32 => Numeric(NumericOp::F32ReinterpretI32),
            wp::F64ReinterpretI64 => Numeric(NumericOp::F64ReinterpretI64),

            wp::I32Extend8S
            | wp::I32Extend16S
            | wp::I64Extend8S
            | wp::I64Extend16S
            | wp::I64Extend32S => Err(ParseIssue::unsupported(offset, WasmExtension::SignExtensionOps))?,

            wp::I32TruncSatF32S
            | wp::I32TruncSatF32U
            | wp::I32TruncSatF64S
            | wp::I32TruncSatF64U
            | wp::I64TruncSatF32S
            | wp::I64TruncSatF32U
            | wp::I64TruncSatF64S
            | wp::I64TruncSatF64U => Err(ParseIssue::unsupported(offset, WasmExtension::NontrappingFloatToInt))?,

            wp::MemoryInit { segment: _, mem: _ }
            | wp::DataDrop { segment: _ }
            | wp::MemoryCopy { src: _, dst: _ }
            | wp::MemoryFill { mem: _ }
            | wp::TableInit {
                segment: _,
                table: _,
            }
            | wp::ElemDrop { segment: _ }
            | wp::TableCopy {
                dst_table: _,
                src_table: _,
            } => Err(ParseIssue::unsupported(offset, WasmExtension::BulkMemoryOperations))?,

            wp::TableFill { table: _ } => Err(ParseIssue::unsupported(offset, WasmExtension::ReferenceTypes))?,

            wp::TableGet { table: _ }
            | wp::TableSet { table: _ }
            | wp::TableGrow { table: _ }
            | wp::TableSize { table: _ } => Err(ParseIssue::unsupported(offset, WasmExtension::ReferenceTypes))?,

            wp::MemoryAtomicNotify { memarg: _ }
            | wp::MemoryAtomicWait32 { memarg: _ }
            | wp::MemoryAtomicWait64 { memarg: _ }
            | wp::AtomicFence { flags: _ }
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
            | wp::I64AtomicRmw32CmpxchgU { memarg: _ } => {
                Err(ParseIssue::unsupported(offset, WasmExtension::ThreadsAtomics))?
            }

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
            | wp::I8x16RoundingAverageU
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
            | wp::I16x8RoundingAverageU
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
        })
    }

    fn parse_memarg(memarg: wasmparser::MemoryImmediate, parser_offset: usize) -> Result<Memarg, ParseError> {
        if memarg.memory != 0 {
            Err(ParseIssue::unsupported(parser_offset, WasmExtension::MultiMemory))?
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

    fn parse_memory_ty(ty: wasmparser::MemoryType, offset: usize) -> Result<MemoryType, ParseError> {
        if ty.memory64 {
            Err(ParseIssue::unsupported(offset, WasmExtension::Memory64))?
        }
        Ok(MemoryType(Limits {
            initial_size: ty
                .initial
                .try_into()
                .expect("guaranteed u32 by wasmparser if !memory64"),
            max_size: ty
                .maximum
                .map(|u| u.try_into().expect("guaranteed u32 by wasmparser if !memory64")),
        }))
    }

    fn parse_table_ty(ty: wasmparser::TableType, offset: usize) -> Result<TableType, ParseError> {
        Ok(TableType(
            parse_elem_ty(ty.element_type, offset)?,
            Limits {
                initial_size: ty.initial,
                max_size: ty.maximum,
            },
        ))
    }

    fn parse_elem_ty(ty: wasmparser::Type, offset: usize) -> Result<ElemType, ParseError> {
        use wasmparser::Type::*;
        match ty {
            I32 | I64 | F32 | F64 => Err(ParseIssue::message(offset, "only reftypes, not value types are allowed as table elements"))?,
            V128 => Err(ParseIssue::message(offset, "only reftypes, not value types are allowed as table elements"))?,
            FuncRef => Ok(ElemType::Anyfunc),
            ExternRef => Err(ParseIssue::unsupported(offset, WasmExtension::ReferenceTypes))?,
            ExnRef => Err(ParseIssue::unsupported(offset, WasmExtension::ExceptionHandling))?,
            Func => Err(ParseIssue::message(offset, "only reftypes, not function types are allowed as table elements"))?,
            EmptyBlockType => Err(ParseIssue::message(offset, "only reftypes, not block types are allowed as table elements"))?,
        }
    }

    fn parse_block_ty(ty: wasmparser::TypeOrFuncType, offset: usize) -> Result<BlockType, ParseError> {
        use wasmparser::TypeOrFuncType::*;
        match ty {
            Type(wasmparser::Type::EmptyBlockType) => Ok(BlockType(None)),
            Type(ty) => Ok(BlockType(Some(parse_val_ty(ty, offset)?))),
            FuncType(_) => Err(ParseIssue::unsupported(offset, WasmExtension::MultiValue))?,
        }
    }

    fn parse_func_ty(ty: wasmparser::FuncType, offset: usize) -> Result<FunctionType, ParseError> {
        let convert_tys = |tys: &[wasmparser::Type]| -> Result<Box<[ValType]>, ParseError> {
            let vec: Vec<ValType> = tys
                .iter()
                .cloned()
                // The `offset` for error reporting is not exactly correct, because it marks the start
                // of the function type, not the individual wrong parameter/result type.
                .map(|ty| parse_val_ty(ty, offset))
                .collect::<Result<_, _>>()?;
            Ok(vec.into())
        };

        Ok(FunctionType {
            params: convert_tys(&ty.params)?,
            results: convert_tys(&ty.returns)?,
        })
    }

    fn parse_global_ty(ty: wasmparser::GlobalType, offset: usize) -> Result<GlobalType, ParseError> {
        Ok(GlobalType(
            parse_val_ty(ty.content_type, offset)?,
            if ty.mutable {
                Mutability::Mut
            } else {
                Mutability::Const
            },
        ))
    }

    fn parse_val_ty(ty: wasmparser::Type, offset: usize) -> Result<ValType, ParseError> {
        use wasmparser::Type;
        match ty {
            Type::I32 => Ok(ValType::I32),
            Type::I64 => Ok(ValType::I64),
            Type::F32 => Ok(ValType::F32),
            Type::F64 => Ok(ValType::F64),
            Type::V128 => Err(ParseIssue::unsupported(offset, WasmExtension::Simd))?,
            Type::FuncRef => Err(ParseIssue::unsupported(offset, WasmExtension::ReferenceTypes))?,
            Type::ExternRef => Err(ParseIssue::unsupported(offset, WasmExtension::ReferenceTypes))?,
            Type::ExnRef => Err(ParseIssue::unsupported(offset, WasmExtension::ExceptionHandling))?,
            Type::Func => Err(ParseIssue::message(offset, "function types are not a valid value type"))?,
            Type::EmptyBlockType => Err(ParseIssue::message(offset, "block types are not a valid value type"))?,
        }
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
        pub fn new_type_section(&mut self, count: u32, type_section_offset: usize) -> Result<(), ParseError> {
            let prev_state = self.0.replace(Vec::with_capacity(u32_to_usize(count)));
            match prev_state {
                Some(_) => Err(ParseIssue::message(type_section_offset, "duplicate type section"))?,
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
                .ok_or(ParseIssue::index(index_offset, index, "type"))?)
        }
    }

}

fn u32_to_usize(u: u32) -> usize {
    u.try_into().expect("u32 to usize should always succeed")
}
