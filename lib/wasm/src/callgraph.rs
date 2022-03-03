use core::fmt;
use std::{path::Path, io::{self, Write}, process::{Command, Stdio}, fs::File, sync::Mutex, iter::FromIterator, cmp::Reverse};

use crate::{wimpl::{Module, FunctionId, self, Expr::Call, Function, Var, Body, analyze::{VarExprMap, VarExprMapResult, collect_call_indirect_idx_expr, abstract_expr, sort_map_count, collect_i32_load_store_arg_expr, print_map_count}}, highlevel::FunctionType, Val};

use crate::wimpl::wimplify::*;

use rustc_hash::{FxHashSet, FxHashMap};
use test_utilities::*;

#[derive(Default, Clone, Eq, PartialEq)]
pub struct CallGraph(FxHashMap<FunctionId, FxHashSet<FunctionId>>);

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
    // FIXME doesn't refer to a particular table, wouldn't be precise in Wasm post MVP.
    InTable,
    // Exported
    TableIndexExpr(wimpl::Expr),
}

#[derive(Clone, Copy, Default)]
pub struct Options {
    pub with_type_constraint: bool,
    pub with_in_table_constraint: bool,
    pub with_index_constraint: bool,
}

pub fn reachable_callgraph(
    module: &wimpl::Module,
    mut reachable_funcs: FxHashSet<FunctionId>,
    options: Options,
) -> anyhow::Result<CallGraph> {
    
    let mut callgraph = CallGraph::default();

    // Collect "declarative constraints" once per function.
    // TODO make lazy: compute constraints only for requested functions on-demand, use memoization.
    let call_target_constraints: FxHashMap<FunctionId, FxHashSet<Target>> = module.functions.iter()
        .map(|func| {
            (func.name(), collect_target_constraints(func, options))
        })
        .collect();
    
    // DEBUG
    // println!("[DONE] collect constraints");
    // println!("constraints: {:?}", function_targets_constraints);


    // TODO I tried various probabilistic set membership datastructures and implementations, but
    // none beat the simpler version of just using a fast FxHashSet contains check.
    // - `bloomfilter::Bloom`: slow, dominating by hashing with SipHash.
    // - `cuckoofilter::Cuckoofilter`: wrong results! during construction of the Cuckoo filter,
    // elements might be thrown out, which would then give false negatives, which we may never have.
    // let mut funcs_in_table_approx = Bloom::new_for_fp_rate(module.functions.len(), 0.01);

    let mut funcs_in_table = FxHashSet::default();
    let mut funcs_by_table_idx: FxHashMap<u32, FunctionId> = FxHashMap::default();
    for table in &module.tables {
        for element in &table.elements {
            let element_offset = &element.offset;
            use crate::highlevel::Instr as wasm;
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

                let func = module.function_by_idx(*func_idx);
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
    // let mut i = 0;
    while let Some(func) = worklist.pop() {
        let calls = call_target_constraints.get(&func).expect("all functions should have been constraints computed for");            

        for call in calls {
            // Solve the constraints to concrete edges.
            let targets = solve_constraints(module, &funcs_by_type, &funcs_in_table, &funcs_by_table_idx, call);

            // Add those as edges to the concrete call graph.
            for target in targets {
                callgraph.add_edge(func.clone(), target.clone());

                // Add target to worklist, if it wasn't already processed 
                // (and everything reachable was processed).
                // TODO Is this check expensive? If yes, can we use a bit set for the set of reachable functions?
                if reachable_funcs.insert(target.clone()) {
                    worklist.push(target);
                }
            }
        }

        // DEBUG
        // i += 1;
        // if i % 1000 == 0 {
        //     println!("[DONE] processing {} functions", i);
        // }
    }

    // // FIXME quick and dirty output: which kinds of constraints do we have (and how frequenty are they).
    // let iter = UNIQUE_CONSTRAINT_EXPRS.lock().unwrap();
    // let mut stats: Vec<_> = iter.iter().map(|(expr, count)| (count, expr)).collect();
    // stats.sort();
    // for (count, expr) in stats {
    //     println!("{:10} {:30}", count, expr);
    // }

    Ok(callgraph)
}

pub fn collect_target_constraints(
    src: &Function,
    options: Options
) -> FxHashSet<Target> {
    // TODO how to handle imported functions? Can they each every exported function?
    // Do we add a direct edge there? Or do we add an abstract "host" node? 
    // Do we merge with a JavaScript call-graph analysis?
    let body = &src.body;

    // Step 1: Build a map of variables to expressions or an overapproximation when variables are
    // assigned multiple times.
    let var_expr = VarExprMap::from_body(body);
    
    // DEBUG
    // println!("{src}\nvar_map: {:#}", var_expr);

    // Step 2:
    // Collect one set of constraints (conjuncts) per call/call_indirect.
    let mut targets = FxHashSet::default();
    use wimpl::Expr::*;
    body.visit_expr_pre_order(|expr| match expr {
        Call { func, args: _} => {
            targets.insert(Target::Direct(func.clone()));
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
                    VarRef(var) => match var_expr.get(var) {
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
            targets.insert(Target::Constrained(constraints));
        }
        _ => {}
    });
    targets
}

// FIXME global variable :(((
lazy_static::lazy_static! {
    // static ref UNIQUE_CONSTRAINT_EXPRS: Mutex<FxHashMap<wimpl::Expr, usize>> = Mutex::new(FxHashMap::default());
    static ref UNIQUE_CONSTRAINT_EXPRS: Mutex<FxHashMap<String, usize>> = Mutex::new(FxHashMap::default());
}

/// _All_ constraints must be satisfied, i.e., conjunction over all constraints.
pub fn solve_constraints<'a>(
    module: &'a wimpl::Module,
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
                    Constraint::TableIndexExpr(wimpl::Expr::Const(Val::I32(idx))) => Box::new(filtered_iter.filter(move |f| {
                        let idx = *idx as u32;
                        Some(&f.name) == funcs_by_table_idx.get(&idx)
                    })),
                    Constraint::TableIndexExpr(expr) => {
                        // TODO
                        continue;
                    },
                }
            }
            Box::new(filtered_iter.map(|f| f.name()))
        },
    }
}

#[test]
fn main() {
    let wimpl_module = wimpl::Module::from_wasm_file("tests/callgraph/callgraph.wasm").expect(""); 
    println!("{}", wimpl_module);     

    let options = Options {
        with_type_constraint: true, // Always true.
        with_in_table_constraint: true, // Assumption: table doesn't change at runtime (by host).
        // with_index_constraint: true,
        ..Options::default()
    };
    let reachable = vec![FunctionId::Name("main".to_string().into())].into_iter().collect();
    let callgraph = reachable_callgraph(&wimpl_module, reachable, options).unwrap();

    println!("{}", callgraph.to_dot());
    callgraph.to_pdf("tests/callgraph/out/callgraph.pdf").unwrap();
}

// #[test]
// fn create_graph() {
//     // TODO 2-3 function wasm file, 1 direct call, 2 call_indirect, 5 functions in total
//     let wimpl_module = wimpl::wimplify("tests/wimpl-wasm-handwritten/calc-dce/add-dce.wasm").expect(""); 
//     println!("{}", wimpl_module); 
//     let callgraph = callgraph(&wimpl_module); 
//     println!("{}", callgraph.to_dot());
//     callgraph.to_pdf("tests/wimpl/calc-dce/callgraph.pdf").unwrap();
// }

// #[test]
// fn calc_virtual() {
//     let wimpl_module = wimpl::wimplify("tests/wimpl-wasm-handwritten/calc-virtual/add.wasm").expect(""); 
//     println!("{}", wimpl_module);     
//     let callgraph = callgraph(&wimpl_module); 
//     println!("{}", callgraph.to_dot());
//     callgraph.to_pdf("tests/wimpl/calc-dce/callgraph.pdf").unwrap();
// }

#[test]
fn data_gathering() {
    
    let mut file = File::create("data.csv").unwrap();
    writeln!(file, "path, num_functions, num_functions_exported, num_calls, num_ind_calls, unq functions in elem, num_funcs_with_call_ind, num_funcs_with_memory_access, reachable_ratio_trivial, reachable_ratio_ty_only, reachable_ratio_in_table_only, reachable_ratio_ty_and_in_table").unwrap(); 
    
    const WASM_TEST_INPUTS_DIR: &str = "tests/";

    let mut all_idx_exprs: FxHashMap<String, usize> = FxHashMap::default();
    let mut all_i32_load_store_addr_exprs: FxHashMap<String, usize> = FxHashMap::default();
    let mut all_i32_store_value_exprs: FxHashMap<String, usize> = FxHashMap::default();

    for path in wasm_files(WASM_TEST_INPUTS_DIR).unwrap() {
        println!("{}", path.display());
        let wimpl_module = Module::from_wasm_file(&path).expect(&format!("could not decode valid wasm file '{}'", path.display()));

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

        for fun in wimpl_module.functions {
            let mut flag_call_ind = false; 
            let mut flag_load_store = false; 
            
            for stmt in fun.body.0 {
                use wimpl::Stmt::*;
                use wimpl::Expr::*;
                match stmt {
                    Assign { lhs: _, rhs: Call{ func: _, args: _}, type_: _ } |
                    Expr(Call { func: _, args: _}) => {
                        num_calls += 1; 
                    }, 

                    Assign { lhs: _, rhs: CallIndirect { type_: _, table_idx: _, args: _ }, type_: _ } |
                    Expr(CallIndirect { type_: _, table_idx: _, args: _ }) => {
                        num_ind_calls += 1;
                        flag_call_ind = true; 
                    },
                    
                    Store { op: _, memarg: _, addr: _, value: _ } |
                    Assign { lhs:_, type_:_, rhs: Load{op:_, memarg:_, addr:_} } |
                    Expr(Load{op:_, memarg:_, addr:_}) => {
                        flag_load_store = true; 
                    }, 

                    _ => (), 
                    
                }
            }
            
            if flag_call_ind {
                num_funcs_with_call_ind += 1; 
                if flag_load_store {
                    num_funcs_with_memory_access += 1; 
                }
            }

        } 

        let mut element_funcs = FxHashSet::default(); 
        for tab in &wimpl_module.tables {
            for elem in &tab.elements {
                for func_idx in &elem.functions {
                    //let func = wimpl_module.function_by_idx(*func_idx);
                    element_funcs.insert(func_idx.to_u32()); 
                }
            }
        }

        fn callgraph_reachable_funcs_ratio(path: impl AsRef<Path>, options: Options) -> f64 {
            let wimpl_module = Module::from_wasm_file(&path).unwrap();
            let exported_funcs = wimpl_module.functions.iter()
                .filter(|func| !func.export.is_empty())
                .map(|func| func.name())
                .collect::<FxHashSet<_>>();
            
            reachable_callgraph(&wimpl_module, exported_funcs, options).unwrap().functions().len() as f64 
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
                let reachable_funcs_count = reachable_callgraph(&wimpl_module, reachable, options).unwrap().functions().len();
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
            with_index_constraint: true, // FIXME for output
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

        writeln!(file, "\"{}\",{},{},{},{},{},{},{},{},{},{},{}", 
            path.display(), num_functions, num_functions_exported, num_calls, num_ind_calls, 
            element_funcs.len(), num_funcs_with_call_ind, 
            num_funcs_with_memory_access, 
            reachable_ratio_trivial, reachable_ratio_ty_only, reachable_ratio_in_table_only, reachable_ratio_ty_and_in_table
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