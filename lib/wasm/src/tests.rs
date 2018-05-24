use test_utilities::*;
use ast::{lowlevel, highlevel};

static TEST_INPUTS: &'static str = "../../tests/inputs";

#[test]
fn decode_encode_is_valid_wasm() {
    for path in wasm_files(TEST_INPUTS) {
        println!("decode \"{}\"", path.display());
        let module = highlevel::Module::from_file(&path).unwrap();

        println!("encode again");
        let output_path = &output_file(path, "encode");
        module.to_file(output_path).unwrap();

        wasm_validate(output_path).unwrap();
    }
}