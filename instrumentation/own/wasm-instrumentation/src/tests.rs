use ast::Module;
use binary::WasmBinary;
use std::fs::File;
use std::io::{BufReader, Cursor, Read, sink, Write};
use std::path::{Path, PathBuf};
use test::Bencher;
use walkdir::WalkDir;

#[test]
fn decoding_valid_files_works() {
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
