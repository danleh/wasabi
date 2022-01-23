use wasm::*;

use criterion::{criterion_group, criterion_main, Criterion};

// const WASM_TEST_INPUT_LARGE: &str = "../../tests/inputs/real-world/bananabread/bb.wasm";
const WASM_TEST_INPUT_LARGE: &str = "../../tests/inputs/real-world/unreal-engine-4/UE4Game-HTML5-Shipping.wasm";

fn bench_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser");
    group.bench_function("old", |b| {
        b.iter(|| highlevel::Module::from_file_with_offsets(WASM_TEST_INPUT_LARGE))
    });
    group.bench_function("old, low-level only", |b| {
        b.iter(|| lowlevel::Module::from_file_with_offsets(WASM_TEST_INPUT_LARGE))
    });
    group.bench_function("new", |b| {
        b.iter(|| highlevel::Module::from_file_with_offsets_wasmparser(WASM_TEST_INPUT_LARGE))
    });
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);
