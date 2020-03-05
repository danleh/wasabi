use std::io;
use std::marker::PhantomData;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use wasabi_leb128::*;
use rayon::prelude::*;

use crate::ast::*;
use crate::ast::lowlevel::*;

/* Trait and impl for decoding/encoding between binary format (as per spec) and our own formats (see ast module) */

pub trait WasmBinary: Sized {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self>;
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize>;

    /// convenience method
    fn error<E>(reason: E) -> io::Result<Self>
        where E: Into<Box<dyn std::error::Error + Send + Sync>>
    {
        Err(io::Error::new(io::ErrorKind::InvalidData, reason))
    }
}


/* Primitive types */

impl WasmBinary for u8 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_u8()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_u8(*self)?;
        Ok(1)
    }
}

impl WasmBinary for u32 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_leb128()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_leb128(*self)
    }
}

impl WasmBinary for usize {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_leb128()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        if *self > u32::max_value() as usize {
            Self::error("WASM spec does not allow unsigned larger than u32")?;
        }
        writer.write_leb128(*self)
    }
}

impl WasmBinary for i32 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_leb128()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_leb128(*self)
    }
}

impl WasmBinary for i64 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_leb128()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_leb128(*self)
    }
}

impl WasmBinary for f32 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_f32::<LittleEndian>()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_f32::<LittleEndian>(*self)?;
        Ok(4)
    }
}

impl WasmBinary for f64 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        reader.read_f64::<LittleEndian>()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        writer.write_f64::<LittleEndian>(*self)?;
        Ok(8)
    }
}


/* Generic "AST combinators" */

impl<T: WasmBinary> WasmBinary for WithSize<T> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let _forget_original_size = u32::decode(reader)?;
        Ok(WithSize(T::decode(reader)?))
    }

    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let mut buf = Vec::new();
        let new_size = self.0.encode(&mut buf)?;

        // write new size, then contents from buffer to actual writer
        let mut bytes_written = new_size.encode(writer)?;
        writer.write_all(&buf)?;
        bytes_written += new_size;

        Ok(bytes_written)
    }
}

/// Do not blindly trust the decoded element_count for pre-allocating a vector, but instead limit
/// the pre-allocation to some sensible size (e.g., 1 MB).
/// Otherwise a (e.g., malicious) Wasm file could request very large amounts of memory just by
/// having a large vec-size in the binary.
/// (We got struck by plenty such out of memory errors when testing our Wasm parser with AFL.
/// See tests/invalid/oom-large-vector-size/oom.wasm)
fn limit_prealloc_capacity<T>(element_count: usize) -> usize {
    // Limit to 1 MB, which should be hit almost never for real (benign) Wasm binaries:
    // The Wasm vectors with the largest number of elements are most likely bodies of functions
    // (i.e., vectors of instructions), and a single function is likely not having that many
    // instructions.
    const PREALLOC_LIMIT_BYTES: usize = 1 << 20;
    // Vec::with_capacity() takes number of elements, not bytes; so divide bytes by element size.
    let element_limit = PREALLOC_LIMIT_BYTES / size_of::<T>();
    std::cmp::min(element_count, element_limit)
}

// see https://webassembly.github.io/spec/core/binary/conventions.html#vectors
impl<T: WasmBinary> WasmBinary for Vec<T> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let element_count = usize::decode(reader)?;

        let mut vec: Vec<T> = Vec::with_capacity(limit_prealloc_capacity::<T>(element_count));
        for _ in 0..element_count {
            vec.push(T::decode(reader)?);
        };

        Ok(vec)
    }

    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let mut bytes_written = self.len().encode(writer)?;
        for element in self.iter() {
            bytes_written += element.encode(writer)?;
        }
        Ok(bytes_written)
    }
}

impl WasmBinary for String {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        // reuse Vec<u8> implementation, then consume buf so no re-allocation is necessary
        let buf: Vec<u8> = Vec::decode(reader)?;
        String::from_utf8(buf).map_err(|e| io::Error::new(
            io::ErrorKind::InvalidData,
            format!("utf-8 conversion error: {}", e.to_string())))
    }

    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let mut bytes_written = self.len().encode(writer)?;
        for byte in self.bytes() {
            bytes_written += byte.encode(writer)?;
        }
        Ok(bytes_written)
    }
}

/// provide parallel decoding/encoding when explicitly requested by Parallel<...> marker struct
impl<T: WasmBinary + Send + Sync> WasmBinary for Parallel<Vec<WithSize<T>>> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let element_count = usize::decode(reader)?;

        // read all elements into buffers of the given size (non-parallel, but hopefully fast)
        let mut bufs = Vec::with_capacity(limit_prealloc_capacity::<Vec<u8>>(element_count));
        for _ in 0..element_count {
            let num_bytes = usize::decode(reader)?;
            let mut buf = vec![0u8; num_bytes];
            reader.read_exact(&mut buf)?;
            bufs.push(buf);
        }

        // parallel decode of each buffer
        let decoded: io::Result<Vec<WithSize<T>>> = bufs.into_par_iter()
            .map(|buf| -> io::Result<WithSize<T>> {
                Ok(WithSize(T::decode(&mut &buf[..])?))
            })
            .collect();

        Ok(Parallel(decoded?))
    }

    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let new_size = self.0.len();
        let mut bytes_written = new_size.encode(writer)?;

        // encode elements to buffers in parallel
        let encoded: io::Result<Vec<Vec<u8>>> = self.0.par_iter()
            .map(|element: &WithSize<T>| {
                let mut buf = Vec::new();
                element.0.encode(&mut buf)?;
                Ok(buf)
            })
            .collect();

        // write sizes and buffer contents to actual writer (non-parallel, but hopefully fast)
        for buf in encoded? {
            bytes_written += buf.encode(writer)?;
        }

        Ok(bytes_written)
    }
}


/* Special cases that cannot be derived and need a manual impl */

impl WasmBinary for Module {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
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
            match Section::decode(reader) {
                Ok(section) => sections.push(section),
                // FIXME this will silently end parsing on EOF (without a domain-specific Wasm error!)
                // but sometimes, we _do_ expect more data, so not every io::Error::UnexpectedEof
                // is a valid reason to break!
                // In particular, I encountered this error during fuzzing: when a binary gets a very
                // large LEB128, e.g., the size of a Vec (as per the Wasm spec grammar), and we try
                // to parse that many entries of the Vec, and then there are less entries than
                // expected, the resulting UnexpectedEof will silently be caught here.
                // See tests/invalid/oom-large-vector-size.
                // TODO instead of breaking on an error after/during parsing of a section, we should
                // check if there is more data, _before_ parsing the next section.
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e)
            };
        }

        Ok(Module { sections })
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

/// needs manual impl because of block handling: End op-code terminates body, but only if block stack is empty
impl WasmBinary for Expr {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut instructions = Vec::new();

        let mut block_depth = 0;
        while block_depth >= 0 {
            let instr = Instr::decode(reader)?;

            block_depth += match instr {
                Instr::Block(..) | Instr::Loop(..) | Instr::If(..) => 1,
                // Else ends a block, but also starts a new one
                Instr::Else => -1 + 1,
                Instr::End => -1,
                _ => 0
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

/// needs manual impl because of compressed format: even though BlockType is "logically" an enum,
/// it has no tag, because they know that 0x40 (empty block) and ValType are disjoint.
impl WasmBinary for BlockType {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(BlockType(match u8::decode(reader)? {
            0x40 => None,
            byte => {
                let buf = [byte; 1];
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

/// needs manual impl because the tag if max is present comes at the beginning of the struct, not
/// before the max field.
impl WasmBinary for Limits {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Ok(match u8::decode(reader)? {
            0x00 => Limits {
                initial_size: u32::decode(reader)?,
                max_size: None,
            },
            0x01 => Limits {
                initial_size: u32::decode(reader)?,
                max_size: Some(u32::decode(reader)?),
            },
            byte => Self::error(format!("expected tag for Limits, got 0x{:02x}", byte))?
        })
    }

    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let mut bytes_written = 0;
        match self.max_size {
            None => {
                bytes_written += 0x00u8.encode(writer)?;
                bytes_written += self.initial_size.encode(writer)?;
            }
            Some(ref max_size) => {
                bytes_written += 0x01u8.encode(writer)?;
                bytes_written += self.initial_size.encode(writer)?;
                bytes_written += max_size.encode(writer)?;
            }
        }
        Ok(bytes_written)
    }
}

impl<T> WasmBinary for PhantomData<T> {
    fn decode<R: io::Read>(_: &mut R) -> io::Result<Self> { Ok(PhantomData) }
    fn encode<W: io::Write>(&self, _: &mut W) -> io::Result<usize> { Ok(0) }
}