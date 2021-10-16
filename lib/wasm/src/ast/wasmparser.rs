use std::cmp::max;
use std::convert::TryInto;
use std::path::Path;
use std::{fmt, io, iter};

use wasmparser::{BinaryReaderError, Chunk, ImportSectionEntryType, Parser, Payload, TypeDef};

use crate::error::{AddErrInfo, Error};
use crate::highlevel::{Code, Function, ImportOrPresent, Module};
use crate::lowlevel::Offsets;
use crate::{FunctionType, ValType};

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
    let mut types = Types::none();

    let offset = 0;
    for payload in Parser::new(offset).parse_all(&buf) {
        match payload? {
            Payload::Version { .. } => {}
            Payload::TypeSection(mut reader) => {
                let count = reader.get_count();
                types.set_capacity(count)?;
                for _ in 0..count {
                    let ty = reader.read()?;
                    match ty {
                        TypeDef::Func(ty) => types.add(ty)?,
                        TypeDef::Instance(_) | TypeDef::Module(_) => {
                            Err(UnsupportedError::new(WasmExtension::ModuleLinking))?
                        }
                    }
                }
            }
            Payload::ImportSection(mut reader) => {
                let count = reader.get_count();
                for _ in 0..count {
                    let import = reader.read()?;
                    match import.ty {
                        ImportSectionEntryType::Function(ty_i) => {
                            module.functions.push(Function::new_imported(
                                types.get(ty_i)?,
                                import.module.to_string(),
                                import.field
                                    .ok_or(UnsupportedError::new(WasmExtension::ModuleLinking))?
                                    .to_string(),
                                Vec::new(),
                            ))
                        }
                        // TODO continue here (see left side)
                        ImportSectionEntryType::Table(_) => todo!(),
                        ImportSectionEntryType::Memory(_) => todo!(),
                        ImportSectionEntryType::Tag(_) => todo!(),
                        ImportSectionEntryType::Global(_) => todo!(),
                        ImportSectionEntryType::Module(_) => todo!(),
                        ImportSectionEntryType::Instance(_) => todo!(),
                    }
                }
            }
            Payload::AliasSection(_) => {}
            Payload::InstanceSection(_) => {}
            Payload::FunctionSection(mut reader) => {
                let count = reader.get_count();
                module.functions = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    let ty_i = reader.read()?;
                    let type_ = types.get(ty_i)?;
                    module.functions.push(Function {
                        // TODO use Function::new()
                        type_,
                        code: ImportOrPresent::Present(Code {
                            locals: Vec::new(),
                            body: Vec::new(),
                        }),
                        export: Vec::new(),
                        name: None,
                        param_names: Vec::new(),
                    })
                }
            }
            Payload::TableSection(_) => {}
            Payload::MemorySection(_) => {}
            Payload::TagSection(_) => {}
            Payload::GlobalSection(_) => {}
            Payload::ExportSection(_) => {}
            Payload::StartSection { func, range } => module.start = Some(func.into()),
            Payload::ElementSection(_) => {}
            Payload::DataCountSection { count, range } => {}
            Payload::DataSection(_) => {}
            Payload::CustomSection {
                name,
                data_offset,
                data,
                range,
            } => {}
            Payload::CodeSectionStart { count, range, size } => {}
            Payload::CodeSectionEntry(_) => {}
            Payload::ModuleSectionStart { count, range, size } => {}
            Payload::ModuleSectionEntry { parser, range } => {}
            Payload::UnknownSection {
                id,
                contents,
                range,
            } => {}
            Payload::End => {}
        }
    }

    let offsets = Offsets {
        sections: Vec::new(),
        functions_code: Vec::new(),
    };

    Ok((module, offsets))
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

fn convert_ty(ty: wasmparser::Type) -> ValType {
    match ty {
        wasmparser::Type::I32 => ValType::I32,
        wasmparser::Type::I64 => ValType::I64,
        wasmparser::Type::F32 => ValType::F32,
        wasmparser::Type::F64 => ValType::F64,
        wasmparser::Type::V128 => todo!(),
        wasmparser::Type::FuncRef => todo!(),
        wasmparser::Type::ExternRef => todo!(),
        wasmparser::Type::ExnRef => todo!(),
        wasmparser::Type::Func => todo!(),
        wasmparser::Type::EmptyBlockType => todo!(),
    }
}

// impl<T> AddErrInfo<T> for Result<T, BinaryReaderError> {
//     fn add_err_info<GrammarElement>(self: Result<T, BinaryReaderError>, offset: usize) -> Result<T, Error> {
//         self.map_err(|err|
//             Error::with_source::<GrammarElement, _>(offset, ErrorKind::Leb128, err))
//     }
// }

#[derive(Debug)]
struct UnsupportedError {
    //     offset: usize,
    extension: WasmExtension,
}

impl std::error::Error for UnsupportedError {}

impl fmt::Display for UnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "This module uses a WebAssembly extension we don't support yet: {}",
            self.extension.name()
        )?;
        writeln!(
            f,
            "See {} for more information about the extension.",
            self.extension.url()
        )
    }
}

impl UnsupportedError {
    pub fn new(extension: WasmExtension) -> Self {
        UnsupportedError { extension }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum WasmExtension {
    ModuleLinking,
}

impl WasmExtension {
    pub fn name(self) -> &'static str {
        use WasmExtension::*;
        match self {
            ModuleLinking => "module linking",
        }
    }

    pub fn url(self) -> &'static str {
        use WasmExtension::*;
        match self {
            ModuleLinking => r"https://github.com/WebAssembly/module-linking",
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
        self.0.as_mut()
            .ok_or("missing type section")?
            .push(convert_func_ty(ty));
        Ok(())
    }

    // TODO typed error
    pub fn get(&self, idx: u32) -> Result<FunctionType, &'static str> {
        self.0.as_ref()
            .ok_or("missing type section")?
            .get(u32_to_usize(idx))
            .cloned()
            .ok_or("missing type at index ?")
    }
}

fn u32_to_usize(u: u32) -> usize {
    u.try_into().expect("u32 to usize should always succeed")
}
