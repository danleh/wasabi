use std::convert::TryInto;

use parking_lot::RwLock;
use rayon::prelude::*;
use serde_json;
use wasabi_wasm::Function;
use wasabi_wasm::FunctionType;
use wasabi_wasm::GlobalOp;
use wasabi_wasm::Idx;
use wasabi_wasm::Instr;
use wasabi_wasm::Instr::*;
use wasabi_wasm::Label;
use wasabi_wasm::LocalOp::*;
use wasabi_wasm::MemoryOp;
use wasabi_wasm::Module;
use wasabi_wasm::Mutability;
use wasabi_wasm::Val;
use wasabi_wasm::ValType::*;

use crate::options::Hook;
use crate::options::HookSet;

use self::block_stack::BlockStack;
use self::block_stack::BlockStackElement;
use self::convert_i64::convert_i64_instr;
use self::duplicate_stack::*;
use self::hook_map::HookMap;
use self::static_info::*;
use self::type_stack::TypeStack;

pub mod block_stack;
mod convert_i64;
mod duplicate_stack;
mod hook_map;
mod static_info;
pub mod type_stack;

/// Instruments every instruction in Jalangi-style with a callback that takes inputs, outputs, and
/// other relevant information.
#[allow(clippy::cognitive_complexity)]
pub fn add_hooks(
    module: &mut Module,
    enabled_hooks: HookSet,
    node_js: bool,
) -> Option<(String, usize)> {
    // make sure table is exported, needed for Wasabi runtime to resolve table indices to function indices.
    for table in &mut module.tables {
        if table.export.is_empty() {
            table.export.push("__wasabi_table".into());
        }
    }
    // FIXME is this a valid workaround for wrong Firefox exported function .name property?
    //    if let Some(function) = module.functions.first_mut() {
    //        if function.export.is_empty() {
    //            function.export.push("__wasabi_first_function".into());
    //        }
    //    }

    // NOTE must be after exporting table and function, so that their export names are in the static info object
    let module_info: ModuleInfo = (&*module).into();
    let module_info = RwLock::new(module_info);
    let hooks = HookMap::new(module);

    // add global for start, set to false on the first execution of the start function
    let start_not_executed_global = if enabled_hooks.contains(Hook::Start) {
        Some(module.add_global(I32, Mutability::Mut, vec![Const(Val::I32(1)), End]))
    } else {
        None
    };

    module.functions.par_iter_mut().enumerate().for_each(|(fidx, function): (usize, &mut Function)| {
        let fidx = fidx.into();
        // only instrument non-imported functions
        if function.code().is_none() {
            return;
        }

        // move body out of function, so that function is not borrowed during iteration over the original body
        let original_body = {
            let dummy_body = Vec::new();
            ::std::mem::replace(&mut function.code_mut().expect("internal error: function code should exist, see check above").body, dummy_body)
        };

        // allocate new instrumented body (i.e., do not modify in-place), since there are too many insertions anyway
        // there are at least 3 new instructions per original one (2 const for location + 1 hook call)
        // later increased to 6, since we saw a lot of re-allocations when analyzing Wasabi with heaptrack.
        let mut instrumented_body = Vec::with_capacity(6 * original_body.len());

        // for branch target resolution (i.e., relative labels -> instruction locations)
        let mut block_stack = BlockStack::new(&original_body);
        // for drop/select monomorphization (cannot determine their input types only from instruction, but need this additional type information)
        let mut type_stack = TypeStack::new();

        // execute start hook before anything else
        if module_info.read().start == Some(fidx)
            && enabled_hooks.contains(Hook::Start) {
            instrumented_body.extend_from_slice(&[
                Global(GlobalOp::Get, start_not_executed_global.unwrap()),
                // ...(if this is the start function and it hasn't run yet)
                If(FunctionType::empty()),
                Const(Val::I32(0)),
                Global(GlobalOp::Set, start_not_executed_global.unwrap()),
                fidx.to_const(),
                Const(Val::I32(-1)),
                hooks.start(),
                End,
            ]);
        }

        // function_begin hook
        if enabled_hooks.contains(Hook::Begin) {
            instrumented_body.extend_from_slice(&[
                fidx.to_const(),
                // function begin does not correspond to any instruction, so take -1 as instruction index
                Const(Val::I32(-1)),
                hooks.begin_function()
            ]);
        }

        // remember implicit return for instrumentation: add "synthetic" return hook call to last end
        let implicit_return = !original_body.ends_with(&[Return, End]);

        // WebAssembly's type rules are weird with unreachable code (i.e., code after an
        // "unreachable" instruction or after unconditional branches like return/br/br_table):
        // - Dead code needs to be type-correct for "existing types" on the stack. E.g,
        //   unreachable; i32.const 0; f32.abs
        //   is wrong because the second instruction pops a type-incompatible argument (f32 vs i32).
        // - But "missing types" on the stack are "synthesized" from nothing. E.g.,
        //   unreachable; f32.abs
        //   is type correct as per the spec because unreachable "produces" the f32 on the stack
        //   for the f32.abs instruction out of nothing.
        //
        // (I don't know if any real-world compiler produces unreachable code that exploits (2),
        // but the spec test suite certainly does.
        // Also, I have read somewhere that this is supposedly to make implementing compilers easier,
        // but to me it looks like it makes type checking more _complicated_!?
        // Since we can't change the spec now, I guess we are stuck with it though...)
        //
        // Unfortunately, Wasabi's type checking (in type_stack) is not powerful enough
        // to "produce" the missing types for unreachable code.
        // (TODO I think this needs some form of primitive unification. E.g., checking
        // [unreachable, f32] against [i32, f32, f32]
        // should "expand" or unify the type of the unreachable instruction to/with [i32, f32].)
        //
        // Since I don't want to introduce this kind of complexity, and because unreachable code
        // is by definition never executed, a workaround is to simply not instrument unreachable
        // code. Instead, I just copy it over unaltered. This should still be type-correct
        // because Wasabi's instrumentation always leaves the stack after an instruction + hook
        // the same as without instrumentation.
        //
        // Conceptually, when we encounter an "unreachable" instruction (or br/return/br_table),
        // we switch to "unreachable mode" and do not instrument. We stop the "unreachable mode"
        // (and start to properly instrument again) when we see an end or else (which closes the
        // current dead block).
        //
        // However, because unreachable code can itself contain more blocks, we must actually
        // count the depth for "how far we are in unreachable mode" and only stop once we reach 0.
        let mut unreachable_depth = 0;

        for (iidx, instr) in original_body.into_iter().enumerate() {

            // End or Else could end the current "unreachable" block.
            if unreachable_depth > 0 {
                match instr {
                    Else | End => unreachable_depth -= 1,
                    _ => {}
                };
            }
            // Still unreachable, even after closing the current block?
            if unreachable_depth > 0 {
                // 1. Copy over the instruction unaltered
                instrumented_body.push(instr.clone());
                // 2. If the unreachable code itself contains even deeper blocks, increase the "unreachable depth".
                match instr {
                    // NOTE Else can also open a "deeper" unreachable block, but only if we were unreachable to begin with.
                    Block(_) | Loop(_) | If(_) | Else => unreachable_depth += 1,
                    _ => {}
                };
                // 3. DO NOT instrument unreachable code, since type_stack will throw an exception on
                // instructions that pop types that are "magically produced" by unreachable code.
                continue;
            }

//            println!("{:?}:{:?}: {:?}", fidx.0, iidx, instr);

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
                Nop => if enabled_hooks.contains(Hook::Nop) {
                    // size optimization: replace nop fully with hook
                    instrumented_body.extend_from_slice(&[
                        location.0,
                        location.1,
                        hooks.instr(&instr, &[])
                    ])
                },
                Unreachable => {
                    // hook must come before unreachable instruction, otherwise it prevents hook from being called
                    if enabled_hooks.contains(Hook::Unreachable) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            hooks.instr(&instr, &[]),
                        ])
                    }

                    instrumented_body.push(instr);

                    unreachable_depth = 1;
                }


                /* Control Instructions: Blocks */

                Block(block_ty) => {
                    block_stack.begin_block(iidx);
                    type_stack.begin(block_ty);

                    instrumented_body.push(instr);

                    if enabled_hooks.contains(Hook::Begin) {
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

                    if enabled_hooks.contains(Hook::Begin) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            hooks.begin_loop(),
                        ])
                    }
                }
                If(block_ty) => {
                    block_stack.begin_if(iidx);
                    type_stack.instr(&FunctionType::new(&[I32], &[]));
                    type_stack.begin(block_ty);

                    // if_ hook for the condition (always executed on either branch)
                    if enabled_hooks.contains(Hook::If) {
                        let condition_tmp = function.add_fresh_local(I32);

                        instrumented_body.extend_from_slice(&[
                            Local(Tee, condition_tmp),
                            location.0.clone(),
                            location.1.clone(),
                            Local(Get, condition_tmp),
                            hooks.instr(&instr, &[])
                        ]);
                    }

                    // actual if block start
                    instrumented_body.push(instr);

                    // begin hook (not executed when condition implies else branch)
                    if enabled_hooks.contains(Hook::Begin) {
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

                    if enabled_hooks.contains(Hook::End) {
                        instrumented_body.extend_from_slice(&[
                            location.0.clone(),
                            location.1.clone(),
                            begin_if.to_const(),
                            hooks.end(&if_block),
                        ]);
                    }

                    instrumented_body.push(instr);

                    if enabled_hooks.contains(Hook::Begin) {
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
                        && enabled_hooks.contains(Hook::Return) {
                        if let BlockStackElement::Function { .. } = block {
                            let result_tys = function.type_.results();
                            let result_tmps = function.add_fresh_locals(result_tys);

                            save_stack_to_locals(&mut instrumented_body, &result_tmps);
                            instrumented_body.extend_from_slice(&[
                                location.0,
                                Const(Val::I32(-1)),
                            ]);
                            restore_locals_with_i64_handling(&mut instrumented_body, result_tmps, function);
                            instrumented_body.push(hooks.instr(&Return, result_tys));
                        }
                    }

                    // NOTE there is not duplication of the end hook call for explicit returns,
                    // because the end hook that is inserted now is never called (dead code)

                    if enabled_hooks.contains(Hook::End) {
                        block.append_end_hook_args(&mut instrumented_body, fidx);
                        instrumented_body.push(hooks.end(&block))
                    }

                    instrumented_body.push(instr);
                }


                /* Control Instructions: Branches/Breaks */
                // NOTE hooks must come before instr

                Br(target_label) => {
                    let br_target = block_stack.br_target(target_label);

                    // br hook
                    if enabled_hooks.contains(Hook::Br) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            target_label.to_const(),
                            br_target.absolute_instr.to_const(),
                            hooks.instr(&instr, &[])
                        ])
                    }

                    // end hooks for all intermediate blocks that are "jumped over"
                    if enabled_hooks.contains(Hook::End) {
                        for block in br_target.ended_blocks {
                            block.append_end_hook_args(&mut instrumented_body, fidx);
                            instrumented_body.push(hooks.end(&block));
                        }
                    }

                    instrumented_body.push(instr);

                    unreachable_depth = 1;
                }
                BrIf(target_label) => {
                    type_stack.instr(&FunctionType::new(&[I32], &[]));

                    let br_target = block_stack.br_target(target_label);

                    if enabled_hooks.contains(Hook::BrIf)
                        || enabled_hooks.contains(Hook::End) {

                        // saved condition local is needed by _both_ hooks
                        let condition_tmp = function.add_fresh_local(I32);
                        instrumented_body.push(Local(Tee, condition_tmp));

                        // br_if hook
                        if enabled_hooks.contains(Hook::BrIf) {
                            instrumented_body.extend_from_slice(&[
                                // NOTE see local.tee above
                                location.0.clone(),
                                location.1.clone(),
                                Local(Get, condition_tmp),
                                target_label.to_const(),
                                br_target.absolute_instr.to_const(),
                                hooks.instr(&instr, &[])
                            ]);
                        }

                        // end hooks for all intermediate blocks that are "jumped over"
                        if enabled_hooks.contains(Hook::End) {
                            // call hooks only iff condition is true (-> insert artificial if block)
                            instrumented_body.extend_from_slice(&[
                                // NOTE see local.tee above
                                Local(Get, condition_tmp),
                                If(FunctionType::empty()),
                            ]);
                            for block in br_target.ended_blocks {
                                block.append_end_hook_args(&mut instrumented_body, fidx);
                                instrumented_body.push(hooks.end(&block));
                            }
                            // of the artificially inserted if block before
                            instrumented_body.push(End);
                        }
                    }

                    instrumented_body.push(instr)
                }
                BrTable { ref table, default } => {
                    type_stack.instr(&FunctionType::new(&[I32], &[]));

                    if enabled_hooks.contains(Hook::BrTable)
                        // because end hooks are called at runtime, we need to instrument even if br_table is not enabled
                        || enabled_hooks.contains(Hook::End) {

                        // each br_table instruction gets its own entry in the static info object
                        // that maps table index to label and location
                        module_info.write().br_tables.push(BrTableInfo::from_br_table(table, default, &block_stack, fidx));

                        // NOTE calling the end() hooks for the intermediate blocks is done at runtime
                        // by the br_table low-level hook

                        let target_idx_tmp = function.add_fresh_local(I32);

                        instrumented_body.extend_from_slice(&[
                            Local(Tee, target_idx_tmp),
                            location.0,
                            location.1,
                            Local(Get, target_idx_tmp),
                            Const(Val::I32((module_info.read().br_tables.len() - 1) as i32)),
                            hooks.instr(&instr, &[])
                        ])
                    }

                    instrumented_body.push(instr.clone());

                    unreachable_depth = 1;
                }


                /* Control Instructions: Calls & Returns */

                Return => {
                    type_stack.instr(&FunctionType::new(&[], function.type_.results()));

                    // return hook
                    if enabled_hooks.contains(Hook::Return) {
                        let result_tys = function.type_.results();
                        let result_tmps = function.add_fresh_locals(result_tys);

                        save_stack_to_locals(&mut instrumented_body, &result_tmps);
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                        ]);
                        restore_locals_with_i64_handling(&mut instrumented_body, result_tmps, function);
                        instrumented_body.push(hooks.instr(&instr, result_tys));
                    }

                    // end hooks for all intermediate blocks that are "jumped over"
                    if enabled_hooks.contains(Hook::End) {
                        for block in block_stack.return_target().ended_blocks {
                            block.append_end_hook_args(&mut instrumented_body, fidx);
                            instrumented_body.push(hooks.end(&block));
                        }
                    }

                    instrumented_body.push(instr);

                    unreachable_depth = 1;
                }
                Call(target_func_idx) => {
                    let func_ty = &module_info.read().functions[target_func_idx.to_usize()].type_;
                    type_stack.instr(func_ty);

                    if enabled_hooks.contains(Hook::Call) {
                        /* pre call hook */

                        let arg_tmps = function.add_fresh_locals(func_ty.inputs());

                        save_stack_to_locals(&mut instrumented_body, &arg_tmps);
                        instrumented_body.extend_from_slice(&[
                            location.0.clone(),
                            location.1.clone(),
                            target_func_idx.to_const(),
                        ]);
                        restore_locals_with_i64_handling(&mut instrumented_body, arg_tmps, function);
                        instrumented_body.extend_from_slice(&[
                            hooks.instr(&instr, func_ty.inputs()),
                            instr,
                        ]);

                        /* post call hook */

                        let result_tmps = function.add_fresh_locals(func_ty.results());

                        save_stack_to_locals(&mut instrumented_body, &result_tmps);
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                        ]);
                        restore_locals_with_i64_handling(&mut instrumented_body, result_tmps, function);
                        instrumented_body.push(hooks.call_post(func_ty.results()))
                    } else {
                        instrumented_body.push(instr);
                    }
                }
                CallIndirect(ref func_ty, _ /* table idx == 0 in WASM version 1 */) => {
                    type_stack.instr(&instr.simple_type().unwrap());

                    if enabled_hooks.contains(Hook::Call) {
                        /* pre call hook */

                        let target_table_idx_tmp = function.add_fresh_local(I32);
                        let arg_tmps = function.add_fresh_locals(func_ty.inputs());

                        instrumented_body.push(Local(Set, target_table_idx_tmp));
                        save_stack_to_locals(&mut instrumented_body, &arg_tmps);
                        instrumented_body.extend_from_slice(&[
                            Local(Get, target_table_idx_tmp),
                            location.0.clone(),
                            location.1.clone(),
                            Local(Get, target_table_idx_tmp),
                        ]);
                        restore_locals_with_i64_handling(&mut instrumented_body, arg_tmps, function);
                        instrumented_body.extend_from_slice(&[
                            hooks.instr(&instr, func_ty.inputs()),
                            instr.clone(),
                        ]);

                        /* post call hook */

                        let result_tmps = function.add_fresh_locals(func_ty.results());

                        save_stack_to_locals(&mut instrumented_body, &result_tmps);
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                        ]);
                        restore_locals_with_i64_handling(&mut instrumented_body, result_tmps, function);
                        instrumented_body.push(hooks.call_post(func_ty.results()));
                    } else {
                        instrumented_body.push(instr.clone());
                    }
                }


                /* Parametric Instructions */

                Drop => {
                    let ty = type_stack.pop_val();

                    if enabled_hooks.contains(Hook::Drop) {
                        let tmp = function.add_fresh_local(ty);

                        instrumented_body.extend_from_slice(&[
                            Local(Set, tmp),
                            location.0,
                            location.1,
                        ]);
                        convert_i64_instr(&mut instrumented_body, Local(Get, tmp), ty);
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

                    if enabled_hooks.contains(Hook::Drop) {
                        let condition_tmp = function.add_fresh_local(I32);
                        let arg_tmps = function.add_fresh_locals(&[ty, ty]);

                        save_stack_to_locals(&mut instrumented_body, &[arg_tmps[0], arg_tmps[1], condition_tmp]);
                        instrumented_body.extend_from_slice(&[
                            instr.clone(),
                            location.0,
                            location.1,
                            Local(Get, condition_tmp),
                        ]);
                        restore_locals_with_i64_handling(&mut instrumented_body, arg_tmps, function);
                        // replace select with hook call
                        instrumented_body.push(hooks.instr(&instr, &[ty, ty]));
                    } else {
                        instrumented_body.push(instr);
                    }
                }


                /* Variable Instructions */

                Local(op, local_idx) => {
                    let local_ty = function.param_or_local_type(local_idx);

                    type_stack.instr(&op.to_type(local_ty));

                    instrumented_body.push(instr.clone());

                    // insert hook AFTER instruction, so that we can use local.get instead of duplicating the value through a new local
                    if enabled_hooks.contains(Hook::Local) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            local_idx.to_const(),
                        ]);
                        convert_i64_instr(&mut instrumented_body, Local(Get, local_idx), local_ty);
                        instrumented_body.push(hooks.instr(&instr, &[local_ty]));
                    }
                }
                Global(op, global_idx) => {
                    let global_ty = module_info.read().globals[global_idx.to_usize()];

                    type_stack.instr(&op.to_type(global_ty));

                    instrumented_body.push(instr.clone());

                    // insert hook AFTER instruction, so that we can use global.get instead of duplicating the value through a new local
                    if enabled_hooks.contains(Hook::Global) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                            global_idx.to_const(),
                        ]);
                        convert_i64_instr(&mut instrumented_body, Global(GlobalOp::Get, global_idx), global_ty);
                        instrumented_body.push(hooks.instr(&instr, &[global_ty]));
                    }
                }


                /* Memory Instructions */

                MemorySize(_ /* memory idx == 0 in WASM version 1 */) => {
                    type_stack.instr(&instr.simple_type().unwrap());

                    instrumented_body.push(instr.clone());

                    if enabled_hooks.contains(Hook::MemorySize) {
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
                    type_stack.instr(&instr.simple_type().unwrap());

                    if enabled_hooks.contains(Hook::MemoryGrow) {
                        let input_tmp = function.add_fresh_local(I32);
                        let result_tmp = function.add_fresh_local(I32);

                        instrumented_body.extend_from_slice(&[
                            Local(Tee, input_tmp),
                            instr.clone(),
                            Local(Tee, result_tmp),
                            location.0,
                            location.1,
                            Local(Get, input_tmp),
                            Local(Get, result_tmp),
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

                    if enabled_hooks.contains(Hook::Load) {
                        let addr_tmp = function.add_fresh_local(ty.inputs()[0]);
                        let value_tmp = function.add_fresh_local(ty.results()[0]);

                        instrumented_body.extend_from_slice(&[
                            Local(Tee, addr_tmp),
                            instr.clone(),
                            Local(Tee, value_tmp),
                            location.0,
                            location.1,
                            Const(Val::I32(memarg.offset as i32)),
                            Const(Val::I32(memarg.alignment_exp as i32)),
                        ]);
                        restore_locals_with_i64_handling(&mut instrumented_body, [addr_tmp, value_tmp], function);
                        instrumented_body.push(hooks.instr(&instr, &[]));
                    } else {
                        instrumented_body.push(instr);
                    }
                }
                Store(op, memarg) => {
                    let ty = op.to_type();
                    type_stack.instr(&ty);

                    if enabled_hooks.contains(Hook::Store) {
                        let addr_tmp = function.add_fresh_local(ty.inputs()[0]);
                        let value_tmp = function.add_fresh_local(ty.inputs()[1]);

                        save_stack_to_locals(&mut instrumented_body, &[addr_tmp, value_tmp]);
                        instrumented_body.extend_from_slice(&[
                            instr.clone(),
                            location.0,
                            location.1,
                            Const(Val::I32(memarg.offset as i32)),
                            Const(Val::I32(memarg.alignment_exp as i32)),
                        ]);
                        restore_locals_with_i64_handling(&mut instrumented_body, [addr_tmp, value_tmp], function);
                        instrumented_body.push(hooks.instr(&instr, &[]));
                    } else {
                        instrumented_body.push(instr);
                    }
                }


                /* Numeric Instructions */

                Const(val) => {
                    type_stack.instr(&instr.simple_type().unwrap());

                    instrumented_body.push(instr.clone());

                    if enabled_hooks.contains(Hook::Const) {
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                        ]);
                        // optimization: just call T.const again, instead of duplicating result into local
                        convert_i64_instr(&mut instrumented_body, instr.clone(), val.to_type());
                        instrumented_body.push(hooks.instr(&instr, &[]));
                    }
                }
                Unary(_) | Binary(_) => {
                    let ty = instr.simple_type().unwrap();
                    type_stack.instr(&ty);

                    if (enabled_hooks.contains(Hook::Unary) && ty.inputs().len() == 1)
                        || (enabled_hooks.contains(Hook::Binary) && ty.inputs().len() == 2) {
                        let input_tmps = function.add_fresh_locals(ty.inputs());
                        let result_tmps = function.add_fresh_locals(ty.results());

                        save_stack_to_locals(&mut instrumented_body, &input_tmps);
                        instrumented_body.push(instr.clone());
                        save_stack_to_locals(&mut instrumented_body, &result_tmps);
                        instrumented_body.extend_from_slice(&[
                            location.0,
                            location.1,
                        ]);
                        restore_locals_with_i64_handling(&mut instrumented_body, input_tmps.iter().chain( result_tmps.iter()).copied(), function);
                        instrumented_body.push(hooks.instr(&instr, &[]));
                    } else {
                        instrumented_body.push(instr);
                    }
                }
            }
        }

        // finally, switch dummy body out against instrumented body
        function.code_mut().unwrap().body = instrumented_body;
    });

    // actually add the hooks to module and check that inserted Idx is the one on the Hook struct
    let hooks = hooks.finish();
    let hook_count = hooks.len();
    //    let mut hook_list: Vec<(String, FunctionType)> = hooks.iter().map(|hook| (hook.wasm.import.as_ref().map(|opt| opt.1.clone()).unwrap(), hook.wasm.type_.clone())).collect();
    //    hook_list.sort_by_key(|h| h.0.clone());
    //    for hook in hook_list {
    //        println!("{}: {:?}", hook.0, hook.1);
    //    }
    //    println!("{:?}", hook_list.iter().max_by_key(|hook| hook.1.params.len()));

    let mut js_hooks = Vec::new();
    for hook in hooks {
        js_hooks.push(hook.js);
        assert_eq!(hook.idx, module.functions.len().into(), "have other functions been inserted into the module since starting collection of hooks?");
        module.functions.push(hook.wasm);
    }

    Some((
        generate_js(module_info.into_inner(), &js_hooks, node_js),
        hook_count,
    ))
}

/// convenience to hand (function/instr/local/global) indices to hooks
/// must be trait since inherent impl is disallowed by orphan rules for non-crate types (Idx<T>)
trait ToConst {
    fn to_const(self) -> Instr;
}

impl<T> ToConst for Idx<T> {
    fn to_const(self) -> Instr {
        Const(Val::I32(self.to_usize().try_into().unwrap()))
    }
}

impl ToConst for Label {
    fn to_const(self) -> Instr {
        Const(Val::I32(self.to_usize().try_into().unwrap()))
    }
}

impl BlockStackElement {
    fn append_end_hook_args(&self, append_to: &mut Vec<Instr>, fidx: Idx<Function>) {
        match self {
            BlockStackElement::Function { end } => append_to.extend_from_slice(&[
                fidx.to_const(), 
                end.to_const()
            ]),
            BlockStackElement::Block { begin, end }
            | BlockStackElement::Loop { begin, end }
            | BlockStackElement::If { begin_if: begin, end, .. } => append_to.extend_from_slice(&[
                fidx.to_const(),
                end.to_const(),
                begin.to_const()
            ]),
            BlockStackElement::Else { begin_else, begin_if, end } => append_to.extend_from_slice(&[
                fidx.to_const(),
                end.to_const(),
                begin_else.to_const(),
                begin_if.to_const(),
            ]),
        }
    }
    fn end(&self) -> Idx<Instr> {
        use self::block_stack::BlockStackElement::*;
        match self {
            Function { end }
            | Block { end, .. }
            | Loop { end, .. }
            | If { end, .. }
            | Else { end, .. } => *end,
        }
    }
}

fn generate_js(module_info: ModuleInfo, hooks: &[String], node_js: bool) -> String {
    let mut result = r#"/*
* Generated by Wasabi. DO NOT EDIT.
* Contains:
*   - independent of program-to-instrument: long.js dependency, Wasabi loader and runtime
*   - generated from program-to-instrument: static information and low-level hooks
*/

"#.to_string();

    if node_js {
        // For Node.js, write the long.js dependency to a separate file (in main) and
        // only `require()` it here.
        result.push_str("const Long = require('./long.js');");
    } else {
        // Browser case (default):
        // FIXME super hacky: just cat together long.js dependency, program-independent, and
        // program-dependent JavaScript into one big file.
        // * Alternative A: use webpack or other bundler, drawbacks:
        //    - users need to install another tool
        //    - needs to be run after every instrumentation
        // * Alternative B: compile Wasabi itself to WebAssembly, instrument at runtime
        result.push_str("// long.js\n");
        result.push_str(include_str!("../../../js/long.js/long.js")
            .lines()
            .next()
            .expect("could not include long.js dependency"));
    }
    result.push_str("\n\n");

    result.push_str(include_str!("../../../js/runtime.js"));
    result.push('\n');

    result.push_str("Wasabi.module.info = ");
    result.push_str(&serde_json::to_string(&module_info).unwrap());
    result.push_str(";\n\n");

    result.push_str("Wasabi.module.lowlevelHooks = {\n");
    for hook in hooks {
        result.push_str(hook);
        result.push('\n');
    }
    result.push_str("};\n");

    if node_js {
        result.push_str("\nmodule.exports = Wasabi;\n");
    }

    result
}
