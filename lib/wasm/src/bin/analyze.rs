use std::{iter::FromIterator, cmp::Reverse};

use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::{FxHashSet, FxHashMap};
use wasm::{wimpl::{FunctionId, Module, Expr, analyze::{collect_call_indirect_idx_expr, print_map_count, collect_i32_load_store_arg_expr, collect_memory_functions, collect_function_direct_call_count, param_exprs}, callgraph::{self, Options}}};

// Profile with cargo flamegraph --bin callgraph -- tests/wasm/WasmBench-nonCpp/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13.wasm
// Before, allow perf to capture traces:
// echo 1 | sudo tee /proc/sys/kernel/perf_event_paranoid
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("{:?}", args);

    let wasm_path = &args[1];
    let wimpl = Module::from_wasm_file(wasm_path).unwrap();

    for func in &wimpl.functions {
        param_exprs(func);
    }

    println!("most frequent call_indirect expressions:");
    print_map_count(&collect_call_indirect_idx_expr(&wimpl));

    let (addr_exprs, value_exprs) = collect_i32_load_store_arg_expr(&wimpl);
    println!("most frequent i32 load/store addr expressions:");
    print_map_count(&addr_exprs);
    println!("most frequent i32 store value expressions:");
    print_map_count(&value_exprs);

    println!("functions with memory.* instructions:");
    for (func, has_memory_size, has_memory_grow) in collect_memory_functions(&wimpl) {
        println!("  {}: memory.size: {:?}, memory.grow: {:?}", func, has_memory_size, has_memory_grow);
    }

    println!("functions with the highest direct call in-degree:");
    print_map_count(&collect_function_direct_call_count(&wimpl));

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

    let (callgraph, _callsites) = callgraph::reachable_callgraph(&wimpl, exported_funcs.clone(), options).unwrap();

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
