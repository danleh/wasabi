use wasm::ast::*;
use instrument::{direct::*};
use std::fs::{create_dir_all, File};
use std::io::{self, Write, BufWriter};
use std::path::{Path, PathBuf};
use test_utilities::*;

static TEST_INPUTS: &'static str = "../tests/inputs";

#[test]
fn add_empty_function_produces_valid_wasm() {
    for path in wasm_files(TEST_INPUTS) {
        let output = instrument(&path, add_empty_function, "add-empty-function").unwrap();
        wasm_validate(&output).unwrap();
    }
}

#[test]
fn count_calls_instrumentation_produces_valid_wasm() {
    for path in wasm_files(TEST_INPUTS) {
        let output = instrument(&path, count_calls, "count-calls").unwrap();
        wasm_validate(&output).unwrap();
    }
}

#[test]
fn add_hooks_instrumentation_produces_valid_wasm() {
    for path in wasm_files(TEST_INPUTS) {
        let output = instrument(&path, count_calls, "add-hooks").unwrap();
        wasm_validate(&output).unwrap();
    }
}

/* Convenience functions */

/// Read wasm module from test_file, instrument it, and write out to test/output/ directory
fn instrument(test_file: &Path, instrument: impl Fn(&mut highlevel::Module) -> Option<String>, instrument_str: &str) -> io::Result<PathBuf> {
    assert!(test_file.to_string_lossy().contains("test/input"),
            "otherwise creating the output file and directories could fail/overwrite other stuff");
    // TODO replace with test_utilities functions
    let output_dir = "outputs/".to_string() + instrument_str;
    let output_wasm_file = PathBuf::from(test_file.to_string_lossy().replace("input", &output_dir));
    let output_js_file = PathBuf::from(output_wasm_file.to_string_lossy().replace(".wasm", ".js"));
    create_dir_all(output_wasm_file.parent().unwrap_or(&output_wasm_file))?;

    let mut module = highlevel::Module::from_file(test_file)?;
    let generated_js = instrument(&mut module);
    module.to_file(&output_wasm_file)?;
    if let Some(generated_js) = generated_js {
        BufWriter::new(File::create(output_js_file)?).write_all(generated_js.as_bytes())?;
    }

    Ok(output_wasm_file)
}