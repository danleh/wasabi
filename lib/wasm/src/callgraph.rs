use std::{collections::HashSet, path::Path, io::{self, Write}, process::{Command, Stdio}};

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
        let status = child.wait()?;
        assert!(status.success(), "should not fail because we always produce valid dot input!?");
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
                    graph.insert((fun.name.clone(), func.clone()));
                }

                wimpl::Stmt::Assign { lhs: _, expr: wimpl::Expr::CallIndirect { type_, table_idx: _, args: _ }, type_: _ } |
                wimpl::Stmt::Expr{ expr: wimpl::Expr::CallIndirect { type_, table_idx: _, args: _ } } => {
                    
                    //OPTION A: trivial, all functions are reachable
                    for f in &module.functions {
                        graph.insert((fun.name.clone(), f.name.clone()));
                    }

                    let mut funcs_in_table: HashSet<&Function> = HashSet::new();
                    for tab in &module.tables {
                        // TODO interpret table offset for index-based analysis
                        for elem in &tab.elements {
                            for func_idx in &elem.functions {
                                let func = module.function(*func_idx);
                                funcs_in_table.insert(func); 
                            }
                        }
                    } 
                    
                    //OPTION B 
                    for func in &funcs_in_table {
                        graph.insert((fun.name.clone(), func.name.clone())); 
                    }

                    //OPTION C
                    for func in &funcs_in_table {
                        if &func.type_ == type_ {
                            graph.insert((fun.name.clone(), func.name.clone())); 
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