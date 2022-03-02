//! Conversion from standard WebAssembly to Wimpl.

use crate::highlevel;
use crate::wimpl::*; 

/// The mutable state during conversion.
pub struct State<'module> {
    instrs_iter: std::slice::Iter<'module, highlevel::Instr>,
    type_checker: TypeChecker<'module>,

    label_count: usize,
    stack_var_count: usize,

    // The bool is `true` if the label is for a `loop` block, and false for `block` and `if`.
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

    use Expr::*;
    use Var::*;

    // State that is "local" to this nested block (unlike `state`, which is for the whole function).
    let mut var_stack = Vec::new();

    while let Some(instr) = state.instrs_iter.next() {
        let ty = state.type_checker.check_next_instr(instr).map_err(|e| e.to_string())?;

        // DEBUG
        // println!("{}, {}, {:?}", instr, ty, var_stack);

        let ty = match ty {
            // If this code is unreachable, we don't generate any Wimpl code from it, so exit early.
            InferredInstructionType::Unreachable => {
                match instr {
                    highlevel::Instr::End => {
                        state.label_stack.pop();
                        return Ok(false)
                    },
                    highlevel::Instr::Else => {
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
        fn create_fresh_stack_var(state: &mut State, var_stack: &mut Vec<Var>) -> Var {
            let var = Stack(state.stack_var_count);
            state.stack_var_count += 1;
            var_stack.push(var);
            var
        }

        fn local_idx_to_var(context: Context, local_idx: Idx<highlevel::Local>) -> Var {
            let local_idx = local_idx.into_inner();
            let num_params = context.func_ty.inputs().len();
            if local_idx < num_params {
                Param(local_idx)
            } else {
                Local(local_idx - num_params)
            }
        }

        fn blocktype_to_var_and_init(blocktype: BlockType, state: &mut State, var_stack: &mut Vec<Var>, stmts: &mut Vec<Stmt>) -> Option<(Var, ValType)> {
            if let BlockType(Some(type_)) = blocktype {
                let result_var = BlockResult(state.label_count);
                // The block result is available as a variable (for translating branches with values).
                var_stack.push(result_var);

                stmts.push(Stmt::Assign {
                    lhs: result_var,
                    rhs: Const(Val::get_default_value(type_)),
                    type_,
                });

                Some((result_var, type_))
            } else {
                None
            }
        }

        let label_to_instrs = |label: crate::Label, label_stack: &[_], get_value: &mut dyn FnMut() -> Var| -> Vec<Stmt> {
            let (target, block_result) = *label_stack.iter().rev().nth(label.into_inner()).expect("invalid branch label, not in label stack");

            // println!("{:?}\n{:?} {:?} {:?}", label_stack, label, target, return_info);

            match block_result {
                // Target block needs a result, and is not a loop.
                Some((lhs, type_, false)) => vec![
                    Stmt::Assign{
                        lhs,
                        rhs: VarRef(get_value()),
                        type_,
                    },
                    Stmt::Br {
                        target,
                    }
                ],
                // Target block either has no result, or is a loop.
                Some((_, _, true)) | None => vec![Stmt::Br {
                    target,
                }]
            }
        };

        // Conversion of each WebAssembly instruction to (one or more) Wimpl statements:
        match instr {

            highlevel::Instr::Unreachable => stmts_result.push(Stmt::Unreachable),

            highlevel::Instr::Nop => {},

            // Block and loop share so much code, so handle them together.
            highlevel::Instr::Block(blocktype) |
            highlevel::Instr::Loop(blocktype) => {
                let result_var = blocktype_to_var_and_init(*blocktype, state, &mut var_stack, stmts_result);
                let is_loop = match instr {
                    highlevel::Instr::Loop(_) => true,
                    highlevel::Instr::Block(_) => false,
                    _ => unreachable!(),
                };

                let label = Label(state.label_count);
                state.label_count += 1;
                state.label_stack.push((label, result_var.map(|(var, ty)| (var, ty, is_loop))));

                let mut block_body = Vec::new();
                let ends_with_else = wimplify_instrs(&mut block_body, state, context)?;
                assert!(!ends_with_else, "block and loop are terminated by end, not else");

                stmts_result.push(match instr {
                    highlevel::Instr::Block(_) => Stmt::Block {
                        end_label: label,
                        body: Body(block_body),
                    },
                    highlevel::Instr::Loop(_) => Stmt::Loop {
                        begin_label: label,
                        body: Body(block_body),
                    },
                    _ => unreachable!(),
                });
            }

            highlevel::Instr::If(blocktype) => {
                let condition = var_stack.pop().expect("if expects a condition which was not found on the stack");

                let result_var = blocktype_to_var_and_init(*blocktype, state, &mut var_stack, stmts_result);

                let label = Label(state.label_count);
                state.label_count += 1;
                state.label_stack.push((label, result_var.map(|(var, ty)| (var, ty, false))));
                
                let mut if_body = Vec::new();
                let has_else = wimplify_instrs(&mut if_body, state, context)?;
                let else_body = if has_else {
                    let mut else_body = Vec::new();
                    let ends_with_else = wimplify_instrs(&mut else_body, state, context)?;
                    assert!(!ends_with_else, "else block must end with end instruction");
                    Some(Body(else_body))
                } else {
                    None
                };

                // Wrap `if` inside a `block`, because Wimpl ifs don't have a label, but if a 
                // branch wants to exit the if, it needs to target a label.
                // TODO do not generate the surrounding block if no branch targets it
                // -> requires a precomputed map from branches to targets
                stmts_result.push(Stmt::Block {
                    end_label: label,
                    body: Body(vec![Stmt::If {
                        condition,
                        if_body: Body(if_body),
                        else_body,
                    }])
                });
            },

            highlevel::Instr::Else => {
                // Cannot pop because you still want it to be on the label stack while processing the else body.
                let (_, result_info) = *state.label_stack.last().expect("label stack should include if label");

                // assign of the if statement that we just finished processing
                // the required value returned by if (if any) should be at the top of the var_stack
                if let Some((if_result_var, type_, is_loop_block)) = result_info {
                    assert!(!is_loop_block, "if block result should never be have loop flag set");
                    stmts_result.push(Stmt::Assign {
                        lhs: if_result_var,
                        rhs: VarRef(var_stack.pop().expect("if block is producing a value which is expected on the stack")),
                        type_,
                    })
                }

                // End recursive invocation and return converted body of the current block.
                return Ok(true);
            },

            highlevel::Instr::End => {
                let (_, result_info) = state.label_stack.pop().expect("end of a block expects the matching label to be in the label stack");

                if let Some((block_result_var, type_, _is_loop_block)) = result_info {
                    stmts_result.push(Stmt::Assign {
                        lhs: block_result_var,
                        rhs: VarRef(var_stack.pop().expect("the block is producing a value for which it expect a value on the stack")),
                        type_,
                    });
                };

                // End recursive invocation and return converted body of the current block.
                return Ok(false)
            }

            highlevel::Instr::Br(label) => {
                stmts_result.extend(
                    label_to_instrs(*label, &state.label_stack, &mut || var_stack.pop()
                        .expect("br expected a value to return")))
            },

            highlevel::Instr::BrIf(label) => {
                let condition = var_stack.pop().expect("if requires a conditional statement");
                stmts_result.push(Stmt::If {
                    condition,
                    if_body: Body(label_to_instrs(*label, &state.label_stack, &mut || {
                        *var_stack.last().expect("br_if expected value to return")
                    })),
                    else_body: None,
                })
            }

            highlevel::Instr::BrTable { table, default } => {
                let index = var_stack.pop().expect("br_table requires an index into the table to be supplied");

                let mut should_pop = false;
                let get_result_val = &mut || {
                    should_pop = true;
                    *var_stack.last().expect("br_table expected value to return")
                };

                let default = Body(label_to_instrs(*default, &state.label_stack, get_result_val));
                let cases = table.iter().copied().map(|label| Body(label_to_instrs(label, &state.label_stack, get_result_val))).collect();

                if should_pop {
                    var_stack.pop().expect("last succeeded, so pop should as well");
                }

                stmts_result.push(Stmt::Switch {
                    index,
                    cases,
                    default,
                })
            }

            highlevel::Instr::Return => {
                // This points to the block for the overall function body.
                let target = Label(0);

                if let (target_from_stack, Some((return_var, type_, loop_flag))) = *state.label_stack.first().expect("empty label stack, but expected function ") {
                    assert_eq!(target, target_from_stack, "label stack is invalid, should have been the function label");
                    assert!(!loop_flag, "function should not have loop flag set");

                    let return_val = var_stack.pop().expect("return expects a return value");
                    stmts_result.extend([
                        Stmt::Assign{
                            lhs: return_var,
                            type_,
                            rhs: VarRef(return_val)
                        },
                        Stmt::Br { target }
                    ])
                } else {
                    stmts_result.push(Stmt::Br { target })
                }
            }

            highlevel::Instr::Call(func_index) => {
                let n_args = context.module.function(*func_index).type_.inputs().len();
                let rhs = Call {
                    func: FunctionId::from_idx(*func_index, context.module),
                    args: var_stack.split_off(var_stack.len() - n_args),
                };
                stmts_result.push(match ty.results() {
                    [] => Stmt::Expr(rhs),
                    [type_] => Stmt::Assign {
                        lhs: create_fresh_stack_var(state, &mut var_stack),
                        rhs,
                        type_: *type_,
                    },
                    _ => panic!("WebAssembly multi-value extension")
                })
            },

            highlevel::Instr::CallIndirect(func_type, table_index) => {
                assert_eq!(table_index.into_inner(), 0, "WebAssembly MVP must always have a single table");

                let rhs = CallIndirect{
                    type_: func_type.clone(),
                    table_idx: var_stack.pop().expect("call_indirect requires an index"),
                    args: var_stack.split_off(var_stack.len() - func_type.inputs().len()),
                };
                stmts_result.push(match ty.results() {
                    [] => Stmt::Expr(rhs),
                    [type_] => Stmt::Assign {
                        lhs: create_fresh_stack_var(state, &mut var_stack),
                        rhs,
                        type_: *type_
                    },
                    _ => panic!("WebAssembly multi-value extension")
                })
            }

            highlevel::Instr::Drop => {
                // Nothing happens, the stack variable is just not used anymore.
            },

            highlevel::Instr::Select => {
                let condition = var_stack.pop().expect("select requires a value on the stack for the condition");
                let if_result_val = var_stack.pop().expect("select requires a value on the stack for the then case");
                let else_result_val = var_stack.pop().expect("select requires a value on the stack for the else case");
                let type_ = ty.results()[0];
                let lhs = create_fresh_stack_var(state, &mut var_stack);
                stmts_result.extend([
                    Stmt::Assign{
                        lhs,
                        rhs: Const(Val::get_default_value(type_)),
                        type_
                    },
                    Stmt::If {
                        condition,
                        if_body: Body(vec![Stmt::Assign {
                            lhs,
                            rhs: VarRef(if_result_val),
                            type_
                        }]),
                        else_body: Some(Body(vec![Stmt::Assign {
                            lhs,
                            rhs: VarRef(else_result_val),
                            type_
                        }]))
                    }
                ])
            }

            highlevel::Instr::Local(highlevel::LocalOp::Get, local_idx) => {
                stmts_result.push(Stmt::Assign{
                    rhs: VarRef(local_idx_to_var(context, *local_idx)),
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results()[0],
                })
            }

            highlevel::Instr::Local(highlevel::LocalOp::Set, local_idx) => {
                stmts_result.push(Stmt::Assign {
                    lhs: local_idx_to_var(context, *local_idx),
                    type_: ty.inputs()[0],
                    rhs: VarRef(var_stack.pop().expect("local.set expects a value on the stack")),
                })
            }

            highlevel::Instr::Local(highlevel::LocalOp::Tee, local_idx) => {
                stmts_result.push(Stmt::Assign{
                    lhs: local_idx_to_var(context, *local_idx),
                    type_: ty.inputs()[0],
                    rhs: VarRef(*var_stack.last().expect("local.tee expects a value on the stack")),
                })
            }

            highlevel::Instr::Global(highlevel::GlobalOp::Get, global_ind) => {
                let global_var = Global(global_ind.into_inner());
                stmts_result.push(Stmt::Assign{
                    rhs: VarRef(global_var),
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results()[0],
                })
            }

            highlevel::Instr::Global(highlevel::GlobalOp::Set, global_ind) => {
                let global_var = Global(global_ind.into_inner());
                stmts_result.push(Stmt::Assign {
                    lhs: global_var,
                    rhs: VarRef(var_stack.pop().expect("global.set expects a value on the stack")),
                    type_: *ty.inputs().get(0).expect("return type of global.set not found"),
                })
            }

            highlevel::Instr::Load(loadop, memarg) => {
                let rhs = var_stack.pop().expect("Every load consumes a value");
                stmts_result.push(Stmt::Assign {
                    rhs: Load {
                        op: *loadop,
                        memarg: *memarg,
                        addr: rhs,
                    },
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results()[0],
                })
            }

            highlevel::Instr::Store(op, memarg) => {
                stmts_result.push(Stmt::Store {
                    op: *op,
                    memarg: *memarg,
                    addr: var_stack.pop().expect("store consumes a value for address from the stack which was not found"),
                    value: var_stack.pop().expect("store consumes a value to store at the address from the stack which was not found"),
                })
            }

            highlevel::Instr::MemorySize(memory_idx) => {
                assert_eq!(memory_idx.into_inner(), 0, "wasm mvp only has single memory");

                stmts_result.push(Stmt::Assign {
                    rhs: MemorySize{},
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results()[0],
                })
            }

            highlevel::Instr::MemoryGrow(memory_idx) => {
                assert_eq!(memory_idx.into_inner(), 0, "wasm mvp only has single memory");

                stmts_result.push(Stmt::Assign {
                    rhs: MemoryGrow {
                        pages: var_stack.pop().expect("memory.grow has to consume a value from stack"),
                    },
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results()[0],
                })
            }

            highlevel::Instr::Const(val) => {
                stmts_result.push(Stmt::Assign{
                    rhs: Const(*val),
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results()[0],
                })
            }

            highlevel::Instr::Numeric(op) => {
                stmts_result.push(Stmt::Assign {
                    rhs: Numeric {
                        op: *op,
                        args: var_stack.split_off(var_stack.len() - ty.inputs().len()),
                    },
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results()[0],
                })
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
        for (loc_index, loc) in func.locals() {
            let (loc_name, loc_type) = (&loc.name, loc.type_);
            if let Some(_loc_name) = loc_name {
                todo!("you haven't yet implemented locals having names");
            } else {
                stmts_result.push(Stmt::Assign {
                    lhs: Var::Local(loc_index.into_inner() - func.type_.inputs().len()),
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
