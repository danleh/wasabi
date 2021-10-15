use std::cmp::max;
use std::convert::TryInto;
use std::path::Path;
use std::{io, iter};

use wasmparser::{BinaryReaderError, Chunk, Parser, Payload, TypeDef};

use crate::{FunctionType, ValType};
use crate::error::{AddErrInfo, Error};
use crate::highlevel::{Code, Function, ImportOrPresent, Module};
use crate::lowlevel::Offsets;

/// 64 KiB, the minimum amount of bytes read in one chunk from the input reader.
const MIN_READ_SIZE: usize = 64 * 1024;

pub fn parse_module_with_offsets<R: io::Read>(
    mut reader: R,
) -> Result<(Module, Offsets), BinaryReaderError> {
    // TODO Streaming reading: read only 8-64 KiB chunks of the reader, use `Parser::parser()`.
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();

    let mut module = Module::default();

    let mut types = Vec::new();

    let offset = 0;
    for payload in Parser::new(offset).parse_all(&buf) {
        let payload = payload?;
        println!("{:?}", payload);

        match payload {
            Payload::Version { .. } => {}
            Payload::TypeSection(mut reader) => {
                let count = reader.get_count();
                types = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    let ty = reader.read()?;
                    match ty {
                        TypeDef::Func(ty) => types.push(convert_func_ty(ty)),
                        TypeDef::Instance(_) => todo!(),
                        TypeDef::Module(_) => todo!(),
                    }
                }
                assert_eq!(types.len(), count as usize);
            },
            Payload::ImportSection(_) => {},
            Payload::AliasSection(_) => {},
            Payload::InstanceSection(_) => {},
            Payload::FunctionSection(mut reader) => {
                let count = reader.get_count();
                module.functions = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    let ty_i = reader.read()? as usize;
                    let type_ = types.get(ty_i).unwrap().clone();
                    module.functions.push(Function { // TODO use Function::new()
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
            },
            Payload::TableSection(_) => {},
            Payload::MemorySection(_) => {},
            Payload::TagSection(_) => {},
            Payload::GlobalSection(_) => {},
            Payload::ExportSection(_) => {},
            Payload::StartSection { func, range } => {
                module.start = Some(func.into())
            },
            Payload::ElementSection(_) => {},
            Payload::DataCountSection { count, range } => {},
            Payload::DataSection(_) => {},
            Payload::CustomSection {
                name,
                data_offset,
                data,
                range,
            } => {},
            Payload::CodeSectionStart { count, range, size } => {},
            Payload::CodeSectionEntry(_) => {},
            Payload::ModuleSectionStart { count, range, size } => {},
            Payload::ModuleSectionEntry { parser, range } => {},
            Payload::UnknownSection {
                id,
                contents,
                range,
            } => {},
            Payload::End => {},
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
        params: ty.params.iter().cloned().map(convert_ty).collect::<Vec<_>>().into(),
        results: ty.returns.iter().cloned().map(convert_ty).collect::<Vec<_>>().into(),
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
