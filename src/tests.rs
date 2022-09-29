use test_utilities::*;
use wasabi_wasm::Module;

use crate::instrument::{add_hooks, direct};
use crate::options::HookSet;

const TEST_INPUTS: &str = "tests/inputs";

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

/// Utility function.
fn test_instrument(
    instrument: impl Fn(&mut Module) -> Option<String>,
    instrument_name: &'static str,
) {
    println!("Testing {}", instrument_name);
    for path in wasm_files(TEST_INPUTS).unwrap() {
        println!("wasm file {:?}", path);
        let (mut module, _offsets, _warnings) = Module::from_file(&path).unwrap();
        let javascript = instrument(&mut module);

        let output_path = output_file(&path, instrument_name).unwrap();
        module.to_file(&output_path).unwrap();

        wasm_validate(&output_path)
            .unwrap_or_else(|err| panic!("Binary '{}' instrumented with {} is no longer valid\n{err}", path.display(), instrument_name));

        if let Some(javascript) = javascript {
            std::fs::write(output_path.with_extension("wasabi.js"), javascript).unwrap();
        }
    }
}
