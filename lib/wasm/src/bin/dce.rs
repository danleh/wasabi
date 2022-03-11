use std::{io::{self, BufRead}, fs::File};

use rustc_hash::FxHashSet;
use wasm::wimpl::{self, FunctionId, analyze::{print_map_count, collect_call_indirect_idx_expr}};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("{:?}", args);

    let wasm_path = &args[1];
    let exported_funcs_path = &args[2];

    let wimpl = wimpl::Module::from_wasm_file(wasm_path).unwrap();
    let exported_funcs = {
        let file = File::open(exported_funcs_path).unwrap();
        io::BufReader::new(file)
            .lines()
            .map(|line| FunctionId::from_name(line.unwrap()))
            .collect::<FxHashSet<_>>()
    };

    let idx_exprs = collect_call_indirect_idx_expr(&wimpl);
    print_map_count(&idx_exprs);

    // for fun in wimpl_module.functions {
    //     for stmt in fun.body.0 {
        
    //     }
    // }
    
}