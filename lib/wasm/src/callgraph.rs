use core::fmt;
use std::{collections::{HashSet, HashMap, hash_map::Entry}, path::Path, io::{self, Write}, process::{Command, Stdio}, fs::File};

use crate::{wimpl::{Func, self, Expr::Call, Function, Var}, FunctionType};

use test_utilities::*; 

pub struct CallGraph(HashSet<(Func, Func)>);

impl fmt::Debug for CallGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.0); 
        for (x, y) in self.0.iter() {
            writeln!(f, "{} {}", x, y).expect("");
        }; 
        Ok(())
    }
}

impl CallGraph {
    // pub fn reachable_funcs(&self, initially: HashSet<Func>) -> HashSet<Func> {
    //     let mut reachable = initially;
    //
    //     for (src, dest) in &self.0 {
    //         if reachable.contains(src) {
    //             reachable.insert(dest.clone());
    //         }
    //     }
    //
    //     reachable
    // }

    pub fn all(&self) -> HashSet<Func> {
        let mut all = HashSet::new();
        for (src, dest) in &self.0 {
            all.insert(src.clone());
            all.insert(dest.clone());
        }
        all
    }

    pub fn to_dot(&self) -> String {
        let mut dot_file: String = "".to_owned();
        dot_file.push_str("digraph G {\n"); 
        dot_file.push_str("\trankdir=\"LR\";\n"); 
        let mut edges = self.0.iter().collect::<Vec<_>>();
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

pub enum Target {
    Direct(Func),
    Constrained(HashSet<Constraint>)
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Constraint {
    Type(FunctionType),
    InTable,
    // Exported
    TableIndexExpr(wimpl::Expr),
}

#[derive(Clone, Copy, Default)]
pub struct Options {
    with_type_constraint: bool,
    with_in_table_constraint: bool,
    with_index_constraint: bool,
}

pub fn reachable_callgraph(
    module: &wimpl::Module,
    mut reachable: HashSet<Func>,
    options: Options,
) -> anyhow::Result<CallGraph> {
    
    let mut callgraph_edges = CallGraph(HashSet::new());

    // TODO use worklist algorithm, keep track of added edges.
    let mut changed = true;
    while changed {
        for func in &reachable {
            // Collect constraints for all (currently) reachable functions.
            let target_constraints = collect_target_constraints(module, func.clone(), options);
    
            for target_constraints in target_constraints {
                // Solve the constraints to concrete edges.
                let targets = solve_constraints(module, &target_constraints);
    
                // Add those as edges to the concrete call graph.
                callgraph_edges.0.extend(targets.into_iter().map(|target| (func.clone(), target)));
            }
        }

        let old_reachable = reachable.clone();
        reachable = callgraph_edges.all();
        changed = old_reachable != reachable;
    }

    Ok(callgraph_edges)
}

pub fn collect_target_constraints(
    module: &wimpl::Module,
    src: Func,
    options: Options
) -> Vec<Target> {
    let mut targets = Vec::new();

    let src = module.function(src.clone()).expect(&format!("source function not found in module: {:?}", src));

    // TODO how to handle imported functions? Can they each every exported function?
    // Do we add a direct edge there? Or do we add an abstract "host" node? 
    // Do we merge with a JavaScript call-graph analysis?
    let body = &src.body;

    let mut var_expr: HashMap<Var, Option<wimpl::Expr>> = HashMap::new();

    for stmt in &body.0 {
        
        use wimpl::Stmt::*;
        use wimpl::Expr::*;

        match stmt {

            // FIXME recursively go into block, loop, if bodies!
            
            Assign { lhs: _, rhs: Call{ func, args: _}, type_: _ } |
            Expr(Call { func, args: _}) => targets.push(Target::Direct(func.clone())),

            Assign { lhs: _, rhs: CallIndirect { type_, table_idx, args: _ }, type_: _ } |
            Expr(CallIndirect { type_, table_idx, args: _ }) => {
                let mut constraints = HashSet::new();
                if options.with_in_table_constraint {
                    constraints.insert(Constraint::InTable);
                }
                if options.with_type_constraint {
                    constraints.insert(Constraint::Type(type_.clone()));
                }
                if options.with_index_constraint {
                    match var_expr.get(table_idx) {
                        Some(Some(expr)) => {
                            constraints.insert(Constraint::TableIndexExpr(expr.clone()));
                        }
                        // Over-approximation.
                        Some(None) => {},
                        None => unreachable!("uninitialized variable `{}`\nin: {}\nvariable map: {:?}", table_idx, src, var_expr),
                    }
                }
                targets.push(Target::Constrained(constraints));
            }

            // Update variables
            // FIXME call_indirect itself might be a RHS of an assign, then this case doesn't match.
            // FIXME assignment of variable textually AFTER the call_indirect usage, might still
            // flow into the variable in case of loops -> need to do variable map before.
            // TODO fix both issues by pulling var_expr updating up into a separate forward traversal that gives a var_expr_map
            Assign { lhs: var, type_, rhs: expr } => {
                // println!("{}\nbefore {:?}", stmt, var_expr);
                var_expr.entry(*var)
                    // Overapproximate if there is more than one assignment to a variable.
                    .and_modify(|old_expr| *old_expr = None)
                    .or_insert(Some(expr.clone()));
                // println!("after {:?}", var_expr);
            }

            _ => {}

        }
    }

    targets
}

/// _All_ constraints must be satisfied, i.e., conjunction over all constraints.
pub fn solve_constraints(
    module: &wimpl::Module,
    target_constraints: &Target
) -> Vec<Func> {
    let constraints = match target_constraints {
        Target::Direct(f) => return vec![f.clone()],
        Target::Constrained(constraints) => constraints,
    };

    // Start with all functions initially, and remove those not matching a particular constraint.
    let mut valid_funcs = module.functions.iter().collect::<Vec<_>>();

    let mut funcs_in_table: HashSet<&Function> = HashSet::new();
    for table in &module.tables {
        // TODO interpret `table.offset` for index-based analysis, i.e., to get a map from table
        // index to function index after table initialization (assumption: table stays constant).
        for element in &table.elements {
            for func_idx in &element.functions {
                let func = module.function_by_idx(*func_idx);
                funcs_in_table.insert(func); 
            }
        }
    } 

    for constraint in constraints {
        match constraint {
            Constraint::Type(ty) => valid_funcs.retain(|f| &f.type_ == ty),
            Constraint::InTable => valid_funcs.retain(|f| funcs_in_table.contains(f)),
            Constraint::TableIndexExpr(wimpl::Expr::Const(val)) => todo!("constant: {}", val),
            Constraint::TableIndexExpr(expr) => todo!("{}", expr),
        }
    }

    valid_funcs.into_iter().map(|f| f.name.clone()).collect()
}

#[test]
fn main() {
    let wimpl_module = wimpl::wimplify("tests/callgraph.wasm").expect(""); 
    println!("{}", wimpl_module);     

    let options = Options {
        with_type_constraint: true, // Always true.
        with_in_table_constraint: true, // Assumption: table doesn't change at runtime (by host).
        // with_index_constraint: true,
        ..Options::default()
    };
    let reachable = vec![Func::Named("main".to_string())].into_iter().collect::<HashSet<_>>();
    let callgraph = reachable_callgraph(&wimpl_module, reachable, options).unwrap();

    println!("{}", callgraph.to_dot());
    callgraph.to_pdf("tests/callgraph.pdf").unwrap();
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
fn data_gathering () {
    
    let mut file = File::create("data.csv").unwrap();
    writeln!(file, "path, num_functions, num_functions_exported, num_calls, num_ind_calls, unq functions in elem, num_funcs_with_call_ind, num_funcs_with_memory_access, reachable_ratio_trivial, reachable_ratio_ty_only, reachable_ratio_in_table_only, reachable_ratio_ty_and_in_table").unwrap(); 
    
    const WASM_TEST_INPUTS_DIR: &str = "tests/";

    for path in wasm_files(WASM_TEST_INPUTS_DIR).unwrap() {
        println!("{}", path.display());
        let wimpl_module = wimpl::wimplify(&path).expect(&format!("could not decode valid wasm file '{}'", path.display()));
        
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

        let mut element_funcs = HashSet::new(); 
        for tab in &wimpl_module.tables {
            for elem in &tab.elements {
                for func_idx in &elem.functions {
                    //let func = wimpl_module.function_by_idx(*func_idx);
                    element_funcs.insert(func_idx.into_inner()); 
                }
            }
        }

        fn callgraph_reachable_funcs_ratio(path: impl AsRef<Path>, options: Options) -> f64 {
            let wimpl_module = wimpl::wimplify(&path).unwrap();
            let exported_funcs = wimpl_module.functions.iter()
                .filter(|func| !func.export.is_empty())
                .map(|func| func.name())
                .collect::<HashSet<_>>();
            
            reachable_callgraph(&wimpl_module, exported_funcs, options).unwrap().all().len() as f64 
                / wimpl_module.functions.len() as f64
        }
        
        // TODO: remove because (i) not comparable number (not a percentage) (ii) super slow.
        fn callgraph_reachable_funcs_avg(path: impl AsRef<Path>, options: Options) -> f64 {
            let wimpl_module = wimpl::wimplify(&path).unwrap();
            let exported_funcs = wimpl_module.functions.iter()
                .filter(|func| !func.export.is_empty())
                .map(|func| func.name())
                .collect::<HashSet<_>>();
            
            let sum_reachable_count: u64 = exported_funcs.iter().map(|f| {
                let mut reachable = HashSet::new();
                reachable.insert(f.clone());
                let reachable_funcs_count = reachable_callgraph(&wimpl_module, reachable, options).unwrap().all().len();
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
}