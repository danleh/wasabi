use ast::*;
use binary::WasmBinary;
use instrument::*;
use std::fs::{create_dir_all, File};
use std::io::{self, Cursor, Read, sink};
use std::path::{Path, PathBuf};
use test::Bencher;

//struct Bla;
//type BlaVec = Vec<Bla>;
//fn apply_recursive(bla: &mut BlaVec, f: impl Fn(&mut Bla)) {
//    for recursive in bla.iter_mut() {
//        apply_recursive(recursive, |b| f(b))
//    }
//}

/// "main"-like for quick and dirty testing
#[test]
#[ignore]
fn debug() {
    let file = "test/input/hello-emcc.wasm";
//    let mut module = highlevel::Module::from_file(file).unwrap();
//    for (i, func) in module.functions() {
//        println!("{:#?}", func.instructions().collect::<Vec<_>>());
//    }

//    let module = instrumentation_module();
//    println!("{:#?}", module);
//    module.to_file("test/multiple-memories.wasm").unwrap();

    instrument(&Path::new(file), count_calls, "count-calls").unwrap();
}


/* Correctness tests */

#[test]
fn can_lowlevel_decode_valid_wasm() {
    for path in wasm_files("test/input") {
        lowlevel::Module::from_file(&path).expect(&format!("could not decode valid wasm file {} to low-level AST", path.display()));
    }
}

#[test]
fn can_highlevel_decode_valid_wasm() {
    for path in wasm_files("test/input") {
        highlevel::Module::from_file(&path).expect(&format!("could not decode valid wasm file {} to high-level AST", path.display()));
    }
}

#[test]
fn identity_instrumentation_produces_valid_wasm() {
    for path in wasm_files("test/input") {
        let output = instrument(&path, identity, "identity").unwrap();
        wasm_validate(&output).unwrap();
    }
}

#[test]
fn add_empty_function_produces_valid_wasm() {
    for path in wasm_files("test/input") {
        let output = instrument(&path, add_empty_function, "add-empty-function").unwrap();
        wasm_validate(&output).unwrap();
    }
}

#[test]
fn count_calls_instrumentation_produces_valid_wasm() {
    for path in wasm_files("test/input") {
        let output = instrument(&path, count_calls, "count-calls").unwrap();
        wasm_validate(&output).unwrap();
    }
}


/* Test encoding/decoding speed (without any instrumentation) on "large" wasm file (~2MB) */

const LARGE_WASM_FILE: &'static str = "test/input/bananabread/bb.wasm";

#[bench]
fn decode_lowlevel_speed(bencher: &mut Bencher) {
    let mut buf = Vec::new();
    File::open(LARGE_WASM_FILE).unwrap().read_to_end(&mut buf).unwrap();

    bencher.iter(||
        lowlevel::Module::decode(&mut Cursor::new(&buf)).unwrap())
}

#[bench]
fn encode_lowlevel_speed(bencher: &mut Bencher) {
    let module = lowlevel::Module::from_file(LARGE_WASM_FILE).unwrap();

    bencher.iter(||
        module.encode(&mut sink()).unwrap())
}

#[bench]
fn convert_lowlevel_to_highlevel_speed(bencher: &mut Bencher) {
    let module = lowlevel::Module::from_file(LARGE_WASM_FILE).unwrap();

    bencher.iter(|| {
        let _: highlevel::Module = module.clone().into();
    })
}

#[bench]
fn convert_highlevel_to_lowlevel_speed(bencher: &mut Bencher) {
    let module = highlevel::Module::from_file(LARGE_WASM_FILE).unwrap();

    bencher.iter(|| {
        let _: lowlevel::Module = module.clone().into();
    })
}

// as baseline for conversions high-level <-> low-level (where we need to clone -.-)
#[bench]
fn clone_lowlevel_module_speed(bencher: &mut Bencher) {
    let module = lowlevel::Module::from_file(LARGE_WASM_FILE).unwrap();
    bencher.iter(|| module.clone())
}

#[bench]
fn clone_highlevel_module_speed(bencher: &mut Bencher) {
    let module = highlevel::Module::from_file(LARGE_WASM_FILE).unwrap();
    bencher.iter(|| module.clone())
}


/* Convenience functions */

fn wasm_files<P: AsRef<Path>>(dir: P) -> impl Iterator<Item=PathBuf> {
    use walkdir::WalkDir;
    WalkDir::new(dir.as_ref()).into_iter()
        .map(Result::unwrap)
        .map(|entry| entry.path().to_owned())
        .filter(|path| path.extension() == Some("wasm".as_ref()))
}

/// Read wasm module from test_file, instrument it, and write out to test/output/ directory
fn instrument(test_file: &Path, instrument: impl Fn(&mut highlevel::Module), instrument_str: &str) -> io::Result<PathBuf> {
    assert!(test_file.to_string_lossy().contains("test/input"),
            "otherwise creating the output file and directories could fail/overwrite other stuff");
    let output_dir = "output/".to_string() + instrument_str;
    let output_file = PathBuf::from(test_file.to_string_lossy().replace("input", &output_dir));
    create_dir_all(output_file.parent().unwrap_or(&output_file))?;

    let mut module = highlevel::Module::from_file(test_file)?;
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
