use core::fmt;
use std::{collections::{HashSet, HashMap, hash_map::Entry}, path::Path, io::{self, Write}, process::{Command, Stdio}};

use crate::{wimpl::{Func, self, Expr::Call, Function, Var}, FunctionType};

pub struct CallGraph(HashSet<(Func, Func)>);

impl fmt::Display for CallGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("{:?}", self.0); 
        for (x, y) in self.0.iter() {
            writeln!(f, "{} {}", x, y).expect("");
        }; 
        Ok(())
    }
}

impl CallGraph {
    // pub fn reachable_funcs(&self, initially: HashSet<Func>) -> HashSet<Func> {
    //     let mut reachable = initially;

    //     for (src, dest) in &self.0 {
    //         if reachable.contains(src) {
    //             reachable.insert(dest.clone());
    //         }
    //     }

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

// impl fmt::Display for Target {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Edge::Direct(from, to) => write!(f, "{} -> {}", from, to),
//             Edge::Constrained(from, to) => write!(f, "∀ f ∈ module.funcs with  .idx ∈ init(table_elements)"),
//         }
//         todo!()
//     }
// }

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Constraint {
    Type(FunctionType),
    InTable,
    TableIndexExpr(wimpl::Expr),
}

// impl fmt::Display for Constraint {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Constraint::Type(ty) => write!(f, "type == {}", ty),
//             Constraint::InTable => write!(f, "idx ∈ init(table_elements)"),
//         }
//     }
// }

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
            Assign { lhs: var, type_, rhs: expr } => {
                // println!("{}\nbefore {:?}", stmt, var_expr);
                var_expr.entry(*var)
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
        with_index_constraint: true,
        ..Options::default()
    };
    let reachable = vec![Func::Named("main".to_string())].into_iter().collect::<HashSet<_>>();
    let callgraph = reachable_callgraph(&wimpl_module, reachable, options).unwrap();

    println!("{}", callgraph.to_dot());
    callgraph.to_pdf("tests/callgraph.pdf").unwrap();
}

pub fn callgraph(module: &wimpl::Module) -> CallGraph {
    // TODO split "collecting constraints" from "solving constraints to a graph"
     
    let mut graph: HashSet<(Func, Func)> = HashSet::new();
    
    for fun in &module.functions {
        for instr in &fun.body.0 {
            use wimpl::Stmt::*;
            use wimpl::Expr::*;
            match instr {
                
                Assign { lhs: _, rhs: Call{ func, args: _}, type_: _ } |
                Expr(Call { func, args: _}) => {
                    graph.insert((fun.name(), func.clone()));
                }

                Assign { lhs: _, rhs: CallIndirect { type_, table_idx: _, args: _ }, type_: _ } |
                Expr(CallIndirect { type_, table_idx: _, args: _ }) => {
                    let mut funcs_in_table: HashSet<&Function> = HashSet::new();
                    for tab in &module.tables {
                        // TODO interpret table offset for index-based analysis
                        for elem in &tab.elements {
                            for func_idx in &elem.functions {
                                let func = module.function_by_idx(*func_idx);
                                funcs_in_table.insert(func); 
                            }
                        }
                    } 
                    
                    // All functions, no constraint at all.
                    // for f in &module.functions {
                    //     graph.insert((fun.name(), f.name()));
                    // }

                    // Only functions in table, but no ty constraint.
                    // for func in &funcs_in_table {
                    //     graph.insert((fun.name(), func.name())); 
                    // }

                    // All functions, but with ty constraint.
                    // for func in &module.functions {
                    //     if &func.type_ == type_ {
                    //         graph.insert((fun.name(), func.name())); 
                    //     }
                    // }

                    // Functions in table AND type constraint.
                    for func in &funcs_in_table {
                        if &func.type_ == type_ {
                            graph.insert((fun.name(), func.name())); 
                        }
                    }

                    // TODO Daniel make constraints B & C orthogonal
                }
                _ => (), 
                
            }
        }
    }

    // Option D): index based analysis???
    CallGraph(graph) 
    //graph
}

#[cfg(test)]
mod tests {
    use crate::{wimpl::wimplify, callgraph::CallGraph, callgraph};
 
}

#[test]
fn create_graph() {
    // TODO 2-3 function wasm file, 1 direct call, 2 call_indirect, 5 functions in total
    let wimpl_module = wimpl::wimplify("tests/wimpl-wasm-handwritten/calc-dce/add-dce.wasm").expect(""); 
    println!("{}", wimpl_module); 
    let callgraph = callgraph(&wimpl_module); 
    println!("{}", callgraph.to_dot());
    callgraph.to_pdf("tests/wimpl/calc-dce/callgraph.pdf").unwrap();
}

#[test]
fn calc_virtual() {
    let wimpl_module = wimpl::wimplify("tests/wimpl-wasm-handwritten/calc-virtual/add.wasm").expect(""); 
    println!("{}", wimpl_module);     
    let callgraph = callgraph(&wimpl_module); 
    println!("{}", callgraph.to_dot());
    callgraph.to_pdf("tests/wimpl/calc-dce/callgraph.pdf").unwrap();
}
