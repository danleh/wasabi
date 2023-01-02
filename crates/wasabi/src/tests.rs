use test_utilities::*;
use wasabi_wasm::Module;

use rayon::prelude::*;

use crate::instrument::add_hooks;
use crate::instrument::direct;
use crate::options::HookSet;

#[test]
fn add_empty_function_produces_valid_wasm() {
    test_instrument(|module| {
        direct::add_empty_function(module);
        None
    }, "add-empty-function");
}

#[test]
fn count_calls_instrumentation_produces_valid_wasm() {
    test_instrument(|module| {
        direct::count_calls(module);
        None
    }, "count-calls");
}

#[test]
fn add_hooks_instrumentation_produces_valid_wasm() {
    test_instrument(|module| {
        add_hooks(module, HookSet::all(), false).map(|opt| opt.0)
    }, "add-hooks");
}

/// Utility function.
fn test_instrument(instrument: fn(&mut Module) -> Option<String>, instrument_name: &'static str) {
    // Filter out files that are too large to run in CI.
    // FIXME: Wasabi OOM, debug allocations with heaptrack.
    let valid_binaries: Vec<_> = ALL_VALID_TEST_BINARIES
        .par_iter()
        .filter(|path| std::fs::metadata(path).unwrap().len() < 10_000_000)
        .collect();

    valid_binaries.par_iter().for_each(|path| {
        let (mut module, _offsets, _warnings) = Module::from_file(path).unwrap();
        let javascript = instrument(&mut module);

        let output_path = output_file(path, instrument_name).unwrap();
        module.to_file(&output_path).unwrap();

        wasm_validate(&output_path)
            .unwrap_or_else(|err| {
                let bytes = std::fs::read(&output_path).unwrap();
                let size = bytes.len();
                let sha256_hash = sha256::digest(bytes.as_slice());
                panic!("Binary '{}' instrumented with {} is no longer valid\n{err}\nSize: {size}\nSHA256: {sha256_hash}", path.display(), instrument_name)
            });

        if let Some(javascript) = javascript {
            std::fs::write(output_path.with_extension("wasabi.js"), javascript).unwrap();
        }
    });
}
