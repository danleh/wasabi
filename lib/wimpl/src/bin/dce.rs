use std::{io::{self, BufRead, Write}, fs::{File, self}, collections::HashMap};

use rustc_hash::FxHashMap;

use wasm::{highlevel::{self, Instr}, Val, Idx};
use wimpl::{self, FunctionId, analyze::{print_map_count, collect_call_indirect_idx_expr}, callgraph::{Options, reachable_callgraph}, traverse::VisitOptionBodyExt};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("arguments:");
    println!("{:?}", args);

    let wasm_path = &args[1];
    let exports_path = &args[2];
    let dce_wasm_path = &args[3];
    
    let flag_dce = args[4] == "-dce";
    let flag_cg = args[4] == "-cg"; 

    let cg_path = args[5].clone(); 
    
    let mut wasm = highlevel::Module::from_file(wasm_path).unwrap();
    let wimpl = wimpl::wimplify::wimplify(&wasm).unwrap();

    let  mut total_num_exports = 0; 
    for func in &wasm.functions {
        if !func.export.is_empty() {
            total_num_exports += 1;
        }
    }

    let wasm_idx_to_wimpl_func_id = FunctionId::from_module(&wasm);
    
    let mut reachable_idx = Vec::new();
    let mut reachable_funcs = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(exports_path).unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        if record[1].to_string() == "function" {
            let idx = record[2].to_string().parse::<usize>().unwrap();
            // TODO: We cannot do the following: reachable_funcs.push(FunctionId::from_u32(idx, &wasm).expect(""));
            // Because there is a mismatch between function names and function indicies for wimpl/wasm
            // There is some code that does not recognize that an exported function can be referenced by 
            // name as well as by idx. Hence, this code errors saying that constraint not computed for <idx>. 
            // Right now I just ensure that first export names of a function are made initially reachable. 
            // but this should be fixed properly later.
            // FIXME I (Daniel) tried to fix this code, after fixing the non-uniqueness of function names, but I am not
            // sure I got it right. @Michelle, probably better to have a good look whether that still makes sense to you.
            if !reachable_idx.contains(&idx) {
                reachable_idx.push(idx);
                reachable_funcs.push(wasm_idx_to_wimpl_func_id[idx].clone());
            }
        }
        else if record[1].to_string() == "table" {
            for elem in wimpl.table.clone().expect("A table that is exported has to exist in the binary").elements {                
                for func in elem.functions {
                    reachable_funcs.push(func);   
                } 
            }                
        }
    }

    // println!("initially reachable functions:");
    let reachable_exports_count = reachable_funcs.len(); 
    // for func in &reachable_funcs {
    //     println!("  {}", func); 
    // }
    
    // DEBUG
    // for fun in &wimpl.functions {
    //     println!("{}", fun.name);
    //     for stmt in fun.body.0 {
    //     }
    // }

    // let mut call_indirect_count = 0;
    // for func in &wimpl.functions {
    //    func.body.visit_expr_pre_order(|expr| {
    //        if let wimpl::ExprKind::CallIndirect { .. } = &expr.kind {
    //            call_indirect_count += 1
    //        };
    //        // Continue recursively traversing expressions.
    //        true
    //    });
    // }
    
    // println!("call_indirect count: {}", call_indirect_count);
     
    // println!("call_indirect idx expressions:");
    let idx_exprs = collect_call_indirect_idx_expr(&wimpl);
    // print_map_count(&idx_exprs);
    
     
    // println!("call_indirect i32.load constant addresses:");
    // let constant_addr = collect_call_indirect_load_const_addr(&wimpl);
    // print_map_count(&constant_addr);
    // dump_const_addr_json(&constant_addr);
    
    let options = Options {
        with_type_constraint: true,
        with_in_table_constraint: true,
        with_index_constraint: true,
    };
    
    let wimpl_callgraph = reachable_callgraph(&wimpl, reachable_funcs.into_iter().collect(), options).unwrap();
    let callgraph = wimpl_callgraph.callgraph; 
    let callsites = wimpl_callgraph.callsites; 

    if flag_cg { 
        callgraph.to_dot_file(cg_path); 
    }
    // callsites.to_file("./analysis_data/callsite_cg_static.txt").expect("Error while writing callsite info to file"); 
    // callsites.to_detailed_info_file("./analysis_data/callsite_cg_static_detailed.txt").expect("Error while writing callsite info to file"); 

    // DEBUG
    // println!("{:?}", callgraph);
    
    if flag_dce {

        let reachable_funcs = callgraph.functions();

        let mut num_removed_funcs = 0;
        let mut removed_funcs = Vec::new(); 

        for (idx, func) in wasm.functions.iter_mut().enumerate() {
            let func_id = wasm_idx_to_wimpl_func_id[idx].clone();
            let reachable = reachable_funcs.contains(&func_id);
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
        
        let delta_file_size = (original_wasm_file_size as f64) - (dce_wasm_file_size as f64);
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
