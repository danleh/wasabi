extern crate wasabi;
extern crate wasm;

use std::{env, fs, io, path::PathBuf};
use wasabi::instrument::add_hooks;
use wasm::ast::highlevel::Module;

fn main() {
    if let Err(error) = main_inner() {
        eprintln!(r#"Error: {}

Usage: wasabi <input_wasm_file> [<output_dir>]

Produces two files in <output_dir> (default: out/):
  - an instrumented version of the <input_wasm_file> and
  - a JavaScript file with static analysis information, (Wasabi-internal) low-level hooks, Wasabi runtime, and Wasabi loader."#,
                  error);
    }
}

fn main_inner() -> io::Result<()> {
    // skip first argument (program name)
    let mut args = env::args().skip(1);
    let input_file = PathBuf::from(args.next().ok_or(io_err("expected at least one argument"))?);
    let output_dir = PathBuf::from(args.next().unwrap_or("out".to_string()));

    let input_filename_no_ext = input_file.file_stem().ok_or(io_err("invalid input file"))?;

    let mut output_file_stem = output_dir.clone();
    output_file_stem.push(input_filename_no_ext);
    let output_file_wasm = output_file_stem.with_extension("wasm");
    let output_file_js = output_file_stem.with_extension("wasabi.js");

    // instrument Wasm and generate JavaScript
    let mut module = Module::from_file(input_file.clone())?;
    let js = add_hooks(&mut module).unwrap();

    // write output files
    fs::create_dir_all(output_dir)?;
    module.to_file(output_file_wasm)?;
    fs::write(output_file_js, js)
}

fn io_err(str: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, str.to_string())
}