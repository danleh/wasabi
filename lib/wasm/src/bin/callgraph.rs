use rustc_hash::FxHashSet;
use wasm::{wimpl::{wimplify::wimplify, FunctionId, Module}, callgraph::{self, Options}, highlevel::Function};

// Profile with cargo flamegraph --bin callgraph -- tests/wasm/WasmBench-nonCpp/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13.wasm
// Before, allow perf to capture traces:
// echo 1 | sudo tee /proc/sys/kernel/perf_event_paranoid
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("{:?}", args);

    let wasm_path = &args[1];
    let wimpl = Module::from_wasm_file(wasm_path).unwrap();

    let options = Options {
        with_type_constraint: true,
        with_in_table_constraint: true,
        with_index_constraint: true,
        ..Options::default()
    };

    let exported_funcs: FxHashSet<FunctionId> = wimpl.functions.iter()
        .filter(|func| !func.export.is_empty())
        .map(|func| func.name())
        .collect();

    let callgraph = callgraph::reachable_callgraph(&wimpl, exported_funcs.clone(), options).unwrap();

    println!("number of functions: {}", wimpl.functions.len());
    println!("number of exported functions: {}", exported_funcs.len());
    println!("number of reachable functions: {}", callgraph.functions().len());
    println!("number of reachable call graph edges: {}", callgraph.edges().count());
    
    // let dot = callgraph.to_dot();
    // println!("{}", dot);

    if let Some(out_pdf_path) = args.get(2) {
        callgraph.to_pdf(out_pdf_path).unwrap();
    }
}
