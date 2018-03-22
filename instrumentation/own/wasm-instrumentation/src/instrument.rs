use ast::{FunctionType, GlobalType, Idx, Limits, Local, MemoryType, Mutability, ValType, ValType::*};
use ast::highlevel::{Code, Expr, Function, Instr, Instr::*, InstrGroup, InstrGroup::*, Memory, Module};
use std::collections::{HashMap, HashSet};
use std::mem::{discriminant, Discriminant};

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

/// add a new local with type_
/// We don't take whole function, but only locals and function_ty since the code itself is not
/// touched (and we would get some errors with borrowck otherwise).
/// function_ty is necessary since locals are indexed together with function parameters
fn fresh_local(locals: &mut Vec<ValType>, function_ty: &FunctionType, type_: ValType) -> Idx<Local> {
    let idx = locals.len() + function_ty.0.len();
    locals.push(type_);
    idx.into()
}

fn convert_i64_type(ty: &ValType) -> &[ValType] {
    match ty {
        &I64 => &[I32, I32],
        ty => ::std::slice::from_ref(ty),
    }
}

// ty is necessary when the type cannot be determined only from the instr, e.g., for GetLocal
fn convert_i64_instr(instr: Instr, ty: ValType) -> Vec<Instr> {
    match ty {
        I64 => vec![
            instr.clone(),
            I32WrapI64, // low bits
            instr,
            I64Const(32), // shift high bits to the right
            I64ShrS,
            I32WrapI64, // high bits
        ],
        _ => vec![instr],
    }
}

fn add_hook(module: &mut Module, name: impl Into<String>, arg_tys_: &[ValType]) -> Idx<Function> {
    // prepend two I32 for (function idx, instr idx)
    let mut arg_tys = vec![I32, I32];
    arg_tys.extend(arg_tys_.iter()
        // and expand i64 to a tuple of (i32, i32) since there is no JS interop for i64
        .flat_map(convert_i64_type));

    module.add_function_import(
        // hooks do not return anything
        FunctionType(arg_tys, vec![]),
        "hooks".into(),
        name.into())
}

/// specialized version form of the above for monomorphic instructions
fn add_hook_from_instr(module: &mut Module, instr: &Instr) -> (Discriminant<Instr>, Idx<Function>) {
    println!("{}", instr.to_js_hook());
    (discriminant(instr), add_hook(module, instr.to_instr_name(), &match instr.group() {
        Const(ty) => vec![ty],
        Unary { input_ty, result_ty } => vec![input_ty, result_ty],
        Other => unreachable!("function should be only called for \"grouped\" instructions"),
    }))
}

pub fn add_hooks(module: &mut Module) {
    /* add hooks (imported functions, provided by the analysis in JavaScript) */
    // polymorphic hooks:
    // - 1 instruction : N hooks
    // - instruction can take stack arguments/produce results of several types
    // - we need to "monomorphize", i.e., create one hook per occurring polymorphic type
    let result_tys: HashSet<Vec<ValType>> = module.types().iter()
        .map(|function_ty| function_ty.1.clone())
        .collect();
    let return_hooks: HashMap<Vec<ValType>, Idx<Function>> = result_tys.into_iter()
        .map(|return_ty| {
            let hook_idx = add_hook(
                module,
                "return_".to_string() + &return_ty.iter().map(|ty| ty.to_string()).collect::<Vec<_>>().join("_"),
                return_ty.as_slice());
            (return_ty, hook_idx)
        })
        .collect();

    // monomorphic hooks:
    // - 1 hook : 1 instruction
    // - argument/result types are directly determined from the instruction itself
    let hook_call = {
        let monomorph_hooks: HashMap<Discriminant<Instr>, Idx<Function>> = [
            I32Const(0),
            I64Const(0),
            F32Const(0.0),
            F64Const(0.0),
            I32Eqz, I64Eqz,
            I32Clz, I32Ctz, I32Popcnt,
            I64Clz, I64Ctz, I64Popcnt,
            F32Abs, F32Neg, F32Ceil, F32Floor, F32Trunc, F32Nearest,
            F64Abs, F64Neg, F64Ceil, F64Floor, F64Trunc, F64Nearest,
            I32WrapI64,
            I32TruncSF32, I32TruncUF32,
            I32TruncSF64, I32TruncUF64,
            I64ExtendSI32, I64ExtendUI32,
            I64TruncSF32, I64TruncUF32,
            I64TruncSF64, I64TruncUF64,
            F32ConvertSI32, F32ConvertUI32,
            F32ConvertSI64, F32ConvertUI64,
            F32DemoteF64,
            F64ConvertSI32, F64ConvertUI32,
            F64ConvertSI64, F64ConvertUI64,
            F64PromoteF32,
            I32ReinterpretF32,
            I64ReinterpretF64,
            F32ReinterpretI32,
            F64ReinterpretI64,
        ].into_iter()
            .map(|i| add_hook_from_instr(module, i))
            .collect();

        move |instr: &Instr| -> Instr {
            Call(*monomorph_hooks
                .get(&discriminant(instr))
                .expect(&format!("no hook was added for instruction {}", instr.to_instr_name())))
        }
    };

    /* add call to hooks: setup code that copies the returned value, instruction location, call */
    // NOTE we do not need to filter out functions since all hooks are imports and thus won't have
    // Code to instrument anyway...
    for (fidx, function) in module.functions() {
        if let Some(ref mut code) = function.code {
            let result_tys = function.type_.1.as_slice();
            let function_type = &function.type_;
            let locals = &mut code.locals;
            code.body = code.body.iter().cloned().enumerate()
                .flat_map(|(iidx, instr)| {
                    let location = (I32Const(fidx.0 as i32), I32Const(iidx as i32));
                    match (instr.group(), instr) {
                        (_, Return) => {
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
                            instrumented_return.extend_from_slice(&[
                                location.0,
                                location.1,
                            ]);
                            // duplicate results from tmp locals
                            for (&dup_tmp, &result_ty) in result_duplicate_tmps.iter().zip(result_tys.iter()) {
                                instrumented_return.append(&mut convert_i64_instr(GetLocal(dup_tmp), result_ty));
                            }

                            instrumented_return.extend_from_slice(&[
                                Call(*return_hooks.get(result_tys).unwrap()),
                                Return,
                            ]);
                            instrumented_return
                        }
                        (Const(ty), instr) => {
                            let mut instrs = vec![
                                location.0,
                                location.1,
                            ];
                            instrs.append(&mut convert_i64_instr(instr.clone(), ty));
                            instrs.extend_from_slice(&[
                                hook_call(&instr),
                                instr,
                            ]);
                            instrs
                        }
                        (Unary { input_ty, result_ty }, instr) => {
                            // duplicate stack arguments
                            let input_tmp = fresh_local(locals, function_type, input_ty);
                            let result_tmp = fresh_local(locals, function_type, result_ty);

                            let mut instrs = vec![
                                TeeLocal(input_tmp),
                                instr.clone(),
                                SetLocal(result_tmp),
                                location.0,
                                location.1,
                            ];
                            instrs.append(&mut convert_i64_instr(GetLocal(input_tmp), input_ty));
                            instrs.append(&mut convert_i64_instr(GetLocal(result_tmp), result_ty));
                            instrs.extend_from_slice(&[
                                hook_call(&instr),
                                GetLocal(result_tmp),
                            ]);
                            instrs
                        }
                        (_, instr) => vec![instr],
                    }
                })
                .collect();
        }
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
    let counter = module.add_global(I32, Mutability::Mut, vec![I32Const(0), End]);

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