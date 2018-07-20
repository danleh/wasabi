use config::{EnabledHooks, HighLevelHook};
use self::block_stack::{BlockStack, BlockStackElement};
use self::convert_i64::convert_i64_instr;
use self::duplicate_stack::*;
use self::hook_map::HookMap;
use self::static_info::*;
use self::type_stack::TypeStack;
use serde_json;
use wasm::ast::{BlockType, Idx, InstrType, Mutability, Val, ValType::*};
use wasm::ast::highlevel::{Function, GlobalOp::*, Instr, Instr::*, LocalOp::*, Module};

mod convert_i64;
mod static_info;
mod block_stack;
mod type_stack;
mod hook_map;
mod duplicate_stack;

/// instruments every instruction in Jalangi-style with a callback that takes inputs, outputs, and
/// other relevant information.
pub fn add_hooks(module: &mut Module, enabled_hooks: &EnabledHooks) -> Option<String> {
    // make sure table is exported, needed for Wasabi runtime to resolve table indices to function indices.
    for table in &mut module.tables {
        if table.export.is_empty() {
            table.export.push("__wasabi_table".into());
        }
    }

    // NOTE must be after exporting table and function, so that their export names are in the static info object
    let mut module_info: ModuleInfo = (&*module).into();
    let mut hooks = HookMap::new(&module);

    // add global for start, set to false on the first execution of the start function
    let start_not_executed_global = module.add_global(I32, Mutability::Mut, vec![Const(Val::I32(1)), End]);

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
        // for drop/select monomorphization (cannot determine their input types only from instruction, but need this additional type information)
        let mut type_stack = TypeStack::new();

        // execute start hook before anything else
        if module_info.start == Some(fidx)
            && enabled_hooks.is_enabled(HighLevelHook::Start) {
            instrumented_body.extend_from_slice(&[
                Global(GetGlobal, start_not_executed_global),
                // ...(if this is the start function and it hasn't run yet)
                If(BlockType(None)),
                Const(Val::I32(0)),
                Global(SetGlobal, start_not_executed_global),
                fidx.to_const(),
                Const(Val::I32(-1)),
                hooks.start(),
                End,
            ]);
        }

        // function_begin hook
        if enabled_hooks.is_enabled(HighLevelHook::Begin) {
            instrumented_body.extend_from_slice(&[
                fidx.to_const(),
                // function begin does not correspond to any instruction, so take -1 as instruction index
                Const(Val::I32(-1)),
                hooks.begin_function()
            ]);
        }

        // remember implicit return for instrumentation: add "synthetic" return hook call to last end
        let implicit_return = !original_body.ends_with(&[Return, End]);

        for (iidx, instr) in original_body.into_iter().enumerate() {
            let iidx: Idx<Instr> = iidx.into();
            let location = (fidx.to_const(), iidx.to_const());

            /*
             * add calls to hooks, typical instructions inserted for (not necessarily in this order if that saves us a local or so):
             * 1. duplicate instruction inputs via temporary locals
             * 2. call original instruction (except for in a few cases, where the hook is inserted before)
             * 3. duplicate instruction results via temporary locals
             * 4. push instruction location (function + instr index)
             * 5. call hook
             */
            match instr {
                Nop => if enabled_hooks.is_enabled(HighLevelHook::Nop) {
                    // size optimization: replace nop fully with hook
                    instrumented_body.extend_from_slice(&[
                        location.0,
                        location.1,
                        hooks.instr(&instr, &[])
                    ])
                },
                Unreachable => {
                    // hook must come before unreachable instruction, otherwise it prevents hook from being called
                    if enabled_hooks.is_enabled(HighLevelHook::Unreachable) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            hooks.instr(&instr, &[]),
                        ])
                    }

                    instrumented_body.push(instr)
                }


                /* Control Instructions: Blocks */

                Block(block_ty) => {
                    block_stack.begin_block(iidx);
                    type_stack.begin(block_ty);

                    instrumented_body.push(instr);

                    if enabled_hooks.is_enabled(HighLevelHook::Begin) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            hooks.begin_block(),
                        ])
                    }
                }
                Loop(block_ty) => {
                    block_stack.begin_loop(iidx);
                    type_stack.begin(block_ty);

                    instrumented_body.push(instr);

                    if enabled_hooks.is_enabled(HighLevelHook::Begin) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            hooks.begin_loop(),
                        ])
                    }
                }
                If(block_ty) => {
                    block_stack.begin_if(iidx);
                    type_stack.instr(&InstrType::new(&[I32], &[]));
                    type_stack.begin(block_ty);

                    // if_ hook for the condition (always executed on either branch)
                    if enabled_hooks.is_enabled(HighLevelHook::If) {
                        let condition_tmp = function.add_fresh_local(I32);

                        instrumented_body.extend_from_slice(&[
                            Local(TeeLocal, condition_tmp),
                            location.0.clone(),
                            location.1.clone(),
                            Local(GetLocal, condition_tmp),
                            hooks.instr(&instr, &[])
                        ]);
                    }

                    // actual if block start
                    instrumented_body.push(instr);

                    // begin hook (not executed when condition implies else branch)
                    if enabled_hooks.is_enabled(HighLevelHook::Begin) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            hooks.begin_if()
                        ]);
                    }
                }
                Else => {
                    let if_block = block_stack.else_();
                    let begin_if = if let BlockStackElement::If { begin_if, .. } = if_block {
                        begin_if
                    } else {
                        unreachable!()
                    };

                    type_stack.else_();

                    if enabled_hooks.is_enabled(HighLevelHook::End) {
                        instrumented_body.extend_from_slice(&[
                            location.0.clone(),
                            location.1.clone(),
                            begin_if.to_const(),
                            hooks.end(&if_block),
                        ]);
                    }

                    instrumented_body.push(instr);

                    if enabled_hooks.is_enabled(HighLevelHook::Begin) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            begin_if.to_const(),
                            hooks.begin_else(),
                        ])
                    }
                }
                End => {
                    let block = block_stack.end();
                    assert_eq!(iidx, block.end());
                    type_stack.end();

                    // add "synthetic" return hook call for implicit returns
                    if implicit_return
                        && enabled_hooks.is_enabled(HighLevelHook::Return) {
                        if let BlockStackElement::Function { .. } = block {
                            let result_tys = &function.type_.results.clone();
                            let result_tmps = function.add_fresh_locals(result_tys);

                            instrumented_body.append(&mut save_stack_to_locals(&result_tmps));
                            instrumented_body.extend_from_slice(&[
                                location.0,
                                Const(Val::I32(-1)),
                            ]);
                            instrumented_body.append(&mut restore_locals_with_i64_handling(&result_tmps, &function));
                            instrumented_body.push(hooks.instr(&Return, result_tys));
                        }
                    }

                    // NOTE there is not duplication of the end hook call for explicit returns,
                    // because the end hook that is inserted now is never called (dead code)

                    if enabled_hooks.is_enabled(HighLevelHook::End) {
                        instrumented_body.append(&mut block.to_end_hook_args(fidx));
                        instrumented_body.push(hooks.end(&block))
                    }

                    instrumented_body.push(instr)
                }


                /* Control Instructions: Branches/Breaks */
                // NOTE hooks must come before instr

                Br(target_label) => {
                    let br_target = block_stack.br_target(target_label);

                    // br hook
                    if enabled_hooks.is_enabled(HighLevelHook::Br) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            target_label.to_const(),
                            br_target.absolute_instr.to_const(),
                            hooks.instr(&instr, &[])
                        ])
                    }

                    // end hooks for all intermediate blocks that are "jumped over"
                    if enabled_hooks.is_enabled(HighLevelHook::End) {
                        for block in br_target.ended_blocks {
                            instrumented_body.append(&mut block.to_end_hook_args(fidx));
                            instrumented_body.push(hooks.end(&block));
                        }
                    }

                    instrumented_body.push(instr)
                }
                BrIf(target_label) => {
                    type_stack.instr(&InstrType::new(&[I32], &[]));

                    let br_target = block_stack.br_target(target_label);

                    if enabled_hooks.is_enabled(HighLevelHook::BrIf)
                        || enabled_hooks.is_enabled(HighLevelHook::End) {

                        // saved condition local is needed by _both_ hooks
                        let condition_tmp = function.add_fresh_local(I32);
                        instrumented_body.push(Local(TeeLocal, condition_tmp));

                        // br_if hook
                        if enabled_hooks.is_enabled(HighLevelHook::BrIf) {
                            instrumented_body.extend_from_slice(&[
                                // NOTE see tee_local above
                                location.0.clone(),
                                location.1.clone(),
                                Local(GetLocal, condition_tmp),
                                target_label.to_const(),
                                br_target.absolute_instr.to_const(),
                                hooks.instr(&instr, &[])
                            ]);
                        }

                        // end hooks for all intermediate blocks that are "jumped over"
                        if enabled_hooks.is_enabled(HighLevelHook::End) {
                            // call hooks only iff condition is true (-> insert artificial if block)
                            instrumented_body.extend_from_slice(&[
                                // NOTE see tee_local above
                                Local(GetLocal, condition_tmp),
                                If(BlockType(None)),
                            ]);
                            for block in br_target.ended_blocks {
                                instrumented_body.append(&mut block.to_end_hook_args(fidx));
                                instrumented_body.push(hooks.end(&block));
                            }
                            // of the artificially inserted if block before
                            instrumented_body.push(End);
                        }
                    }

                    instrumented_body.push(instr)
                }
                BrTable(ref target_table, default_target) => {
                    type_stack.instr(&InstrType::new(&[I32], &[]));

                    // each br_table instruction gets its own entry in the static info object
                    // that maps table index to label and location
                    module_info.br_tables.push(BrTableInfo::from_br_table(target_table, default_target, &block_stack, fidx));

                    if enabled_hooks.is_enabled(HighLevelHook::BrTable)
                        // because end hooks are called at runtime, we need to instrument even if br_table is not enabled
                        || enabled_hooks.is_enabled(HighLevelHook::End) {

                        // NOTE calling the end() hooks for the intermediate blocks is done at runtime
                        // by the br_table low-level hook

                        let target_idx_tmp = function.add_fresh_local(I32);

                        instrumented_body.extend_from_slice(&[
                            Local(TeeLocal, target_idx_tmp),
                            location.0,
                            location.1,
                            Local(GetLocal, target_idx_tmp),
                            Const(Val::I32((module_info.br_tables.len() - 1) as i32)),
                            hooks.instr(&instr, &[])
                        ])
                    }

                    instrumented_body.push(instr.clone())
                }


                /* Control Instructions: Calls & Returns */

                Return => {
                    type_stack.instr(&InstrType::new(&[], &function.type_.results));

                    // return hook
                    if enabled_hooks.is_enabled(HighLevelHook::Return) {
                        let result_tys = &function.type_.results.clone();
                        let result_tmps = function.add_fresh_locals(result_tys);

                        instrumented_body.append(&mut save_stack_to_locals(&result_tmps));
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                        ]);
                        instrumented_body.append(&mut restore_locals_with_i64_handling(&result_tmps, &function));
                        instrumented_body.push(hooks.instr(&instr, result_tys));
                    }

                    // end hooks for all intermediate blocks that are "jumped over"
                    if enabled_hooks.is_enabled(HighLevelHook::End) {
                        for block in block_stack.return_target().ended_blocks {
                            instrumented_body.append(&mut block.to_end_hook_args(fidx));
                            instrumented_body.push(hooks.end(&block));
                        }
                    }

                    instrumented_body.push(instr)
                }
                Call(target_func_idx) => {
                    let ref func_ty = module_info.functions[target_func_idx.0].type_;
                    type_stack.instr(&func_ty.into());

                    if enabled_hooks.is_enabled(HighLevelHook::Call) {
                        /* pre call hook */

                        let arg_tmps = function.add_fresh_locals(&func_ty.params);

                        instrumented_body.append(&mut save_stack_to_locals(&arg_tmps));
                        instrumented_body.extend_from_slice(&[
                            location.0.clone(),
                            location.1.clone(),
                            target_func_idx.to_const(),
                        ]);
                        instrumented_body.append(&mut restore_locals_with_i64_handling(&arg_tmps, &function));
                        instrumented_body.extend_from_slice(&[
                            hooks.instr(&instr, &func_ty.params),
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
                        instrumented_body.push(hooks.call_post(&func_ty.results))
                    } else {
                        instrumented_body.push(instr);
                    }
                }
                CallIndirect(ref func_ty, _ /* table idx == 0 in WASM version 1 */) => {
                    type_stack.instr(&instr.to_type().unwrap());

                    if enabled_hooks.is_enabled(HighLevelHook::Call) {
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
                            hooks.instr(&instr, &func_ty.params),
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
                        instrumented_body.push(hooks.call_post(&func_ty.results));
                    } else {
                        instrumented_body.push(instr.clone());
                    }
                }


                /* Parametric Instructions */

                Drop => {
                    let ty = type_stack.pop_val();

                    if enabled_hooks.is_enabled(HighLevelHook::Drop) {
                        let tmp = function.add_fresh_local(ty);

                        instrumented_body.extend_from_slice(&[
                            Local(SetLocal, tmp),
                            location.0,
                            location.1,
                        ]);
                        instrumented_body.append(&mut convert_i64_instr(Local(GetLocal, tmp), ty));
                        // replace drop with hook call
                        instrumented_body.push(hooks.instr(&instr, &[ty]));
                    } else {
                        instrumented_body.push(instr);
                    }
                }
                Select => {
                    assert_eq!(type_stack.pop_val(), I32, "select condition should be i32");
                    let ty = type_stack.pop_val();
                    assert_eq!(type_stack.pop_val(), ty, "select arguments should have same type");
                    type_stack.push_val(ty);

                    if enabled_hooks.is_enabled(HighLevelHook::Drop) {
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
                        // replace select with hook call
                        instrumented_body.push(hooks.instr(&instr, &[ty, ty]));
                    } else {
                        instrumented_body.push(instr);
                    }
                }


                /* Variable Instructions */

                Local(op, local_idx) => {
                    let local_ty = function.local_type(local_idx);

                    type_stack.instr(&op.to_type(local_ty));

                    instrumented_body.push(instr.clone());

                    // insert hook AFTER instruction, so that we can use get_local instead of duplicating the value through a new local
                    if enabled_hooks.is_enabled(HighLevelHook::Local) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            local_idx.to_const(),
                        ]);
                        instrumented_body.append(&mut convert_i64_instr(Local(GetLocal, local_idx), local_ty));
                        instrumented_body.push(hooks.instr(&instr, &[local_ty]));
                    }
                }
                Global(op, global_idx) => {
                    let global_ty = module_info.globals[global_idx.0];

                    type_stack.instr(&op.to_type(global_ty));

                    instrumented_body.push(instr.clone());

                    // insert hook AFTER instruction, so that we can use get_global instead of duplicating the value through a new local
                    if enabled_hooks.is_enabled(HighLevelHook::Global) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            global_idx.to_const(),
                        ]);
                        instrumented_body.append(&mut convert_i64_instr(Global(GetGlobal, global_idx), global_ty));
                        instrumented_body.push(hooks.instr(&instr, &[global_ty]));
                    }
                }


                /* Memory Instructions */

                MemorySize(_ /* memory idx == 0 in WASM version 1 */) => {
                    type_stack.instr(&instr.to_type().unwrap());

                    instrumented_body.push(instr.clone());

                    if enabled_hooks.is_enabled(HighLevelHook::MemorySize) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            // optimization: just call memory_size again instead of duplicating result into local
                            instr.clone(),
                            hooks.instr(&instr, &[])
                        ]);
                    }
                }
                MemoryGrow(_ /* memory idx == 0 in WASM version 1 */) => {
                    type_stack.instr(&instr.to_type().unwrap());

                    if enabled_hooks.is_enabled(HighLevelHook::MemoryGrow) {
                        let input_tmp = function.add_fresh_local(I32);
                        let result_tmp = function.add_fresh_local(I32);

                        instrumented_body.extend_from_slice(&[
                            Local(TeeLocal, input_tmp),
                            instr.clone(),
                            Local(TeeLocal, result_tmp),
                            location.0,
                            location.1,
                            Local(GetLocal, input_tmp),
                            Local(GetLocal, result_tmp),
                            hooks.instr(&instr, &[])
                        ]);
                    } else {
                        instrumented_body.push(instr);
                    }
                }

                /* rest are "grouped instructions", i.e., where many instructions can be handled in a similar manner */

                Load(op, memarg) => {
                    let ty = op.to_type();
                    type_stack.instr(&ty);

                    if enabled_hooks.is_enabled(HighLevelHook::Load) {
                        let addr_tmp = function.add_fresh_local(ty.inputs[0]);
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
                        instrumented_body.push(hooks.instr(&instr, &[]));
                    } else {
                        instrumented_body.push(instr);
                    }
                }
                Store(op, memarg) => {
                    let ty = op.to_type();
                    type_stack.instr(&ty);

                    if enabled_hooks.is_enabled(HighLevelHook::Store) {
                        let addr_tmp = function.add_fresh_local(ty.inputs[0]);
                        let value_tmp = function.add_fresh_local(ty.inputs[1]);

                        instrumented_body.append(&mut save_stack_to_locals(&[addr_tmp, value_tmp]));
                        instrumented_body.extend_from_slice(&[
                            instr.clone(),
                            location.0,
                            location.1,
                            Const(Val::I32(memarg.offset as i32)),
                            Const(Val::I32(memarg.alignment as i32)),
                        ]);
                        instrumented_body.append(&mut restore_locals_with_i64_handling(&[addr_tmp, value_tmp], &function));
                        instrumented_body.push(hooks.instr(&instr, &[]));
                    } else {
                        instrumented_body.push(instr);
                    }
                }


                /* Numeric Instructions */

                Const(val) => {
                    type_stack.instr(&instr.to_type().unwrap());

                    instrumented_body.push(instr.clone());

                    if enabled_hooks.is_enabled(HighLevelHook::Const) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                        ]);
                        // optimization: just call T.const again, instead of duplicating result into local
                        instrumented_body.append(&mut convert_i64_instr(instr.clone(), val.to_type()));
                        instrumented_body.push(hooks.instr(&instr, &[]));
                    }
                }
                Numeric(op) => {
                    let ty = op.to_type();
                    type_stack.instr(&ty);

                    if (enabled_hooks.is_enabled(HighLevelHook::Unary) && ty.inputs.len() == 1)
                        || (enabled_hooks.is_enabled(HighLevelHook::Binary) && ty.inputs.len() == 2) {
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
                        instrumented_body.push(hooks.instr(&instr, &[]));
                    } else {
                        instrumented_body.push(instr);
                    }
                }
            }
        }

        // finally, switch dummy body out against instrumented body
        ::std::mem::replace(&mut function.code.as_mut().unwrap().body, instrumented_body);
    }

    // actually add the hooks to module and check that inserted Idx is the one on the Hook struct
    let hooks = hooks.finish();
    let mut js_hooks = Vec::new();
    for hook in hooks {
        js_hooks.push(hook.js);
        assert_eq!(hook.idx, module.functions.len().into(), "have other functions been inserted into the module since starting collection of hooks?");
        module.functions.push(hook.wasm);
    }

    Some(generate_js(module_info, &js_hooks))
}

/// convenience to hand (function/instr/local/global) indices to hooks
/// must be trait since inherent impl is disallowed by orphan rules for non-crate types (Idx<T>)
trait ToConst {
    fn to_const(self) -> Instr;
}

impl<T> ToConst for Idx<T> {
    fn to_const(self) -> Instr {
        Const(Val::I32(self.0 as i32))
    }
}

impl BlockStackElement {
    fn to_end_hook_args(&self, fidx: Idx<Function>) -> Vec<Instr> {
        match self {
            | BlockStackElement::Function { end } => vec![fidx.to_const(), end.to_const()],
            | BlockStackElement::Block { begin, end }
            | BlockStackElement::Loop { begin, end }
            | BlockStackElement::If { begin_if: begin, end, .. } => vec![fidx.to_const(), end.to_const(), begin.to_const()],
            | BlockStackElement::Else { begin_else, begin_if, end } => vec![fidx.to_const(), end.to_const(), begin_else.to_const(), begin_if.to_const()]
        }
    }
    fn end(&self) -> Idx<Instr> {
        match self {
            | BlockStackElement::Function { end }
            | BlockStackElement::Block { end, .. }
            | BlockStackElement::Loop { end, .. }
            | BlockStackElement::If { end, .. }
            | BlockStackElement::Else { end, .. } => *end
        }
    }
}

fn generate_js(module_info: ModuleInfo, hooks: &[String]) -> String {
    // FIXME somewhat hacky: just cat together long.js dependency, program-independent, and
    // program-dependent JavaScript into one big file.
    // * Alternative A: use webpack or other bundler, drawbacks:
    //    - users need to install another tool
    //    - needs to be run after every instrumentation
    // * Alternative B: compile Wasabi itself to WebAssembly, instrument at runtime
    format!(r#"/*
 * Generated by Wasabi. DO NOT EDIT.
 * Contains:
 *   - independent of program-to-instrument: long.js dependency, Wasabi loader and runtime
 *   - generated from program-to-instrument: static information and low-level hooks
 */

// long.js
{}

{}

Wasabi.module.info = {};

Wasabi.module.lowlevelHooks = {{
    {}
}};
"#,
            include_str!("../../../lib/long.js/long.js").lines().next().unwrap(),
            include_str!("../../../lib/runtime.js"),
            serde_json::to_string(&module_info).unwrap(),
            hooks.iter().flat_map(|s| s.split("\n")).collect::<Vec<&str>>().join("\n    "))
}