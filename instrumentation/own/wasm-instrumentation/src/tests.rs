use ast::lowlevel::Module;
use binary::WasmBinary;
use instrument::*;
use std::fs::{create_dir_all, File};
use std::io::{self, Cursor, Read, sink};
use std::path::{Path, PathBuf};
use test::Bencher;

/// "main"-like for quick and dirty testing
#[test]
#[ignore]
fn debug() {
    let file = "test/input/hello-manual.wasm";
    instrument(&Path::new(file), count_calls, "count-calls").unwrap();
}


/* Correctness tests */

#[test]
fn decode_valid() {
    for path in wasm_files("test/input") {
        Module::from_file(path).expect("files in test/input are valid wasm, so decoding should not panic");
    }
}

#[test]
fn identity_roundtrip() {
    for path in wasm_files("test/input") {
        let output_path = instrument(&path, identity, "identity").unwrap();

        let mut input_bytes = Vec::new();
        File::open(&path).unwrap().read_to_end(&mut input_bytes).unwrap();

        let mut output_bytes = Vec::new();
        File::open(&output_path).unwrap().read_to_end(&mut output_bytes).unwrap();

        assert!(input_bytes == output_bytes, "{}: encoding and decoding did not round-trip", path.display());
    }
}

#[test]
fn add_trivial_type_valid() {
    for path in wasm_files("test/input") {
        let output = instrument(&path, add_trivial_type, "add-trivial-type").unwrap();
        wasm_validate(&output).unwrap();
    }
}

#[test]
fn count_calls_valid() {
    for path in wasm_files("test/input") {
        let output = instrument(&path, count_calls, "count-calls").unwrap();
        wasm_validate(&output).unwrap();
    }
}


/* Test encoding/decoding speed (without any instrumentation) on "large" wasm file (~2MB) */

#[bench]
fn decode_speed(bencher: &mut Bencher) {
    let mut buf = Vec::new();
    File::open("test/input/bananabread/bb.wasm").unwrap().read_to_end(&mut buf).unwrap();

    bencher.iter(|| {
        Module::decode(&mut Cursor::new(&buf)).unwrap();
    })
}

#[bench]
fn encode_speed(bencher: &mut Bencher) {
    let module = Module::from_file("test/input/bananabread/bb.wasm").unwrap();

    bencher.iter(|| {
        module.encode(&mut sink()).unwrap();
    })
}


/* Convenience functions */

fn wasm_files<P: AsRef<Path>>(dir: P) -> impl Iterator<Item=PathBuf> {
    use walkdir::WalkDir;
    WalkDir::new(dir.as_ref()).into_iter()
        .map(Result::unwrap)
        .map(|entry| entry.path().to_owned())
        .filter(|path| path.extension() == Some("wasm".as_ref()))
}

fn instrument(test_file: &Path, instrument: impl Fn(&mut Module), instrument_str: &str) -> io::Result<PathBuf> {
    assert!(test_file.to_string_lossy().contains("test/input"),
            "otherwise creating the output file and directories could fail/overwrite other stuff");
    let output_dir = "output/".to_string() + instrument_str;
    let output_file = PathBuf::from(test_file.to_string_lossy().replace("input", &output_dir));
    create_dir_all(output_file.parent().unwrap_or(&output_file))?;

    let mut module = Module::from_file(test_file)?;
    instrument(&mut module);
    module.to_file(&output_file)?;
    Ok(output_file)
}

fn wasm_validate(path: &Path) -> Result<(), String> {
    use std::process::Command;
    let validate_output = Command::new("wasm-validate")
        .arg(path)
        .output()
        .map_err(|err| err.to_string())
        .unwrap();
    if validate_output.status.success() {
        Ok(())
    } else {
        Err(format!("invalid wasm file {}\n", path.display()) +
            &String::from_utf8(validate_output.stderr).unwrap())
    }
}
