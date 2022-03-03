use std::{iter::FromIterator, cmp::Reverse};

use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::{FxHashSet, FxHashMap};
use wasm::{wimpl::{FunctionId, Module, Expr}, callgraph::{self, Options}};

// Profile with cargo flamegraph --bin callgraph -- tests/wasm/WasmBench-nonCpp/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13.wasm
// Before, allow perf to capture traces:
// echo 1 | sudo tee /proc/sys/kernel/perf_event_paranoid
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("{:?}", args);

    let wasm_path = &args[1];
    let wimpl = Module::from_wasm_file(wasm_path).unwrap();

    println!("most frequent call_indirect expressions:");
    let idx_exprs = collect_call_indirect_idx_expr(&wimpl);
    for (expr, count) in idx_exprs.iter().take(30) {
        println!("{:8}  {}", count, expr);
    }

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

fn collect_call_indirect_idx_expr(module: &Module) -> Vec<(String, usize)> {
    let mut result: FxHashMap<String, usize> = FxHashMap::default();
    for func in &module.functions {
        use Expr::*;
        func.body.visit_expr_pre_order(|expr| 
            if let CallIndirect { type_: _, table_idx, args: _ } = expr {
                let table_idx = table_idx.to_string();

                // HACK remove some stuff that is irrelevant for our analysis
                lazy_static! {
                    static ref MEMARG: Regex = Regex::new(r"\s+offset=\d+\s+").unwrap();
                    static ref PARAM: Regex = Regex::new(r"p\d+").unwrap();
                    static ref LOCAL: Regex = Regex::new(r"l\d+").unwrap();
                    static ref CONST: Regex = Regex::new(r"const \d+").unwrap();
                }
                let table_idx = MEMARG.replace_all(&table_idx, "");
                let table_idx = PARAM.replace_all(&table_idx, "<param>");
                let table_idx = LOCAL.replace_all(&table_idx, "<local>");
                let table_idx = CONST.replace_all(&table_idx, "const <const>");

                *result.entry(table_idx.to_string()).or_default() += 1;
            }
        );
    }
    let mut result = Vec::from_iter(result);
    result.sort_by_key(|(expr, count)| Reverse((*count, expr.clone())));
    result
}