extern crate byteorder;
extern crate leb128;

use std::fs::File;
use std::io::{self, BufReader, Error};
use std::io::ErrorKind::InvalidData;
use byteorder::{ReadBytesExt, LittleEndian};

pub trait ReadWasmExt: io::Read + Sized {
    fn read_byte(&mut self) -> io::Result<u8> {
        use byteorder::ReadBytesExt;
        self.read_u8()
    }

    fn read_u32_leb128(&mut self) -> io::Result<u32> {
        match leb128::read::unsigned(self) {
            Err(leb128::read::Error::IoError(io_err)) => Err(io_err),
            Err(leb128::read::Error::Overflow) => Err(Error::new(InvalidData, "overflow")),
            Ok(value) if value > u32::max_value() as u64 => Err(Error::new(InvalidData, "overflow")),
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
    Type(Vec<FuncType>)
}

#[derive(Debug)]
pub struct FuncType;

trait ParseWasm: Sized {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self>;
}

impl ParseWasm for Module {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if &magic_number != b"\0asm" {
            return Err(Error::new(InvalidData, "magic bytes do not match"));
        }

        let version = reader.read_u32::<LittleEndian>()?;
        if version != 1 {
            return Err(io::Error::new(InvalidData, "not version 1"));
        }

        Ok(Module {
            version,
            sections: vec![],
        })
    }
}

impl ParseWasm for Section {
    fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let type_ = reader.read_byte()?;
        let size = reader.read_u32_leb128()?;

        // TODO parallelize by jumping forward size bytes for each section

        match type_ {
            1 => Ok(Section::Type(vec![])),
            _ => unimplemented!()
        }
    }
}

fn main() {
    let file = File::open("test/type-func.wasm").unwrap();
    let mut buf_reader = BufReader::new(file);
    println!("{:?}", Module::parse(&mut buf_reader));
}
//
//fn parse_section<R: io::Read>(reader: &mut R) -> Section {
//    let section_type = reader.read_byte().unwrap();
//    let section_size = reader.read_u32_leb128().unwrap();
//    println!("type: {}, size: {}", section_type, section_size);
//    match section_type{
//        1 => parse_type_section(&mut reader.take(section_size as u64)),
//        _ => unimplemented!()
//    }
//}

fn parse_type_section<R: io::Read>(reader: &mut R) -> Section {
    let num_funcs = reader.read_u32_leb128().unwrap();
    let mut funcs = Vec::new();
    println!("funcs: {}", num_funcs);
    // TODO implement vec() combinator
    for i in 0..num_funcs {
        assert_eq!(reader.read_byte().unwrap(), 0x60, "function type has incorrect byte");
        let num_params = reader.read_u32_leb128().unwrap();
        for j in 0..num_params {
            println!("param valtype: {}", reader.read_byte().unwrap());
        }
        let num_results = reader.read_u32_leb128().unwrap();
        for j in 0..num_results {
            println!("result valtype: {}", reader.read_byte().unwrap());
        }
        funcs.push(FuncType);
    }
    Section::Type(funcs)
}