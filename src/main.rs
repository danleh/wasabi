use std::{env, fs, io, path::PathBuf};
use wasabi::config::EnabledHooks;
use wasabi::instrument::add_hooks;
use wasm::ast::highlevel::Module;

// TODO use proper command-line option parser like clap, or structopt on top of it (https://docs.rs/structopt/0.2.10/structopt/)
// TODO use failure crate and failure::Error type for error handling or use custom error trait

fn main() {
    if let Err(error) = main_inner() {
        eprintln!(r#"Error: {}

Usage: wasabi [options] <input_wasm_file> [<output_dir>]

Produces two files in <output_dir> (default: out/):
  - an instrumented version of the <input_wasm_file> and
  - a JavaScript file with static analysis information, (Wasabi-internal) low-level hooks, Wasabi runtime, and Wasabi loader.

Options:
  --hooks=<comma-separated list>     Instrument ONLY for the given hooks (e.g. call, load, store, ...).
  --no-hooks=<comma-separated list>  Instrument for all BUT the given hooks.
                                     (Default: Instrument for all hooks.)"#,
                  error);
    }
}

fn main_inner() -> io::Result<()> {
    let (options, args): (Vec<String>, Vec<String>) = env::args()
        // skip first argument (program name)
        .skip(1)
        // --hooks and --no-hooks options
        .partition(|arg| arg.starts_with("--hooks") || arg.starts_with("--no-hooks"));
    let mut args = args.into_iter();
    let input_file = PathBuf::from(
        args.next()
            .ok_or(io_err("expected at least one argument"))?,
    );
    let output_dir = PathBuf::from(args.next().unwrap_or("out".to_string()));

    let input_filename_no_ext = input_file.file_stem().ok_or(io_err("invalid input file"))?;
    eprintln!("input_filename_no_ext: {:?}", input_filename_no_ext);

    let mut output_file_wasm = input_filename_no_ext.to_owned();
    output_file_wasm.push(".wasm");
    let output_file_wasm = output_dir.join(output_file_wasm);

    let mut output_file_js = input_filename_no_ext.to_owned();
    output_file_js.push(".wasabi.js");
    let output_file_js = output_dir.join(output_file_js);

    let enabled_hooks = match options.as_slice() {
        [] => EnabledHooks::all(),
        [option] if option.starts_with("--hooks=") => {
            EnabledHooks::from_hooks(option.trim_start_matches("--hooks="))?
        }
        [option] if option.starts_with("--no-hooks=") => {
            EnabledHooks::from_no_hooks(option.trim_start_matches("--no-hooks="))?
        }
        _ => {
            return Err(io_err(
                "invalid options, can only give --hooks=... OR --no-hooks=...",
            ))
        }
    };

    // instrument Wasm and generate JavaScript
    let mut module = Module::from_file(input_file.clone())?;
    let js = add_hooks(&mut module, &enabled_hooks).unwrap();

    // write output files
    fs::create_dir_all(output_dir)?;
    module.to_file(output_file_wasm)?;
    fs::write(output_file_js, js)
}

fn io_err(str: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, str.to_string())
}
