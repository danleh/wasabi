use std::{collections::HashSet, path::Path, io, process::Command};

use crate::{wimpl::{Func, self, Expr::Call, Function}, FunctionType};

pub struct CallGraph(HashSet<(Func, Func)>);

impl CallGraph {
    pub fn to_dot(&self) -> String {
        let mut dot_file: String = "".to_owned();
        dot_file.push_str("digraph G {\n"); 
        dot_file.push_str("\trankdir=\"LR\";\n"); 
        for (from_fn, to_fn) in self.0.clone() {
            dot_file.push_str(&format!("\t\"{}\"->\"{}\";\n", from_fn, to_fn)); 
        }
        dot_file.push_str("}\n");         
        dot_file
    }

    pub fn to_png(&self, path: impl AsRef<Path>) -> io::Result<()> {
        //dot -Tps filename.dot -o outfile.ps
        Command::new("dot")
                .args(["-Tps", &self.to_dot(), "-o", format!("{}", path)])
                .spawn()
                .expect(""); 
        // invoke CLI tool with path as argument and to_dot() as stdinput
        Ok(())
    }
}

pub fn callgraph(module: &wimpl::Module) -> CallGraph {
    // TODO split "collecting constraints" from "solving constraints to a graph"

    let mut graph: HashSet<(Func, Func)> = HashSet::new();
    
    for fun in &module.functions {
        for instr in &fun.instrs.0 {
            match instr {
                
                wimpl::Stmt::Assign { lhs: _, expr: wimpl::Expr::Call{ func, args: _}, type_: _ } |
                wimpl::Stmt::Expr{ expr: wimpl::Expr::Call { func, args: _} } => {
                    graph.insert((fun.name, *func));
                }

                wimpl::Stmt::Assign { lhs: _, expr: wimpl::Expr::CallIndirect { type_, table_idx, args: _ }, type_: _ } |
                wimpl::Stmt::Expr{ expr: wimpl::Expr::CallIndirect { type_, table_idx, args: _ } } => {
                    
                    //OPTION A
                    for f in &module.functions {
                        graph.insert((fun.name, f.name));
                    }

                    let mut funcs_in_table: HashSet<(usize, FunctionType)> = HashSet::new();
                    for tab in &module.tables {
                        for elem in &tab.elements {
                            for fun in &elem.functions {
                                funcs_in_table.insert((fun.into_inner(), module.function(*fun).type_)); 
                            }
                        }
                    } 
                    
                    //OPTION B 
                    for (func, _) in funcs_in_table {
                        graph.insert((fun.name, Func::Idx(func))); 
                    }

                    //OPTION C
                    for (func, fn_type) in funcs_in_table {
                        if fun.type_ == fn_type {
                            graph.insert((fun.name, Func::Idx(func))); 
                        }
                    }
                }
                _ => (), 
                
            }
        }
    }
        // Option B): add all functions in the table
        // collect all function idx. in the elements
        // let mut funcs_in_table = Set()
        // for elem in table.elements
            // for func in elem.funcs
                // funcs_in_table.add(func)
        // for func2 in funcs_in_table
            // graph.add((func, func2))

        // Option C): Type-drive analysis
        // repeat stuff from B), except last step
        // for func2 in funcs_in_table
            // if func2.type == call_indirect.type
                // graph.add((func, func2))

        // Option D): index based analysis???
    CallGraph(graph) 
    //graph
}

#[cfg(test)]
mod tests {

    #[test]
    fn create_graph() {
        // parse wasm to hl module
        // convert hl to wimpl
        // run callgraph
        // manual: inspect call graph
    }
}