#![feature(attr_literals)]

#[macro_use]
extern crate custom_derive;
extern crate byteorder;
extern crate rayon;

use ast::Module;
use binary::WasmBinary;
use std::io;

mod leb128;
mod ast;
mod binary;

// TODO test with WASM spec test suite
// TODO "streaming AST" API: return Module {} after reading only the first 8 bytes, implement
// Iterator<Item = Section> for Module -> Module must somehow retain the reader to do so...

macro_rules! debug {
    ( $fmt:expr, $( $args:expr ),* ) => {
        let should_output = std::env::args().nth(2).is_none(); // give "silent" or so as second argument
        if should_output {
            println!($fmt, $( $args ),* );
        }
    };
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