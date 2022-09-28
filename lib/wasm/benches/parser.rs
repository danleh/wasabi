use wasm::*;

use criterion::{criterion_group, criterion_main, Criterion};

// const WASM_TEST_INPUT_LARGE: &str = "../../tests/inputs/real-world/bananabread/bb.wasm";
const WASM_TEST_INPUT_LARGE: &str = "../../tests/inputs/real-world/unreal-engine-4/UE4Game-HTML5-Shipping.wasm";

fn bench_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser");
    group.bench_function("new", |b| {
        b.iter(|| Module::from_file(WASM_TEST_INPUT_LARGE))
    });
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);


// /*
//  * Speed benchmarks (for parallelization of decoding/encoding) on a "large" wasm file (~2MB for now)
//  */

// benchmark_group!(benches, decode_lowlevel_speed, encode_lowlevel_speed,
//     convert_lowlevel_to_highlevel_speed, convert_highlevel_to_lowlevel_speed,
//     clone_lowlevel_module_speed, clone_highlevel_module_speed,
// );
// benchmark_main!(benches);

// fn decode_lowlevel_speed(bencher: &mut Bencher) {
// let mut buf = Vec::new();
// File::open(WASM_TEST_INPUT_LARGE).unwrap().read_to_end(&mut buf).unwrap();

// bencher.iter(|| {
// let mut state = DecodeState::new();
// lowlevel::Module::decode(&mut buf.as_slice(), &mut state).unwrap()
// })
// }

// fn encode_lowlevel_speed(bencher: &mut Bencher) {
// let module = lowlevel::Module::from_file(WASM_TEST_INPUT_LARGE).unwrap();

// bencher.iter(||
// module.encode(&mut io::sink()).unwrap())
// }

// fn convert_lowlevel_to_highlevel_speed(bencher: &mut Bencher) {
// let module = lowlevel::Module::from_file(WASM_TEST_INPUT_LARGE).unwrap();

// bencher.iter(|| {
// let _: highlevel::Module = module.clone().into();
// })
// }

// fn convert_highlevel_to_lowlevel_speed(bencher: &mut Bencher) {
// let module = highlevel::Module::from_file(WASM_TEST_INPUT_LARGE).unwrap();

// bencher.iter(|| {
// let _: lowlevel::Module = (&module).into();
// })
// }

// // as baseline for conversions high-level <-> low-level (where we need to clone -.-)
// fn clone_lowlevel_module_speed(bencher: &mut Bencher) {
// let module = lowlevel::Module::from_file(WASM_TEST_INPUT_LARGE).unwrap();
// bencher.iter(|| module.clone())
// }

// fn clone_highlevel_module_speed(bencher: &mut Bencher) {
// let module = highlevel::Module::from_file(WASM_TEST_INPUT_LARGE).unwrap();
// bencher.iter(|| module.clone())
// }
