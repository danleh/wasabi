use crate::ast::{highlevel, lowlevel};
use crate::binary::WasmBinary;
use std::fs::File;
use std::io::{self, Read};
use test::Bencher;
use test_utilities::*;

const TEST_INPUTS: &'static str = "../../tests/inputs";
const LARGE_WASM_FILE: &'static str = "../../tests/inputs/real-world/bananabread/bb.wasm";

#[test]
fn decode_encode_is_valid_wasm() {
    for path in wasm_files(TEST_INPUTS).unwrap() {
        let module = highlevel::Module::from_file(&path)
            .expect(&format!("could not decode valid wasm file '{}'", path.display()));

        let output_path = &output_file(path, "encode").unwrap();
        module.to_file(output_path)
            .expect(&format!("could not encode wasm to file '{}'", output_path.display()));

        wasm_validate(output_path)
            .expect(&format!("could not validate wasm output file '{}'", output_path.display()));
    }
}

/*
 * Speed benchmarks (for parallelization of decoding/encoding) on a "large" wasm file (~2MB for now)
 */

#[bench]
fn decode_lowlevel_speed(bencher: &mut Bencher) {
    let mut buf = Vec::new();
    File::open(LARGE_WASM_FILE).unwrap().read_to_end(&mut buf).unwrap();

    bencher.iter(||
        lowlevel::Module::decode(&mut io::Cursor::new(&buf)).unwrap())
}

#[bench]
fn encode_lowlevel_speed(bencher: &mut Bencher) {
    let module = lowlevel::Module::from_file(LARGE_WASM_FILE).unwrap();

    bencher.iter(||
        module.encode(&mut io::sink()).unwrap())
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
