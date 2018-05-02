use ast::{self, BlockType, FunctionType, GlobalType, Idx, InstrType, Memarg, Mutability, Val, ValType, ValType::*};
use ast::highlevel::{Function, Global, GlobalOp::*, Instr, Instr::*, LoadOp::*, LocalOp::*, Module, NumericOp::*, StoreOp::*};
use std::collections::HashMap;
use super::block_stack::{BlockStack, BlockStackElement};
use super::convert_i64::{convert_i64_instr, convert_i64_type};
use super::js_codegen::{append_mangled_tys, js_codegen};
use super::static_info::*;
use super::type_stack::TypeStack;

/// instruments every instruction in Jalangi-style with a callback that takes inputs, outputs, and
/// other relevant information.
pub fn add_hooks(module: &mut Module) -> Option<String> {
    /*
     * make sure every function and table is exported,
     * needed for Wasabi runtime to resolve table indices to function indices
     */
    for table in &mut module.tables {
        if let None = table.export {
            table.export = Some("wasabi_table".into());
        }
    }
    for (fidx, function) in module.functions() {
        if let None = function.export {
            function.export = Some(format!("wasabi_function_{}", fidx.0));
        }
    }

    let mut module_info: ModuleInfo = (&*module).into();
    // TODO use something more meaningful than Strings, e.g., a LowlevelHook struct or so...
    let mut on_demand_hooks = Vec::new();

    /* add hooks (imported functions, provided by the analysis in JavaScript) */

    // polymorphic hooks:
    // - 1 instruction : N hooks
    // - instruction can take stack arguments/produce results of several types
    // - we need to "monomorphize", i.e., create one hook per occurring polymorphic type
    let mut polymorphic_hooks = PolymorphicHookMap::new();

    // collect some info, necessary for monomorphization of polymorphic hooks
    let (mut unique_arg_tys, mut unique_result_tys): (Vec<Vec<ValType>>, Vec<Vec<ValType>>) = module.functions.iter()
        .map(|func| (func.type_.params.clone(), func.type_.results.clone()))
        .unzip();
    unique_result_tys.sort();
    unique_result_tys.dedup();
    unique_arg_tys.sort();
    unique_arg_tys.dedup();

    // returns
    polymorphic_hooks.add(module, Return, &[], unique_result_tys.as_slice(), &mut on_demand_hooks);

    // locals and globals
    let primitive_tys = &[vec![I32], vec![I64], vec![F32], vec![F64]];
    polymorphic_hooks.add(module, Local(GetLocal, 0.into()), &[I32], primitive_tys, &mut on_demand_hooks);
    polymorphic_hooks.add(module, Local(SetLocal, 0.into()), &[I32], primitive_tys, &mut on_demand_hooks);
    polymorphic_hooks.add(module, Local(TeeLocal, 0.into()), &[I32], primitive_tys, &mut on_demand_hooks);
    polymorphic_hooks.add(module, Global(GetGlobal, 0.into()), &[I32], primitive_tys, &mut on_demand_hooks);
    polymorphic_hooks.add(module, Global(SetGlobal, 0.into()), &[I32], primitive_tys, &mut on_demand_hooks);

    // drop and select
    polymorphic_hooks.add(module, Drop, &[], primitive_tys, &mut on_demand_hooks);
    polymorphic_hooks.add(module, Select, &[I32], &[vec![I32, I32], vec![I64, I64], vec![F32, F32], vec![F64, F64]], &mut on_demand_hooks);

    // calls
    polymorphic_hooks.add(module, Call(0.into()), &[I32], unique_arg_tys.as_slice(), &mut on_demand_hooks); // I32 = target func idx
    polymorphic_hooks.add(module, CallIndirect(FunctionType::new(vec![], vec![]), 0.into()), &[I32], unique_arg_tys.as_slice(), &mut on_demand_hooks); // I32 = target table idx
    // manually add call_post hook since it does not directly correspond to an instruction
    // FIXME get rid of this special case, now that the hook_maps are string based anyway...
    let call_post_hooks: HashMap<&[ValType], Idx<Function>> = unique_result_tys.iter()
        .map(|tys| {
            let tys = tys.as_slice();
            (tys, add_hook(module, append_mangled_tys("call_post".into(), tys), tys))
        }).collect();

    // monomorphic hooks:
    // - 1 hook : 1 instruction
    // - argument/result types are directly determined from the instruction itself
    let start_hook = add_hook(module, "start", &[]);
    let if_hook = add_hook(module, "if_", &[/* condition */ I32]);
    // [I32, I32] for label and target instruction index (determined statically)
    let br_hook = add_hook(module, "br", &[I32, I32]);
    let br_if_hook = add_hook(module, "br_if", &[/* condition */ I32, /* target label and instr */ I32, I32]);
    let br_table_hook = add_hook(module, "br_table", &[/* br_table_info_idx */ I32, /* table_idx */ I32]);

    // *_end hooks also give instruction index of corresponding begin (except for functions,
    // where it implicitly is -1 anyway)
    // else_* hooks also give instruction index of corresponding if
    let begin_function_hook = add_hook(module, "begin_function", &[]);
    let end_function_hook = add_hook(module, "end_function", &[]);
    let begin_block_hook = add_hook(module, "begin_block", &[]);
    let end_block_hook = add_hook(module, "end_block", &[I32]);
    let begin_loop_hook = add_hook(module, "begin_loop", &[]);
    let end_loop_hook = add_hook(module, "end_loop", &[I32]);
    let begin_if_hook = add_hook(module, "begin_if", &[]);
    let end_if_hook = add_hook(module, "end_if", &[I32]);
    let begin_else_hook = add_hook(module, "begin_else", &[I32]);
    let end_else_hook = add_hook(module, "end_else", &[I32, I32]);

    let nop_hook = add_hook(module, "nop", &[]);
    let unreachable_hook = add_hook(module, "unreachable", &[]);

    let memory_size_hook = add_hook(module, "memory_size", &[I32]);
    let memory_grow_hook = add_hook(module, "memory_grow", &[I32, I32]);

    // TODO make this a struct of its own, similar to PolymorphicHookMap
    let monomorphic_hook_call = {
        let monomorphic_hooks: HashMap<&'static str, Idx<Function>> = [
            Const(Val::I32(0)),
            Const(Val::I64(0)),
            Const(Val::F32(0.0)),
            Const(Val::F64(0.0)),

            // Unary
            Numeric(I32Eqz), Numeric(I64Eqz),
            Numeric(I32Clz), Numeric(I32Ctz), Numeric(I32Popcnt),
            Numeric(I64Clz), Numeric(I64Ctz), Numeric(I64Popcnt),
            Numeric(F32Abs), Numeric(F32Neg), Numeric(F32Ceil), Numeric(F32Floor), Numeric(F32Trunc), Numeric(F32Nearest), Numeric(F32Sqrt),
            Numeric(F64Abs), Numeric(F64Neg), Numeric(F64Ceil), Numeric(F64Floor), Numeric(F64Trunc), Numeric(F64Nearest), Numeric(F64Sqrt),
            Numeric(I32WrapI64),
            Numeric(I32TruncSF32), Numeric(I32TruncUF32),
            Numeric(I32TruncSF64), Numeric(I32TruncUF64),
            Numeric(I64ExtendSI32), Numeric(I64ExtendUI32),
            Numeric(I64TruncSF32), Numeric(I64TruncUF32),
            Numeric(I64TruncSF64), Numeric(I64TruncUF64),
            Numeric(F32ConvertSI32), Numeric(F32ConvertUI32),
            Numeric(F32ConvertSI64), Numeric(F32ConvertUI64),
            Numeric(F32DemoteF64),
            Numeric(F64ConvertSI32), Numeric(F64ConvertUI32),
            Numeric(F64ConvertSI64), Numeric(F64ConvertUI64),
            Numeric(F64PromoteF32),
            Numeric(I32ReinterpretF32),
            Numeric(I64ReinterpretF64),
            Numeric(F32ReinterpretI32),
            Numeric(F64ReinterpretI64),

            // Binary
            Numeric(I32Eq), Numeric(I32Ne), Numeric(I32LtS), Numeric(I32LtU), Numeric(I32GtS), Numeric(I32GtU), Numeric(I32LeS), Numeric(I32LeU), Numeric(I32GeS), Numeric(I32GeU),
            Numeric(I64Eq), Numeric(I64Ne), Numeric(I64LtS), Numeric(I64LtU), Numeric(I64GtS), Numeric(I64GtU), Numeric(I64LeS), Numeric(I64LeU), Numeric(I64GeS), Numeric(I64GeU),
            Numeric(F32Eq), Numeric(F32Ne), Numeric(F32Lt), Numeric(F32Gt), Numeric(F32Le), Numeric(F32Ge),
            Numeric(F64Eq), Numeric(F64Ne), Numeric(F64Lt), Numeric(F64Gt), Numeric(F64Le), Numeric(F64Ge),
            Numeric(I32Add), Numeric(I32Sub), Numeric(I32Mul), Numeric(I32DivS), Numeric(I32DivU), Numeric(I32RemS), Numeric(I32RemU), Numeric(I32And), Numeric(I32Or), Numeric(I32Xor), Numeric(I32Shl), Numeric(I32ShrS), Numeric(I32ShrU), Numeric(I32Rotl), Numeric(I32Rotr),
            Numeric(I64Add), Numeric(I64Sub), Numeric(I64Mul), Numeric(I64DivS), Numeric(I64DivU), Numeric(I64RemS), Numeric(I64RemU), Numeric(I64And), Numeric(I64Or), Numeric(I64Xor), Numeric(I64Shl), Numeric(I64ShrS), Numeric(I64ShrU), Numeric(I64Rotl), Numeric(I64Rotr),
            Numeric(F32Add), Numeric(F32Sub), Numeric(F32Mul), Numeric(F32Div), Numeric(F32Min), Numeric(F32Max), Numeric(F32Copysign),
            Numeric(F64Add), Numeric(F64Sub), Numeric(F64Mul), Numeric(F64Div), Numeric(F64Min), Numeric(F64Max), Numeric(F64Copysign),

            // Memory
            Load(I32Load, Memarg::default()), Load(I32Load8S, Memarg::default()), Load(I32Load8U, Memarg::default()), Load(I32Load16S, Memarg::default()), Load(I32Load16U, Memarg::default()),
            Load(I64Load, Memarg::default()), Load(I64Load8S, Memarg::default()), Load(I64Load8U, Memarg::default()), Load(I64Load16S, Memarg::default()), Load(I64Load16U, Memarg::default()), Load(I64Load32S, Memarg::default()), Load(I64Load32U, Memarg::default()),
            Load(F32Load, Memarg::default()),
            Load(F64Load, Memarg::default()),
            Store(I32Store, Memarg::default()), Store(I32Store8, Memarg::default()), Store(I32Store16, Memarg::default()),
            Store(I64Store, Memarg::default()), Store(I64Store8, Memarg::default()), Store(I64Store16, Memarg::default()), Store(I64Store32, Memarg::default()),
            Store(F32Store, Memarg::default()),
            Store(F64Store, Memarg::default()),
        ].into_iter()
            .map(|i| add_hook_from_instr(module, i, &mut on_demand_hooks))
            .collect();

        move |instr: &Instr| -> Instr {
            Call(*monomorphic_hooks
                .get(instr.to_name())
                .expect(&format!("no hook was added for instruction {}", instr.to_name())))
        }
    };

    // add global for start, set to false on the first execution of the start function
    let start_not_executed_global = {
        module.globals.push(Global {
            type_: GlobalType(I32, Mutability::Mut),
            init: Some(vec![Const(Val::I32(1)), End]),
            import: None,
            export: None,
        });
        module.globals.len() - 1
    };

    /* add call to hooks: setup code that copies the returned value, instruction location, call */
    // NOTE we do not need to filter out functions since all hooks are imports and thus won't have
    // Code to instrument anyway...
    for (fidx, function) in module.functions() {
        // only instrument non-imported functions
        if function.code.is_none() {
            continue;
        }

        // move body out of function, so that function is not borrowed during iteration over the original body
        let original_body = {
            let dummy_body = Vec::new();
            ::std::mem::replace(&mut function.code.as_mut().unwrap().body, dummy_body)
        };

        // allocate new instrumented body (i.e., do not modify in-place), since there are too many insertions anyway
        // there are at least 3 new instructions per original one (2 const for location + 1 hook call)
        let mut instrumented_body = Vec::with_capacity(4 * original_body.len());

        // for branch target resolution (i.e., relative labels -> instruction locations)
        let mut block_stack = BlockStack::new(&original_body);
        // for drop/select monomorphization (cannot read their argument types of the opcodes directly)
        let mut type_stack = TypeStack::new();

        // execute start hook before anything else (if this is the start function and it hasn't run yet)
        if module_info.start == Some(fidx) {
            instrumented_body.extend_from_slice(&[
                Global(GetGlobal, start_not_executed_global.into()),
                If(BlockType(None)),
                Const(Val::I32(0)),
                Global(SetGlobal, start_not_executed_global.into()),
                fidx.into(),
                Const(Val::I32(-1)),
                Call(start_hook),
                End,
            ]);
        }

        // add function_begin hook...
        instrumented_body.extend_from_slice(&[
            fidx.into(),
            // ...which does not correspond to any instruction, so take -1 as instruction index
            Const(Val::I32(-1)),
            Call(begin_function_hook)
        ]);

        // check for implicit return now, since body gets consumed below
        let implicit_return = !original_body.ends_with(&[Return, End]);

        for (iidx, instr) in original_body.into_iter().enumerate() {
            let iidx: Idx<Instr> = iidx.into();
            let location = (fidx.into(), iidx.into());
            match instr {
                // size optimization: replace nop fully with hook
                Nop => instrumented_body.extend_from_slice(&[
                    location.0,
                    location.1,
                    Call(nop_hook)
                ]),
                // hook must come before unreachable instruction, otherwise it prevents hook from being called
                Unreachable => instrumented_body.extend_from_slice(&[
                    location.0,
                    location.1,
                    Call(unreachable_hook),
                    instr
                ]),


                /* Control Instructions: Blocks */

                Block(block_ty) => {
                    block_stack.begin_block(iidx);
                    type_stack.begin(block_ty);

                    instrumented_body.extend_from_slice(&[
                        instr,
                        location.0,
                        location.1,
                        Call(begin_block_hook),
                    ]);
                }
                Loop(block_ty) => {
                    block_stack.begin_loop(iidx);
                    type_stack.begin(block_ty);

                    instrumented_body.extend_from_slice(&[
                        instr,
                        location.0,
                        location.1,
                        Call(begin_loop_hook),
                    ]);
                }
                If(block_ty) => {
                    block_stack.begin_if(iidx);
                    type_stack.begin(block_ty);

                    let condition_tmp = function.add_fresh_local(I32);

                    instrumented_body.extend_from_slice(&[
                        // if_ hook for the condition (always executed on either branch)
                        Local(TeeLocal, condition_tmp),
                        location.0.clone(),
                        location.1.clone(),
                        Local(GetLocal, condition_tmp),
                        Call(if_hook),
                        // actual if block start
                        instr,
                        // begin hook (not executed when condition implies else branch)
                        location.0,
                        location.1,
                        Call(begin_if_hook),
                    ]);
                }
                Else => {
                    let if_begin = block_stack.else_();
                    type_stack.else_();

                    instrumented_body.extend_from_slice(&[
                        location.0.clone(),
                        location.1.clone(),
                        if_begin.into(),
                        Call(end_if_hook),
                        instr,
                        location.0,
                        location.1,
                        if_begin.into(),
                        Call(begin_else_hook),
                    ]);
                }
                End => {
                    let block = block_stack.end();
                    type_stack.end();

                    instrumented_body.extend_from_slice(&[
                        location.0,
                        location.1,
                    ]);
                    instrumented_body.append(&mut match block {
                        BlockStackElement::Function { .. } =>
                            vec![Call(end_function_hook)],
                        BlockStackElement::Block { begin, .. } =>
                            vec![begin.into(), Call(end_block_hook)],
                        BlockStackElement::Loop { begin, .. } =>
                            vec![begin.into(), Call(end_loop_hook)],
                        BlockStackElement::If { begin_if, .. } =>
                            vec![begin_if.into(), Call(end_if_hook)],
                        BlockStackElement::Else { begin_if, begin_else, .. } =>
                            vec![begin_if.into(), begin_else.into(), Call(end_else_hook)]
                    });
                    instrumented_body.push(instr);
                }


                /* Control Instructions: Branches/Breaks */
                // NOTE hooks must come before instr

                Br(target_label) => instrumented_body.extend_from_slice(&[
                    location.0,
                    location.1,
                    target_label.into(),
                    block_stack.br_target(target_label).into(),
                    Call(br_hook),
                    instr
                ]),
                BrIf(target_label) => {
                    type_stack.instr(&InstrType::new(&[I32], &[]));

                    let condition_tmp = function.add_fresh_local(I32);

                    instrumented_body.extend_from_slice(&[
                        Local(TeeLocal, condition_tmp),
                        location.0,
                        location.1,
                        target_label.into(),
                        block_stack.br_target(target_label).into(),
                        Local(GetLocal, condition_tmp),
                        Call(br_if_hook),
                        instr
                    ]);
                }
                BrTable(ref target_table, default_target) => {
                    type_stack.instr(&InstrType::new(&[I32], &[]));

                    // each br_table instruction gets its own entry in the static info object
                    // that maps table index to label and location
                    module_info.br_tables.push(BrTableInfo::from_br_table(target_table, default_target, &block_stack, fidx));

                    let target_idx_tmp = function.add_fresh_local(I32);

                    instrumented_body.extend_from_slice(&[
                        Local(TeeLocal, target_idx_tmp),
                        location.0,
                        location.1,
                        Const(Val::I32((module_info.br_tables.len() - 1) as i32)),
                        Local(GetLocal, target_idx_tmp),
                        Call(br_table_hook),
                        instr.clone()
                    ]);
                }


                /* Control Instructions: Calls & Returns */

                Return => {
                    type_stack.instr(&InstrType::new(&[], &function.type_.results));

                    let result_tys = function.type_.results.clone();
                    let result_tmps = function.add_fresh_locals(&result_tys);

                    instrumented_body.append(&mut save_stack_to_locals(&result_tmps));
                    instrumented_body.extend_from_slice(&[
                        location.0,
                        location.1,
                    ]);
                    instrumented_body.append(&mut restore_locals_with_i64_handling(&result_tmps, &function));
                    instrumented_body.extend_from_slice(&[
                        polymorphic_hooks.get_call(&instr, result_tys),
                        instr,
                    ]);
                }
                Call(target_func_idx) => {
                    let ref func_ty = module_info.functions[target_func_idx.0].type_;
                    type_stack.instr(&func_ty.into());

                    /* pre call hook */

                    let arg_tmps = function.add_fresh_locals(&func_ty.params);

                    instrumented_body.append(&mut save_stack_to_locals(&arg_tmps));
                    instrumented_body.extend_from_slice(&[
                        location.0.clone(),
                        location.1.clone(),
                        target_func_idx.into(),
                    ]);
                    instrumented_body.append(&mut restore_locals_with_i64_handling(&arg_tmps, &function));
                    instrumented_body.extend_from_slice(&[
                        polymorphic_hooks.get_call(&instr, func_ty.params.clone()),
                        instr,
                    ]);

                    /* post call hook */

                    let result_tmps = function.add_fresh_locals(&func_ty.results);

                    instrumented_body.append(&mut save_stack_to_locals(&result_tmps));
                    instrumented_body.extend_from_slice(&[
                        location.0,
                        location.1,
                    ]);
                    instrumented_body.append(&mut restore_locals_with_i64_handling(&result_tmps, &function));
                    instrumented_body.push(Call(*call_post_hooks.get(func_ty.results.as_slice()).expect("no call_post hook for tys")));
                }
                CallIndirect(ref func_ty, _ /* table idx == 0 in WASM version 1 */) => {
                    type_stack.instr(&func_ty.into());

                    /* pre call hook */

                    let target_table_idx_tmp = function.add_fresh_local(I32);
                    let arg_tmps = function.add_fresh_locals(&func_ty.params);

                    instrumented_body.push(Local(SetLocal, target_table_idx_tmp));
                    instrumented_body.append(&mut save_stack_to_locals(&arg_tmps));
                    instrumented_body.extend_from_slice(&[
                        Local(GetLocal, target_table_idx_tmp),
                        location.0.clone(),
                        location.1.clone(),
                        Local(GetLocal, target_table_idx_tmp),
                    ]);
                    instrumented_body.append(&mut restore_locals_with_i64_handling(&arg_tmps, &function));
                    instrumented_body.extend_from_slice(&[
                        polymorphic_hooks.get_call(&instr, func_ty.params.clone()),
                        instr.clone(),
                    ]);

                    /* post call hook */

                    let result_tmps = function.add_fresh_locals(&func_ty.results);

                    instrumented_body.append(&mut save_stack_to_locals(&result_tmps));
                    instrumented_body.extend_from_slice(&[
                        location.0,
                        location.1,
                    ]);
                    instrumented_body.append(&mut restore_locals_with_i64_handling(&result_tmps, &function));
                    instrumented_body.push(Call(*call_post_hooks.get(func_ty.results.as_slice()).expect("no call_post hook for tys")));
                }


                /* Parametric Instructions */

                Drop => {
                    let ty = type_stack.pop_val();

                    let tmp = function.add_fresh_local(ty);

                    instrumented_body.extend_from_slice(&[
                        Local(SetLocal, tmp),
                        location.0,
                        location.1,
                    ]);
                    instrumented_body.append(&mut convert_i64_instr(Local(GetLocal, tmp), ty));
                    instrumented_body.push(polymorphic_hooks.get_call(&instr, vec![ty]));
                }
                Select => {
                    assert_eq!(type_stack.pop_val(), I32, "select condition should be i32");
                    let ty = type_stack.pop_val();
                    assert_eq!(type_stack.pop_val(), ty, "select arguments should have same type");
                    type_stack.push_val(ty);

                    let condition_tmp = function.add_fresh_local(I32);
                    let arg_tmps = function.add_fresh_locals(&[ty, ty]);

                    instrumented_body.append(&mut save_stack_to_locals(&[arg_tmps[0], arg_tmps[1], condition_tmp]));
                    instrumented_body.extend_from_slice(&[
                        instr.clone(),
                        location.0,
                        location.1,
                        Local(GetLocal, condition_tmp),
                    ]);
                    instrumented_body.append(&mut restore_locals_with_i64_handling(&arg_tmps, &function));
                    instrumented_body.push(polymorphic_hooks.get_call(&instr, vec![ty, ty]));
                }


                /* Variable Instructions */

                Local(op, local_idx) => {
                    let local_ty = function.local_type(local_idx);

                    type_stack.instr(&op.to_type(local_ty));

                    instrumented_body.extend_from_slice(&[
                        instr.clone(),
                        location.0,
                        location.1,
                        local_idx.into(),
                    ]);
                    instrumented_body.append(&mut convert_i64_instr(Local(GetLocal, local_idx), local_ty));
                    instrumented_body.push(polymorphic_hooks.get_call(&instr, vec![local_ty]));
                }
                Global(op, global_idx) => {
                    let global_ty = module_info.globals[global_idx.0];

                    type_stack.instr(&op.to_type(global_ty));

                    instrumented_body.extend_from_slice(&[
                        instr.clone(),
                        location.0,
                        location.1,
                        global_idx.into(),
                    ]);
                    instrumented_body.append(&mut convert_i64_instr(Global(GetGlobal, global_idx), global_ty));
                    instrumented_body.push(polymorphic_hooks.get_call(&instr, vec![global_ty]));
                }


                /* Memory Instructions */

                MemorySize(_ /* memory idx == 0 in WASM version 1 */) => {
                    type_stack.instr(&instr.to_type().unwrap());

                    instrumented_body.extend_from_slice(&[
                        instr.clone(),
                        location.0,
                        location.1,
                        // optimization: just call memory_size again instead of duplicating result into local
                        instr,
                        Call(memory_size_hook)
                    ]);
                }
                MemoryGrow(_ /* memory idx == 0 in WASM version 1 */) => {
                    type_stack.instr(&instr.to_type().unwrap());

                    let input_tmp = function.add_fresh_local(I32);
                    let result_tmp = function.add_fresh_local(I32);

                    instrumented_body.extend_from_slice(&[
                        Local(TeeLocal, input_tmp),
                        instr,
                        Local(TeeLocal, result_tmp),
                        location.0,
                        location.1,
                        Local(GetLocal, input_tmp),
                        Local(GetLocal, result_tmp),
                        Call(memory_grow_hook)
                    ]);
                }

                // rest are "grouped instructions", i.e., where many instructions can be handled in a similar manner
                Load(op, memarg) => {
                    let ty = op.to_type();
                    type_stack.instr(&ty);

                    let addr_tmp = function.add_fresh_local(I32);
                    let value_tmp = function.add_fresh_local(ty.results[0]);

                    instrumented_body.extend_from_slice(&[
                        Local(TeeLocal, addr_tmp),
                        instr.clone(),
                        Local(TeeLocal, value_tmp),
                        location.0,
                        location.1,
                        Const(Val::I32(memarg.offset as i32)),
                        Const(Val::I32(memarg.alignment as i32)),
                    ]);
                    instrumented_body.append(&mut restore_locals_with_i64_handling(&[addr_tmp, value_tmp], &function));
                    instrumented_body.push(monomorphic_hook_call(&instr));
                }
                Store(op, memarg) => {
                    let ty = op.to_type();
                    type_stack.instr(&ty);

                    let addr_tmp = function.add_fresh_local(I32);
                    let value_tmp = function.add_fresh_local(ty.inputs[0]);

                    instrumented_body.append(&mut save_stack_to_locals(&[addr_tmp, value_tmp]));
                    instrumented_body.extend_from_slice(&[
                        instr.clone(),
                        location.0,
                        location.1,
                        Const(Val::I32(memarg.offset as i32)),
                        Const(Val::I32(memarg.alignment as i32)),
                    ]);
                    instrumented_body.append(&mut restore_locals_with_i64_handling(&[addr_tmp, value_tmp], &function));
                    instrumented_body.push(monomorphic_hook_call(&instr));
                }


                /* Numeric Instructions */

                Const(val) => {
                    type_stack.instr(&instr.to_type().unwrap());

                    instrumented_body.extend_from_slice(&[
                        instr.clone(),
                        location.0,
                        location.1,
                    ]);
                    // optimization: just call T.const again, instead of duplicating result into local
                    instrumented_body.append(&mut convert_i64_instr(instr.clone(), val.to_type()));
                    instrumented_body.push(monomorphic_hook_call(&instr));
                }
                Numeric(op) => {
                    let ty = op.to_type();
                    type_stack.instr(&ty);

                    let input_tmps = function.add_fresh_locals(&ty.inputs);
                    let result_tmps = function.add_fresh_locals(&ty.results);

                    instrumented_body.append(&mut save_stack_to_locals(&input_tmps));
                    instrumented_body.push(instr.clone());
                    instrumented_body.append(&mut save_stack_to_locals(&result_tmps));
                    instrumented_body.extend_from_slice(&[
                        location.0,
                        location.1,
                    ]);
                    instrumented_body.append(&mut restore_locals_with_i64_handling(
                        &[input_tmps, result_tmps].concat(),
                        &function));
                    instrumented_body.push(monomorphic_hook_call(&instr));
                }
            }
        }

        // add return hook, if function has an implicit return
        // (can be distinguished from actual returns in analysis because of -1 as instr location)
        if implicit_return {
            let result_tys = function.type_.results.clone();
            let result_tmps = function.add_fresh_locals(&result_tys);

            assert_eq!(instrumented_body.pop(), Some(End));
            instrumented_body.append(&mut save_stack_to_locals(&result_tmps));
            instrumented_body.extend_from_slice(&[
                fidx.into(),
                Const(Val::I32(-1)),
            ]);
            instrumented_body.append(&mut restore_locals_with_i64_handling(&result_tmps, &function));
            instrumented_body.extend_from_slice(&[
                polymorphic_hooks.get_call(&Return, result_tys),
                End,
            ]);
        }

        // finally, switch dummy body out against instrumented body
        ::std::mem::replace(&mut function.code.as_mut().unwrap().body, instrumented_body);
    }

    Some(js_codegen(module_info, &on_demand_hooks))
}

fn add_hook(module: &mut Module, name: impl Into<String>, arg_tys_: &[ValType]) -> Idx<Function> {
    // prepend two I32 for (function idx, instr idx)
    let mut arg_tys = vec![I32, I32];
    arg_tys.extend(arg_tys_.iter()
        // and expand i64 to a tuple of (i32, i32) since there is no JS interop for i64
        .flat_map(convert_i64_type));

    module.add_function_import(
        // hooks do not return anything
        FunctionType::new(arg_tys, vec![]),
        "wasabi_hooks".into(),
        name.into())
}

// TODO put this in the MonomorphicHookMap.add() function instead
/// specialized version form of the above for monomorphic instructions
fn add_hook_from_instr(module: &mut Module, instr: &Instr, hooks: &mut Vec<String>) -> (&'static str, Idx<Function>) {
    hooks.push(instr.to_js_hook());
    (instr.to_name(), add_hook(module, instr.to_name(), &match instr {
        Const(val) => vec![val.to_type()],
        Numeric(op) => {
            let ty = op.to_type();
            [ty.inputs, ty.results].concat().into()
        }
        // for address, offset and alignment
        Load(op, _) => vec![I32, I32, I32, op.to_type().results[0]],
        Store(op, _) => vec![I32, I32, I32, op.to_type().inputs[0]],
        _ => unreachable!("function should be only called for \"grouped\" instructions"),
    }))
}

struct PolymorphicHookMap(HashMap<(&'static str, Vec<ValType>), Idx<Function>>);

impl PolymorphicHookMap {
    pub fn new() -> Self {
        PolymorphicHookMap(HashMap::new())
    }
    pub fn add(&mut self, module: &mut Module, instr: Instr, non_poly_args: &[ValType], tys: &[Vec<ValType>], hooks: &mut Vec<String>) {
        for tys in tys {
            hooks.push(instr.to_poly_js_hook(tys.as_slice()));
            let hook_name = append_mangled_tys(instr.to_name().to_string(), tys.as_slice());
            let hook_idx = add_hook(module, hook_name, &[non_poly_args, tys.as_slice()].concat());
            self.0.insert(
                (instr.to_name(), tys.clone()),
                hook_idx);
        }
    }
    pub fn get_call(&self, instr: &Instr, tys: Vec<ValType>) -> Instr {
        let error = format!("no hook was added for {} with types {:?}", instr.to_name(), tys);
        Call(*self.0
            .get(&(instr.to_name(), tys))
            .expect(&error))
    }
}

/// helper function to save top locals.len() values into locals with the given index
/// types of locals must match stack, not enforced by this function!
fn save_stack_to_locals(locals: &[Idx<ast::Local>]) -> Vec<Instr> {
    let mut instrs = Vec::new();
    // copy stack values into locals
    for &local in locals.iter().skip(1).rev() {
        instrs.push(Local(SetLocal, local));
    }
    // optimization: for first local on the stack / last one saved use tee_local instead of set_local + get_local
    for &local in locals.iter().next() {
        instrs.push(Local(TeeLocal, local));
    }
    // and restore (saving has removed them from the stack)
    for &local in locals.iter().skip(1) {
        instrs.push(Local(GetLocal, local));
    }
    return instrs;
}

/// function is necessary to get the types of the locals
fn restore_locals_with_i64_handling(locals: &[Idx<ast::Local>], function: &Function) -> Vec<Instr> {
    let mut instrs = Vec::new();
    for &local in locals {
        instrs.append(&mut convert_i64_instr(Local(GetLocal, local), function.local_type(local)));
    }
    return instrs;
}

/// convenience to hand (function/instr/local/global) indices to hooks
impl<T> Into<Instr> for Idx<T> {
    fn into(self) -> Instr {
        Const(Val::I32(self.0 as i32))
    }
}
