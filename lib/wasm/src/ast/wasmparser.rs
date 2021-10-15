use std::cmp::max;
use std::convert::TryInto;
use std::{io, iter};
use std::path::Path;

use wasmparser::{BinaryReaderError, Chunk, Parser, Payload};

use crate::error::{AddErrInfo, Error};
use crate::highlevel::Module;
use crate::lowlevel::Offsets;

/// 64 KiB, the minimum amount of bytes read in one chunk from the input reader.
const MIN_READ_SIZE: usize = 64 * 1024;

pub fn parse_module_with_offsets<R: io::Read>(mut reader: R) -> Result<(Module, Offsets), BinaryReaderError> {
    let mut buf = Vec::with_capacity(MIN_READ_SIZE);
    let mut data = &buf[..];
    let mut offset: u64 = 0;
    let mut parser = Parser::new(offset);
    let mut eof = false;

    loop {

        println!();
        println!("current buf capacity: {}", buf.capacity());
        println!("current buf len:      {}", buf.len());
        println!("buf: {:?}", buf);

        let (payload, consumed_bytes) = match parser.parse(data, eof)? {
            Chunk::NeedMoreData(size_hint) => {
                // Guaranteed by `Parser::parser()` contract.
                assert!(!eof);

                let increase_buf_size = max(MIN_READ_SIZE, size_hint as usize);
                println!("size_hint:            {}", size_hint);

                let len = buf.len();
                buf.extend((0..increase_buf_size).map(|_| 0u8));
                let bytes_read = reader.read(&mut buf[len..]).unwrap();

                println!("read bytes:           {}", bytes_read);

                buf.truncate(len + bytes_read);
                eof = bytes_read == 0;

                continue;                
            }
            Chunk::Parsed { consumed, payload } => (payload, consumed),
        };

        println!("consumed_bytes:    {}", consumed_bytes);
        println!("{:?}", payload);

        if let Payload::End = payload {
            break;
        }

        buf.drain(..consumed_bytes);
    
    }

    todo!()
}

// impl<T> AddErrInfo<T> for Result<T, BinaryReaderError> {
//     fn add_err_info<GrammarElement>(self: Result<T, BinaryReaderError>, offset: usize) -> Result<T, Error> {
//         self.map_err(|err|
//             Error::with_source::<GrammarElement, _>(offset, ErrorKind::Leb128, err))
//     }
// }