#![feature(attr_literals, specialization, conservative_impl_trait, test)]

#[macro_use]
extern crate custom_derive;
extern crate byteorder;
extern crate rayon;
extern crate walkdir;
extern crate clap;
extern crate test;
extern crate tempfile;

use ast::Module;
use binary::WasmBinary;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

mod leb128;
mod ast;
mod binary;
mod instrument;
mod display;
mod utils;
#[cfg(test)]
mod tests;

fn main() {
    let args = App::new("wasm-instrument")
        .arg(Arg::with_name("silent").short("s").long("silent"))
        .arg(Arg::with_name("input").required(true))
        .arg(Arg::with_name("output").short("o").takes_value(true).required(true))
        .arg(Arg::with_name("instrumentation").default_value("identity"))
        .get_matches();

    let silent = args.is_present("silent");
    let input = args.value_of("input").unwrap();
    let output = args.value_of("output").unwrap();
    let instrument = match args.value_of("instrumentation").unwrap() {
        "identity" => instrument::identity,
        "add" => instrument::add_trivial_type,
        "count-calls" => instrument::count_call_instructions,
        instrumentation => unimplemented!("instrumentation {}", instrumentation)
    };

    std::process::exit(match || -> io::Result<()> {
        let mut module = Module::decode(&mut BufReader::new(File::open(input)?))?;

        if !silent {
            println!("Before:");
//            println!("{:#?}", module);
            println!("{}", module.display());
        }

        if !silent {
            println!("running instrumentation {}...\n", args.value_of("instrumentation").unwrap());
        }
        instrument(&mut module);

        if !silent {
            println!("After:");
//            println!("{:#?}", module);
            println!("{}", module.display());
        }

        let bytes_written = module.encode(&mut BufWriter::new(File::create(output)?))?;
        println!("written encoded Module to {}, {} bytes", output, bytes_written);
        Ok(())
    }() {
        Ok(_) => 0,
        Err(ref e) => {
            eprintln!("error: {}", e);
            1
        }
    });
}