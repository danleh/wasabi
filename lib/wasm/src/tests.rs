use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

use bencher::{Bencher, benchmark_group, benchmark_main};
use test_utilities::*;

use crate::{highlevel, lowlevel};
use crate::WasmBinary;

const TEST_INPUTS: &'static str = "../../tests/inputs";
const LARGE_WASM_FILE: &'static str = "../../tests/inputs/real-world/bananabread/bb.wasm";

#[test]
fn decode_encode_is_valid_wasm() {
    for path in wasm_files(TEST_INPUTS).unwrap() {
        println!("{}", path.display());
        let module = highlevel::Module::from_file(&path)
            .expect(&format!("could not decode valid wasm file '{}'", path.display()));

        let output_path = &output_file(path, "encode").unwrap();
        module.into_file(output_path)
            .expect(&format!("could not encode wasm to file '{}'", output_path.display()));

        wasm_validate(output_path)
            .expect(&format!("could not validate wasm output file '{}'", output_path.display()));
    }
}

#[test]
fn error_offsets_correct() {
    fn assert_error_offset(binary: Vec<u8>, expected_offset: usize) {
        let mut offset = 0;
        let result = lowlevel::Module::decode(&mut binary.as_slice(), &mut offset);
        assert!(result.is_err(), "binary {:?} was not invalid, but should have been", binary);
        let err = result.err().unwrap();
        assert_eq!(err.offset(), expected_offset, "\nfull error: {}\n(source: {:?})", err, err.source());
    }

    let wrong_magic_number = b"Xasm\x01\x00\x00\x00".to_vec();
    assert_error_offset(wrong_magic_number, 0);

    let wrong_version = b"\x00asm\x02\x00\x00\x00".to_vec();
    assert_error_offset(wrong_version, 4);

    let wrong_section_id = b"\x00asm\x01\x00\x00\x00\xff".to_vec();
    assert_error_offset(wrong_section_id, 8);

    let valid_wasm_header = b"\x00asm\x01\x00\x00\x00".as_ref();
    let valid_code_element = &[
        // Size in bytes
        2,
        // Local count
        0,
        // Body: only end instruction
        0xb
    ].as_ref();

    let section_size_too_short = [valid_wasm_header, &[
        // Section id: code
        10,
        // WRONG Section size in bytes: should be 4 bit is 2
        2,
        // Code elements
        1,
    ], valid_code_element].concat();
    assert_error_offset(section_size_too_short, 9);

    let section_size_too_long = [valid_wasm_header, &[10, 6 /* instead of 4 */, 1], valid_code_element].concat();
    assert_error_offset(section_size_too_long, 9);

    let code_element_size_too_short = [valid_wasm_header, &[
        // Code section, size in bytes, element count
        10, 5, 1,
        // Code element 1
        // WRONG Size in bytes: should be 3
        2,
        // Local count
        0,
        // Body: nop, end
        0x1, 0xb
    ]].concat();
    // NOTE Different from the section size, the size in the "skip list"-like vector in the code
    // section causes the error to be attributed to the position where the Eof appears, not the
    // section size.
    assert_error_offset(code_element_size_too_short, 14);

    let code_element_size_too_long = [valid_wasm_header, &[
        // Code section, size in bytes, element count
        10, 6, 1,
        // Code element 1
        // WRONG Size in bytes: should be 3
        4,
        // Local count
        0,
        // Body: nop, end
        0x1, 0xb,
        // fill the file with more crap so that the read() call succeeds at least
        0xff
    ]].concat();
    // When the element size was too long, the error will be attributed to the size.
    assert_error_offset(code_element_size_too_long, 11);

    let invalid_instruction = [valid_wasm_header, &[
        // Code section, size in bytes, element count
        10, 5, 1,
        // Code element size, local count
        3, 0,
        // Body: WRONG
        0xff, 0xb
    ]].concat();
    assert_error_offset(invalid_instruction, 13);
}



/*
 * Speed benchmarks (for parallelization of decoding/encoding) on a "large" wasm file (~2MB for now)
 */

benchmark_group!(benches, decode_lowlevel_speed, encode_lowlevel_speed,
                          convert_lowlevel_to_highlevel_speed, convert_highlevel_to_lowlevel_speed,
                          clone_lowlevel_module_speed, clone_highlevel_module_speed);
benchmark_main!(benches);

fn decode_lowlevel_speed(bencher: &mut Bencher) {
    let mut buf = Vec::new();
    File::open(LARGE_WASM_FILE).unwrap().read_to_end(&mut buf).unwrap();

    bencher.iter(|| {
        let mut offset = 0;
        lowlevel::Module::decode(&mut buf.as_slice(), &mut offset).unwrap()
    })
}

fn encode_lowlevel_speed(bencher: &mut Bencher) {
    let module = lowlevel::Module::from_file(LARGE_WASM_FILE).unwrap();

    bencher.iter(||
        module.encode(&mut io::sink()).unwrap())
}

fn convert_lowlevel_to_highlevel_speed(bencher: &mut Bencher) {
    let module = lowlevel::Module::from_file(LARGE_WASM_FILE).unwrap();

    bencher.iter(|| {
        let _: highlevel::Module = module.clone().into();
    })
}

fn convert_highlevel_to_lowlevel_speed(bencher: &mut Bencher) {
    let module = highlevel::Module::from_file(LARGE_WASM_FILE).unwrap();

    bencher.iter(|| {
        let _: lowlevel::Module = module.clone().into();
    })
}

// as baseline for conversions high-level <-> low-level (where we need to clone -.-)
fn clone_lowlevel_module_speed(bencher: &mut Bencher) {
    let module = lowlevel::Module::from_file(LARGE_WASM_FILE).unwrap();
    bencher.iter(|| module.clone())
}

fn clone_highlevel_module_speed(bencher: &mut Bencher) {
    let module = highlevel::Module::from_file(LARGE_WASM_FILE).unwrap();
    bencher.iter(|| module.clone())
}
