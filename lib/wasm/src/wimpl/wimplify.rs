//! Conversion from standard WebAssembly to Wimpl.

use std::convert::TryInto;

use crate::highlevel;
use crate::wimpl::*; 

/// The mutable state during conversion.
pub struct State<'module> {
    instrs_iter: std::slice::Iter<'module, highlevel::Instr>,
    type_checker: TypeChecker<'module>,

    label_count: u32,
    stack_var_count: u32,

    // The bool is `true` if the label is for a `loop` block, and false for `block` and `if`.
    // TODO maybe this doesn't need ValType, because the type is carried along in the expr stack?
    #[allow(clippy::type_complexity)]
    label_stack: Vec<(Label, Option<(Var, ValType, bool)>)>,
}

/// The immutable context information required but never mutated during conversion.
#[derive(Clone, Copy)]
pub struct Context<'module> {
    module: &'module highlevel::Module,
    func_ty: &'module FunctionType,
}

fn wimplify_instrs<'module>(
    stmts_result: &mut Vec<Stmt>,
    state: &mut State,
    context: Context<'module>,
) -> Result</* was_else */ bool, String> {

    use crate::highlevel::Instr as wasm;
    use Expr::*;
    use Var::*;

    // State that is "local" to this nested block (unlike `state`, which is for the whole function).
    let mut expr_stack: Vec<(Expr, ValType)> = Vec::new();

    while let Some(instr) = state.instrs_iter.next() {
        let ty = state.type_checker.check_next_instr(instr).map_err(|e| e.to_string())?;

        // DEBUG
        // println!("instr: {}, {}, {:?}", instr, ty, expr_stack);

        let ty = match ty {
            // If the following code (until the next end or else) is unreachable, 
            // don't generate any Wimpl code from it, so continue until then.
            InferredInstructionType::Unreachable => {
                match instr {
                    // But end and else terminate the current nested block, so return in that case.
                    wasm::End => {
                        state.label_stack.pop();
                        return Ok(false)
                    },
                    wasm::Else => {
                        // Unlike for end, don't pop here because the nested "else" block still follows.
                        return Ok(true)
                    }
                    _ => continue
                }
            },
            InferredInstructionType::Reachable(ty) => ty,
        };


        // Utility functions for the conversion:

        // Only call this function when you have finished translating the instructions, i.e., after
        // you have popped all inputs from the `var_stack`, since this changes `var_stack`.
        fn create_fresh_stack_var(state: &mut State) -> Var {
            let var = Stack(state.stack_var_count);
            state.stack_var_count += 1;
            var
        }

        // Use this when an _expression_ on the stack would need to be duplicated (which is not
        // valid, as it would change semantics and likely performance when doubly evaluated).
        // So evaluate the expr and store in an intermediate fresh variable. Then return this 
        // variable for future uses.
        fn pop_expr_and_assign_to_fresh_stack_var(state: &mut State, expr_stack: &mut Vec<(Expr, ValType)>, stmts_result: &mut Vec<Stmt>) -> Expr {
            let (expr, type_) = expr_stack.pop().expect("expected value on the stack");
            let var = create_fresh_stack_var(state);
            stmts_result.push(Stmt::Assign {
                lhs: var,
                type_,
                rhs: expr,
            });
            VarRef(var)
        }

        // To make sure the Wimpl evaluation order is equivalent to WebAssembly, before pushing a
        // statement, we must make sure (by calling this function) that all expressions still
        // "dormant" on the stack are actually executed beforehand (e.g., due to side effects).
        // To be able to refer to their values later on, we replace those expressions by references 
        // to the freshly created variables holding the evaluation result instead.
        // TODO call this whenever calling stmts.push(...)
        fn materialize_all_exprs_as_stmts(state: &mut State, expr_stack: &mut Vec<(Expr, ValType)>, stmts_result: &mut Vec<Stmt>) {
            for (expr, type_) in expr_stack {
                let var = create_fresh_stack_var(state);
                let expr = std::mem::replace(expr, VarRef(var));
                stmts_result.push(Stmt::Assign {
                    lhs: var,
                    type_: *type_,
                    rhs: expr
                })
            }
        }

        fn local_idx_to_var(context: Context, local_idx: Idx<highlevel::Local>) -> Var {
            let local_idx = local_idx.to_u32();
            let num_params: u32 = context.func_ty.inputs().len().try_into().expect("more than 2^32 parameters");
            if local_idx < num_params {
                Param(local_idx)
            } else {
                Local(local_idx - num_params)
            }
        }

        fn create_block_label_and_var(state: &mut State, expr_stack: &mut Vec<(Expr, ValType)>, blocktype: BlockType, is_loop: bool) -> Label {
            let label = Label(state.label_count);
            state.label_count += 1;

            if let Some(type_) = blocktype.0 {
                let result_var = BlockResult(label.0);
                state.label_stack.push((label, Some((result_var, type_, is_loop))));
    
                // The result of the block is then at the top of the stack after the block.
                expr_stack.push((VarRef(result_var), type_));
            } else {
                state.label_stack.push((label, None));
            }

            label
        }

        fn wasm_label_to_wimpl_label_and_block_var(state: &State, wasm_label: crate::Label) -> (Label, Option<(Var, ValType)>) {
            let (wimpl_label, block_result) = *state.label_stack.iter().rev().nth(wasm_label.to_usize()).expect("invalid branch label, not in label stack");
            match block_result {
                // Target block is not a loop and needs a result, so return the result variable.
                Some((block_result_var, type_, false)) => (wimpl_label, Some((block_result_var, type_))),
                // Target block either has no result, or is a loop. Because loops/blocks have no 
                // inputs in Wasm MVP and br to loops jump to the beginning, there is no result
                // that would need to be assigned in that case.
                Some((_, _, true)) | None => (wimpl_label, None)
            }
        }

        // Conversion of each WebAssembly instruction to (one or more) Wimpl statements:
        match instr {

            wasm::Unreachable => {
                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);
                stmts_result.push(Stmt::Unreachable)
            },

            wasm::Nop => {},

            wasm::Block(blocktype) => {
                // Do this before any statements are pushed out, or expressions added to the stack
                // (e.g., the variable holding the result.)
                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);

                let label = create_block_label_and_var(state, &mut expr_stack, *blocktype, false);

                // DEBUG
                // println!("block: {}, {:?}", blocktype, state.label_stack);

                let mut block_body = Vec::new();
                let ends_with_else = wimplify_instrs(&mut block_body, state, context)?;
                assert!(!ends_with_else, "block should be terminated by end, not else");

                stmts_result.push(
                    Stmt::Block {
                        body: Body(block_body),
                        end_label: label,
                    }
                );
            }
            wasm::Loop(blocktype) => {
                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);

                let label = create_block_label_and_var(state, &mut expr_stack, *blocktype, true);

                let mut loop_body = Vec::new();
                let ends_with_else = wimplify_instrs(&mut loop_body, state, context)?;
                assert!(!ends_with_else, "loop should be terminated by end, not else");

                stmts_result.push(
                    Stmt::Loop {
                        begin_label: label,
                        body: Body(loop_body),
                    }
                );
            }

            wasm::If(blocktype) => {
                let (condition, condition_ty) = expr_stack.pop().expect("if expects a condition which was not found on the stack");
                assert_eq!(condition_ty, ValType::I32);

                let label = create_block_label_and_var(state, &mut expr_stack, *blocktype, false);
                
                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);
                
                let mut if_body = Vec::new();
                let has_else = wimplify_instrs(&mut if_body, state, context)?;
                let else_body = if has_else {
                    let mut else_body = Vec::new();
                    let ends_with_else = wimplify_instrs(&mut else_body, state, context)?;
                    assert!(!ends_with_else, "else block should be terminated by end not else");
                    Some(Body(else_body))
                } else {
                    None
                };

                // Wrap `if` inside a `block`, because Wimpl ifs don't have a label, but if a 
                // branch wants to exit the if, it needs to target a label.
                // TODO do not generate the surrounding block if no branch targets it
                // -> requires a precomputed map from branches to targets
                stmts_result.push(Stmt::Block {
                    body: Body(vec![Stmt::If {
                        condition,
                        if_body: Body(if_body),
                        else_body,
                    }]),
                    end_label: label,
                });
            },

            wasm::Else => {
                // Cannot pop because you still want it to be on the label stack while processing the else body.
                let (_, result_info) = *state.label_stack.last().expect("label stack should include if label");

                // Assign result of the if statement that we just finished processing.
                if let Some((if_result_var, type_, is_loop_block)) = result_info {
                    assert!(!is_loop_block, "if block result should never have loop flag set");
                    let value = expr_stack.pop().expect("else expects if result value on the stack").0;
                    
                    // Because the stack should be empty at the end of a block, we don't need to
                    // materialize expressions here.
                    
                    stmts_result.push(Stmt::Assign {
                        lhs: if_result_var,
                        type_,
                        rhs: value,
                    })
                }

                assert!(expr_stack.is_empty(), "should not contain superfluous expressions");

                // End recursive invocation and return converted body of the current block.
                return Ok(true);
            },

            wasm::End => {
                let (_, result_info) = state.label_stack.pop().expect("end of a block expects the matching label to be in the label stack");

                if let Some((block_result_var, type_, _is_loop_block)) = result_info {
                    let value = expr_stack.pop().expect("end expects if/block/loop result value on the stack").0;

                    // Because the stack should be empty at the end of a block, we don't need to
                    // materialize expressions here.
                    
                    stmts_result.push(Stmt::Assign {
                        lhs: block_result_var,
                        type_,
                        rhs: value,
                    });
                };

                assert!(expr_stack.is_empty(), "should not contain superfluous expressions");

                // End recursive invocation and return converted body of the current block.
                return Ok(false)
            }

            wasm::Br(label) => {
                let (wimpl_label, block_var) = wasm_label_to_wimpl_label_and_block_var(state, *label);
                
                // Handle dataflow ("transported value through branch") by explicit assignment.
                if let Some((var, type_)) = block_var {
                    let value = expr_stack.pop().expect("br to this label expects a value").0;
                    materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);
                    stmts_result.push(Stmt::Assign {
                        lhs: var,
                        type_,
                        rhs: value,
                    })
                } else {
                    materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);
                }

                stmts_result.push(Stmt::Br { target: wimpl_label })
            },

            // Translate br_if as if + (unconditional) br.
            wasm::BrIf(label) => {
                let (condition, condition_ty) = expr_stack.pop().expect("br_if expects a conditional on the stack");
                assert_eq!(condition_ty, ValType::I32);

                let (wimpl_label, block_var) = wasm_label_to_wimpl_label_and_block_var(state, *label);
                
                // Materialize also the (maybe) value of the branch target in a variable.
                // This is necessary because its expression shall not be duplicated, once for the
                // case if the branch is taken (assigned to target result variable), once if the
                // branch is not taken (in the expression stack).
                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);

                let mut if_body = Vec::with_capacity(2);
                if let Some((block_var, type_)) = block_var {
                    // The value "transported" by the branch, needs to be duplicated, in case the
                    // branch is not taken.
                    // Since we just materialized this, this is going to be a `VarRef` and cloning
                    // it is fine (doesn't alter semantics or introduce duplicate expressions).
                    let branch_value_var = expr_stack.last().expect("br_if to this label expects a value").0.clone();
                    if_body.push(Stmt::Assign {
                        lhs: block_var,
                        type_,
                        rhs: branch_value_var,
                    });
                }
                if_body.push(Stmt::Br { target: wimpl_label });

                stmts_result.push(Stmt::If {
                    condition,
                    if_body: Body(if_body),
                    else_body: None,
                })
            }

            // FIXME write test case for this
            wasm::BrTable { table, default } => {
                let (table_index_expr, table_index_ty) = expr_stack.pop().expect("br_table expects an index into the table on the stack");
                assert_eq!(table_index_ty, ValType::I32);

                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);

                let br_with_maybe_assign = move |label: crate::Label, state: &mut State, expr_stack: &Vec<(Expr, ValType)>| -> Vec<Stmt> {
                    let (wimpl_label, block_var) = wasm_label_to_wimpl_label_and_block_var(state, label);
                    if let Some((block_var, type_)) = block_var {
                        // Same as for br_if above, see there.
                        let branch_value_var = expr_stack.last().expect("this br_table expects a value").0.clone();
                        vec![
                            Stmt::Assign {
                                lhs: block_var,
                                type_,
                                rhs: branch_value_var,
                            },
                            Stmt::Br { target: wimpl_label }
                        ]
                    } else {
                        vec![Stmt::Br { target: wimpl_label }]
                    }
                };

                stmts_result.push(Stmt::Switch {
                    index: table_index_expr,
                    cases: table.iter().map(|label| Body(br_with_maybe_assign(*label, state, &mut expr_stack))).collect(),
                    default: Body(br_with_maybe_assign(*default, state, &mut expr_stack)),
                })
            }

            wasm::Return => {
                // This points to the block for the overall function body.
                let target = Label(0);

                if let (target_from_label_stack, Some((return_var, type_, loop_flag))) = state.label_stack.first().expect("empty label stack, but expected function ") {
                    assert_eq!(target, *target_from_label_stack, "label stack is invalid, should have been the function label");
                    assert!(!loop_flag, "function body block should not have loop flag set");

                    let return_val = expr_stack.pop().expect("return expects a return value").0;
                    stmts_result.extend([
                        Stmt::Assign {
                            lhs: *return_var,
                            type_: *type_,
                            rhs: return_val
                        },
                        Stmt::Br { target }
                    ])
                } else {
                    stmts_result.push(Stmt::Br { target })
                }
            }

            wasm::Call(func_index) => {
                let n_args = context.module.function(*func_index).type_.inputs().len();
                let call_expr = Call {
                    func: FunctionId::from_idx(*func_index, context.module),
                    args: expr_stack.split_off(expr_stack.len() - n_args).into_iter().map(|(expr, _ty)| expr).collect(),
                };

                // If the call doesn't produce a value, emit it as a statement for its side-effects,
                // otherwise push its result on the stack.
                match ty.results() {
                    [] => {
                        materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);
                        stmts_result.push(Stmt::Expr(call_expr))
                    },
                    [type_] => expr_stack.push((call_expr, *type_)),
                    _ => panic!("WebAssembly multi-value extension")
                }
            },

            wasm::CallIndirect(func_type, table_index) => {
                assert_eq!(table_index.to_usize(), 0, "WebAssembly MVP must always have a single table");

                let (table_index_expr, table_index_ty) = expr_stack.pop().expect("call_indirect expects a table index on the stack");
                assert_eq!(table_index_ty, ValType::I32);

                let n_args = func_type.inputs().len();
                let call_expr = CallIndirect {
                    type_: func_type.clone(),
                    table_idx: Box::new(table_index_expr),
                    args: expr_stack.split_off(expr_stack.len() - n_args).into_iter().map(|(expr, _ty)| expr).collect(),
                };

                // Analoguous to call above.
                match ty.results() {
                    [] => {
                        materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);
                        stmts_result.push(Stmt::Expr(call_expr))
                    },
                    [type_] => expr_stack.push((call_expr, *type_)),
                    _ => panic!("WebAssembly multi-value extension")
                }
            }

            wasm::Drop => {
                let expr = expr_stack.pop().expect("drop expects a value on the stack").0;

                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);

                // Emit as a statement because maybe it was executed for its side-effect, e.g., calls.
                stmts_result.push(Stmt::Expr(expr))
            },

            // Translate as if block assigning to a fresh stack variable.
            wasm::Select => {
                let (condition, condition_ty) = expr_stack.pop().expect("select expects a conditional on the stack");
                assert_eq!(condition_ty, ValType::I32);

                let type_ = ty.results()[0];
                
                // We need to materialize the if and else case, regardless of whether they are used,
                // to match the semantics of Wasm, where both are always evaluated.
                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);
                let if_result_var = expr_stack.pop().expect("select expects if value on the stack").0;
                let else_result_var = expr_stack.pop().expect("select expects else value on the stack").0;

                let result_var = create_fresh_stack_var(state);

                stmts_result.push(
                    Stmt::If {
                        condition,
                        if_body: Body(vec![Stmt::Assign {
                            lhs: result_var,
                            type_,
                            rhs: if_result_var,
                        }]),
                        else_body: Some(Body(vec![Stmt::Assign {
                            lhs: result_var,
                            type_,
                            rhs: else_result_var,
                        }]))
                    }
                )
            }

            wasm::Local(highlevel::LocalOp::Get, local_idx) => {
                let type_ = ty.results()[0];

                expr_stack.push((VarRef(local_idx_to_var(context, *local_idx)), type_))
            }

            wasm::Local(highlevel::LocalOp::Set, local_idx) => {
                let (rhs, type_) = expr_stack.pop().expect("local.set expects a value on the stack");
                assert_eq!(type_, ty.inputs()[0]);

                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);

                stmts_result.push(Stmt::Assign {
                    lhs: local_idx_to_var(context, *local_idx),
                    type_,
                    rhs,
                })
            }

            wasm::Local(highlevel::LocalOp::Tee, local_idx) => {
                // The stack needs to contain the value at this point of setting the local, so
                // materialize the argument to local.tee as well!
                // See tests/wimpl_expected/local_tee.wat for a problematic example otherwise.
                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);
                let (value_var, type_) = expr_stack.pop().expect("local.tee expects a value on the stack");
                assert_eq!(type_, ty.inputs()[0]);

                stmts_result.push(Stmt::Assign {
                    lhs: local_idx_to_var(context, *local_idx),
                    type_,
                    rhs: value_var,
                })
            }

            wasm::Global(highlevel::GlobalOp::Get, global_idx) => {
                let type_ = ty.results()[0];

                expr_stack.push((VarRef(Global(global_idx.to_u32())), type_))
            }

            wasm::Global(highlevel::GlobalOp::Set, global_idx) => {
                let (rhs, type_) = expr_stack.pop().expect("local.set expects a value on the stack");
                assert_eq!(type_, ty.inputs()[0]);

                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);

                stmts_result.push(Stmt::Assign {
                    lhs: Global(global_idx.to_u32()),
                    type_,
                    rhs,
                })
            }

            wasm::Load(loadop, memarg) => {
                let (addr, addr_ty) = expr_stack.pop().expect("load expects an address on the stack");
                assert_eq!(addr_ty, ValType::I32);

                let type_ = ty.results()[0];

                expr_stack.push((
                    Load {
                        op: *loadop,
                        memarg: *memarg,
                        addr: Box::new(addr),
                    },
                    type_
                ))
            }

            wasm::Store(op, memarg) => {
                let (addr, addr_ty) = expr_stack.pop().expect("store expects an address on the stack");
                assert_eq!(addr_ty, ValType::I32);

                let value = expr_stack.pop().expect("store expects a value to store on the stack").0;

                materialize_all_exprs_as_stmts(state, &mut expr_stack, stmts_result);

                stmts_result.push(Stmt::Store {
                    op: *op,
                    memarg: *memarg,
                    addr,
                    value,
                })
            }

            wasm::MemorySize(memory_idx) => {
                assert_eq!(memory_idx.to_usize(), 0, "Wasm MVP only has a single memory");

                expr_stack.push((MemorySize, ty.results()[0]))
            }

            wasm::MemoryGrow(memory_idx) => {
                assert_eq!(memory_idx.to_usize(), 0, "Wasm MVP only has a single memory");

                let (pages, pages_ty) = expr_stack.pop().expect("memory.grow expects a value on the stack");
                assert_eq!(pages_ty, ValType::I32);

                expr_stack.push((MemoryGrow { pages: Box::new(pages) }, ty.results()[0]))
            }

            wasm::Const(val) => {
                expr_stack.push((Const(*val), ty.results()[0]))
            }

            wasm::Numeric(op) => {
                let args = expr_stack.split_off(expr_stack.len() - ty.inputs().len()).into_iter().map(|(expr, _ty)| expr).collect();
                expr_stack.push((Numeric { op: *op, args }, ty.results()[0]))
            }
        }
    }

    Ok(false)
}

pub fn wimplify(module: &highlevel::Module) -> Result<Module, String> {
    let mut wimpl_funcs = Vec::new();
    for (idx, func) in module.functions() {

        // TODO move into own function, e.g., Function::wimplify() or Function::from(hl::Function).
        //initialize the local variables
        let mut stmts_result = Vec::with_capacity(func.local_count() + 1);
        for (local_idx, loc) in func.locals() {
            let (loc_name, loc_type) = (&loc.name, loc.type_);
            if let Some(_loc_name) = loc_name {
                todo!("you haven't yet implemented locals having names");
            } else {
                stmts_result.push(Stmt::Assign {
                    lhs: Var::Local(local_idx.to_u32() - func.type_.inputs().len() as u32),
                    rhs: Expr::Const(Val::get_default_value(loc_type)),
                    type_: loc_type,
                })
            }
        }

        //translate the instructions in the function
        if let Some(code) = func.code() {
            let context = Context {
                module,
                func_ty: &func.type_
            };

            let mut state = State {
                instrs_iter: code.body.as_slice().iter(),
                type_checker: TypeChecker::begin_function(func, module),
                label_stack: Vec::new(),
                label_count: 1, // 0 is already used by the function body block.
                stack_var_count: 0,
            };

            let return_var = match func.type_.results() {
                [] => None,
                [ty] => Some((Var::Return(0), *ty, false)),
                _ => unimplemented!("only WebAssembly MVP is supported, not multi-value extension")
            };
            state.label_stack.push((Label(0), return_var));

            let was_else = wimplify_instrs(&mut stmts_result, &mut state, context)?;
            assert!(!was_else, "function should not end with else");
        }

        wimpl_funcs.push(Function {
            type_: func.type_,
            body: Body(stmts_result),
            name: FunctionId::from_idx(idx, module),
            export: func.export.clone(), 
        });
    }

    Ok(Module{
        functions: wimpl_funcs,
        globals: module.globals.clone(),
        tables: module.tables.clone(),
    })
}
