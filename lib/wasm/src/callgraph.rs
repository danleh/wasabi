use std::{collections::HashSet, path::Path, io};

use crate::wimpl::{Func, self};

pub struct CallGraph(HashSet<(Func, Func)>);

impl CallGraph {
    pub fn to_dot(&self) -> String {
        // read up on dotfile syntax
        todo!()
    }

    pub fn to_png(&self, path: impl AsRef<Path>) -> io::Result<()> {
        // invoke CLI tool with path as argument and to_dot() as stdinput
        todo!()
    }
}

pub fn callgraph(module: &wimpl::Module) -> CallGraph {
    // TODO split "collecting constraints" from "solving constraints to a graph"

    //let graph = HashSet::new();
    // iterate over all funcs in module
    // iterate over all instr in func
    // if instr == Call
        // graph.add((func, call.func))
    // if instr == CallIndirect
        // Option A): add all functions
        // for func2 over all funcs in module
            // graph.add((func, func2))

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
    todo!() 
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