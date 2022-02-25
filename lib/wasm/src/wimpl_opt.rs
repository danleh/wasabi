//use crate::{wimpl::{Func, self, Expr::Call, Function, Var}, FunctionType};

fn constant_propogation (module: wimpl::Module) -> wimpl::Module {

}

pub fn wimpl_optimize (path: impl AsRef<Path>) -> Result<Module, String> {
    let module = wimplify_module(&highlevel::Module::from_file(path.as_ref()).expect("path should point to a valid wasm file")); 
    let module = constant_propagation(module);     
}

#[test]
fn test() {
    println!("hi"); 
}

