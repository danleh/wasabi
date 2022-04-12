use std::{io::{self, BufRead, Write}, fs::{File, self}, collections::HashMap};

use rustc_hash::{FxHashMap};
use wasm::{wimpl::{self, FunctionId, analyze::{print_map_count, collect_call_indirect_idx_expr}, callgraph::{Options, reachable_callgraph}, traverse::VisitOptionBodyExt}, highlevel::{self, Instr}, Val};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("arguments:");
    println!("{:?}", args);

    let wasm_path = &args[1];
    let exported_funcs_path = &args[2];
    let dce_wasm_path = &args[3];

    let mut wasm = highlevel::Module::from_file(wasm_path).unwrap();
    let wimpl = wimpl::wimplify::wimplify(&wasm).unwrap();

    
    let  mut total_num_exports = 0; 
    for func in &wasm.functions {
        if !func.export.is_empty() {
            total_num_exports += 1;
        }
    }
    
    
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
    let reachable_exports_count = reachable_funcs.len(); 
    for func in &reachable_funcs {
        println!("  {}", func); 
    }
    
    // DEBUG
    // for fun in &wimpl.functions {
    //     println!("{}", fun.name);
    //     for stmt in fun.body.0 {
    //     }
    // }

    let mut call_indirect_count = 0;
    for func in &wimpl.functions {
        func.body.visit_expr_pre_order(|expr| {
            if let wimpl::ExprKind::CallIndirect { .. } = &expr.kind {
                call_indirect_count += 1
            };
            // Continue recursively traversing expressions.
            true
        });
    }
    
    println!("call_indirect count: {}", call_indirect_count);
     
    println!("call_indirect idx expressions:");
    let idx_exprs = collect_call_indirect_idx_expr(&wimpl);
    print_map_count(&idx_exprs);
    
     
    println!("call_indirect i32.load constant addresses:");
    let constant_addr = collect_call_indirect_load_const_addr(&wimpl);
    print_map_count(&constant_addr);
    dump_const_addr_json(&constant_addr).unwrap();
    
    let options = Options {
        with_type_constraint: true,
        with_in_table_constraint: true,
        with_index_constraint: true,
    };
    let (callgraph, callsites) = reachable_callgraph(&wimpl, reachable_funcs.into_iter().collect(), options).unwrap();
    
    callsites.to_file("./analysis_data/callsite_cg_static.txt").expect("Error while writing callsite info to file"); 

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
    
    let retained = wasm.functions.len()-num_removed_funcs;
    let percentage_removed_funcs = (num_removed_funcs as f64 / wasm.functions.len() as f64) * 100.0; 

    println!("statistics:");
    println!("  number of retained functions: {}", retained); 
    println!("  number of removed functions: {} ({:.2}%)", num_removed_funcs, percentage_removed_funcs); 
    println!("  removed functions:");
    for func in removed_funcs {
        println!("    {}", func); 
    }
    
    let original_wasm_file_size = fs::metadata(wasm_path).unwrap().len();
    let dce_wasm_file_size = fs::metadata(dce_wasm_path).unwrap().len();
    let delta_file_size = original_wasm_file_size - dce_wasm_file_size;
    let delta_file_percentage = (delta_file_size as f64 / original_wasm_file_size as f64) * 100.0; 
    
    
    
    println!("  size reduction: {} bytes ({:.2}%)", delta_file_size, delta_file_percentage); 
    let total: f64 = idx_exprs.iter().map(|(_, count)| *count as f64).sum();
    let (highest_expr, highest_expr_count) = idx_exprs.iter().max_by(|a,b|a.1.cmp(b.1)).unwrap(); 
    let highest_expr_percent = *highest_expr_count as f64 / total * 100.0; 
    let mut f = File::create("./analysis_data/analysis-stats.json").unwrap();
    writeln!(f, "
{{
    \"wasm-file-analyzed\": \"{}\", 
    \"#total-exports\": {},
    \"#reachable-exports\": {},
    \"#functions\": {}, 
    \"#retained-functions\": {}, 
    \"#removed-functions\": {},
    \"#%removed\": {:.2}, 
    \"size-reduction(bytes)\": {}, 
    \"%size-reduction\": {:.2}, 
    \"highest-expr-for-idx\": \"{}\", 
    \"#%highest-expr-fox-idx-percent\": {:.2} 
}}
    ", 
    wasm_path, 
    total_num_exports, 
    reachable_exports_count,
    wasm.functions.len(),
    retained, 
    num_removed_funcs, 
    percentage_removed_funcs, 
    delta_file_size,
    delta_file_percentage, 
    highest_expr, 
    highest_expr_percent).unwrap();
}

/// Returns a slightly abstracted form of the call_indirect expressions, sorted descending by count.
pub fn collect_call_indirect_load_const_addr(module: &wimpl::Module) -> FxHashMap<u32, usize> {
    let mut result: FxHashMap<u32, usize> = FxHashMap::default();
    for func in &module.functions {
        use wimpl::ExprKind::*;
        func.body.visit_expr_pre_order(|expr| {
            if let CallIndirect { type_: _, table_idx, args: _ } = &expr.kind {
                if let Load {
                    op: crate::highlevel::LoadOp::I32Load,
                    addr,
                } = &table_idx.kind {
                    if let Const(Val::I32(const_addr)) = &addr.kind {
                        *result.entry(*const_addr as u32).or_default() += 1;
                    }
                }
            }
            true
        });
    }
    result
}

fn dump_const_addr_json(addr_to_count: &FxHashMap<u32, usize>) -> io::Result<()> {
    let mut f = File::create("./analysis_data/call_indirect_load_const_addr.json")?;
    writeln!(f, "[")?;
    for (idx, (addr, _count)) in addr_to_count.iter().enumerate() {
        if idx == addr_to_count.len()-1 {
            writeln!(f, "{}", addr)?; 

        } else {
            writeln!(f, "{}, ", addr)?; 
        }
    }
    writeln!(f, "]")
}
