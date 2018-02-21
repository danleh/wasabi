use ast::highlevel::{Code, Instr, Module, VisitExpr};
use ast::highlevel::Expr;
use ast::highlevel::Instr::*;
use ast::{FunctionType, GlobalType};
use ast::Mutability::*;
use ast::ValType::*;

// TODO Idea: provide two options of connecting user analysis (i.e., client instrumentation code)
// with the instrumented binary (i.e., the "host" code + hooks + import of callbacks):
// A) "dynamic/late binding": instrument host code once, write many analyses as separate WASM
//    modules. Use the JavaScript/host WASM API to "link" the two together at runtime, i.e.,
//    export analysis functions as JS functions and provide them as import object to the
//    instrumented binary.
//    Possibly suuper slow since we go WASM <-> JS <-> WASM
// B) "static binding": Still build the two modules seperately. But use "wasm linker" and "wasm
//    inliner" to optimize the cross-language boundary away.
//
//    Step 1 WASM linker: Append all contents from the analysis module onto the host binary. Then
//    replace all imported functions (in the host binary) with code from the exported functions
//    of the analysis module IFF their names match.
//
//    Step 2 inlining (possibly by external tool, WABT?):
//    Trivial inlining: if function body is empty (since most callbacks won't be used by the
//    analysis module), remove the call to the function + setup of function arguments

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
