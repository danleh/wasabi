use ast::{FunctionType, Mutability::*, ValType::*};
use ast::highlevel::{Instr::*, Module};

/* trivial or "low-level" instrumentations, i.e., where the byte code is manually modified */

pub fn identity(_: &mut Module) -> Option<String> { None }

pub fn add_empty_function(module: &mut Module) -> Option<String> {
    module.add_function(
        FunctionType::new(vec![], vec![]),
        vec![],
        vec![End]);
    None
}

pub fn count_calls(module: &mut Module) -> Option<String> {
    let counter = module.add_global(I32, Mut, vec![I32Const(0), End]);

    let getter = module.add_function(
        FunctionType::new(vec![], vec![I32]),
        vec![],
        vec![GetGlobal(counter), End]);
    module.function(getter).export = Some("get_counter".into());

    let increment = module.add_function(
        FunctionType::new(vec![], vec![]),
        vec![],
        vec![
            GetGlobal(counter),
            I32Const(1),
            I32Add,
            SetGlobal(counter),
            End
        ]);

    for (i, function) in module.functions() {
        // ignore the functions we added
        if i != getter && i != increment {
            function.modify_instr(|instr| match instr {
                Call(..) | CallIndirect(..) => vec![Call(increment), instr],
                instr => vec![instr]
            })
        }
    }

    None
}