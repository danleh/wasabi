#![feature(attr_literals)]

#[macro_use]
extern crate wasm_derive;
extern crate byteorder;
extern crate rayon;

use byteorder::{ReadBytesExt, WriteBytesExt};
use rayon::prelude::*;
use std::io;

// TODO test with WASM spec test suite

mod leb128;
use leb128::{Leb128, ReadLeb128, WriteLeb128};

mod ast;
use ast::*;

macro_rules! debug {
    ( $fmt:expr, $( $args:expr ),* ) => {
        let should_output = std::env::args().nth(2).is_none(); // give "silent" or so as second argument
        if should_output {
            println!($fmt, $( $args ),* );
        }
    };
}

pub trait Wasm: Sized {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self>;
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize>;

    /// convenience method
    fn error<E>(reason: E) -> io::Result<Self>
        where E: Into<Box<std::error::Error + Send + Sync>>
    {
        Err(io::Error::new(io::ErrorKind::InvalidData, reason))
    }
}

impl Wasm for u8 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_u8()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_u8(*self)?;
        Ok(1)
    }
}

impl Wasm for Leb128<u32> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_leb128()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_leb128(self)
    }
}

impl Wasm for Leb128<i32> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_leb128()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_leb128(self)
    }
}

impl Wasm for Leb128<i64> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_leb128()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_leb128(self)
    }
}

impl Wasm for f32 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_f32::<byteorder::LittleEndian>()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_f32::<byteorder::LittleEndian>(*self)?;
        Ok(4)
    }
}

impl Wasm for f64 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_f64::<byteorder::LittleEndian>()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_f64::<byteorder::LittleEndian>(*self)?;
        Ok(8)
    }
}


impl<T: Wasm> Wasm for WithSize<T> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(WithSize {
            size: Leb128::decode(reader)?,
            content: T::decode(reader)?,
        })
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        // write contents to buffer to know size
        let mut buf = Vec::with_capacity(self.size.value as usize);
        let new_size = self.content.encode(&mut buf)?;

        // write size, then contents from buffer to actual writer
        let mut bytes_written = self.size.map(new_size as u32).encode(writer)?;
        writer.write_all(&buf)?;
        bytes_written += new_size;

        Ok(bytes_written)
    }
}

impl<T: Wasm> Wasm for Leb128<Vec<T>> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let size: Leb128<u32> = Leb128::decode(reader)?;

        let mut vec: Vec<T> = Vec::with_capacity(size.value as usize);
        for _ in 0..size.value {
            vec.push(T::decode(reader)?);
        };

        Ok(size.map(vec))
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let new_size = self.len() as u32;

        let mut bytes_written = self.map(new_size).encode(writer)?;
        for element in self.iter() {
            bytes_written += element.encode(writer)?;
        }

        Ok(bytes_written)
    }
}

impl Wasm for Leb128<String> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let buf: Leb128<Vec<u8>> = Leb128::decode(reader)?;
        match String::from_utf8(buf.value) {
            Ok(string) => Ok(Leb128 {
                value: string,
                byte_count: buf.byte_count,
            }),
            Err(e) => Self::error(format!("utf-8 conversion error: {}", e.to_string())),
        }
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let new_size = self.len() as u32;

        let mut bytes_written = self.map(new_size).encode(writer)?;
        for byte in self.bytes() {
            bytes_written += byte.encode(writer)?;
        }

        Ok(bytes_written)
    }
}

impl<T: Wasm + Send + Sync> Wasm for Parallel<T> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let num_elements: Leb128<u32> = Leb128::decode(reader)?;
        let mut bufs = Vec::with_capacity(num_elements.value as usize);

        // read all elements into buffers of the given size (non-parallel, but hopefully fast)
        for _ in 0..num_elements.value {
            let num_bytes = Leb128::decode(reader)?;
            let mut buf = vec![0u8; num_bytes.value as usize];
            reader.read_exact(&mut buf)?;
            bufs.push(WithSize {
                size: num_bytes,
                content: buf,
            });
        }

        // parallel decode of each buffer
        let decoded: io::Result<Vec<WithSize<T>>> = bufs.into_par_iter()
            .map(|buf| -> io::Result<_> {
                Ok(WithSize {
                    size: buf.size,
                    content: T::decode(&mut &buf.content[..])?,
                })
            })
            .collect();
        let decoded = decoded?;

        Ok(Parallel(num_elements.map(decoded)))
    }

    // TODO refactor this to be sure no WithSize is forgotten or superfluous
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let vec = &self.0;
        let new_size = vec.map(vec.len() as u32);
        let mut bytes_written = new_size.encode(writer)?;

        // encode elements to buffers in parallel
        let encoded: io::Result<Vec<WithSize<Vec<u8>>>> = vec.par_iter()
            .map(|element| {
                let mut buf = Vec::with_capacity(element.size.value as usize);
                let new_size = element.content.encode(&mut buf)?;
                Ok(WithSize {
                    size: element.size.map(new_size as u32),
                    content: buf,
                })
            })
            .collect();

        // write sizes and buffer contents to actual writer (non-parallel, but hopefully fast)
        for buf in encoded? {
            bytes_written += buf.size.encode(writer)?;
            writer.write_all(&buf.content)?;
            bytes_written += buf.size.value as usize; // FIXME double cast, first to u32, now back
        }

        Ok(bytes_written)
    }
}

/// have to implement manually because of strange compressed format:
/// no tag, because they know that 0x40 and ValType are disjoint
impl Wasm for BlockType {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(BlockType(match u8::decode(reader)? {
            0x40 => None,
            byte => {
                let mut buf = [byte; 1];
                Some(ValType::decode(&mut &buf[..])?)
            }
        }))
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        match self {
            &BlockType(None) => 0x40u8.encode(writer),
            &BlockType(Some(ref val_type)) => val_type.encode(writer)
        }
    }
}

impl Wasm for Module {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if &magic_number != b"\0asm" {
            return Self::error("magic bytes do not match");
        }

        let version = reader.read_u32::<byteorder::LittleEndian>()?;
        if version != 1 {
            return Self::error("not version 1");
        }

        let mut sections = Vec::new();
        loop {
            match Section::decode(reader) {
                Ok(section) => {
                    debug!("decoded section: {:?}", section);
                    sections.push(section)
                }
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e)
            };
        }

        Ok(Module { version, sections })
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_all(b"\0asm")?;
        writer.write_all(&[1, 0, 0, 0])?;
        let mut bytes_written = 8;
        for section in &self.sections {
            bytes_written += section.encode(writer)?;
        }
        Ok(bytes_written)
    }
}

impl Wasm for Expr {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut instructions = Vec::new();

        let mut found_end = false;
        while !found_end {
            let instr = Instr::decode(reader)?;

            match instr {
                Instr::Else(..) | Instr::End => found_end = true,
                _ => {}
            };

            instructions.push(instr);
        }

        Ok(Expr(instructions))
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let mut bytes_written = 0;
        for instruction in &self.0 {
            bytes_written += instruction.encode(writer)?;
        }
        Ok(bytes_written)
    }
}

fn main() {
    let default_file_name = "test/hello-emcc.wasm";
    let file_name = std::env::args().nth(1).unwrap_or(default_file_name.into());

    std::process::exit(match || -> io::Result<()> {
        use std::fs::File;
        let mut buf_reader = io::BufReader::new(File::open(&file_name)?);
        let module = Module::decode(&mut buf_reader)?;
        debug!("{:#?}", module);

        // TODO implement actual instrumentation, not just this dummy function add
        // match module.sections[0] {
        //     Section::Type(ref mut _0) => _0.content.push(FuncType {params: Vec::new(), results: Vec::new()}),
        //     _ => {}
        // };

        let encoded_file_name = file_name.replace(".wasm", ".encoded.wasm");
        let mut buf_writer = io::BufWriter::new(File::create(&encoded_file_name)?);
        let bytes_written = module.encode(&mut buf_writer)?;
        println!("written encoded Module to {}, {} bytes", encoded_file_name, bytes_written);
        Ok(())
    }() {
        Ok(_) => 0,
        Err(ref e) => {
            eprintln!("Error: {}", e);
            1
        }
    });
}
