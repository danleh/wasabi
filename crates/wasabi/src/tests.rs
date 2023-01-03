use std::sync::atomic::AtomicUsize;

use test_utilities::*;
use wasabi_wasm::Module;

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
    let skipped_count = AtomicUsize::new(0);

    for_each_valid_wasm_binary_in_test_set(|path| {
        // Filter out files that are too large to run in CI.
        // FIXME: Wasabi OOM, debug allocations with heaptrack.
        if instrument_name == "add-hooks" && std::fs::metadata(path).unwrap().len() > 10_000_000 {
            skipped_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            return;
        }

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

    let skipped_count = skipped_count.into_inner();
    if skipped_count > 0 {
        println!("Skipped instrumenting {skipped_count} .wasm input files because they are too large for CI.");
    }
}
