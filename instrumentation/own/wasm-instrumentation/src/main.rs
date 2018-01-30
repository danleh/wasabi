#![feature(attr_literals)]

#[macro_use]
extern crate parse_wasm_derive;

extern crate byteorder;
extern crate leb128;

use std::fs::File;
use std::io::{self, BufReader, Error};
use std::io::ErrorKind::InvalidData;
use byteorder::{ReadBytesExt, LittleEndian};

pub trait ParseWasm: Sized {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self>;

    /// convenience method
    fn error<E>(reason: E) -> io::Result<Self>
        where E: Into<Box<std::error::Error + Send + Sync>>
    {
        Err(Error::new(InvalidData, reason))
    }
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
            Err(leb128::read::Error::Overflow) => Self::error("leb128 to u32 overflow"),
            Ok(value) if value > u32::max_value() as u64 => Self::error("leb128 to u32 overflow"),
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
            Err(e) => Self::error(format!("utf-8 conversion error: {}", e.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct WithSize<T>(u32, T);

impl<T: ParseWasm> ParseWasm for WithSize<T> {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let size = u32::parse(reader)?;
        // TODO parallelize section and/or function decoding by jumping forward size bytes
        // we can do this generically in here: read size into a buf, spawn rayon work-stealing
        // thread for rest of the parsing in the buf
        Ok(WithSize(size, T::parse(reader)?))
    }
}

#[derive(Debug)]
pub struct Module {
    version: u32,
    sections: Vec<Section>,
}

#[derive(ParseWasm, Debug)]
pub enum Section {
    #[tag = 1] Type(WithSize<Vec<FuncType>>),
    #[tag = 2] Import(WithSize<Vec<Import>>),
    #[tag = 3] Function(WithSize<Vec<TypeIdx>>),
    #[tag = 8] Start(WithSize<FuncIdx>),
    #[tag = 10] Code(WithSize<Vec<WithSize<Func>>>),
}

#[derive(ParseWasm, Debug)]
#[tag = 0x60]
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
pub struct Import {
    module: String,
    name: String,
    type_: ImportType,
}

#[derive(ParseWasm, Debug)]
pub enum ImportType {
    #[tag = 0x0] Function(TypeIdx),
}

#[derive(ParseWasm, Debug, PartialEq)]
pub struct TypeIdx(u32);

#[derive(ParseWasm, Debug, PartialEq)]
pub struct FuncIdx(u32);

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
    #[tag = 0x10] Call(FuncIdx),

    #[tag = 0x1a] Drop,
    #[tag = 0x1b] Select,

    #[tag = 0x41] I32Const(u32),
}

#[derive(ParseWasm, Debug)]
pub struct Memarg {
    alignment: u32,
    offset: u32,
}

impl ParseWasm for Module {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if &magic_number != b"\0asm" {
            return Self::error("magic bytes do not match");
        }

        let version = reader.read_u32::<LittleEndian>()?;
        if version != 1 {
            return Self::error("not version 1");
        }

        let mut sections = Vec::new();
        loop {
            match Section::parse(reader) {
                Ok(section) => sections.push(section),
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e)
            };
        }

        Ok(Module { version, sections })
    }
}

impl ParseWasm for Func {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
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
    let file = File::open("test/hello-manual.wasm").unwrap();
    let mut buf_reader = BufReader::new(file);
    println!("{:#?}", Module::parse(&mut buf_reader).map_err(|err| err.to_string()));
}
