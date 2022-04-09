use std::convert::TryInto;

mod error {
    use crate::extensions::WasmExtension;

    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct ParseError(
        // Put the actual error behind a box, to keep the size down to a single pointer.
        Box<ParseErrorInner>
    );

    /// Used both for warnings (recoverable, i.e., parsing can continue afterwards) and errors
    /// (not recoverable, i.e., parsing stops and does not return an AST).
    // TODO if this is used for warnings, rename into ParseIssue again
    #[derive(Debug, thiserror::Error)]
    pub enum ParseErrorInner {
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
        Io(#[from] std::io::Error)
    }

    // Allow conversion of everything that can be converted into a `ParseErrorInner` also into the
    // `ParseError` wrapper directly.
    impl<T> From<T> for ParseError 
    where T : Into<ParseErrorInner>
    {
        fn from(err: T) -> Self {
            ParseError(Box::new(err.into()))
        }
    }

    // Convenience constructors.
    impl ParseErrorInner {
        pub fn message(offset: usize, message: &'static str) -> Self {
            ParseErrorInner::Message { offset, message }
        }

        pub fn index(offset: usize, index: u32, index_space: &'static str) -> Self {
            ParseErrorInner::Index { offset, index, index_space }
        }

        pub fn unsupported(offset: usize, extension: WasmExtension) -> Self {
            ParseErrorInner::Unsupported { offset, extension }
        }
    }

    #[derive(Debug, thiserror::Error)]
    #[error(transparent)]
    pub struct EncodeError(
        // Put the actual error behind a box, to keep the size down to a single pointer.
        Box<EncodeErrorInner>
    );

    #[derive(Debug, thiserror::Error)]
    pub enum EncodeErrorInner {
        #[error("error while encoding WebAssembly module to binary format: {}", .0)]
        Message(String),

        #[error(transparent)]
        Io(#[from] std::io::Error)
    }

    impl EncodeError {
        pub fn message(message: String) -> Self {
            EncodeError(Box::new(EncodeErrorInner::Message(message)))
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
    
    use super::error::{ParseError, ParseErrorInner};
    
    use super::u32_to_usize;

    // The streaming API of wasmparser is a bit cumbersome, so implement reading 
    // from bytes fully resident in memory first. 
    // TODO Add a second API from streaming sources, i.e., `io::Read` like here:
    // https://docs.rs/wasmparser/latest/wasmparser/struct.Parser.html#examples
    pub fn parse_module_with_offsets(bytes: &[u8]) -> Result<(Module, Offsets, Vec<ParseErrorInner>), ParseError> {
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
                                Err(ParseErrorInner::unsupported(type_offset, WasmExtension::ModuleLinking))?
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
                            .ok_or(ParseErrorInner::unsupported(import_offset, WasmExtension::ModuleLinking))?
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
                                Err(ParseErrorInner::unsupported(import_offset, WasmExtension::ExceptionHandling))?
                            }
                            ImportSectionEntryType::Module(_) | ImportSectionEntryType::Instance(_) => {
                                // Same issue regarding `import_offset`.
                                Err(ParseErrorInner::unsupported(import_offset, WasmExtension::ModuleLinking))?
                            }
                        }

                        import_offset = reader.original_position();
                    }
                }
                Payload::AliasSection(reader) => Err(ParseErrorInner::unsupported(reader.range().start, WasmExtension::ModuleLinking))?,
                Payload::InstanceSection(reader) => Err(ParseErrorInner::unsupported(reader.range().start, WasmExtension::ModuleLinking))?,
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
                Payload::TagSection(reader) => Err(ParseErrorInner::unsupported(reader.range().start, WasmExtension::ExceptionHandling))?,
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
                                .ok_or(ParseErrorInner::index(export_offset, index_u32, "function"))?
                                .export
                                .push(name),
                            ExternalKind::Table => module
                                .tables
                                .get_mut(index)
                                // Same issue regarding `export_offset`.
                                .ok_or(ParseErrorInner::index(export_offset, index_u32, "table"))?
                                .export
                                .push(name),
                            ExternalKind::Memory => module
                                .memories
                                .get_mut(index)
                                // Same issue regarding `export_offset`.
                                .ok_or(ParseErrorInner::index(export_offset, index_u32, "memory"))?
                                .export
                                .push(name),
                            ExternalKind::Global => module
                                .globals
                                .get_mut(index)
                                // Same issue regarding `export_offset`.
                                .ok_or(ParseErrorInner::index(export_offset, index_u32, "global"))?
                                .export
                                .push(name),
                            ExternalKind::Tag => {
                                // Same issue regarding `export_offset`.
                                Err(ParseErrorInner::unsupported(export_offset, WasmExtension::ExceptionHandling))?
                            }
                            ExternalKind::Type => {
                                // Same issue regarding `export_offset`.
                                Err(ParseErrorInner::unsupported(export_offset, WasmExtension::TypeImports))?
                            },
                            ExternalKind::Module | ExternalKind::Instance => {
                                // Same issue regarding `export_offset`.
                                Err(ParseErrorInner::unsupported(export_offset, WasmExtension::ModuleLinking))?
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
                        Err(ParseErrorInner::message(range.start, "duplicate start section"))?
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
                                ElementItem::Expr(_) => Err(ParseErrorInner::unsupported(item_offset, WasmExtension::ReferenceTypes))?,
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
                                    .ok_or(ParseErrorInner::index(element_offset, table_index, "table"))?;

                                if table.type_.0 != element_ty {
                                    Err(ParseErrorInner::message(element_offset, "table and element type do not match"))?
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
                                Err(ParseErrorInner::unsupported(element_offset, WasmExtension::BulkMemoryOperations))?
                            }
                            ElementKind::Declared => {
                                Err(ParseErrorInner::unsupported(element_offset, WasmExtension::ReferenceTypes))?
                            }
                        }

                        element_offset = reader.original_position();
                    }
                }
                Payload::DataCountSection { count: _, range } => {
                    Err(ParseErrorInner::unsupported(range.start, WasmExtension::BulkMemoryOperations))?
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
                                    .ok_or(ParseErrorInner::index(data_offset, memory_index, "memory"))?;

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
                                Err(ParseErrorInner::unsupported(data_offset, WasmExtension::BulkMemoryOperations))?
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
                    let mut name_parsing = || -> Result<(), ParseErrorInner> {
                        let mut reader = NameSectionReader::new(data, data_offset)?;
                        while !reader.eof() {
                            let offset = reader.original_position();
                            let name_subsection = reader.read()?;
                            use wasmparser::Name;
                            match name_subsection {
                                Name::Module(name) => {
                                    let prev = module.name.replace(name.get_name()?.to_string());
                                    if let Some(_) = prev {
                                        warnings.push(ParseErrorInner::message(offset, "duplicate module name"))
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
                                            .ok_or(ParseErrorInner::index(offset, function_index, "function"))?
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
                                            .ok_or(ParseErrorInner::index(offset, function_index, "function"))?;
        
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
                                    warnings.push(ParseErrorInner::unsupported(offset, WasmExtension::ExtendedNameSection))
                                }
                                | Name::Unknown {
                                    ty: _,
                                    data: _,
                                    range: _,
                                } => warnings.push(ParseErrorInner::message(offset, "unknown name subsection")),
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
                                .ok_or(ParseErrorInner::index(offset, func_index, "function"))?;
                            function.code = ImportOrPresent::Present(code?);
                        }
                    }
                }
                Payload::ModuleSectionStart {
                    count: _,
                    range,
                    size: _,
                } => Err(ParseErrorInner::unsupported(range.start, WasmExtension::ModuleLinking))?,
                Payload::ModuleSectionEntry {
                    parser: _,
                    range,
                } => Err(ParseErrorInner::unsupported(range.start, WasmExtension::ModuleLinking))?,
                Payload::UnknownSection {
                    id: _,
                    contents: _,
                    range,
                } => Err(ParseErrorInner::message(range.start, "unknown section"))?,
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
                Err(ParseErrorInner::unsupported(offset, WasmExtension::ExceptionHandling))?
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
                    Err(ParseErrorInner::unsupported(offset, WasmExtension::ReferenceTypes))?
                }
                CallIndirect(types.get(index, offset+1)?, 0usize.into())
            }

            wp::ReturnCall { function_index: _ }
            | wp::ReturnCallIndirect {
                index: _,
                table_index: _,
            } => Err(ParseErrorInner::unsupported(offset, WasmExtension::TailCalls))?,

            wp::Drop => Drop,
            wp::Select => Select,

            wp::TypedSelect { ty: _ } => Err(ParseErrorInner::unsupported(offset, WasmExtension::ReferenceTypes))?,

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
                    Err(ParseErrorInner::unsupported(offset, WasmExtension::MultiMemory))?
                }
                MemorySize(0u32.into())
            }
            wp::MemoryGrow { mem, mem_byte: _ } => {
                if mem != 0 {
                    Err(ParseErrorInner::unsupported(offset, WasmExtension::MultiMemory))?
                }
                MemoryGrow(0u32.into())
            }

            wp::I32Const { value } => Const(Val::I32(value)),
            wp::I64Const { value } => Const(Val::I64(value)),
            wp::F32Const { value } => Const(Val::F32(OrderedFloat(f32::from_bits(value.bits())))),
            wp::F64Const { value } => Const(Val::F64(OrderedFloat(f64::from_bits(value.bits())))),

            wp::RefNull { ty: _ } | wp::RefIsNull | wp::RefFunc { function_index: _ } => {
                Err(ParseErrorInner::unsupported(offset, WasmExtension::ReferenceTypes))?
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
            | wp::I64Extend32S => Err(ParseErrorInner::unsupported(offset, WasmExtension::SignExtensionOps))?,

            wp::I32TruncSatF32S
            | wp::I32TruncSatF32U
            | wp::I32TruncSatF64S
            | wp::I32TruncSatF64U
            | wp::I64TruncSatF32S
            | wp::I64TruncSatF32U
            | wp::I64TruncSatF64S
            | wp::I64TruncSatF64U => Err(ParseErrorInner::unsupported(offset, WasmExtension::NontrappingFloatToInt))?,

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
            } => Err(ParseErrorInner::unsupported(offset, WasmExtension::BulkMemoryOperations))?,

            wp::TableFill { table: _ } => Err(ParseErrorInner::unsupported(offset, WasmExtension::ReferenceTypes))?,

            wp::TableGet { table: _ }
            | wp::TableSet { table: _ }
            | wp::TableGrow { table: _ }
            | wp::TableSize { table: _ } => Err(ParseErrorInner::unsupported(offset, WasmExtension::ReferenceTypes))?,

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
                Err(ParseErrorInner::unsupported(offset, WasmExtension::ThreadsAtomics))?
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
            | wp::F64x2PromoteLowF32x4 => Err(ParseErrorInner::unsupported(offset, WasmExtension::Simd))?,
        })
    }

    fn parse_memarg(memarg: wasmparser::MemoryImmediate, parser_offset: usize) -> Result<Memarg, ParseError> {
        if memarg.memory != 0 {
            Err(ParseErrorInner::unsupported(parser_offset, WasmExtension::MultiMemory))?
        }
        let offset: u32 = memarg
            .offset
            .try_into()
            .map_err(|_| ParseErrorInner::unsupported(parser_offset, WasmExtension::Memory64))?;
        Ok(Memarg {
            alignment_exp: memarg.align,
            offset,
        })
    }

    fn parse_memory_ty(ty: wasmparser::MemoryType, offset: usize) -> Result<MemoryType, ParseError> {
        if ty.memory64 {
            Err(ParseErrorInner::unsupported(offset, WasmExtension::Memory64))?
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
            I32 | I64 | F32 | F64 => Err(ParseErrorInner::message(offset, "only reftypes, not value types are allowed as table elements"))?,
            V128 => Err(ParseErrorInner::message(offset, "only reftypes, not value types are allowed as table elements"))?,
            FuncRef => Ok(ElemType::Anyfunc),
            ExternRef => Err(ParseErrorInner::unsupported(offset, WasmExtension::ReferenceTypes))?,
            ExnRef => Err(ParseErrorInner::unsupported(offset, WasmExtension::ExceptionHandling))?,
            Func => Err(ParseErrorInner::message(offset, "only reftypes, not function types are allowed as table elements"))?,
            EmptyBlockType => Err(ParseErrorInner::message(offset, "only reftypes, not block types are allowed as table elements"))?,
        }
    }

    fn parse_block_ty(ty: wasmparser::TypeOrFuncType, offset: usize) -> Result<BlockType, ParseError> {
        use wasmparser::TypeOrFuncType::*;
        match ty {
            Type(wasmparser::Type::EmptyBlockType) => Ok(BlockType(None)),
            Type(ty) => Ok(BlockType(Some(parse_val_ty(ty, offset)?))),
            FuncType(_) => Err(ParseErrorInner::unsupported(offset, WasmExtension::MultiValue))?,
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
            Type::V128 => Err(ParseErrorInner::unsupported(offset, WasmExtension::Simd))?,
            Type::FuncRef => Err(ParseErrorInner::unsupported(offset, WasmExtension::ReferenceTypes))?,
            Type::ExternRef => Err(ParseErrorInner::unsupported(offset, WasmExtension::ReferenceTypes))?,
            Type::ExnRef => Err(ParseErrorInner::unsupported(offset, WasmExtension::ExceptionHandling))?,
            Type::Func => Err(ParseErrorInner::message(offset, "function types are not a valid value type"))?,
            Type::EmptyBlockType => Err(ParseErrorInner::message(offset, "block types are not a valid value type"))?,
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
                Some(_) => Err(ParseErrorInner::message(type_section_offset, "duplicate type section"))?,
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
                .ok_or(ParseErrorInner::index(index_offset, index, "type"))?)
        }
    }

}

pub mod encode {
    use std::convert::TryInto;
    use std::collections::HashMap;

    use wasm_encoder as ll;

    /// Add marker types for type-safe `Idx<T>` for the low-level binary format.
    /// Since I cannot extend wasm_encoder and there cannot be another, clashing `ll` module, wrap
    /// this in an intermediate `marker` module.
    mod marker {
        pub mod ll {
            pub struct FunctionType;
            pub struct Function;
            pub struct Global;
            pub struct Table;
            pub struct Memory;
        }
    }

    use crate::highlevel as hl;
    use crate::{FunctionType, Idx};

    use super::error::EncodeError;

    #[derive(Default)]
    struct EncodeState {
        types_idx: HashMap<FunctionType, Idx<marker::ll::FunctionType>>,

        // Mapping of indices from the high-level AST to the low-level binary format.
        // This is necessary, because in the WebAssembly binary format all imported elements
        // (functions, globals, etc.) come before (i.e., have a lower index) than all
        // "locally-defined" (i.e., non-imported) elements.
        // We thus have to re-index functions, globals, etc. and use this here to do so.
        function_idx: HashMap<Idx<hl::Function>, Idx<marker::ll::Function>>,
        global_idx: HashMap<Idx<hl::Global>, Idx<marker::ll::Global>>,

        table_idx: HashMap<Idx<hl::Table>, Idx<marker::ll::Table>>,
        memory_idx: HashMap<Idx<hl::Memory>, Idx<marker::ll::Memory>>,
    }

    macro_rules! encode_state_idx_fns {
        ($insert_fn: ident, $map_fn: ident, $field: ident, $ty: ident) => {
            /// Add a new mapping from a high-level index to the low-level index in the binary.
            /// 
            /// Unlike for types, the high-level index must be new, otherwise the function panics.
            /// If you want to map an existing index, use `map_*_idx` instead.
            fn $insert_fn(&mut self, highlevel_idx: Idx<hl::$ty>) -> Idx<marker::ll::$ty> {
                let new_lowlevel_idx = Idx::from(self.$field.len());
                let was_new = self.$field.insert(highlevel_idx, new_lowlevel_idx).is_none();
                assert!(was_new, "highlevel index {:?} was inserted twice", highlevel_idx);
                new_lowlevel_idx
            }

            fn $map_fn(&self, highlevel_idx: Idx<hl::$ty>) -> Idx<marker::ll::$ty> {
                *self.$field.get(&highlevel_idx).expect("high-level index was not present in mapping, but should have been")
            }
        };
    }
        
    impl EncodeState {
        fn get_or_insert_type(&mut self, type_: FunctionType) -> Idx<marker::ll::FunctionType> {
            // This means types are ordered by their first appearance in the module.
            let new_idx = Idx::from(self.types_idx.len());
            *self.types_idx.entry(type_).or_insert(new_idx)
        }

        encode_state_idx_fns!(insert_function_idx, map_function_idx, function_idx, Function);
        encode_state_idx_fns!(insert_global_idx, map_global_idx, global_idx, Global);

        encode_state_idx_fns!(insert_table_idx, map_table_idx, table_idx, Table);
        encode_state_idx_fns!(insert_memory_idx, map_memory_idx, memory_idx, Memory);
    }
    
    pub fn encode_module(module: &hl::Module) -> Result<Vec<u8>, EncodeError> {
        let mut lowlevel = wasm_encoder::Module::new();
        let mut state = EncodeState::default();

        // Note that the order in which the high-level AST is traversed is not equal to the order
        // in which low-level sections are written out to the binary. 
        // Consider the type section: It must be the FIRST section of the low-level binary, but the
        // set of all types in the module is only known once we have iterated over all functions, 
        // globals, etc. during conversion to the low-level binary format.
        // Alternatively, one could iterate also twice over the high-level module. Once to collect
        // all types, then write the type section, and then once again to encode the rest of the
        // module. This would make the lifetime of the allocated sections a bit shorter.
        // In principle, however, all sections must be fully resident in memory before one can write
        // them out anyway, because the section size in bytes is prepended to its contents.

        // First, traverse all imported functions, globals, etc., such that they are at the 
        // beginning of all index spaces.
        let import_section = encode_imports(module, &mut state);

        // Then traverse all non-imported functions, globals, etc., such that their indices and
        // types are in `state`.
        let (function_section, code_section) = encode_functions(module, &mut state);
        let global_section = encode_globals(module, &mut state)?;

        let (table_section, element_section) = encode_tables(module, &mut state)?;
        let (memory_section, data_section) = encode_memories(module, &mut state)?;

        // Now, `state` contains all types that appear in the module, so we are ready encode the
        // type section.
        let type_section = encode_types(&state);

        // Then, write all sections in the correct order into the binary.
        // For the section order, see https://webassembly.github.io/spec/core/binary/modules.html#binary-module
        if !type_section.is_empty() {
            lowlevel.section(&type_section);
        }
        if !import_section.is_empty() {
            lowlevel.section(&import_section);
        }
        if !function_section.is_empty() {
            lowlevel.section(&function_section);
        }
        if !table_section.is_empty() {
            lowlevel.section(&table_section);
        }
        if !memory_section.is_empty() {
            lowlevel.section(&memory_section);
        }
        if !global_section.is_empty() {
            lowlevel.section(&global_section);
        }
        let export_section = encode_exports(module, &mut state);
        if !export_section.is_empty() {
            lowlevel.section(&export_section);
        }
        if let Some(function_idx) = module.start {
            lowlevel.section(&ll::StartSection {
                function_index: state.map_function_idx(function_idx).to_u32()
            });
        }
        if !element_section.is_empty() {
            lowlevel.section(&element_section);
        }
        if !code_section.is_empty() {
            lowlevel.section(&code_section);
        }
        if !data_section.is_empty() {
            lowlevel.section(&data_section);
        }

        // TODO .name

        // TODO custom sections in between all the others
        
        Ok(lowlevel.finish())
    }

    fn encode_imports(module: &hl::Module, state: &mut EncodeState) -> ll::ImportSection {
        let mut import_section = ll::ImportSection::new();

        macro_rules! add_imports {
            ($elem_iter: ident, $state_insert_fn: ident, $ll_import_type: ident, $hl_to_ll_closure: expr) => {
                for (hl_idx, elem) in module.$elem_iter() {
                    if let Some((module_name, name)) = elem.import() {
                        state.$state_insert_fn(hl_idx);
                        import_section.import(
                            module_name, 
                            Some(name),
                            ll::EntityType::$ll_import_type($hl_to_ll_closure(elem))
                        );
                    }
                }
            }
        }

        add_imports!(functions, insert_function_idx, Function, |f: &hl::Function| state.get_or_insert_type(f.type_.clone()).to_u32());
        add_imports!(globals, insert_global_idx, Global, |g: &hl::Global| ll::GlobalType::from(g.type_));

        add_imports!(tables, insert_table_idx, Table, |t: &hl::Table| ll::TableType::from(t.type_));
        add_imports!(memories, insert_memory_idx, Memory, |m: &hl::Memory| ll::MemoryType::from(m.type_));

        import_section
    }

    fn encode_exports(module: &hl::Module, state: &mut EncodeState) -> ll::ExportSection {
        let mut export_section = ll::ExportSection::new();

        macro_rules! add_exports {
            ($elem_iter: ident, $ll_export_type: ident, $map_idx_fn: ident) => {
                for (hl_idx, elem) in module.$elem_iter() {
                    for name in &elem.export {
                        export_section.export(
                            name,
                            ll::Export::$ll_export_type(state.$map_idx_fn(hl_idx).to_u32())
                        );
                    }
                }
            }
        }

        add_exports!(functions, Function, map_function_idx);
        add_exports!(globals, Global, map_global_idx);

        add_exports!(tables, Table, map_table_idx);
        add_exports!(memories, Memory, map_memory_idx);

        export_section
    }

    fn encode_functions(module: &hl::Module, state: &mut EncodeState) -> (ll::FunctionSection, ll::CodeSection) {
        let mut function_section = ll::FunctionSection::new();
        let mut code_section = ll::CodeSection::new();

        for (function_idx, function) in module.functions() {
            if let Some(code) = function.code() {
                state.insert_function_idx(function_idx);

                let ll_type_idx = state.get_or_insert_type(function.type_.clone());
                function_section.function(ll_type_idx.to_u32());

                let ll_locals_iter = code.locals.iter().map(|local| ll::ValType::from(local.type_));
                let mut ll_function = ll::Function::new_with_locals_types(ll_locals_iter);
                for instr in &code.body {
                    ll_function.instruction(&encode_instruction(instr, state));
                }
                code_section.function(&ll_function);
            }
        }

        (function_section, code_section)
    }

    fn encode_globals(module: &hl::Module, state: &mut EncodeState) -> Result<ll::GlobalSection, EncodeError> {
        let mut global_section = ll::GlobalSection::new();

        for (global_idx, global) in module.globals() {
            if let Some(init) = global.init() {
                state.insert_global_idx(global_idx);
                let ll_init = encode_single_instruction_with_end(init, state)?;
                global_section.global(ll::GlobalType::from(global.type_), &ll_init);
            }
        }

        Ok(global_section)
    }

    fn encode_tables(module: &hl::Module, state: &mut EncodeState) -> Result<(ll::TableSection, ll::ElementSection), EncodeError> {
        let mut table_section = ll::TableSection::new();
        let mut element_section = ll::ElementSection::new();

        for (table_idx, table) in module.tables() {
            let ll_table_idx = state.insert_table_idx(table_idx);
            table_section.table(ll::TableType::from(table.type_));

            for hl_element in &table.elements {
                // `wasm-encoder` uses None as the table index to signify the MVP binary format.
                // Use that whenever possible, to avoid producing a binary using extensions.
                let ll_table_idx = if ll_table_idx.to_u32() == 0 {
                    None
                } else {
                    Some(ll_table_idx.to_u32())
                };
                let ll_offset = encode_single_instruction_with_end(&hl_element.offset, state)?;
                let ll_elements: Vec<u32> = hl_element.functions.iter()
                    .map(|function_idx| state.map_function_idx(*function_idx).to_u32())
                    .collect();
                let ll_elements = ll::Elements::Functions(ll_elements.as_slice());
                element_section.active(ll_table_idx, &ll_offset, ll::ValType::FuncRef, ll_elements);
            }
        }

        Ok((table_section, element_section))
    }

    fn encode_memories(module: &hl::Module, state: &mut EncodeState) -> Result<(ll::MemorySection, ll::DataSection), EncodeError> {
        let mut memory_section = ll::MemorySection::new();
        let mut data_section = ll::DataSection::new();

        for (memory_idx, memory) in module.memories() {
            let ll_memory_idx = state.insert_memory_idx(memory_idx);
            memory_section.memory(ll::MemoryType::from(memory.type_));

            for data in &memory.data {
                let ll_offset = encode_single_instruction_with_end( &data.offset, state)?;
                let ll_data = data.bytes.iter().copied();
                data_section.active(ll_memory_idx.to_u32(), &ll_offset, ll_data);
            }
        }

        Ok((memory_section, data_section))
    }

    fn encode_single_instruction_with_end(instrs: &[hl::Instr], state: &mut EncodeState) -> Result<ll::Instruction<'static>, EncodeError> {
        match instrs {
            [single_instr, hl::Instr::End] => Ok(encode_instruction(single_instr, state)),
            _ => Err(EncodeError::message(format!("expected exactly one instruction, followed by an end, but got {:?}. If there is more than one instruction, this is not supported by wasm-encoder for an unknown reason.", instrs))),
        }
    }

    fn encode_instruction(hl_instr: &hl::Instr, state: &mut EncodeState) -> ll::Instruction<'static> {
        match *hl_instr {
            hl::Instr::Unreachable => ll::Instruction::Unreachable,
            hl::Instr::Nop => ll::Instruction::Nop,

            hl::Instr::Block(block_type) => ll::Instruction::Block(block_type.into()),
            hl::Instr::Loop(block_type) => ll::Instruction::Loop(block_type.into()),
            hl::Instr::If(block_type) => ll::Instruction::If(block_type.into()),
            hl::Instr::Else => ll::Instruction::Else,
            hl::Instr::End => ll::Instruction::End,
            
            hl::Instr::Br(label) => ll::Instruction::Br(label.to_u32()),
            hl::Instr::BrIf(label) => ll::Instruction::BrIf(label.to_u32()),
            hl::Instr::BrTable { ref table, default } => ll::Instruction::BrTable(
                table.iter().map(|label| label.to_u32()).collect(),
                default.to_u32()
            ),
            
            hl::Instr::Return => ll::Instruction::Return,
            hl::Instr::Call(function_idx) => ll::Instruction::Call(state.map_function_idx(function_idx).to_u32()),
            hl::Instr::CallIndirect(ref function_type, table_idx) => ll::Instruction::CallIndirect {
                ty: state.get_or_insert_type(function_type.clone()).to_u32(),
                table: state.map_table_idx(table_idx).to_u32(),
            },
            
            hl::Instr::Drop => ll::Instruction::Drop,
            hl::Instr::Select => ll::Instruction::Select,
            
            hl::Instr::Local(hl::LocalOp::Get, local_idx) => ll::Instruction::LocalGet(local_idx.to_u32()),
            hl::Instr::Local(hl::LocalOp::Set, local_idx) => ll::Instruction::LocalSet(local_idx.to_u32()),
            hl::Instr::Local(hl::LocalOp::Tee, local_idx) => ll::Instruction::LocalTee(local_idx.to_u32()),
            hl::Instr::Global(hl::GlobalOp::Get, global_idx) => ll::Instruction::GlobalGet(state.map_global_idx(global_idx).to_u32()),
            hl::Instr::Global(hl::GlobalOp::Set, global_idx) => ll::Instruction::GlobalSet(state.map_global_idx(global_idx).to_u32()),

            hl::Instr::Load(hl::LoadOp::I32Load, memarg) => ll::Instruction::I32Load(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I64Load, memarg) => ll::Instruction::I64Load(memarg.into()),
            hl::Instr::Load(hl::LoadOp::F32Load, memarg) => ll::Instruction::F32Load(memarg.into()),
            hl::Instr::Load(hl::LoadOp::F64Load, memarg) => ll::Instruction::F64Load(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I32Load8S, memarg) => ll::Instruction::I32Load8_S(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I32Load8U, memarg) => ll::Instruction::I32Load8_U(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I32Load16S, memarg) => ll::Instruction::I32Load16_S(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I32Load16U, memarg) => ll::Instruction::I32Load16_U(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I64Load8S, memarg) => ll::Instruction::I64Load8_S(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I64Load8U, memarg) => ll::Instruction::I64Load8_U(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I64Load16S, memarg) => ll::Instruction::I64Load16_S(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I64Load16U, memarg) => ll::Instruction::I64Load16_U(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I64Load32S, memarg) => ll::Instruction::I64Load32_S(memarg.into()),
            hl::Instr::Load(hl::LoadOp::I64Load32U, memarg) => ll::Instruction::I64Load32_U(memarg.into()),
            
            hl::Instr::Store(hl::StoreOp::I32Store, memarg) => ll::Instruction::I32Store(memarg.into()),
            hl::Instr::Store(hl::StoreOp::I64Store, memarg) => ll::Instruction::I64Store(memarg.into()),
            hl::Instr::Store(hl::StoreOp::F32Store, memarg) => ll::Instruction::F32Store(memarg.into()),
            hl::Instr::Store(hl::StoreOp::F64Store, memarg) => ll::Instruction::F64Store(memarg.into()),
            hl::Instr::Store(hl::StoreOp::I32Store8, memarg) => ll::Instruction::I32Store8(memarg.into()),
            hl::Instr::Store(hl::StoreOp::I32Store16, memarg) => ll::Instruction::I32Store16(memarg.into()),
            hl::Instr::Store(hl::StoreOp::I64Store8, memarg) => ll::Instruction::I64Store8(memarg.into()),
            hl::Instr::Store(hl::StoreOp::I64Store16, memarg) => ll::Instruction::I64Store16(memarg.into()),
            hl::Instr::Store(hl::StoreOp::I64Store32, memarg) => ll::Instruction::I64Store32(memarg.into()),
    
            hl::Instr::MemorySize(memory_idx) => ll::Instruction::MemorySize(state.map_memory_idx(memory_idx).to_u32()),
            hl::Instr::MemoryGrow(memory_idx) => ll::Instruction::MemoryGrow(state.map_memory_idx(memory_idx).to_u32()),
    
            hl::Instr::Const(crate::Val::I32(value)) => ll::Instruction::I32Const(value),
            hl::Instr::Const(crate::Val::I64(value)) => ll::Instruction::I64Const(value),
            hl::Instr::Const(crate::Val::F32(value)) => ll::Instruction::F32Const(value.into_inner()),
            hl::Instr::Const(crate::Val::F64(value)) => ll::Instruction::F64Const(value.into_inner()),
    
            hl::Instr::Numeric(hl::NumericOp::I32Eqz) => ll::Instruction::I32Eqz,
            hl::Instr::Numeric(hl::NumericOp::I32Eq) => ll::Instruction::I32Eq,
            hl::Instr::Numeric(hl::NumericOp::I32Ne) => ll::Instruction::I32Ne,
            hl::Instr::Numeric(hl::NumericOp::I32LtS) => ll::Instruction::I32LtS,
            hl::Instr::Numeric(hl::NumericOp::I32LtU) => ll::Instruction::I32LtU,
            hl::Instr::Numeric(hl::NumericOp::I32GtS) => ll::Instruction::I32GtS,
            hl::Instr::Numeric(hl::NumericOp::I32GtU) => ll::Instruction::I32GtU,
            hl::Instr::Numeric(hl::NumericOp::I32LeS) => ll::Instruction::I32LeS,
            hl::Instr::Numeric(hl::NumericOp::I32LeU) => ll::Instruction::I32LeU,
            hl::Instr::Numeric(hl::NumericOp::I32GeS) => ll::Instruction::I32GeS,
            hl::Instr::Numeric(hl::NumericOp::I32GeU) => ll::Instruction::I32GeU,
            hl::Instr::Numeric(hl::NumericOp::I64Eqz) => ll::Instruction::I64Eqz,
            hl::Instr::Numeric(hl::NumericOp::I64Eq) => ll::Instruction::I64Eq,
            hl::Instr::Numeric(hl::NumericOp::I64Ne) => ll::Instruction::I64Ne,
            hl::Instr::Numeric(hl::NumericOp::I64LtS) => ll::Instruction::I64LtS,
            hl::Instr::Numeric(hl::NumericOp::I64LtU) => ll::Instruction::I64LtU,
            hl::Instr::Numeric(hl::NumericOp::I64GtS) => ll::Instruction::I64GtS,
            hl::Instr::Numeric(hl::NumericOp::I64GtU) => ll::Instruction::I64GtU,
            hl::Instr::Numeric(hl::NumericOp::I64LeS) => ll::Instruction::I64LeS,
            hl::Instr::Numeric(hl::NumericOp::I64LeU) => ll::Instruction::I64LeU,
            hl::Instr::Numeric(hl::NumericOp::I64GeS) => ll::Instruction::I64GeS,
            hl::Instr::Numeric(hl::NumericOp::I64GeU) => ll::Instruction::I64GeU,
            hl::Instr::Numeric(hl::NumericOp::F32Eq) => ll::Instruction::F32Eq,
            hl::Instr::Numeric(hl::NumericOp::F32Ne) => ll::Instruction::F32Ne,
            hl::Instr::Numeric(hl::NumericOp::F32Lt) => ll::Instruction::F32Lt,
            hl::Instr::Numeric(hl::NumericOp::F32Gt) => ll::Instruction::F32Gt,
            hl::Instr::Numeric(hl::NumericOp::F32Le) => ll::Instruction::F32Le,
            hl::Instr::Numeric(hl::NumericOp::F32Ge) => ll::Instruction::F32Ge,
            hl::Instr::Numeric(hl::NumericOp::F64Eq) => ll::Instruction::F64Eq,
            hl::Instr::Numeric(hl::NumericOp::F64Ne) => ll::Instruction::F64Ne,
            hl::Instr::Numeric(hl::NumericOp::F64Lt) => ll::Instruction::F64Lt,
            hl::Instr::Numeric(hl::NumericOp::F64Gt) => ll::Instruction::F64Gt,
            hl::Instr::Numeric(hl::NumericOp::F64Le) => ll::Instruction::F64Le,
            hl::Instr::Numeric(hl::NumericOp::F64Ge) => ll::Instruction::F64Ge,
            hl::Instr::Numeric(hl::NumericOp::I32Clz) => ll::Instruction::I32Clz,
            hl::Instr::Numeric(hl::NumericOp::I32Ctz) => ll::Instruction::I32Ctz,
            hl::Instr::Numeric(hl::NumericOp::I32Popcnt) => ll::Instruction::I32Popcnt,
            hl::Instr::Numeric(hl::NumericOp::I32Add) => ll::Instruction::I32Add,
            hl::Instr::Numeric(hl::NumericOp::I32Sub) => ll::Instruction::I32Sub,
            hl::Instr::Numeric(hl::NumericOp::I32Mul) => ll::Instruction::I32Mul,
            hl::Instr::Numeric(hl::NumericOp::I32DivS) => ll::Instruction::I32DivS,
            hl::Instr::Numeric(hl::NumericOp::I32DivU) => ll::Instruction::I32DivU,
            hl::Instr::Numeric(hl::NumericOp::I32RemS) => ll::Instruction::I32RemS,
            hl::Instr::Numeric(hl::NumericOp::I32RemU) => ll::Instruction::I32RemU,
            hl::Instr::Numeric(hl::NumericOp::I32And) => ll::Instruction::I32And,
            hl::Instr::Numeric(hl::NumericOp::I32Or) => ll::Instruction::I32Or,
            hl::Instr::Numeric(hl::NumericOp::I32Xor) => ll::Instruction::I32Xor,
            hl::Instr::Numeric(hl::NumericOp::I32Shl) => ll::Instruction::I32Shl,
            hl::Instr::Numeric(hl::NumericOp::I32ShrS) => ll::Instruction::I32ShrS,
            hl::Instr::Numeric(hl::NumericOp::I32ShrU) => ll::Instruction::I32ShrU,
            hl::Instr::Numeric(hl::NumericOp::I32Rotl) => ll::Instruction::I32Rotl,
            hl::Instr::Numeric(hl::NumericOp::I32Rotr) => ll::Instruction::I32Rotr,
            hl::Instr::Numeric(hl::NumericOp::I64Clz) => ll::Instruction::I64Clz,
            hl::Instr::Numeric(hl::NumericOp::I64Ctz) => ll::Instruction::I64Ctz,
            hl::Instr::Numeric(hl::NumericOp::I64Popcnt) => ll::Instruction::I64Popcnt,
            hl::Instr::Numeric(hl::NumericOp::I64Add) => ll::Instruction::I64Add,
            hl::Instr::Numeric(hl::NumericOp::I64Sub) => ll::Instruction::I64Sub,
            hl::Instr::Numeric(hl::NumericOp::I64Mul) => ll::Instruction::I64Mul,
            hl::Instr::Numeric(hl::NumericOp::I64DivS) => ll::Instruction::I64DivS,
            hl::Instr::Numeric(hl::NumericOp::I64DivU) => ll::Instruction::I64DivU,
            hl::Instr::Numeric(hl::NumericOp::I64RemS) => ll::Instruction::I64RemS,
            hl::Instr::Numeric(hl::NumericOp::I64RemU) => ll::Instruction::I64RemU,
            hl::Instr::Numeric(hl::NumericOp::I64And) => ll::Instruction::I64And,
            hl::Instr::Numeric(hl::NumericOp::I64Or) => ll::Instruction::I64Or,
            hl::Instr::Numeric(hl::NumericOp::I64Xor) => ll::Instruction::I64Xor,
            hl::Instr::Numeric(hl::NumericOp::I64Shl) => ll::Instruction::I64Shl,
            hl::Instr::Numeric(hl::NumericOp::I64ShrS) => ll::Instruction::I64ShrS,
            hl::Instr::Numeric(hl::NumericOp::I64ShrU) => ll::Instruction::I64ShrU,
            hl::Instr::Numeric(hl::NumericOp::I64Rotl) => ll::Instruction::I64Rotl,
            hl::Instr::Numeric(hl::NumericOp::I64Rotr) => ll::Instruction::I64Rotr,
            hl::Instr::Numeric(hl::NumericOp::F32Abs) => ll::Instruction::F32Abs,
            hl::Instr::Numeric(hl::NumericOp::F32Neg) => ll::Instruction::F32Neg,
            hl::Instr::Numeric(hl::NumericOp::F32Ceil) => ll::Instruction::F32Ceil,
            hl::Instr::Numeric(hl::NumericOp::F32Floor) => ll::Instruction::F32Floor,
            hl::Instr::Numeric(hl::NumericOp::F32Trunc) => ll::Instruction::F32Trunc,
            hl::Instr::Numeric(hl::NumericOp::F32Nearest) => ll::Instruction::F32Nearest,
            hl::Instr::Numeric(hl::NumericOp::F32Sqrt) => ll::Instruction::F32Sqrt,
            hl::Instr::Numeric(hl::NumericOp::F32Add) => ll::Instruction::F32Add,
            hl::Instr::Numeric(hl::NumericOp::F32Sub) => ll::Instruction::F32Sub,
            hl::Instr::Numeric(hl::NumericOp::F32Mul) => ll::Instruction::F32Mul,
            hl::Instr::Numeric(hl::NumericOp::F32Div) => ll::Instruction::F32Div,
            hl::Instr::Numeric(hl::NumericOp::F32Min) => ll::Instruction::F32Min,
            hl::Instr::Numeric(hl::NumericOp::F32Max) => ll::Instruction::F32Max,
            hl::Instr::Numeric(hl::NumericOp::F32Copysign) => ll::Instruction::F32Copysign,
            hl::Instr::Numeric(hl::NumericOp::F64Abs) => ll::Instruction::F64Abs,
            hl::Instr::Numeric(hl::NumericOp::F64Neg) => ll::Instruction::F64Neg,
            hl::Instr::Numeric(hl::NumericOp::F64Ceil) => ll::Instruction::F64Ceil,
            hl::Instr::Numeric(hl::NumericOp::F64Floor) => ll::Instruction::F64Floor,
            hl::Instr::Numeric(hl::NumericOp::F64Trunc) => ll::Instruction::F64Trunc,
            hl::Instr::Numeric(hl::NumericOp::F64Nearest) => ll::Instruction::F64Nearest,
            hl::Instr::Numeric(hl::NumericOp::F64Sqrt) => ll::Instruction::F64Sqrt,
            hl::Instr::Numeric(hl::NumericOp::F64Add) => ll::Instruction::F64Add,
            hl::Instr::Numeric(hl::NumericOp::F64Sub) => ll::Instruction::F64Sub,
            hl::Instr::Numeric(hl::NumericOp::F64Mul) => ll::Instruction::F64Mul,
            hl::Instr::Numeric(hl::NumericOp::F64Div) => ll::Instruction::F64Div,
            hl::Instr::Numeric(hl::NumericOp::F64Min) => ll::Instruction::F64Min,
            hl::Instr::Numeric(hl::NumericOp::F64Max) => ll::Instruction::F64Max,
            hl::Instr::Numeric(hl::NumericOp::F64Copysign) => ll::Instruction::F64Copysign,
            hl::Instr::Numeric(hl::NumericOp::I32WrapI64) => ll::Instruction::I32WrapI64,
            hl::Instr::Numeric(hl::NumericOp::I32TruncF32S) => ll::Instruction::I32TruncF32S,
            hl::Instr::Numeric(hl::NumericOp::I32TruncF32U) => ll::Instruction::I32TruncF32U,
            hl::Instr::Numeric(hl::NumericOp::I32TruncF64S) => ll::Instruction::I32TruncF64S,
            hl::Instr::Numeric(hl::NumericOp::I32TruncF64U) => ll::Instruction::I32TruncF64U,
            hl::Instr::Numeric(hl::NumericOp::I64ExtendI32S) => ll::Instruction::I64ExtendI32S,
            hl::Instr::Numeric(hl::NumericOp::I64ExtendI32U) => ll::Instruction::I64ExtendI32U,
            hl::Instr::Numeric(hl::NumericOp::I64TruncF32S) => ll::Instruction::I64TruncF32S,
            hl::Instr::Numeric(hl::NumericOp::I64TruncF32U) => ll::Instruction::I64TruncF32U,
            hl::Instr::Numeric(hl::NumericOp::I64TruncF64S) => ll::Instruction::I64TruncF64S,
            hl::Instr::Numeric(hl::NumericOp::I64TruncF64U) => ll::Instruction::I64TruncF64U,
            hl::Instr::Numeric(hl::NumericOp::F32ConvertI32S) => ll::Instruction::F32ConvertI32S,
            hl::Instr::Numeric(hl::NumericOp::F32ConvertI32U) => ll::Instruction::F32ConvertI32U,
            hl::Instr::Numeric(hl::NumericOp::F32ConvertI64S) => ll::Instruction::F32ConvertI64S,
            hl::Instr::Numeric(hl::NumericOp::F32ConvertI64U) => ll::Instruction::F32ConvertI64U,
            hl::Instr::Numeric(hl::NumericOp::F32DemoteF64) => ll::Instruction::F32DemoteF64,
            hl::Instr::Numeric(hl::NumericOp::F64ConvertI32S) => ll::Instruction::F64ConvertI32S,
            hl::Instr::Numeric(hl::NumericOp::F64ConvertI32U) => ll::Instruction::F64ConvertI32U,
            hl::Instr::Numeric(hl::NumericOp::F64ConvertI64S) => ll::Instruction::F64ConvertI64S,
            hl::Instr::Numeric(hl::NumericOp::F64ConvertI64U) => ll::Instruction::F64ConvertI64U,
            hl::Instr::Numeric(hl::NumericOp::F64PromoteF32) => ll::Instruction::F64PromoteF32,
            hl::Instr::Numeric(hl::NumericOp::I32ReinterpretF32) => ll::Instruction::I32ReinterpretF32,
            hl::Instr::Numeric(hl::NumericOp::I64ReinterpretF64) => ll::Instruction::I64ReinterpretF64,
            hl::Instr::Numeric(hl::NumericOp::F32ReinterpretI32) => ll::Instruction::F32ReinterpretI32,
            hl::Instr::Numeric(hl::NumericOp::F64ReinterpretI64) => ll::Instruction::F64ReinterpretI64,
        }
    }

    /// Encode the types in the order in that we gave indices to them.
    fn encode_types(state: &EncodeState) -> ll::TypeSection {
        let mut type_section = ll::TypeSection::new();
        
        let mut types_ordered: Vec<(&FunctionType, Idx<marker::ll::FunctionType>)> = state.types_idx.iter().map(|(t, i)| (t, *i)).collect();
        types_ordered.sort_unstable_by_key(|&(_, idx)| idx);
        assert_eq!(
            state.types_idx.len(),
            types_ordered.last().map(|(_, idx)| idx.into_inner() + 1).unwrap_or(0),
            "type index space should not have any holes, mapping: {:?}",
            state.types_idx
        );
        for (type_, _) in types_ordered {
            type_section.function(
                type_.params.iter().copied().map(ll::ValType::from),
                type_.results.iter().copied().map(ll::ValType::from)
            );
        }

        type_section
    }

    impl From<crate::GlobalType> for ll::GlobalType {
        fn from(hl_global_type: crate::GlobalType) -> Self {
            Self {
                val_type: hl_global_type.0.into(),
                mutable: match hl_global_type.1 {
                    crate::Mutability::Const => false,
                    crate::Mutability::Mut => true,
                },
            }
        }
    }

    impl From<crate::TableType> for ll::TableType {
        fn from(hl_table_type: crate::TableType) -> Self {
            Self {
                element_type: ll::ValType::FuncRef,
                minimum: hl_table_type.1.initial_size,
                maximum: hl_table_type.1.max_size,
            }
        }
    }

    impl From<crate::MemoryType> for ll::MemoryType {
        fn from(hl_memory_type: crate::MemoryType) -> Self {
            Self {
                minimum: hl_memory_type.0.initial_size.try_into().expect("u32 to u64 should always succeed"),
                maximum: hl_memory_type.0.max_size.map(|u32| u32.try_into().expect("u32 to u64 should always succeed")),
                memory64: false,
            }
        }
    }

    impl From<crate::ValType> for ll::ValType {
        fn from(hl_val_type: crate::ValType) -> Self {
            use crate::ValType::*;
            match hl_val_type {
                I32 => ll::ValType::I32,
                I64 => ll::ValType::I64,
                F32 => ll::ValType::F32,
                F64 => ll::ValType::F64,
            }
        }
    }

    impl From<crate::BlockType> for ll::BlockType {
        fn from(hl_block_type: crate::BlockType) -> Self {
            match hl_block_type.0 {
                Some(val_type) => ll::BlockType::Result(val_type.into()),
                None => ll::BlockType::Empty,
            }
        }
    }

    impl From<crate::Memarg> for ll::MemArg {
        fn from(hl_memarg: crate::Memarg) -> Self {
            Self {
                offset: hl_memarg.offset.try_into().expect("u32 to u64 should always succeed"),
                align: hl_memarg.alignment_exp.into(),
                memory_index: 0,
            }
        }
    }
}

fn u32_to_usize(u: u32) -> usize {
    u.try_into().expect("u32 to usize should always succeed")
}
