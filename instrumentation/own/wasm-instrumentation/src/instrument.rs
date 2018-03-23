use ast::{FunctionType, GlobalType, Idx, Limits, Local, Memarg, MemoryType, Mutability, ValType, ValType::*};
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
        Binary { first_ty, second_ty, result_ty } => vec![first_ty, second_ty, result_ty],
        // for address, offset and alignment
        MemoryLoad(ty, _) => vec![I32, I32, I32, ty],
        MemoryStore(ty, _) => vec![I32, I32, I32, ty],
        Other => unreachable!("function should be only called for \"grouped\" instructions"),
    }))
}

struct PolymorphicHookMap(HashMap<(Discriminant<Instr>, Vec<ValType>), Idx<Function>>);

impl PolymorphicHookMap {
    pub fn new() -> Self {
        PolymorphicHookMap(HashMap::new())
    }
    pub fn add(&mut self, module: &mut Module, instr: Instr, tys: impl IntoIterator<Item = Vec<ValType>>) {
        for tys in tys {
            let hook_name = instr.to_instr_name() + "_" + &tys.iter().map(|ty| ty.to_string()).collect::<Vec<_>>().join("_");
            let hook_idx = add_hook(module, hook_name, tys.as_slice());
            self.0.insert(
                (discriminant(&instr), tys.clone()),
                hook_idx);
        }
    }
    pub fn get_call(&self, instr: &Instr, tys: Vec<ValType>) -> Instr {
        let error = format!("no hook was added for instruction {} with types {:?}", instr.to_instr_name(), tys);
        Call(*self.0
            .get(&(discriminant(instr), tys))
            .expect(&error))
    }
}

pub fn add_hooks(module: &mut Module) {
    /* add hooks (imported functions, provided by the analysis in JavaScript) */
    // polymorphic hooks:
    // - 1 instruction : N hooks
    // - instruction can take stack arguments/produce results of several types
    // - we need to "monomorphize", i.e., create one hook per occurring polymorphic type
    let mut polymorphic_hooks = PolymorphicHookMap::new();

    // returns
    let result_tys: HashSet<Vec<ValType>> = module.types().iter()
        .map(|function_ty| function_ty.1.clone())
        .collect();
    polymorphic_hooks.add(module, Return, result_tys);

    // calls

    // monomorphic hooks:
    // - 1 hook : 1 instruction
    // - argument/result types are directly determined from the instruction itself
    let current_memory_hook = add_hook(module, "current_memory", &[I32]);
    let grow_memory_hook = add_hook(module, "grow_memory", &[I32, I32]);

//    let get_local_hook = add_hook(module, "get_local", )

    let monomorphic_hook_call = {
        let monomorphic_hooks: HashMap<Discriminant<Instr>, Idx<Function>> = [
            I32Const(0),
            I64Const(0),
            F32Const(0.0),
            F64Const(0.0),

            // Unary
            I32Eqz, I64Eqz,
            I32Clz, I32Ctz, I32Popcnt,
            I64Clz, I64Ctz, I64Popcnt,
            F32Abs, F32Neg, F32Ceil, F32Floor, F32Trunc, F32Nearest, F32Sqrt,
            F64Abs, F64Neg, F64Ceil, F64Floor, F64Trunc, F64Nearest, F64Sqrt,
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

            // Binary
            I32Eq, I32Ne, I32LtS, I32LtU, I32GtS, I32GtU, I32LeS, I32LeU, I32GeS, I32GeU,
            I64Eq, I64Ne, I64LtS, I64LtU, I64GtS, I64GtU, I64LeS, I64LeU, I64GeS, I64GeU,
            F32Eq, F32Ne, F32Lt, F32Gt, F32Le, F32Ge,
            F64Eq, F64Ne, F64Lt, F64Gt, F64Le, F64Ge,
            I32Add, I32Sub, I32Mul, I32DivS, I32DivU, I32RemS, I32RemU, I32And, I32Or, I32Xor, I32Shl, I32ShrS, I32ShrU, I32Rotl, I32Rotr,
            I64Add, I64Sub, I64Mul, I64DivS, I64DivU, I64RemS, I64RemU, I64And, I64Or, I64Xor, I64Shl, I64ShrS, I64ShrU, I64Rotl, I64Rotr,
            F32Add, F32Sub, F32Mul, F32Div, F32Min, F32Max, F32Copysign,
            F64Add, F64Sub, F64Mul, F64Div, F64Min, F64Max, F64Copysign,

            // Memory
            I32Load(Memarg::default()), I32Load8S(Memarg::default()), I32Load8U(Memarg::default()), I32Load16S(Memarg::default()), I32Load16U(Memarg::default()),
            I64Load(Memarg::default()), I64Load8S(Memarg::default()), I64Load8U(Memarg::default()), I64Load16S(Memarg::default()), I64Load16U(Memarg::default()), I64Load32S(Memarg::default()), I64Load32U(Memarg::default()),
            F32Load(Memarg::default()),
            F64Load(Memarg::default()),
            I32Store(Memarg::default()), I32Store8(Memarg::default()), I32Store16(Memarg::default()),
            I64Store(Memarg::default()), I64Store8(Memarg::default()), I64Store16(Memarg::default()), I64Store32(Memarg::default()),
            F32Store(Memarg::default()),
            F64Store(Memarg::default()),
        ].into_iter()
            .map(|i| add_hook_from_instr(module, i))
            .collect();

        move |instr: &Instr| -> Instr {
            Call(*monomorphic_hooks
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
                        (_, instr @ CurrentMemory(_ /* TODO memory idx == 0 in WASM version 1 */)) => {
                            let result_tmp = fresh_local(locals, function_type, I32);
                            vec![
                                instr,
                                TeeLocal(result_tmp),
                                location.0,
                                location.1,
                                GetLocal(result_tmp),
                                Call(current_memory_hook)
                            ]
                        }
                        (_, instr @ GrowMemory(_ /* TODO memory idx == 0 in WASM version 1 */)) => {
                            let input_tmp = fresh_local(locals, function_type, I32);
                            let result_tmp = fresh_local(locals, function_type, I32);
                            vec![
                                TeeLocal(input_tmp),
                                instr,
                                TeeLocal(result_tmp),
                                location.0,
                                location.1,
                                GetLocal(input_tmp),
                                GetLocal(result_tmp),
                                Call(grow_memory_hook)
                            ]
                        }
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
                            for &dup_tmp in result_duplicate_tmps.iter().rev() {
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
                                polymorphic_hooks.get_call(&Return, result_tys.to_vec()),
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
                                monomorphic_hook_call(&instr),
                                instr,
                            ]);
                            instrs
                        }
                        (Unary { input_ty, result_ty }, instr) => {
                            // duplicate stack arguments
                            let input_tmp = fresh_local(locals, function_type, input_ty);
                            let result_tmp = fresh_local(locals, function_type, result_ty);

                            let mut instrs = vec![
                                // save input before
                                TeeLocal(input_tmp),
                                // execute original instr
                                instr.clone(),
                                // save result after
                                SetLocal(result_tmp),
                                location.0,
                                location.1,
                            ];
                            // restore saved input and result
                            instrs.append(&mut convert_i64_instr(GetLocal(input_tmp), input_ty));
                            instrs.append(&mut convert_i64_instr(GetLocal(result_tmp), result_ty));
                            instrs.extend_from_slice(&[
                                monomorphic_hook_call(&instr),
                                // restore result after hook call
                                GetLocal(result_tmp),
                            ]);
                            instrs
                        }
                        (Binary { first_ty, second_ty, result_ty }, instr) => {
                            // duplicate stack arguments
                            let first_tmp = fresh_local(locals, function_type, first_ty);
                            let second_tmp = fresh_local(locals, function_type, second_ty);
                            let result_tmp = fresh_local(locals, function_type, result_ty);

                            let mut instrs = vec![
                                // save input before
                                TeeLocal(first_tmp),
                                TeeLocal(second_tmp),
                                // execute original instr
                                instr.clone(),
                                // save result after
                                SetLocal(result_tmp),
                                location.0,
                                location.1,
                            ];
                            // restore saved input and result
                            instrs.append(&mut convert_i64_instr(GetLocal(first_tmp), first_ty));
                            instrs.append(&mut convert_i64_instr(GetLocal(second_tmp), second_ty));
                            instrs.append(&mut convert_i64_instr(GetLocal(result_tmp), result_ty));
                            instrs.extend_from_slice(&[
                                monomorphic_hook_call(&instr),
                                // restore result after hook call
                                GetLocal(result_tmp),
                            ]);
                            instrs
                        }
                        (MemoryLoad(ty, memarg), instr) => {
                            // duplicate stack arguments
                            let addr_tmp = fresh_local(locals, function_type, I32);
                            let value_tmp = fresh_local(locals, function_type, ty);

                            let mut instrs = vec![
                                // save input before
                                TeeLocal(addr_tmp),
                                // execute original instr
                                instr.clone(),
                                // save result after
                                SetLocal(value_tmp),
                                location.0,
                                location.1,
                                GetLocal(addr_tmp),
                                I32Const(memarg.offset as i32),
                                I32Const(memarg.alignment as i32),
                            ];
                            instrs.append(&mut convert_i64_instr(GetLocal(value_tmp), ty));
                            instrs.extend_from_slice(&[
                                monomorphic_hook_call(&instr),
                                // restore result after hook call
                                GetLocal(value_tmp),
                            ]);
                            instrs
                        }
                        (MemoryStore(ty, memarg), instr) => {
                            // duplicate stack arguments
                            let addr_tmp = fresh_local(locals, function_type, I32);
                            let value_tmp = fresh_local(locals, function_type, ty);

                            let mut instrs = vec![
                                // save input before
                                SetLocal(value_tmp),
                                TeeLocal(addr_tmp),
                                GetLocal(value_tmp),
                                // execute original instr
                                instr.clone(),
                                location.0,
                                location.1,
                                GetLocal(addr_tmp),
                                I32Const(memarg.offset as i32),
                                I32Const(memarg.alignment as i32),
                            ];
                            instrs.append(&mut convert_i64_instr(GetLocal(value_tmp), ty));
                            instrs.push(monomorphic_hook_call(&instr));
                            instrs
                        }
                        // TODO Begin(Function | Block | If | Else)
                        // TODO End(Function | Block | If | Else) (needs stack of open blocks to match with begin)
                        // TODO Get, Set, Tee Local|Global
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