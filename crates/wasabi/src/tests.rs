use test_utilities::*;
use wasabi_wasm::Module;

use crate::instrument::{add_hooks, direct};
use crate::options::HookSet;

const TEST_INPUTS: &str = "tests/inputs";

// TODO unify test input files of lib/wasm and wasabi
const WASMBENCH_DIR: &str = "lib/wasabi_wasm/tests/WasmBench/valid-no-extensions";
const WASMBENCH_EXCLUDED_FILES: [&str; 3] = [
    // Valid, but creates very large allocations because it has >500k locals in >1k functions.
    "31fa012442fd637fca221db4fda94262e99759ab9667147cbedde083aabcc065",
    // Is actually invalid according to wasm-validate, not sure why it wasn't filtered out before.
    "4b1082f1c2d634aaebbe9b70331ac6639ab3fe7b0a52459ea4f6baa4f82a82ad",
    // Panics in the old parser, but works in the new one -> strictly better, so ignore.
    "a50d67cbaf770807cc1d1723ebc56333188b681538bf3f7679659b184d2f8020"
];

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

    let wasm_files = wasm_files(TEST_INPUTS).unwrap();

    // TODO Test on all of WasmBench
    // let mut wasm_files = wasm_files(WASMBENCH_DIR).unwrap();
    // for hash in WASMBENCH_EXCLUDED_FILES {
    //     wasm_files.retain(|path| !path.to_string_lossy().contains(hash));
    // }

    for path in wasm_files {
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
