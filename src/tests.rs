use instrument::{add_hooks, direct::*};
use test_utilities::*;
use wasm::ast::highlevel::Module;

const TEST_INPUTS: &'static str = "tests/inputs";

#[test]
fn add_empty_function_produces_valid_wasm() {
    test_instrument(add_empty_function, "add-empty-function");
}

#[test]
fn count_calls_instrumentation_produces_valid_wasm() {
    test_instrument(count_calls, "count-calls");
}

#[test]
fn add_hooks_instrumentation_produces_valid_wasm() {
    test_instrument(add_hooks, "add-hooks");
}

/// utility function
fn test_instrument(instrument: impl Fn(&mut Module) -> Option<String>, instrument_name: &'static str) {
    for path in wasm_files(TEST_INPUTS) {
        let mut module = Module::from_file(&path).unwrap();
        instrument(&mut module);

        let output_path = output_file(&path, instrument_name);
        module.to_file(&output_path).unwrap();

        wasm_validate(&output_path)
            .expect(&format!("could not instrument wasm file '{}' with {}", path.display(), instrument_name));
    }
}