use wasabi_wasm::BinaryOp::*;
use wasabi_wasm::FunctionType;
use wasabi_wasm::GlobalOp::*;
use wasabi_wasm::Instr::*;
use wasabi_wasm::Module;
use wasabi_wasm::Mutability;
use wasabi_wasm::Val;
use wasabi_wasm::ValType::*;

/* Direct or "low-level" instrumentations, i.e., where the byte code is manually modified. */

pub fn add_empty_function(module: &mut Module) {
    module.add_function(FunctionType::new(&[], &[]), vec![], vec![End]);
}

pub fn count_calls(module: &mut Module) {
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
            Binary(I32Add),
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
}
