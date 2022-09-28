use test_utilities::*;
use wasm::highlevel::Module;

use crate::instrument::{add_hooks, direct};
use crate::options::HookSet;

const TEST_INPUTS: &'static str = "tests/inputs";

#[test]
fn add_empty_function_produces_valid_wasm() {
    test_instrument(direct::add_empty_function, "add-empty-function");
}

#[test]
fn count_calls_instrumentation_produces_valid_wasm() {
    test_instrument(direct::count_calls, "count-calls");
}

#[test]
fn add_hooks_instrumentation_produces_valid_wasm() {
    fn add_all_hooks(module: &mut Module) -> Option<String> {
        add_hooks(module, HookSet::all(), false)
    }
    test_instrument(add_all_hooks, "add-hooks");
}

/// utility function
fn test_instrument(
    instrument: impl Fn(&mut Module) -> Option<String>,
    instrument_name: &'static str,
) {
    println!("Testing {}", instrument_name);
    for path in wasm_files(TEST_INPUTS).unwrap() {
        println!("wasm file {:?}", path);
        let mut module = Module::from_file(&path).unwrap();
        let javascript = instrument(&mut module);

        let output_path = output_file(&path, instrument_name).unwrap();
        module.to_file(&output_path).unwrap();

        wasm_validate(&output_path).expect(&format!(
            "could not instrument wasm file '{}' with {}",
            path.display(),
            instrument_name
        ));

        for javascript in javascript {
            ::std::fs::write(output_path.with_extension("wasabi.js"), javascript).unwrap();
        }
    }
}
