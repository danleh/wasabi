#![feature(attr_literals)]
#[macro_use]
extern crate wasm_derive;
extern crate byteorder;
extern crate leb128;
extern crate rayon;

use std::io;
use rayon::prelude::*;

// TODO test with WASM spec test suite

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
        use byteorder::ReadBytesExt;
        reader.read_u8()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        use byteorder::WriteBytesExt;
        writer.write_u8(*self)?;
        Ok(1)
    }
}

// TODO save LEB128 encoding with value to make sure decoding-encoding round-trips
impl Wasm for u32 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        match leb128::read::unsigned(reader) {
            Err(leb128::read::Error::IoError(io_err)) => Err(io_err),
            Err(leb128::read::Error::Overflow) => Self::error("leb128 to u32 overflow"),
            Ok(value) if value > u32::max_value() as u64 => Self::error("leb128 to u32 overflow"),
            Ok(value) => Ok(value as u32),
        }
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        leb128::write::unsigned(writer, *self as u64)
    }
}

// TODO save LEB128 encoding with value to make sure decoding-encoding round-trips
impl Wasm for u64 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        match leb128::read::unsigned(reader) {
            Err(leb128::read::Error::IoError(io_err)) => Err(io_err),
            Err(leb128::read::Error::Overflow) => Self::error("leb128 to u64 overflow"),
            Ok(value) => Ok(value),
        }
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        leb128::write::unsigned(writer, *self as u64)
    }
}

// TODO save LEB128 encoding with value to make sure decoding-encoding round-trips
impl Wasm for i32 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        match leb128::read::signed(reader) {
            Err(leb128::read::Error::IoError(io_err)) => Err(io_err),
            Err(leb128::read::Error::Overflow) => Self::error("leb128 to i32 overflow"),
            Ok(value) if value > i32::max_value() as i64 => Self::error("leb128 to i32 overflow"),
            Ok(value) => Ok(value as i32),
        }
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        leb128::write::signed(writer, *self as i64)
    }
}

// TODO save LEB128 encoding with value to make sure decoding-encoding round-trips
impl Wasm for i64 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        match leb128::read::signed(reader) {
            Err(leb128::read::Error::IoError(io_err)) => Err(io_err),
            Err(leb128::read::Error::Overflow) => Self::error("leb128 to i64 overflow"),
            Ok(value) => Ok(value),
        }
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        leb128::write::signed(writer, *self as i64)
    }
}

impl Wasm for f32 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        use byteorder::ReadBytesExt;
        reader.read_f32::<byteorder::LittleEndian>()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        use byteorder::WriteBytesExt;
        writer.write_f32::<byteorder::LittleEndian>(*self)?;
        Ok(4)
    }
}

impl Wasm for f64 {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        use byteorder::ReadBytesExt;
        reader.read_f64::<byteorder::LittleEndian>()
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        use byteorder::WriteBytesExt;
        writer.write_f64::<byteorder::LittleEndian>(*self)?;
        Ok(8)
    }
}

impl<T: Wasm> Wasm for Vec<T> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let size = u32::decode(reader)?;
        let mut vec: Vec<T> = Vec::with_capacity(size as usize);
        for _ in 0..size {
            vec.push(T::decode(reader)?);
        };
        Ok(vec)
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let mut bytes_written = (self.len() as u32).encode(writer)?;
        for element in self {
            bytes_written += element.encode(writer)?;
        }
        Ok(bytes_written)
    }
}

impl Wasm for String {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let size = u32::decode(reader)?;
        let mut buf = vec![0u8; size as usize];
        reader.read_exact(&mut buf)?;
        match String::from_utf8(buf) {
            Ok(str) => Ok(str),
            Err(e) => Self::error(format!("utf-8 conversion error: {}", e.to_string())),
        }
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let mut bytes_written = (self.len() as u32).encode(writer)?;
        for byte in self.bytes() {
            bytes_written += byte.encode(writer)?;
        }
        Ok(bytes_written)
    }
}

#[derive(Debug)]
pub struct WithSize<T> {
    /// as a hint for allocating the write buffer
    old_size: u32,
    content: T,
}

impl<T: Wasm> Wasm for WithSize<T> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let old_size = u32::decode(reader)?;
        Ok(WithSize {
            old_size,
            content: T::decode(reader)?,
        })
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        // write contents to buffer to know size
        let mut buf = Vec::with_capacity(self.old_size as usize);
        let new_size = self.content.encode(&mut buf)?;

        // write size, then contents from buffer to actual writer
        let mut bytes_written = (new_size as u32).encode(writer)?;
        writer.write_all(&buf)?;
        bytes_written += new_size;

        Ok(bytes_written)
    }
}

#[derive(Debug)]
pub struct Parallel<T>(Vec<WithSize<T>>);

impl<T: Wasm + Send> Wasm for Parallel<T> {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let num_elements = u32::decode(reader)?;
        let mut bufs = Vec::with_capacity(num_elements as usize);

        // read all elements into buffers of the given size (non-parallel, but hopefully fast)
        for _ in 0..num_elements {
            let num_bytes = u32::decode(reader)?;
            let mut buf = vec![0u8; num_bytes as usize];
            reader.read_exact(&mut buf)?;
            bufs.push(buf);
        }

        // parallel decode of each buffer
        let decoded: Vec<WithSize<T>> = bufs.into_par_iter()
            .map(|buf| {
                let old_size = buf.len() as u32;
                WithSize {
                    old_size,
                    content: T::decode(&mut &buf[..]).unwrap(), // FIXME
                }
            })
            .collect();

        Ok(Parallel(decoded))
    }
    fn encode<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        // TODO parallelize this
        self.0.encode(writer)
    }
}

#[derive(Debug)]
pub struct Module {
    version: u32,
    sections: Vec<Section>,
}

#[derive(Wasm, Debug)]
pub enum Section {
    // TODO custom
    #[tag = 1] Type(WithSize<Vec<FuncType>>),
    #[tag = 2] Import(WithSize<Vec<Import>>),
    #[tag = 3] Function(WithSize<Vec<TypeIdx>>),
    // TODO table
    // TODO memory
    #[tag = 6] Global(WithSize<Vec<Global>>),
    #[tag = 7] Export(WithSize<Vec<Export>>),
    #[tag = 8] Start(WithSize<FuncIdx>),
    #[tag = 9] Element(WithSize<Vec<Element>>),
    #[tag = 10] Code(WithSize<Parallel<Func>>),
    #[tag = 11] Data(WithSize<Vec<Data>>),
}

#[derive(Wasm, Debug)]
pub struct Data(MemoryIdx, /* offset given by constant expr */ Expr, Vec<u8>);

#[derive(Wasm, Debug)]
pub struct Global(GlobalType, Expr);

#[derive(Wasm, Debug)]
pub struct Element(TableIdx, Expr, Vec<FuncIdx>);

#[derive(Wasm, Debug)]
#[tag = 0x60]
pub struct FuncType {
    params: Vec<ValType>,
    results: Vec<ValType>,
}

#[derive(Wasm, Debug, PartialEq)]
pub enum ValType {
    #[tag = 0x7f] I32,
    #[tag = 0x7e] I64,
    #[tag = 0x7d] F32,
    #[tag = 0x7c] F64,
}

#[derive(Wasm, Debug)]
pub struct Import {
    module: String,
    name: String,
    type_: ImportType,
}

#[derive(Wasm, Debug)]
pub struct Export {
    name: String,
    type_: ExportType,
}

#[derive(Wasm, Debug)]
pub enum ImportType {
    #[tag = 0x0] Function(TypeIdx),
    #[tag = 0x1] Table(TableType),
    #[tag = 0x2] Memory(Limits),
    #[tag = 0x3] Global(GlobalType),
}

#[derive(Wasm, Debug)]
pub enum ExportType {
    #[tag = 0x0] Function(TypeIdx),
    #[tag = 0x1] Table(TableIdx),
    #[tag = 0x2] Memory(MemoryIdx),
    #[tag = 0x3] Global(GlobalIdx),
}

#[derive(Wasm, Debug)]
#[tag = 0x70]
pub struct TableType(Limits);

#[derive(Wasm, Debug)]
pub enum Limits {
    #[tag = 0x00] Min(u32),
    #[tag = 0x01] MinMax(u32, u32),
}

#[derive(Wasm, Debug)]
pub struct GlobalType(ValType, Mut);

#[derive(Wasm, Debug)]
pub enum Mut {
    #[tag = 0x00] Const,
    #[tag = 0x01] Var,
}

#[derive(Debug, PartialEq)]
pub struct BlockType(Option<ValType>);

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

#[derive(Wasm, Debug)]
pub struct Func(Vec<Locals>, Expr);

#[derive(Wasm, Debug)]
pub struct Locals {
    count: u32,
    type_: ValType,
}

#[derive(Debug, PartialEq)]
pub struct Expr(Vec<Instr>);

#[derive(Wasm, Debug, PartialEq)]
pub enum Instr {
    #[tag = 0x00] Unreachable,
    #[tag = 0x01] Nop,

    // NOTE block nesting is handled by Expr::decode which returns as soon as an Else or End is found
    #[tag = 0x02] Block(BlockType, Expr),
    #[tag = 0x03] Loop(BlockType, Expr),
    #[tag = 0x04] If(BlockType, Expr),
    #[tag = 0x05] Else(Expr),
    #[tag = 0x0b] End,

    #[tag = 0x0c] Br(LabelIdx),
    #[tag = 0x0d] BrIf(LabelIdx),
    #[tag = 0x0e] BrTable(Vec<LabelIdx>, LabelIdx),

    #[tag = 0x0f] Return,
    #[tag = 0x10] Call(FuncIdx),
    #[tag = 0x11] CallIndirect(TypeIdx, /* unused, always 0x00 in WASM version 1 */ u8),

    #[tag = 0x1a] Drop,
    #[tag = 0x1b] Select,

    #[tag = 0x20] GetLocal(LocalIdx),
    #[tag = 0x21] SetLocal(LocalIdx),
    #[tag = 0x22] TeeLocal(LocalIdx),
    #[tag = 0x23] GetGlobal(GlobalIdx),
    #[tag = 0x24] SetGlobal(GlobalIdx),

    #[tag = 0x28] I32Load(Memarg),
    #[tag = 0x29] I64Load(Memarg),
    #[tag = 0x2a] F32Load(Memarg),
    #[tag = 0x2b] F64Load(Memarg),
    #[tag = 0x2c] I32Load8S(Memarg),
    #[tag = 0x2d] I32Load8U(Memarg),
    #[tag = 0x2e] I32Load16S(Memarg),
    #[tag = 0x2f] I32Load16U(Memarg),
    #[tag = 0x30] I64Load8S(Memarg),
    #[tag = 0x31] I64Load8U(Memarg),
    #[tag = 0x32] I64Load16S(Memarg),
    #[tag = 0x33] I64Load16U(Memarg),
    #[tag = 0x34] I64Load32S(Memarg),
    #[tag = 0x35] I64Load32U(Memarg),
    #[tag = 0x36] I32Store(Memarg),
    #[tag = 0x37] I64Store(Memarg),
    #[tag = 0x38] F32Store(Memarg),
    #[tag = 0x39] F64Store(Memarg),
    #[tag = 0x3a] I32Store8(Memarg),
    #[tag = 0x3b] I32Store16(Memarg),
    #[tag = 0x3c] I64Store8(Memarg),
    #[tag = 0x3d] I64Store16(Memarg),
    #[tag = 0x3e] I64Store32(Memarg),
    #[tag = 0x3f] CurrentMemory(/* unused, always 0x00 in WASM version 1 */ u8),
    #[tag = 0x40] GrowMemory(/* unused, always 0x00 in WASM version 1 */ u8),

    #[tag = 0x41] I32Const(i32),
    #[tag = 0x42] I64Const(i64),
    #[tag = 0x43] F32Const(f32),
    #[tag = 0x44] F64Const(f64),

    #[tag = 0x45] I32Eqz,
    #[tag = 0x46] I32Eq,
    #[tag = 0x47] I32Ne,
    #[tag = 0x48] I32LtS,
    #[tag = 0x49] I32LtU,
    #[tag = 0x4a] I32GtS,
    #[tag = 0x4b] I32GtU,
    #[tag = 0x4c] I32LeS,
    #[tag = 0x4d] I32LeU,
    #[tag = 0x4e] I32GeS,
    #[tag = 0x4f] I32GeU,
    #[tag = 0x50] I64Eqz,
    #[tag = 0x51] I64Eq,
    #[tag = 0x52] I64Ne,
    #[tag = 0x53] I64LtS,
    #[tag = 0x54] I64LtU,
    #[tag = 0x55] I64GtS,
    #[tag = 0x56] I64GtU,
    #[tag = 0x57] I64LeS,
    #[tag = 0x58] I64LeU,
    #[tag = 0x59] I64GeS,
    #[tag = 0x5a] I64GeU,
    #[tag = 0x5b] F32Eq,
    #[tag = 0x5c] F32Ne,
    #[tag = 0x5d] F32Lt,
    #[tag = 0x5e] F32Gt,
    #[tag = 0x5f] F32Le,
    #[tag = 0x60] F32Ge,
    #[tag = 0x61] F64Eq,
    #[tag = 0x62] F64Ne,
    #[tag = 0x63] F64Lt,
    #[tag = 0x64] F64Gt,
    #[tag = 0x65] F64Le,
    #[tag = 0x66] F64Ge,
    #[tag = 0x67] I32Clz,
    #[tag = 0x68] I32Ctz,
    #[tag = 0x69] I32Popcnt,
    #[tag = 0x6a] I32Add,
    #[tag = 0x6b] I32Sub,
    #[tag = 0x6c] I32Mul,
    #[tag = 0x6d] I32DivS,
    #[tag = 0x6e] I32DivU,
    #[tag = 0x6f] I32RemS,
    #[tag = 0x70] I32RemU,
    #[tag = 0x71] I32And,
    #[tag = 0x72] I32Or,
    #[tag = 0x73] I32Xor,
    #[tag = 0x74] I32Shl,
    #[tag = 0x75] I32ShrS,
    #[tag = 0x76] I32ShrU,
    #[tag = 0x77] I32Rotl,
    #[tag = 0x78] I32Rotr,
    #[tag = 0x79] I64Clz,
    #[tag = 0x7a] I64Ctz,
    #[tag = 0x7b] I64Popcnt,
    #[tag = 0x7c] I64Add,
    #[tag = 0x7d] I64Sub,
    #[tag = 0x7e] I64Mul,
    #[tag = 0x7f] I64DivS,
    #[tag = 0x80] I64DivU,
    #[tag = 0x81] I64RemS,
    #[tag = 0x82] I64RemU,
    #[tag = 0x83] I64And,
    #[tag = 0x84] I64Or,
    #[tag = 0x85] I64Xor,
    #[tag = 0x86] I64Shl,
    #[tag = 0x87] I64ShrS,
    #[tag = 0x88] I64ShrU,
    #[tag = 0x89] I64Rotl,
    #[tag = 0x8a] I64Rotr,
    #[tag = 0x8b] F32Abs,
    #[tag = 0x8c] F32Neg,
    #[tag = 0x8d] F32Ceil,
    #[tag = 0x8e] F32Floor,
    #[tag = 0x8f] F32Trunc,
    #[tag = 0x90] F32Nearest,
    #[tag = 0x91] F32Sqrt,
    #[tag = 0x92] F32Add,
    #[tag = 0x93] F32Sub,
    #[tag = 0x94] F32Mul,
    #[tag = 0x95] F32Div,
    #[tag = 0x96] F32Min,
    #[tag = 0x97] F32Max,
    #[tag = 0x98] F32Copysign,
    #[tag = 0x99] F64Abs,
    #[tag = 0x9a] F64Neg,
    #[tag = 0x9b] F64Ceil,
    #[tag = 0x9c] F64Floor,
    #[tag = 0x9d] F64Trunc,
    #[tag = 0x9e] F64Nearest,
    #[tag = 0x9f] F64Sqrt,
    #[tag = 0xa0] F64Add,
    #[tag = 0xa1] F64Sub,
    #[tag = 0xa2] F64Mul,
    #[tag = 0xa3] F64Div,
    #[tag = 0xa4] F64Min,
    #[tag = 0xa5] F64Max,
    #[tag = 0xa6] F64Copysign,
    #[tag = 0xa7] I32WrapI64,
    #[tag = 0xa8] I32TruncSF32,
    #[tag = 0xa9] I32TruncUF32,
    #[tag = 0xaa] I32TruncSF64,
    #[tag = 0xab] I32TruncUF64,
    #[tag = 0xac] I64ExtendSI32,
    #[tag = 0xad] I64ExtendUI32,
    #[tag = 0xae] I64TruncSF32,
    #[tag = 0xaf] I64TruncUF32,
    #[tag = 0xb0] I64TruncSF64,
    #[tag = 0xb1] I64TruncUF64,
    #[tag = 0xb2] F32ConvertSI32,
    #[tag = 0xb3] F32ConvertUI32,
    #[tag = 0xb4] F32ConvertSI64,
    #[tag = 0xb5] F32ConvertUI64,
    #[tag = 0xb6] F32DemoteF64,
    #[tag = 0xb7] F64ConvertSI32,
    #[tag = 0xb8] F64ConvertUI32,
    #[tag = 0xb9] F64ConvertSI64,
    #[tag = 0xba] F64ConvertUI64,
    #[tag = 0xbb] F64PromoteF32,
    #[tag = 0xbc] I32ReinterpretF32,
    #[tag = 0xbd] I64ReinterpretF64,
    #[tag = 0xbe] F32ReinterpretI32,
    #[tag = 0xbf] F64ReinterpretI64,
}

#[derive(Wasm, Debug, PartialEq)]
pub struct TypeIdx(u32);

#[derive(Wasm, Debug, PartialEq)]
pub struct FuncIdx(u32);

#[derive(Wasm, Debug, PartialEq)]
pub struct TableIdx(u32);

#[derive(Wasm, Debug, PartialEq)]
pub struct MemoryIdx(u32);

#[derive(Wasm, Debug, PartialEq)]
pub struct GlobalIdx(u32);

#[derive(Wasm, Debug, PartialEq)]
pub struct LocalIdx(u32);

#[derive(Wasm, Debug, PartialEq)]
pub struct LabelIdx(u32);

#[derive(Wasm, Debug, PartialEq)]
pub struct Memarg {
    alignment: u32,
    offset: u32,
}

impl Wasm for Module {
    fn decode<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        let mut magic_number = [0u8; 4];
        reader.read_exact(&mut magic_number)?;
        if &magic_number != b"\0asm" {
            return Self::error("magic bytes do not match");
        }

        use byteorder::ReadBytesExt;
        let version = reader.read_u32::<byteorder::LittleEndian>()?;
        if version != 1 {
            return Self::error("not version 1");
        }

        let mut sections = Vec::new();
        loop {
            match Section::decode(reader) {
                Ok(section) => {
//                    println!("found section: {:?}", section); // DEBUG
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

    use std::fs::File;
    let mut buf_reader = io::BufReader::new(File::open(&file_name).unwrap());
    let mut module = match Module::decode(&mut buf_reader) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    if std::env::args().nth(2).is_none() {
        println!("{:#?}", module);
    }

    let encoded_file_name = file_name.replace(".wasm", ".encoded.wasm");
    let mut buf_writer = io::BufWriter::new(File::create(&encoded_file_name).unwrap());

//    match module.sections[0] {
//        Section::Type(ref mut _0) => _0.content.push(FuncType {params: Vec::new(), results: Vec::new()}),
//        _ => {}
//    };

    let bytes_written = module.encode(&mut buf_writer).unwrap();
    println!("written encoded Module to {}, {} bytes", encoded_file_name, bytes_written);
}
