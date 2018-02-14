use ast::Module;
use binary::WasmBinary;
use instrument;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, Cursor, Read, sink, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use test::Bencher;
use walkdir::WalkDir;

#[test]
#[ignore]
fn quick_output() {
    let path = "test/input/hello-manual.wasm";
    let mut module = Module::decode(&mut BufReader::new(File::open(path).unwrap())).unwrap();
    instrument::add_trivial_type(&mut module);
    println!("{}", module.display());
}

#[test]
fn decoding_valid_files_doesnt_panic() {
    for path in wasm_files("test/input") {
        Module::decode(&mut BufReader::new(File::open(path).unwrap())).unwrap();
    }
}

#[test]
fn decoding_and_encoding_roundtrips() {
    for path in wasm_files("test/input") {
        let mut wasm_binary_input = Vec::new();
        File::open(&path).unwrap().read_to_end(&mut wasm_binary_input).unwrap();

        let module = Module::decode(&mut Cursor::new(&wasm_binary_input)).unwrap();

        let mut wasm_binary_output = Vec::new();
        module.encode(&mut wasm_binary_output).unwrap();

        let mut output_file = File::create(path.to_string_lossy().replace("input", "output/identity")).unwrap();
        output_file.write_all(&mut &wasm_binary_output[..]).unwrap();

        assert!(wasm_binary_input == wasm_binary_output,
                "{}: encoding and decoding did not round-trip", path.display());
    }
}

#[test]
fn count_calls_produces_valid_wasm() {
    for path in wasm_files("test/input") {
        let mut module = Module::decode(&mut BufReader::new(File::open(&path).unwrap())).unwrap();
        instrument::count_call_instructions(&mut module);

        let output_path = path.to_string_lossy().replace("input", "output/count-calls");
        create_dir_all(Path::new(&output_path).parent().unwrap()).unwrap();
        module.encode(&mut BufWriter::new(File::create(&output_path).unwrap())).unwrap();

        let validate_output = Command::new("wasm-validate")
            .arg(output_path)
            .output()
            .unwrap();

        assert!(validate_output.status.success(),
                "count-calls instrumentation does not validate for {}\n{}",
                path.display(),
                String::from_utf8(validate_output.stderr).unwrap());
    }
}


/* Test encoding/decoding speed (without any instrumentation) on "large" wasm file (~2MB) */

#[bench]
fn decoding_speed(bencher: &mut Bencher) {
    let mut buf = Vec::new();
    File::open("test/input/bananabread/bb.wasm").unwrap().read_to_end(&mut buf).unwrap();

    bencher.iter(|| {
        Module::decode(&mut Cursor::new(&buf)).unwrap();
    })
}

#[bench]
fn encoding_speed(bencher: &mut Bencher) {
    let module = Module::decode(&mut BufReader::new(File::open("test/input/bananabread/bb.wasm").unwrap())).unwrap();

    bencher.iter(|| {
        module.encode(&mut sink()).unwrap();
    })
}


/* Convenience functions */

fn wasm_files<P: AsRef<Path>>(dir: P) -> impl Iterator<Item=PathBuf> {
    WalkDir::new(dir.as_ref()).into_iter()
        .map(Result::unwrap)
        .map(|entry| entry.path().to_owned())
        .filter(|path| path.extension() == Some("wasm".as_ref()))
}
