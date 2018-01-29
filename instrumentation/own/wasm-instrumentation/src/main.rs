extern crate byteorder;
extern crate leb128;

use std::fs::File;
use std::io::{self, BufReader, Error};
use std::io::ErrorKind::InvalidData;
use byteorder::{ReadBytesExt, LittleEndian};

/// convenience method
fn wasm_error<T, E>(reason: E) -> io::Result<T>
    where E: Into<Box<std::error::Error + Send + Sync>>
{
    Err(Error::new(InvalidData, reason))
}

pub trait ReadWasmExt: io::Read + Sized {
    fn read_byte(&mut self) -> io::Result<u8> {
        use byteorder::ReadBytesExt;
        self.read_u8()
    }

    fn read_u32_leb128(&mut self) -> io::Result<u32> {
        match leb128::read::unsigned(self) {
            Err(leb128::read::Error::IoError(io_err)) => Err(io_err),
            Err(leb128::read::Error::Overflow) => wasm_error("overflow"),
            Ok(value) if value > u32::max_value() as u64 => wasm_error("overflow"),
            Ok(value) => Ok(value as u32),
        }
    }
}

impl<R: io::Read> ReadWasmExt for R {}

#[derive(Debug)]
pub struct Module {
    version: u32,
    sections: Vec<Section>,
}

#[derive(Debug)]
pub enum Section {
    // TODO custom derive + some "tag" attribute to distinguish cases?
    Type(Vec<FuncType>),
    Function(Vec<TypeIdx>),
    Code(Vec<Code>),
}

#[derive(Debug)]
pub struct FuncType {
    params: Vec<ValType>,
    results: Vec<ValType>,
}

#[derive(Debug)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug)]
pub struct TypeIdx(u32);

#[derive(Debug)]
pub struct Code {
    func: Func
}

#[derive(Debug)]
pub struct Func {
    locals: Vec<Locals>,
    expr: Expr // TODO replace with Vec<Instr> since the Expr only contains the end byte 0x0b anyway
}

#[derive(Debug)]
pub struct Locals;

#[derive(Debug)]
pub struct Expr;

#[derive(Debug)]
pub enum Instr {
    Nop,
    // TODO some simple instr
}

trait ParseWasm: Sized {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self>;
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
        let type_ = reader.read_byte()?;
        // TODO parallelize by jumping forward size bytes for each section
        let _size = reader.read_u32_leb128()?;

        Ok(match type_ {
            1 => Section::Type(Vec::parse(reader)?),
            3 => Section::Function(Vec::parse(reader)?),
            10 => Section::Code(Vec::parse(reader)?),
            s => unimplemented!("section type {}", s)
        })
    }
}

impl<T: ParseWasm> ParseWasm for Vec<T> {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let size = reader.read_u32_leb128()?;
        let mut vec: Vec<T> = Vec::with_capacity(size as usize);
        for _ in 0..size {
            vec.push(T::parse(reader)?);
        };
        Ok(vec)
    }
}

impl ParseWasm for FuncType {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        if reader.read_byte()? != 0x60 {
            return wasm_error("wrong byte, expected functype");
        }

        let params = Vec::parse(reader)?;
        let results = Vec::parse(reader)?;

        Ok(FuncType { params, results })
    }
}

impl ParseWasm for ValType {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(match reader.read_byte()? {
            0x7f => ValType::I32,
            0x7e => ValType::I64,
            0x7d => ValType::F32,
            0x7c => ValType::F64,
            _ => wasm_error("wrong byte, expected valtype")?
        })
    }
}

impl ParseWasm for TypeIdx {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(TypeIdx(reader.read_u32_leb128()?))
    }
}

impl ParseWasm for Code {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let _size = reader.read_u32_leb128()?;
        // TODO parallelize function decoding by jumping forward _size bytes

        Ok(Code { func: Func::parse(reader)? })
    }
}

impl ParseWasm for Func {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Func {
            locals: Vec::parse(reader)?,
            expr: Expr::parse(reader)?
        })
    }
}

impl ParseWasm for Locals {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        unimplemented!()
    }
}

impl ParseWasm for Expr {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        loop {
            match reader.read_byte()? {
                0x0b => break,
                byte => println!("instr byte 0x{:02x}", byte) // FIXME
            }
        }
        Ok(Expr)
    }
}

fn main() {
    let file = File::open("test/type-func.wasm").unwrap();
    let mut buf_reader = BufReader::new(file);
    println!("{:?}", Module::parse(&mut buf_reader));
}
