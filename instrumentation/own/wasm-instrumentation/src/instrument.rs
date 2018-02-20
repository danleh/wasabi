use ast::highlevel::{Code, Instr, Module, VisitExpr};
use ast::highlevel::Expr;
use ast::highlevel::Instr::*;
use ast::lowlevel::{FunctionType, GlobalType};
use ast::lowlevel::Mutability::*;
use ast::lowlevel::ValType::*;

pub fn identity(_: &mut Module) {}

pub fn add_empty_function(module: &mut Module) {
    module.add_function(
        FunctionType(vec![], vec![]),
        vec![],
        vec![End]);
}

pub fn count_calls(module: &mut Module) {
    let counter = module.add_global(
        GlobalType(I32, Mut),
        vec![I32Const(0), End]);

    let getter = module.add_function(
        FunctionType(vec![], vec![I32]),
        vec![],
        vec![GetGlobal(counter), End]);
    module.function(getter).export = Some("get_counter".into());

    let increment = module.add_function(
        FunctionType(vec![], vec![]),
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
            if let Some(Code { ref mut body, .. }) = function.code {
                VisitExpr::bottom_up(body, &|instrs: &mut Expr| {
                    let mut last_call_instr_idx = 0;
                    while let Some(call_instr_idx) = instrs[last_call_instr_idx..].iter().position(Instr::is_call) {
                        instrs.insert(call_instr_idx, Call(increment));
                        last_call_instr_idx = call_instr_idx + 2;
                    }
                });
            }
        }
    }
}

//// TODO implement actual instrumentations:
//// - call instruction counting
//// - counting which function gets called how often
//// - BB counting
//// ...
