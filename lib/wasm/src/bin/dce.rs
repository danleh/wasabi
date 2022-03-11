use std::{io::{self, BufRead}, fs::{File, self}, collections::HashMap};

use rustc_hash::FxHashSet;
use wasm::{wimpl::{self, FunctionId, analyze::{print_map_count, collect_call_indirect_idx_expr}, callgraph::{Options, reachable_callgraph}}, highlevel::{self, Instr}};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("arguments:");
    println!("{:?}", args);

    let wasm_path = &args[1];
    let exported_funcs_path = &args[2];
    let dce_wasm_path = &args[3];

    let mut wasm = highlevel::Module::from_file(wasm_path).unwrap();
    let wimpl = wimpl::wimplify::wimplify(&wasm).unwrap();

    let reachable_funcs = {
        let file = File::open(exported_funcs_path).unwrap();
        io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap().trim().to_owned())
            .filter(|line| !line.is_empty())
            .map(FunctionId::from_name)
            .collect::<Vec<_>>()
    };

    println!("initially reachable functions:");
    for func in &reachable_funcs {
        println!("  {}", func); 
    }
    
    // DEBUG
    // for fun in &wimpl.functions {
    //     println!("{}", fun.name);
    //     for stmt in fun.body.0 {
    //     }
    // }
    
    println!("call_indirect idx expressions:");
    let idx_exprs = collect_call_indirect_idx_expr(&wimpl);
    print_map_count(&idx_exprs);

    let options = Options {
        with_type_constraint: true,
        with_in_table_constraint: true,
        with_index_constraint: true,
    };
    let callgraph = reachable_callgraph(&wimpl, reachable_funcs.into_iter().collect(), options).unwrap();
    // DEBUG
    // println!("{:?}", callgraph);
    let reachable_funcs = callgraph.functions();

    let wasm_idx_to_wimpl_func_id = wasm.functions()
        .map(|(idx, _)| (idx, FunctionId::from_idx(idx, &wasm)))
        .collect::<HashMap<_, _>>();

    let mut num_removed_funcs = 0;
    let mut removed_funcs = Vec::new(); 

    for (idx, func) in wasm.functions_mut() {
        let func_id = wasm_idx_to_wimpl_func_id.get(&idx).expect("we just converted all wasm Idx to wimpl FunctionId successfully!?");
        let reachable = reachable_funcs.contains(func_id);
        if !reachable {
            num_removed_funcs += 1; 
            removed_funcs.push(func_id); 

            // We cannot remove the function itself, because that would change the function indices,
            // and then we would have to update all instructions, tables, etc.
            // HACK We do the next best thing, which is remove everything from the function.
            if let Some(code) = func.code_mut() {
                code.body = vec![Instr::Unreachable, Instr::End];
                code.locals = Vec::new();
            }
            func.export = Vec::new();
            func.name = None;
        }
    }
    
    wasm.to_file(dce_wasm_path).unwrap();

    println!("statistics:");
    println!("  number of removed functions: {} ({:.2}%)", num_removed_funcs, (num_removed_funcs as f64 / wasm.functions.len() as f64) * 100.0); 
    println!("  removed functions:");
    for func in removed_funcs {
        println!("    {}", func); 
    }
    
    let original_wasm_file_size = fs::metadata(wasm_path).unwrap().len();
    let dce_wasm_file_size = fs::metadata(dce_wasm_path).unwrap().len();
    let delta_file_size = original_wasm_file_size - dce_wasm_file_size;
    println!("  size reduction: {} bytes ({:.2}%)", delta_file_size, (delta_file_size as f64 / original_wasm_file_size as f64) * 100.0); 

}