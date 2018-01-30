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

#[derive(Debug)]
pub enum Section {
    Type(Vec<FuncType>),
    Function(Vec<TypeIdx>),
    Code(Vec<Func>),
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

#[derive(ParseWasm, Debug)]
pub enum Instr {
    #[tag = 0x00] Unreachable,
    #[tag = 0x01] Nop,
    // TODO https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions

    #[tag = 0x1a] Drop,
    #[tag = 0x1b] Select,
//    Const(i32)

// what I would want:
//    Const<T: ValType>(underlying<T>)
//    I32Const(u32) = 0x41,
}
/*
enum Test {
    #[tag = 0x41] I32Const(u32),
}
// should generate:
impl ParseWasm for Test {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(match reader.read_byte()? {
            0x41 => Test::I32Const(Memarg::parse(reader)?),
            byte => return wasm_error(format!("expected tag for Test, got 0x{:02x}", byte))
        })
    }
}
*/

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

impl ParseWasm for Section {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let type_ = u8::parse(reader)?;
        // TODO parallelize by jumping forward size bytes for each section
        let _size = u32::parse(reader)?;

        Ok(match type_ {
            1 => Section::Type(Vec::parse(reader)?),
            3 => Section::Function(Vec::parse(reader)?),
            10 => Section::Code(Vec::parse(reader)?),
            s => unimplemented!("section type {}", s)
        })
    }
}

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
        let _size = u32::parse(reader)?;
        // TODO parallelize function decoding by jumping forward _size bytes
        let locals = Vec::parse(reader)?;

        // instructions
        loop {
            match u8::parse(reader)? {
                0x0b => break,
                byte => {
                    println!("instr byte 0x{:02x}", byte);
//                    println!("{:?}", Instr::from_u8(byte))
                } // FIXME
            }
        }

        Ok(Func { locals, instructions: Vec::new() })
    }
}

fn main() {
    let file = File::open("test/type-func.wasm").unwrap();
    let mut buf_reader = BufReader::new(file);
    println!("{:?}", Module::parse(&mut buf_reader));
}
