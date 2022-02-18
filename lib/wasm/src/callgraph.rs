use core::fmt;
use std::{collections::HashSet, path::Path, io::{self, Write}, process::{Command, Stdio}};

use crate::{wimpl::{Func, self, Expr::Call, Function}, FunctionType};

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
    pub fn to_dot(&self) -> String {
        let mut dot_file: String = "".to_owned();
        dot_file.push_str("digraph G {\n"); 
        dot_file.push_str("\trankdir=\"LR\";\n"); 
        for (from_fn, to_fn) in &self.0 {
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

pub enum Edge {
    Direct(Func, Func),
    Constrained(Func, HashSet<Constraint>)
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // match self {
        //     Edge::Direct(from, to) => write!(f, "{} -> {}", from, to),
        //     // Edge::Constrained(from, to) => write!(f, "∀ f ∈ module.funcs with  .idx ∈ init(table_elements)"),
        // }
        todo!()
    }
}

pub enum Constraint {
    Type(FunctionType),
    InTable,
    // TableIndex(wimpl::Expr),
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constraint::Type(ty) => write!(f, ".type == {}", ty),
            Constraint::InTable => write!(f, ".idx ∈ init(table_elements)"),
        }
    }
}

pub struct CallGraphOptions {
    with_type_constraint: bool,
    with_in_table_constraint: bool,
}

pub fn collect_reachable_constraints(
    module: &wimpl::Module,
    mut reachable: HashSet<Func>,
    options: CallGraphOptions
) -> HashSet<Edge> {
    let mut edges = HashSet::new();

    for func in reachable {
        let func = module.function(func).expect("function name not found in module");

        // TODO how to handle imported functions? Can they each every exported function?
        // Do we add a direct edge there? Or do we add an abstract "host" node? 
        // Do we merge with a JavaScript call-graph analysis?
        let body = func.body;

        for stmt in body.0 {
            
            use wimpl::Stmt::*;
            use wimpl::Expr::*;
            // match stmt {
                
            //     Assign { lhs: _, rhs: Call{ func, args: _}, type_: _ } |
            //     Expr(Call { func, args: _}) => {
            //         edges.insert((fun.name(), func.clone()));
            //     }

            //     Assign { lhs: _, rhs: CallIndirect { type_, table_idx: _, args: _ }, type_: _ } |
            //     Expr(CallIndirect { type_, table_idx: _, args: _ }) => {
            //     }
            // }
        }

    }

    // edges

    todo!()
}

pub fn solve_constraints(
    module: &wimpl::Module, 
    constraints: &HashSet<Constraint>
) -> HashSet<Func> {

    let mut funcs_in_table: HashSet<&Function> = HashSet::new();
    for table in &module.tables {
        // TODO interpret table offset for index-based analysis
        for element in &table.elements {
            for func_idx in &element.functions {
                let func = module.function_by_idx(*func_idx);
                funcs_in_table.insert(func); 
            }
        }
    } 

    let mut valid_funcs = module.functions.iter().collect::<Vec<_>>();
    for constraint in constraints {
        match constraint {
            Constraint::Type(ty) => valid_funcs.retain(|f| &f.type_ == ty),
            Constraint::InTable => valid_funcs.retain(|f| funcs_in_table.contains(f)),
        }
    }
    valid_funcs.into_iter().map(|f| f.name.clone()).collect()
}

#[test]
fn main() {
    let wimpl_module = wimpl::wimplify("tests/callgraph.wasm").expect(""); 
    println!("{}", wimpl_module);     
    let callgraph = callgraph(&wimpl_module);
    println!("{}", callgraph.to_dot());
    callgraph.to_pdf("tests/callgraph-constraint-in-table-ty.pdf").unwrap();
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
