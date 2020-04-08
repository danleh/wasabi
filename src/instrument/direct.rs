use wasm::{FunctionType, Mutability, Val, ValType::*};
use wasm::highlevel::{GlobalOp::*, Instr::*, Module, NumericOp::*};

/* Direct or "low-level" instrumentations, i.e., where the byte code is manually modified. */

pub fn add_empty_function(module: &mut Module) -> Option<String> {
    module.add_function(FunctionType::new(&[], &[]), vec![], vec![End]);
    None
}

pub fn count_calls(module: &mut Module) -> Option<String> {
    let counter = module.add_global(I32, Mutability::Mut, vec![Const(Val::I32(0)), End]);

    let getter = module.add_function(
        FunctionType::new(&[], &[I32]),
        vec![],
        vec![Global(Get, counter), End],
    );
    module.function_mut(getter).export = vec!["get_counter".into()];

    let increment = module.add_function(
        FunctionType::new(&[], &[]),
        vec![],
        vec![
            Global(Get, counter),
            Const(Val::I32(1)),
            Numeric(I32Add),
            Global(Set, counter),
            End,
        ],
    );

    for (i, function) in module.functions_mut() {
        // ignore the functions we added
        if i != getter && i != increment {
            function.modify_instrs(|instr| match instr {
                Call(..) | CallIndirect(..) => vec![Call(increment), instr],
                instr => vec![instr],
            })
        }
    }

    None
}
