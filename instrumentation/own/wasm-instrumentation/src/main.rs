extern crate byteorder;
extern crate leb128;

use std::fs::File;
use std::io::{self, Read, BufReader};

pub trait ReadWasmExt: io::Read + Sized {
    fn read_byte(&mut self) -> Result<u8, io::Error> {
        use byteorder::ReadBytesExt;
        self.read_u8()
    }

    fn read_u32_leb128(&mut self) -> Result<u32, leb128::read::Error> {
        let value = leb128::read::unsigned(self)?;
        if value > u32::max_value() as u64 {
            Err(leb128::read::Error::Overflow)
        } else {
            Ok(value as u32)
        }
    }
}

impl<R: io::Read> ReadWasmExt for R {}

#[derive(Debug)]
pub struct Module {
    version: u32,
    sections: Vec<Section>
}

#[derive(Debug)]
pub enum Section {
    Type(Vec<FuncType>)
}

#[derive(Debug)]
pub struct FuncType;

fn main() {
    let file = File::open("test/type-func.wasm").unwrap();
    let mut buf_reader = BufReader::new(file);

//    for byte in buf_reader.bytes() {
//        println!("{:?}", byte.unwrap());
//    }

    let mut magic_number = [0u8; 4];
    buf_reader.read_exact(&mut magic_number).unwrap();
    assert_eq!(&magic_number, b"\0asm", "magic bytes do not match");

    use byteorder::{ReadBytesExt, LittleEndian};
    let version = buf_reader.read_u32::<LittleEndian>().unwrap();
    assert_eq!(version, 1, "not version 1");

    println!("{:?}", parse_section(&mut buf_reader));

    println!("{:?}", Module { version, sections: Vec::new() })
}

fn parse_section<R: Read>(reader: &mut R) -> Section {
    let section_type = reader.read_byte().unwrap();
    let section_size = reader.read_u32_leb128().unwrap();
    println!("type: {}, size: {}", section_type, section_size);
    match section_type{
        1 => parse_type_section(&mut reader.take(section_size as u64)),
        _ => unimplemented!()
    }
}

fn parse_type_section<R: Read>(reader: &mut R) -> Section {
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