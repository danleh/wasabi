#![feature(attr_literals)]

#[macro_use]
extern crate parse_wasm_derive;

extern crate byteorder;
extern crate leb128;

use std::fs::File;
use std::io::{self, BufReader, Error};
use std::io::ErrorKind::InvalidData;
use byteorder::{ReadBytesExt, LittleEndian};

trait ParseWasm: Sized {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self>;
}

impl ParseWasm for u8 {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        use byteorder::ReadBytesExt;
        reader.read_u8()
    }
}

// TODO save LEB128 encoding with u32 value to make sure decoding-encoding round-trips
impl ParseWasm for u32 {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        match leb128::read::unsigned(reader) {
            Err(leb128::read::Error::IoError(io_err)) => Err(io_err),
            Err(leb128::read::Error::Overflow) => wasm_error("leb128 to u32 overflow"),
            Ok(value) if value > u32::max_value() as u64 => wasm_error("leb128 to u32 overflow"),
            Ok(value) => Ok(value as u32),
        }
    }
}

impl<T: ParseWasm> ParseWasm for Vec<T> {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let size = u32::parse(reader)?;
        let mut vec: Vec<T> = Vec::with_capacity(size as usize);
        for _ in 0..size {
            vec.push(T::parse(reader)?);
        };
        Ok(vec)
    }
}

impl ParseWasm for String {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let size = u32::parse(reader)?;
        let mut buf = vec![0u8; size as usize];
        reader.read_exact(&mut buf)?;
        match String::from_utf8(buf) {
            Ok(str) => Ok(str),
            Err(e) => wasm_error(e.to_string()), // TODO better error message
        }
    }
}

/// convenience method
fn wasm_error<T, E>(reason: E) -> io::Result<T>
    where E: Into<Box<std::error::Error + Send + Sync>>
{
    Err(Error::new(InvalidData, reason))
}

#[derive(Debug)]
pub struct Module {
    version: u32,
    sections: Vec<Section>,
}

#[derive(ParseWasm, Debug)]
pub enum Section {
    #[tag = 1]
    Type(WithSize<Vec<FuncType>>),
    //    #[tag = 1] Import(u32, Vec<Import>),
    #[tag = 3] Function(u32, Vec<TypeIdx>),
    #[tag = 10] Code(u32, Vec<Func>),
}

#[derive(Debug)]
pub struct WithSize<T>(u32, T);
impl<T: ParseWasm> ParseWasm for WithSize<T> {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(WithSize(
            u32::parse(reader)?,
            T::parse(reader)?
        ))
    }
}

#[derive(Debug)]
pub struct FuncType {
    params: Vec<ValType>,
    results: Vec<ValType>,
}

#[derive(ParseWasm, Debug)]
pub enum ValType {
    #[tag = 0x7f] I32,
    #[tag = 0x7e] I64,
    #[tag = 0x7d] F32,
    #[tag = 0x7c] F64,
}

#[derive(ParseWasm, Debug)]
pub struct TypeIdx(u32);

#[derive(Debug)]
pub struct Func {
    locals: Vec<ValType>,
    instructions: Vec<Instr>,
}

#[derive(ParseWasm, Debug, PartialEq)]
pub enum Instr {
    #[tag = 0x0b] End, // inserted for easier handling of if/blocks/function ends

    #[tag = 0x00] Unreachable,
    #[tag = 0x01] Nop,

    #[tag = 0x1a] Drop,
    #[tag = 0x1b] Select,

    #[tag = 0x41] I32Const(u32),
}

#[derive(ParseWasm, Debug)]
struct Memarg {
    alignment: u32,
    offset: u32,
}

impl ParseWasm for Module {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if &magic_number != b"\0asm" {
            return wasm_error("magic bytes do not match");
        }

        let version = reader.read_u32::<LittleEndian>()?;
        if version != 1 {
            return wasm_error("not version 1");
        }

        let mut sections = Vec::new();
        loop {
            let section = Section::parse(reader);
            match section {
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                _ => {}
            };
            sections.push(section?);
        }

        Ok(Module { version, sections })
    }
}

//impl ParseWasm for Section {
//    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
//        let type_ = u8::parse(reader)?;
//        // TODO parallelize by jumping forward size bytes for each section
//        let _size = u32::parse(reader)?;
//
//        Ok(match type_ {
//            1 => Section::Type(Vec::parse(reader)?),
//            3 => Section::Function(Vec::parse(reader)?),
//            10 => Section::Code(Vec::parse(reader)?),
//            s => wasm_error(format!("unknown section type {}", s))?
//        })
//    }
//}

impl ParseWasm for FuncType {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        if u8::parse(reader)? != 0x60 {
            return wasm_error("wrong byte, expected functype");
        }

        Ok(FuncType {
            params: Vec::parse(reader)?,
            results: Vec::parse(reader)?,
        })
    }
}

impl ParseWasm for Func {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        // TODO parallelize function decoding by jumping forward _size bytes
        let _size = u32::parse(reader)?;

        let locals = Vec::parse(reader)?;
        let mut instructions = Vec::new();
        let mut blocks_to_end = 0;

        loop {
            let instr = Instr::parse(reader)?;

            if instr == Instr::End {
                blocks_to_end -= 1;
            }
            instructions.push(instr);
            if blocks_to_end < 0 {
                break;
            }
        }

        Ok(Func { locals, instructions })
    }
}

fn main() {
    let file = File::open("test/type-func.wasm").unwrap();
    let mut buf_reader = BufReader::new(file);
    println!("{:?}", Module::parse(&mut buf_reader).map_err(|err| err.to_string()));
}
