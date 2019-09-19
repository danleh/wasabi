use wasm::ast::highlevel::{GlobalOp::*, Instr::*, Module, NumericOp::*};
use wasm::ast::{FunctionType, Mutability, Val, ValType::*};

/* direct or "low-level" instrumentations, i.e., where the byte code is manually modified */

pub fn add_empty_function(module: &mut Module) -> Option<String> {
    module.add_function(FunctionType::new(vec![], vec![]), vec![], vec![End]);
    None
}

pub fn count_calls(module: &mut Module) -> Option<String> {
    let counter = module.add_global(I32, Mutability::Mut, vec![Const(Val::I32(0)), End]);

    let getter = module.add_function(
        FunctionType::new(vec![], vec![I32]),
        vec![],
        vec![Global(GetGlobal, counter), End],
    );
    module.function(getter).export = vec!["get_counter".into()];

    let increment = module.add_function(
        FunctionType::new(vec![], vec![]),
        vec![],
        vec![
            Global(GetGlobal, counter),
            Const(Val::I32(1)),
            Numeric(I32Add),
            Global(SetGlobal, counter),
            End,
        ],
    );

    for (i, function) in module.functions() {
        // ignore the functions we added
        if i != getter && i != increment {
            function.modify_instr(|instr| match instr {
                Call(..) | CallIndirect(..) => vec![Call(increment), instr],
                instr => vec![instr],
            })
        }
    }

    None
}
