use core::fmt;
use std::{path::Path, io::{self, Write}, process::{Command, Stdio}, fs::{File, self}, sync::Mutex, iter::FromIterator, cmp::Reverse, default, collections::{HashSet, HashMap}};
use rustc_hash::{FxHashSet, FxHashMap};

use wasabi_wasm::{self as wasm, FunctionType, Val, ValType};

use crate::{Module, FunctionId, ExprKind::{Call, self}, Function, Var, Body, analyze::{VarExprMap, VarExprMapResult, collect_call_indirect_idx_expr, abstract_expr, sort_map_count, collect_i32_load_store_arg_expr, print_map_count, approx_i32_eval, I32Range}, Expr, Stmt};
use crate::{InstrId, StmtKind};
use crate::wimplify::*;

#[derive(Default, Clone, Eq, PartialEq)]
pub struct WimplCallGraph {
    pub callgraph: CallGraph, 
    pub callsites: CallSites
}

#[derive(Default, Clone, Eq, PartialEq)]
pub struct CallGraph(FxHashMap<FunctionId, FxHashSet<FunctionId>>);

// Get each edge in the call graph 
#[derive(Default, Clone, Eq, PartialEq)]
pub struct CallSites(std::collections::BTreeMap<
    (wasm::Idx<wasm::Function>, Option<wasm::Idx<wasm::Instr>>), 
    (wasm::Idx<wasm::Function>, Option<Vec<Constraint>>)
>);

impl CallSites {
    pub fn add_edge(&mut self, 
        wasm_loc: Option<wasm::Idx<wasm::Instr>>, 
        src: wasm::Idx<wasm::Function>, 
        target: wasm::Idx<wasm::Function>,
        target_info: Option<Vec<Constraint>>, 
    ){
        self.0.insert(
            (src, wasm_loc), 
            (target, target_info), 
        );
    }
    
    pub fn to_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut file = File::create(path)?;    
        for ((instr_num, src), (target, _target_info)) in self.0.clone() {
            write!(file, "f{}", instr_num.to_u32())?; 
            if let Some(src) = src { write!(file, ":{}", src.to_u32())? }; 
            writeln!(file, " -> f{} ", target.to_u32())?; 
        }; 
        Ok(())
    }

    pub fn to_detailed_info_file(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut file = File::create(path)?;    
        for ((instr_num, src), (target, target_constraints)) in self.0.clone() {
            write!(file, "f{}", instr_num.to_u32())?; 
            if let Some(src) = src { write!(file, ":{}", src.to_u32())? }; 
            write!(file, " -> f{} ", target.to_u32())?; 
            if src.is_none() {
                write!(file, "(f{} is an imported func) ", target.to_u32())?
            }; 
            match target_constraints {
                None => write!(file, "(direct call)")?,
                Some(target_constraints) => {
                    write!(file, "(indirect call) ")?;
                    write!(file, "[")?; 
                    for (idx, constraint) in target_constraints.iter().enumerate() {
                        match constraint {
                            Constraint::Type(_func_ty) => write!(file, "Type")?,
                            Constraint::InTable => write!(file, "InTable")?,
                            Constraint::Exported => write!(file, "Exported")?,
                            Constraint::TableIndexExpr(expr) => write!(file, "TableIndexExpr({})", expr.kind.get_expr_kind())?,
                        }
                        if idx < target_constraints.len()-1 {
                            write!(file, ", ")?; 
                        }                         
                    } 
                    write!(file, "]")?; 
                },
            }
            writeln!(file)?; 
        }; 
        Ok(())
    }
}


impl fmt::Debug for CallGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.0)?; 
        for (source, targets) in self.0.iter() {
            for target in targets {
                writeln!(f, "{} -> {}", source, target)?;
            }
        }; 
        Ok(())
    }
}

impl CallGraph {
    /// Returns true if the edge was _not_ yet part of the callgraph.
    pub fn add_edge(&mut self, source: FunctionId, target: FunctionId) -> bool {
        let targets = self.0.entry(source).or_default();
        targets.insert(target)
    }

    pub fn edges(&self) -> impl Iterator<Item=(FunctionId, FunctionId)> + '_ {
        self.0.iter()
            .flat_map(|(source, targets)| 
                targets.iter()
                .map(move |target| (source.clone(), target.clone())))
    }

    pub fn functions(&self) -> FxHashSet<FunctionId> {
        let mut result = FxHashSet::default();
        for (source, target) in self.edges() {
            result.insert(source);
            result.insert(target);
        }
        result
    }

    pub fn to_dot_file(&self, path: impl AsRef<Path>) {
        let mut dot_file: String = "".to_owned();
        dot_file.push_str("digraph G {\n"); 
        dot_file.push_str("\trankdir=\"LR\";\n");

        // Sort edges to make output deterministic.
        let mut edges = self.edges().collect::<Vec<_>>();
        edges.sort();

        for (from_fn, to_fn) in edges {
            dot_file.push_str(&format!("\t\"{}\"->\"{}\";\n", from_fn, to_fn)); 
        }
        dot_file.push_str("}\n");         
        fs::write(path, dot_file).expect("Unable to write file");
    }

    pub fn to_dot(&self) -> String {
        let mut dot_file: String = "".to_owned();
        dot_file.push_str("digraph G {\n"); 
        dot_file.push_str("\trankdir=\"LR\";\n");

        // Sort edges to make output deterministic.
        let mut edges = self.edges().collect::<Vec<_>>();
        edges.sort();

        for (from_fn, to_fn) in edges {
            dot_file.push_str(&format!("\t\"{}\"->\"{}\";\n", from_fn, to_fn)); 
        }
        dot_file.push_str("}\n");         
        
        dot_file
    }

    /// Requires `dot` on `PATH`.
    pub fn to_pdf(&self, path: impl AsRef<Path>) -> io::Result<()> {
        // Invoke Graphviz CLI tool with path as argument and to_dot() as stdin.
        // Command line: dot -Tpdf -o outfile.pdf < stdin
        let mut child = Command::new("dot")
                .args(["-Tpdf", "-o"])
                .arg(path.as_ref())
                .stdin(Stdio::piped())
                .spawn()?;
        let mut stdin = child.stdin.take().expect("correctly requested stdin piped above!?");
        stdin.write_all(self.to_dot().as_bytes())?;
        drop(stdin);
        let status = child.wait()?;
        assert!(status.success(), "should not fail because we always produce valid dot input!?");
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Target {
    Direct(FunctionId),
    Constrained(Vec<Constraint>)
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Constraint {
    Type(FunctionType),
    // TODO Doesn't refer to a particular table, wouldn't be precise in Wasm post MVP.
    InTable,
    // This can go to any exported function:
    Exported, 
    TableIndexExpr(crate::Expr),
}

#[derive(Clone, Copy, Default)]
pub struct Options {
    pub with_type_constraint: bool,
    pub with_in_table_constraint: bool,
    pub with_index_constraint: bool,
    // TODO if true, assumes that an imported function can call any exported function.
    // pub imported_functions_conservative: bool,
}

pub fn reachable_callgraph(
    module: &crate::Module,
    mut reachable_funcs: FxHashSet<FunctionId>,
    options: Options,
) -> anyhow::Result<WimplCallGraph> {
    
    let mut wimpl_callgraph = WimplCallGraph::default(); 
    
    // Make everything in the table initially reachable if the table is exported.
    if let Some(table) = &module.table {
        if !table.export.is_empty() {
            // The table is exported so add all the functions in the table to the graph.
            for elem in &table.elements {                
                for func in &elem.functions {
                    reachable_funcs.insert(func.clone());   
                }
            }
        }
    }

    println!("{:?}", reachable_funcs);
    // Collect "declarative constraints" once per function.
    // TODO make lazy: compute constraints only for requested functions on-demand, use memoization.
    let call_target_constraints: FxHashMap<FunctionId, FxHashSet<(Option<crate::InstrId>, Target)>> = module.functions.iter()
        .map(|func| {
            (func.name(), collect_target_constraints(func, module, options))
        })
        .collect();
    
    // DEBUG
    /* println!("[DONE] collect constraints");
    for (id, c) in &call_target_constraints {
        println!("ID:{:?}, constraints: {:?}\n", id, c);
    } */


    // TODO I tried various probabilistic set membership datastructures and implementations, but
    // none beat the simpler version of just using a fast FxHashSet contains check.
    // - `bloomfilter::Bloom`: slow, dominating by hashing with SipHash.
    // - `cuckoofilter::Cuckoofilter`: wrong results! during construction of the Cuckoo filter,
    // elements might be thrown out, which would then give false negatives, which we may never have.
    // let mut funcs_in_table_approx = Bloom::new_for_fp_rate(module.functions.len(), 0.01);

    let mut funcs_in_table = FxHashSet::default();
    let mut funcs_by_table_idx: FxHashMap<u32, FunctionId> = FxHashMap::default();
    if let Some(table) = &module.table {
        for element in &table.elements {
            let element_offset = &element.offset;
            use wasabi_wasm::Instr as wasm;
            let element_offset = match element_offset.as_slice() {
                [wasm::Const(Val::I32(offset)), wasm::End] => *offset as u32,
                // TODO overapproximate: if its an expression we cannot compute statically (like an imported global)
                // then make this `funcs_by_table_idx` an Option::None, and then the constraint becomes worthless
                // during solving.
                expr => {
                    println!("statically evaluate element offset expression: {:?}", expr);
                    // FIXME ALL EDGES COULD BE WRONG!!
                    0
                }
            };
            
            for (idx_in_table, func_idx) in element.functions.iter().enumerate() {
                
                let idx_in_table = idx_in_table as u32;

                let func = module.function((*func_idx).clone()).expect("Each Wimpl function name should map to its corresponding function");
                funcs_in_table.insert(func.name());

                let duplicate_element_init = funcs_by_table_idx.insert(element_offset+idx_in_table, func.name());
                assert_eq!(duplicate_element_init, None, "table index {} is initialized twice", element_offset+idx_in_table)
            }
        }
    }

    // DEBUG
    // println!("[DONE] collect funcs in table");

    // Optimization: generate map from FuncTy -> Vec<Func>, such that we 
    // can quickly filter by function type in solve_constraints (~25% of total runtime!)
    // pass that to solve_constraints.
    
    let mut funcs_by_type: FxHashMap<FunctionType, Vec<&Function>> = FxHashMap::default();
    for func in &module.functions {
        let funcs_with_type = funcs_by_type.entry(func.type_).or_default();
        funcs_with_type.push(func);
    }
    
    
    // Solve constraints for all functions in "worklist" and add their targets to worklist, until
    // this is empty.
    let mut worklist = reachable_funcs.iter().cloned().collect::<Vec<_>>();
    let mut i = 0;
    println!("functions in worklist: {}", worklist.len()); 
    while let Some(func) = worklist.pop() {        

        let calls = call_target_constraints.get(&func).unwrap_or_else(|| panic!("all functions should have been constraints computed for, but not found for '{}'", func));            

        for (instr_id, call) in calls {
            // Solve the constraints to concrete edges.

            // TODO: make targets (FunctionId, Idx<Function>) 
            let targets = solve_constraints(module, &funcs_by_type, &funcs_in_table, &funcs_by_table_idx, call);

            // Add those as edges to the concrete call graph.
            for target in targets {
                wimpl_callgraph.callgraph.add_edge(func.clone(), target.clone());
                    
                let (wasm_loc, wasm_src_id) = match instr_id {
                    Some(instr_id) => {
                        let wasm_loc = module.metadata.instr_location_map.get(instr_id).expect("Metadata should contain all instruction ids"); 
                        (Some(wasm_loc.1), wasm_loc.0)
                    },
                    None => {
                        // this is an imported function 
                        (None, *module.metadata.func_id_to_orig_idx_map.get(&func).expect("Each Wimpl FunctionId should be mapped to it's Wasm Idx<Function>"))
                    },
                }; 
                
                let wasm_target = *module.metadata.func_id_to_orig_idx_map.get(&target).expect("Each Wimpl FunctionId should be mapped to it's Wasm Idx<Function>"); 
                // FIXME cleanup this, I think we don't use callsite information anyway?
                let wasm_target_info = match call {
                    Target::Direct(_function_id) => None,
                    Target::Constrained(constraints) => Some(constraints.clone()),
                }; 
                wimpl_callgraph.callsites.add_edge(
                    wasm_loc, wasm_src_id,
                    wasm_target, wasm_target_info);
                   

                // Add target to worklist, if it wasn't already processed 
                // (and everything reachable was processed).
                // TODO Is this check expensive? If yes, can we use a bit set for the set of reachable functions?
                if reachable_funcs.insert(target.clone()) {
                    worklist.push(target);
                }
            }
        }
        
        // DEBUG
        i += 1;
        if i % 1000 == 0 {
            println!("[DONE] processing {} functions", i);
        }
    }
    
    
    // // FIXME quick and dirty output: which kinds of constraints do we have (and how frequenty are they).
    // let iter = UNIQUE_CONSTRAINT_EXPRS.lock().unwrap();
    // let mut stats: Vec<_> = iter.iter().map(|(expr, count)| (count, expr)).collect();
    // stats.sort();
    // for (count, expr) in stats {
    //     println!("{:10} {:30}", count, expr);
    // }

    Ok(wimpl_callgraph)
}

pub fn collect_target_constraints(
    src: &Function,
    module: &crate::Module,
    options: Options
) -> FxHashSet<(Option<crate::InstrId>, Target)> {
    
    // TODO: Discuss
    // How to handle imported functions? Can they each every exported function?
    // Do we add a direct edge there? Or do we add an abstract "host" node? 
    // Do we merge with a JavaScript call-graph analysis?
    // What if the table is also exported? 
    let body = match src.body.as_ref() {
        Some(body) => body,
        // Imported function:
        None => {
            let mut target = FxHashSet::default(); 

            // Any exported function can be called from an imported function.
            target.insert((None, Target::Constrained(vec![Constraint::Exported])));

            // And also the host code can reach functions through the table, if the table itself
            // is exported.
            if let Some(table) = &module.table {
                if !table.export.is_empty() {
                    target.insert((None, Target::Constrained(vec![Constraint::InTable])));              
                }
            }
            
            return target
        },
    };     

    // Step 1: Build a map of variables to expressions or an overapproximation when variables are
    // assigned multiple times.
    let var_expr = VarExprMap::from_body(body);
    
    // DEBUG
    // println!("{src}\nvar_map: {:#}", var_expr);

    // Step 2:
    // Collect one set of constraints (conjuncts) per call/call_indirect.
    let mut targets = FxHashSet::default();
    use crate::ExprKind::*;
    
    body.visit_expr_pre_order(|expr| {
        match &expr.kind {
            Call { func, args: _} => {
                targets.insert((Some(expr.id), Target::Direct(func.clone())));
            }
            CallIndirect { type_, table_idx, args: _ } => {
                let mut constraints = Vec::default();
                if options.with_type_constraint {
                    constraints.push(Constraint::Type(*type_));
                }
                if options.with_in_table_constraint {
                    constraints.push(Constraint::InTable);
                }
                if options.with_index_constraint {
                    let expr = match table_idx.as_ref() {
                        // The table_idx expression refers to a variable, try to resolve that...
                        Expr { id: _, kind: VarRef(var) } => match var_expr.get(*var) {
                            VarExprMapResult::Precise(expr) => Some(expr),
                            VarExprMapResult::Top => None,
                            // TODO handle parameter variables with inter-procedural analysis, 
                            // but for now we don't produce a constraint for those.
                            VarExprMapResult::Uninitialized(Var::Param(_)) => None,
                            // TODO handle uninitialized globals with some global read/write analysis.
                            VarExprMapResult::Uninitialized(Var::Global(_)) => None,
                            VarExprMapResult::Uninitialized(uninitialized) => unreachable!("non-parameter variables should always be initialized, but: {} was not", uninitialized)
                        }
                        expr => Some(expr)
                    };

                    let expr_abstracted = expr.map(abstract_expr).unwrap_or_else(|| "<unresolved var>".into());
                    *UNIQUE_CONSTRAINT_EXPRS.lock().expect("lock poisoning doesnt happen").entry(expr_abstracted).or_default() += 1;

                    if let Some(expr) = expr {
                        constraints.push(Constraint::TableIndexExpr(expr.clone()));
                    }
                }
                targets.insert((Some(expr.id), Target::Constrained(constraints)));
            }
            _ => {}
        }
        true
    });
    //println!("Func:{:?} targets: {:?}\n", src.name , targets);
    
    targets
}

// FIXME global variable :(((
lazy_static::lazy_static! {
    // static ref UNIQUE_CONSTRAINT_EXPRS: Mutex<FxHashMap<wimpl::Expr, usize>> = Mutex::new(FxHashMap::default());
    static ref UNIQUE_CONSTRAINT_EXPRS: Mutex<FxHashMap<String, usize>> = Mutex::new(FxHashMap::default());
}

/// _All_ constraints must be satisfied, i.e., conjunction over all constraints.
pub fn solve_constraints<'a>(
    module: &'a crate::Module,
    funcs_by_type: &'a FxHashMap<FunctionType, Vec<&'a Function>>,
    funcs_in_table: &'a FxHashSet<FunctionId>,
    // funcs_in_table_approx: &'a Bloom<Func>,
    funcs_by_table_idx: &'a FxHashMap</* index in the table */ u32, FunctionId>,
    target_constraints: &'a Target
) -> Box<dyn Iterator<Item=FunctionId> + 'a> {
    match target_constraints {
        Target::Direct(f) => Box::new(std::iter::once(f.clone())),
        Target::Constrained(constraints) => {
            
            let mut filtered_iter: Box<dyn Iterator<Item=&Function>> = 
                constraints.iter()
                    // If there is a type constraint, start with the much narrowed down set of
                    // function types already.
                    .find_map(|c| match c {
                        Constraint::Type(ty) => Some(*ty),
                        _ => None,
                    })
                    .and_then(|ty| funcs_by_type.get(&ty))
                    .map(|vec| Box::new(vec.iter().cloned()) as Box<dyn Iterator<Item=&Function>>)
                    .unwrap_or_else(|| Box::new(module.functions.iter()) as Box<dyn Iterator<Item=&Function>>);
            
            for constraint in constraints {
                use crate::ExprKind::*;
                // Build up filtering iterator at runtime, adding all constraints from before.
                filtered_iter = match constraint {
                    Constraint::Type(ty) => Box::new(filtered_iter.filter(move |f| &f.type_ == ty)),
                    Constraint::InTable => Box::new(filtered_iter.filter(move |f| {
                        // FIXME This is actually slower than the HashSet lookup, likely due to
                        // `bloomfilter::Bloom` using SipHash, not the fast FxHash of FxHashSet.
                        // Optimization: If the Bloom filter already says the function is not in the
                        // table, then we don't need to check the slower hash set.
                        // if !funcs_in_table_approx.check(&f.name) {
                        //     return false;
                        // }
                        funcs_in_table.contains(&f.name)
                    })),
                    // TODO If we have a TableIndexExpr constraint, we already know the function 
                    // immediately, so just use the result of `funcs_by_table_idx.get()` instead of
                    // filtering.
                    Constraint::TableIndexExpr(Expr { id: _, kind: Const(Val::I32(idx)) }) => Box::new(filtered_iter.filter(move |f| {
                        let idx = *idx as u32;
                        Some(&f.name) == funcs_by_table_idx.get(&idx)
                    })),
                    Constraint::TableIndexExpr(expr) => {
                        let is_i32_expr = match &expr.kind {
                            Unary(op, _) => op.to_type().results()[0] == ValType::I32, 
                            Binary(op, _, _) => op.to_type().results()[0] == ValType::I32,
                            _ => false,
                        };
                        if !is_i32_expr {
                            continue;
                        }
                        let idx_range = approx_i32_eval(expr);
                        
                        // DEBUG
                        // println!("{} -> {:?}", expr, idx_range);

                        if idx_range == I32Range::default() {
                            filtered_iter
                        } else {
                            Box::new(filtered_iter.filter(move |f| {
                                for idx in idx_range.iter() {
                                    if Some(&f.name) == funcs_by_table_idx.get(&idx) {
                                        return true;
                                    }
                                }
                                false
                            }))
                        }
                    },
                    Constraint::Exported => {
                        Box::new(filtered_iter.filter(move |f| {
                            !&f.export.is_empty()
                        }))
                    },
                    // TODO Load expressions -> needs pointer analysis
                }
            }
            Box::new(filtered_iter.map(|f| f.name()))
        },
    }
}

#[test]
fn main() {
    let wimpl_module = crate::Module::from_wasm_file("tests/callgraph/callgraph.wasm").expect(""); 
    println!("{}", wimpl_module);     

    let options = Options {
        with_type_constraint: true, // Always true.
        with_in_table_constraint: true, // Assumption: table doesn't change at runtime (by host).
        // with_index_constraint: true,
        ..Options::default()
    };
    let reachable = vec![FunctionId::Name("main".to_string().into())].into_iter().collect();
    let callgraph = reachable_callgraph(&wimpl_module, reachable, options).unwrap().callgraph;

    println!("{}", callgraph.to_dot());
    callgraph.to_pdf("tests/callgraph/out/callgraph.pdf").expect("Error which making pdf of callgraph");
}

// #[test]
// fn create_graph() {
//     // TODO 2-3 function wasm file, 1 direct call, 2 call_indirect, 5 functions in total
//     let wimpl_module = crate::wimplify("tests/wimpl-wasm-handwritten/calc-dce/add-dce.wasm").expect(""); 
//     println!("{}", wimpl_module); 
//     let callgraph = callgraph(&wimpl_module); 
//     println!("{}", callgraph.to_dot());
//     callgraph.to_pdf("tests/wimpl/calc-dce/callgraph.pdf").unwrap();
// }

// #[test]
// fn calc_virtual() {
//     let wimpl_module = crate::wimplify("tests/wimpl-wasm-handwritten/calc-virtual/add.wasm").expect(""); 
//     println!("{}", wimpl_module);     
//     let callgraph = callgraph(&wimpl_module); 
//     println!("{}", callgraph.to_dot());
//     callgraph.to_pdf("tests/wimpl/calc-dce/callgraph.pdf").unwrap();
// }

#[test]
fn data_gathering() {
    
    let mut file = File::create("data.csv").unwrap();
    writeln!(file, "path, num_functions, num_functions_exported, num_calls, num_ind_calls, unq functions in elem, num_funcs_with_call_ind, num_funcs_with_memory_access, reachable_ratio_trivial, reachable_ratio_ty_only, reachable_ratio_in_table_only, reachable_ratio_ty_and_in_table_and_idx_expr").unwrap(); 
    
    const WASM_TEST_INPUTS_DIR: &str = "../wasm/tests/wasm/";

    let mut all_idx_exprs: FxHashMap<String, usize> = FxHashMap::default();
    let mut all_i32_load_store_addr_exprs: FxHashMap<String, usize> = FxHashMap::default();
    let mut all_i32_store_value_exprs: FxHashMap<String, usize> = FxHashMap::default();

    for entry in walkdir::WalkDir::new(&WASM_TEST_INPUTS_DIR) {
        let path = entry.unwrap().path().to_owned();
        // Select only .wasm files, not directories or files with other extensions.
        if path.is_dir() || path.extension().and_then(|s| s.to_str()) != Some("wasm") {
            continue;
        }

        println!("{}", path.display());
        let wimpl_module = Module::from_wasm_file(&path).unwrap_or_else(|_| panic!("could not decode valid wasm file '{}'", path.display()));

        let idx_exprs = collect_call_indirect_idx_expr(&wimpl_module);
        for (expr, count) in idx_exprs.iter() {
            *all_idx_exprs.entry(expr.clone()).or_default() += *count;
        }
        print_map_count(&idx_exprs);

        let (addr_exprs, value_exprs) = collect_i32_load_store_arg_expr(&wimpl_module);
        for (expr, count) in addr_exprs {
            *all_i32_load_store_addr_exprs.entry(expr).or_default() += count;
        }
        for (expr, count) in value_exprs {
            *all_i32_store_value_exprs.entry(expr).or_default() += count;
        }

        // let wimpl_module = wimpl::wimplify("tests/wimpl-wasm-handwritten/calc-virtual/add.wasm").expect(""); 
        // let callgraph = callgraph(&wimpl_module); 

        let num_functions = wimpl_module.functions.len(); 
        let num_functions_exported = wimpl_module.functions.iter().filter(|func| !func.export.is_empty()).count();
        let mut num_calls = 0; 
        let mut num_ind_calls = 0; 
        let mut num_funcs_with_call_ind = 0; 
        let mut num_funcs_with_memory_access = 0; 

        // FIXME Wrong after recursive Wimpl!! Use traverse::visit_pre_order
        todo!("FIXME!!! these results cannot be trusted, all if this here below must be replaced with traverse::visit_pre_order!");
        // for fun in wimpl_module.functions {
        //     let mut flag_call_ind = false; 
        //     let mut flag_load_store = false; 
            
        //     for stmt in fun.body.0 {
        //         use wimpl::StmtKind::*;
        //         use wimpl::ExprKind::*;
        //         match &stmt.kind {
        //             Assign { lhs: _, rhs: Call{ func: _, args: _}, type_: _ } |
        //             Expr(Call { func: _, args: _}) => {
        //                 num_calls += 1; 
        //             }, 

        //             Assign { lhs: _, rhs: CallIndirect { type_: _, table_idx: _, args: _ }, type_: _ } |
        //             Expr(CallIndirect { type_: _, table_idx: _, args: _ }) => {
        //                 num_ind_calls += 1;
        //                 flag_call_ind = true; 
        //             },
                    
        //             Store { op: _, addr: _, value: _ } |
        //             Assign { lhs:_, type_:_, rhs: Load{op:_, addr:_} } |
        //             Expr(Load{op:_, addr:_}) => {
        //                 flag_load_store = true; 
        //             }, 

        //             _ => (), 
                    
        //         }
        //     }
            
        //     if flag_call_ind {
        //         num_funcs_with_call_ind += 1; 
        //         if flag_load_store {
        //             num_funcs_with_memory_access += 1; 
        //         }
        //     }

        // } 

        let mut element_funcs = FxHashSet::default(); 
        if let Some(table) = &wimpl_module.table {
            for elem in &table.elements {
                for func_idx in &elem.functions {
                    //let func = wimpl_module.function_by_idx(*func_idx);
                    element_funcs.insert(func_idx); 
                }
            }
        }

        fn callgraph_reachable_funcs_ratio(path: impl AsRef<Path>, options: Options) -> f64 {
            let wimpl_module = Module::from_wasm_file(&path).unwrap();
            let exported_funcs = wimpl_module.functions.iter()
                .filter(|func| !func.export.is_empty())
                .map(|func| func.name())
                .collect::<FxHashSet<_>>();
            
            reachable_callgraph(&wimpl_module, exported_funcs, options).unwrap().callgraph.functions().len() as f64 
                / wimpl_module.functions.len() as f64
        }
        
        // TODO: remove because (i) not comparable number (not a percentage) (ii) super slow.
        fn callgraph_reachable_funcs_avg(path: impl AsRef<Path>, options: Options) -> f64 {
            let wimpl_module = Module::from_wasm_file(&path).unwrap();
            let exported_funcs = wimpl_module.functions.iter()
                .filter(|func| !func.export.is_empty())
                .map(|func| func.name())
                .collect::<FxHashSet<_>>();
            
            let sum_reachable_count: u64 = exported_funcs.iter().map(|f| {
                let mut reachable = FxHashSet::default();
                reachable.insert(f.clone());
                let reachable_funcs_count = reachable_callgraph(&wimpl_module, reachable, options).unwrap().callgraph.functions().len();
                reachable_funcs_count as u64
            }).sum();
            (sum_reachable_count as f64) / (exported_funcs.len() as f64)
        }
        
        let reachable_ratio_trivial = callgraph_reachable_funcs_ratio(&path, Options {
            with_type_constraint: false,
            with_in_table_constraint: false,
            with_index_constraint: false,
        });
        let reachable_ratio_ty_only = callgraph_reachable_funcs_ratio(&path, Options {
            with_type_constraint: true,
            with_in_table_constraint: false,
            with_index_constraint: false,
        });
        let reachable_ratio_in_table_only = callgraph_reachable_funcs_ratio(&path, Options {
            with_type_constraint: false,
            with_in_table_constraint: true,
            with_index_constraint: false,
        });
        // I believe this is what Wassail does
        // https://github.com/acieroid/wassail/blob/3187d8f2e3ffbbc2b3d90233da6cd25589110038/lib/analysis/call_graph/call_graph.ml#L16
        let reachable_ratio_ty_and_in_table = callgraph_reachable_funcs_ratio(&path, Options {
            with_type_constraint: true,
            with_in_table_constraint: true,
            with_index_constraint: false,
        });
        let reachable_ratio_ty_and_in_table_and_idx_expr = callgraph_reachable_funcs_ratio(&path, Options {
            with_type_constraint: true,
            with_in_table_constraint: true,
            with_index_constraint: true,
        });

        // Binaryen meta-DCE does none of the above, instead (citing my email from July 2021)
        // indirect calls just adds all functions in the indirect function call table initializer ("elements section") as reachability roots, no further analysis
        // see https://github.com/WebAssembly/binaryen/blob/11ada63fbb7ba982c92f22fa1fb0e39cebe3f194/src/tools/wasm-metadce.cpp#L228
        // so it is neither dependent on the set of reachable functions at the start
        // nor type based
        // so roughly like in_table_only, but without the iterative part of our algorithm

        // Twiggy: essentially equivalent to Binaryen meta-DCE
        // adds one reachability root for the whole indirect call table and adds outgoing edges from that pseudo node to all functions in the table. I.e., all indirect calls are collapsed into one reachability class, not type-aware for indirect calls
        // https://github.com/rustwasm/twiggy/blob/195feee4045f0b89d7cba7492900131ac89803dd/parser/wasm_parse/mod.rs#L664 (table as root)
        // https://github.com/rustwasm/twiggy/blob/195feee4045f0b89d7cba7492900131ac89803dd/parser/wasm_parse/mod.rs#L858 (table -> element)
        // https://github.com/rustwasm/twiggy/blob/195feee4045f0b89d7cba7492900131ac89803dd/parser/wasm_parse/mod.rs#L864 (edge from element -> concrete func idx)
        
        // wasp does also neither of the above, they (unsoundly) disregard all call_indirects

        if reachable_ratio_ty_and_in_table != reachable_ratio_ty_and_in_table_and_idx_expr {
            println!("{}\nwithout idx_expr: {}\nwith idx_expr: {}", path.display(), reachable_ratio_ty_and_in_table, reachable_ratio_ty_and_in_table_and_idx_expr);
        }

        writeln!(file, "\"{}\",{},{},{},{},{},{},{},{},{},{},{}", 
            path.display(), num_functions, num_functions_exported, num_calls, num_ind_calls, 
            element_funcs.len(), num_funcs_with_call_ind, 
            num_funcs_with_memory_access, 
            reachable_ratio_trivial, reachable_ratio_ty_only, reachable_ratio_in_table_only, reachable_ratio_ty_and_in_table_and_idx_expr
        ).unwrap();
    }

    println!("call_indirect idx expressions for all binaries:");
    print_map_count(&all_idx_exprs);
    
    println!("call_indirect idx constraints (after resolving with VarExprMap!) for all binaries:");
    let all_idx_constraints = UNIQUE_CONSTRAINT_EXPRS.lock().unwrap().clone();
    print_map_count(&all_idx_constraints);

    println!("i32 load/store addr expressions for all binaries:");
    print_map_count(&all_i32_load_store_addr_exprs);

    println!("i32 store value expressions for all binaries:");
    print_map_count(&all_i32_store_value_exprs);

}

// A variable v is live at a program point p, if some path from p to program exit contains 
// an r-value occurrence of v which is not preceded by an l-value occurrence of v.

// Gen_n  = { v | var v is used (rhs occurance) in a basic block n 
//                and is not preceded by a definition (lhs occurance) of v }
// Kill_n = { v | basic block n contains a definition of v }

// For each block we define an In and Out set which tells us which variables ar live at the start and end of a block.

// In_k = Gen_k ∪ ( Out_k - Kill_k )
// Out_k = { if end of block, {} 
//           else,            In_i ∪ In_j

// TODO: use traverse for gen-kill set creation, cannot use for In, Out 
// fixed point iteration vs [worklist] - delay for later 
// pen vs paper 
// reaching definitions analysis - maybe not 

fn get_gen_kill_sets (stmt: &Stmt, gen_kill_map : &mut HashMap<InstrId, (HashSet<Var>, HashSet<Var>)>) {
    
    let mut gen: HashSet<Var> = HashSet::new();
    let mut kill: HashSet<Var> = HashSet::new();

    fn expr_gen_kill (expr:&Expr, gen: &mut HashSet<Var>) {
        match &expr.kind {
            ExprKind::VarRef(var) => {gen.insert(*var);},
            ExprKind::Const(_) => (),
            ExprKind::MemorySize => (),
            ExprKind::Load { op: _, addr } => expr_gen_kill(addr.as_ref(), gen),
            ExprKind::MemoryGrow { pages } => expr_gen_kill(pages.as_ref(), gen),
            ExprKind::Unary(_, subexpr) => expr_gen_kill(subexpr.as_ref(), gen),
            ExprKind::Binary(_, subexpr1, subexpr2) => {
                expr_gen_kill(subexpr1.as_ref(), gen); 
                expr_gen_kill(subexpr2.as_ref(), gen);                             
            },
            Call { func: _, args } => args.iter().for_each(|arg| expr_gen_kill(arg, gen)),
            ExprKind::CallIndirect { type_: _, table_idx, args } => {
                expr_gen_kill(table_idx.as_ref(), gen); 
                args.iter().for_each(|arg| expr_gen_kill(arg, gen)); 
            },
        }
    } 

    match &stmt.kind {
        StmtKind::Unreachable => (),
        StmtKind::Expr(expr) => expr_gen_kill(expr, &mut gen),
        StmtKind::Assign { lhs, type_: _, rhs } => {
            kill.insert(*lhs); 
            expr_gen_kill(rhs, &mut gen); 
        },
        StmtKind::Store { op: _, addr, value } => {
            expr_gen_kill(addr, &mut gen);
            expr_gen_kill(value, &mut gen);
        },
        StmtKind::Br { target: _ } => (),
        StmtKind::Loop { begin_label: _, body } |
        StmtKind::Block { body, end_label: _ } => body.0.iter().for_each(|stmt| get_gen_kill_sets(stmt, gen_kill_map)), 
        StmtKind::If { condition, if_body, else_body } => {
            expr_gen_kill(condition, &mut gen); 
            if_body.0.iter().for_each(|stmt| get_gen_kill_sets(stmt, gen_kill_map));
            if let Some(else_body) = else_body { else_body.0.iter().for_each(|stmt| get_gen_kill_sets(stmt, gen_kill_map)); }            
        },
        StmtKind::Switch { index, cases, default } => {
            expr_gen_kill(index, &mut gen); 
            cases.iter().for_each(|case|case.0.iter().for_each(|stmt| get_gen_kill_sets(stmt, gen_kill_map))); 
            default.0.iter().for_each(|stmt|get_gen_kill_sets(stmt, gen_kill_map)); 
        },
    }

    gen_kill_map.insert(stmt.id, (gen, kill)); 
}


fn liveness_analysis (module: Module) {
    let mut gen_kill_map = HashMap::new();
    for func in module.functions {
        if let Some(body) = func.body {
            for stmt in body.0 {
                get_gen_kill_sets(&stmt, &mut gen_kill_map);
                //println!("!Stmt:{} Gen:{:?} Kill:{:?}", stmt, gen, kill); 
            }            
        } 
    }
    
    // TODO: make one hashmap instead of two? create HashMap<InstrId, Instr> where enum Instr { Stmt, Expr }
    for (id, (gen, kill)) in gen_kill_map {
        let stmt =  module.metadata.id_stmt_map.get(&id); 
        match stmt {
            Some(stmt) => println!("Stmt:{}, Gen:{:?}, Kill:{:?}", stmt, gen, kill), 
            None =>  {
                println!("Stmt:{}, Gen:{:?}, Kill:{:?}", module.metadata.id_expr_map.get(&id).expect("msg"), gen, kill); 
            },
        }; 
    }
    println!(); 
    println!(); 
}

#[test]
fn main_ () {
    const WIMPL_TEST_INPUTS_DIR: &str = "tests/wimpl/wimplify_expected/";
    
    // Sort for deterministic order.
    let mut files: Vec<std::path::PathBuf> = walkdir::WalkDir::new(&WIMPL_TEST_INPUTS_DIR).into_iter().map(|entry| entry.unwrap().path().to_owned()).collect();
    files.sort();

    for wimpl_path in files {
        // Find all files, where a <name>.wimpl file and a <name>.wasm file are next to each other.
        if let Some("wimpl") = wimpl_path.extension().and_then(|os_str| os_str.to_str()) {
            let wasm_path = wimpl_path.with_extension("wasm");
            if wasm_path.exists() {
                println!("\t{}", wimpl_path.display());
                
                let wimpl_module = Module::from_wasm_file(wasm_path).unwrap();

                liveness_analysis(wimpl_module); 
                //let actual = wimpl_module.functions[0].clone().body.expect("the first function of the example should not be imported");

                //println!("{}",actual); 
            }
        }
    }
}