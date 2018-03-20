use ast::{FunctionType, GlobalType, Idx, Limits, MemoryType, ValType};
use ast::highlevel::{Code, Expr, Function, Instr, Memory, Module};
use ast::highlevel::Instr::*;
use ast::Local;
use ast::Mutability::*;
use ast::ValType::*;
use std::collections::HashMap;
use std::collections::HashSet;

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
//    FIXME Problem with linking: only one memory and table section allowed, what to do if two?
//    - for Memory: replace all memory operations (in particular CurrentMemory and GrowMemory) with
//                  own versions, where the second memory is placed at an offset into the same
//                  memory space as the first one.
//    - for Tables: not so easy because of the default label target
//
//    Step 2 inlining (possibly by external tool, WABT?):
//    Trivial inlining: if function body is empty (since most callbacks won't be used by the
//    analysis module), remove the call to the function + setup of function arguments

fn expand_i64_type(v: &ValType) -> &[ValType] {
    match v {
        &I64 => &[I32, I32],
        v => ::std::slice::from_ref(v),
    }
}

fn val_type_char(v: &ValType) -> char {
    match *v {
        I32 => 'i',
        I64 => 'I',
        F32 => 'f',
        F64 => 'F',
    }
}

fn types_string(v: &[ValType]) -> String {
    v.iter().map(val_type_char).collect()
}

fn fresh_local(locals: &mut Vec<ValType>, function_ty: &FunctionType, type_: ValType) -> Idx<Local> {
    let idx = locals.len() + function_ty.0.len();
    locals.push(type_);
    idx.into()
}

fn add_hook(module: &mut Module, name: String, arg_tys_: &[ValType]) -> Idx<Function> {
    // prepend two I32 for (function idx, instr idx)
    let mut arg_tys = vec![I32, I32];
    arg_tys.extend(arg_tys_.iter()
        // and expand i64 to a tuple of (i32, i32) since there is no JS interop for i64
        .flat_map(expand_i64_type));

    module.add_function_import(
        // hooks do not return anything
        FunctionType(arg_tys, vec![]),
        "hooks".into(),
        name)
}

pub fn add_hooks(module: &mut Module) {
    /* add hooks: one imported function per occurring result type */
    let result_tys: HashSet<Vec<ValType>> = module.types().iter()
        .map(|function_ty| function_ty.1.clone())
        .collect();
    let return_hooks: HashMap<&[ValType], Idx<Function>> = result_tys.iter()
        .map(|return_ty| {
            let hook_idx = add_hook(
                module,
                "return_".to_string() + &types_string(&return_ty),
                return_ty.as_slice());
            (return_ty.as_slice(), hook_idx)
        })
        .collect();
    println!("{:?}", return_hooks);

    /* add call to hooks: setup code that copies the returned value, instruction location, call */
    for (fidx, function) in module.functions() {
        if let Some(ref mut code) = function.code {
            let result_tys = function.type_.1.as_slice();
            let function_type = &function.type_;
            let locals = &mut code.locals;
            code.body = code.body.iter().cloned().enumerate()
                .flat_map(|(iidx, instr)| match instr {
                    Return => {
                        let result_duplicate_tmps: Vec<_> = result_tys.iter()
                            .map(|result_ty| fresh_local(locals, function_type, *result_ty))
                            .collect();

                        let mut instrumented_return = Vec::new();

                        // copy results into tmp locals
                        for &dup_tmp in result_duplicate_tmps.iter() {
                            instrumented_return.push(SetLocal(dup_tmp));
                        }
                        // and restore (saving has removed them from the stack)
                        for &dup_tmp in result_duplicate_tmps.iter() {
                            instrumented_return.push(GetLocal(dup_tmp));
                        }

                        // instruction location
                        instrumented_return.push(I32Const(fidx.0 as i32));
                        instrumented_return.push(I32Const(iidx as i32));
                        // duplicate results from tmp locals
                        for (&dup_tmp, result_ty) in result_duplicate_tmps.iter().zip(result_tys.iter()) {
                            if result_ty == &I64 {
                                instrumented_return.extend_from_slice(&[
                                    GetLocal(dup_tmp),
                                    I32WrapI64, // low bits
                                    GetLocal(dup_tmp),
                                    I64Const(32),
                                    I64ShrS,
                                    I32WrapI64, // high bits
                                ]);
                            } else {
                                instrumented_return.push(GetLocal(dup_tmp));
                            }
                        }

                        instrumented_return.push(Call(*return_hooks.get(result_tys).unwrap()));
                        instrumented_return.push(Return);
                        instrumented_return
                    }
                    instr => {
                        vec![instr]
                    }
                })
                .collect();
        }
//        println!("{:?}", function);
    }
}

/* trivial or "low-level" instrumentations, i.e., where the byte code is manually modified and not
   a higher-level, Jalangi-style "instrumentation hook API" is provided. */

pub fn identity(_: &mut Module) {}

pub fn add_empty_function(module: &mut Module) {
    module.add_function(
        FunctionType(vec![], vec![]),
        vec![],
        vec![End]);
}

pub fn count_calls(module: &mut Module) {
    let counter = module.add_global(I32, Mut, vec![I32Const(0), End]);

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
            function.modify_instr(|instr| match instr {
                Call(..) | CallIndirect(..) => vec![Call(increment), instr],
                instr => vec![instr]
            })
        }
    }
}