use wasm::{wimpl::wimplify::wimplify, callgraph};

// Profile with cargo flamegraph --bin callgraph -- tests/wasm/WasmBench-nonCpp/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13.wasm
// Before, allow perf to capture traces:
// echo 1 | sudo tee /proc/sys/kernel/perf_event_paranoid
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("{:?}", args);

    let wasm_path = &args[1];
    let wimpl = wimplify(wasm_path).unwrap();

    let options = callgraph::Options {
        with_type_constraint: true,
        with_in_table_constraint: true,
        with_index_constraint: false,
    };

    let exported_funcs = wimpl.functions.iter()
        .filter(|func| !func.export.is_empty())
        .map(|func| func.name())
        .collect();

    let callgraph = callgraph::reachable_callgraph(&wimpl, exported_funcs, options).unwrap();
    
    // let dot = callgraph.to_dot();
    // println!("{}", dot);

    if let Some(out_pdf_path) = args.get(2) {
        callgraph.to_pdf(out_pdf_path).unwrap();
    }
}
