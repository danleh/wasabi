use std::collections::HashMap;

use crate::wimpl::*;
use crate::wimpl::wimplify::*;

// TODO optimization to unwrap blocks/loops whose label is never the target of any branch.
// Just needs two forward passes: 1. collect all branch targets in a HashSet, 2. unwrap block if not contained.
// Useful to remove the block generated from WebAssembly ifs


// TODO optimization to replace assignments to dead variables with expression statements.

// TODO optimization to replace pure (side-effect free) expression statements.

// TODO constant propagation

#[derive(Debug)]
enum Value {
    Top, //TODO we dont have any reassignments so top will never be assigned 
    Var(Var)
}

fn populate_var_map<'module>(
    stmts: &mut impl Iterator<Item=&'module Stmt>, 
    map: &mut HashMap<Var, Value>,
    rev_map: &mut HashMap<Var, Var>,
    replaced_vars: &mut Vec<Var>
) -> Vec<Stmt> {
    let mut res: Vec<Stmt> = Vec::new(); 

    while let Some(stmt) = stmts.next() { 
        
        let mut check = |
            lhs: Var,
            map: &mut HashMap<Var, Value>, 
            replaced_vars: &mut Vec<Var>
        | -> Option<Var> { //why on earth should it be mutable 
            if map.contains_key(&lhs) {
                match *map.get_mut(&lhs).expect("lhs should exist in var map") {
                    Value::Top => None,
                    Value::Var(rhs) => {
                        replaced_vars.push(lhs.clone()); 
                        Some(rhs)
                    }
                }
            } else {
                None   
            }
        }; 

        let mut check_vec = |
            lhs_vec: Vec<Var>,
            map: &mut HashMap<Var, Value>, 
            replaced_vars: &mut Vec<Var>, 
        | -> Vec<Var> {
            let mut res_vec = Vec::new();
            for lhs in lhs_vec {
                if let Some(replacement_lhs) = check(lhs, map, replaced_vars) {
                    res_vec.push(replacement_lhs)
                } else {
                    res_vec.push(lhs); 
                }
            }; 
            res_vec
        };
        

        match stmt {
            
            Stmt::Assign { lhs, type_, rhs } => {
                
                if rev_map.contains_key(lhs) {
                    let (_, val) = rev_map.remove_entry(lhs).unwrap();
                    *map.get_mut(&val).expect("variable should exist in rev_map") = Value::Top;
                }

                let stmt_type = *type_; 

                match rhs {
                    
                    Expr::VarRef(var) => {
                        
                        if map.contains_key(lhs) {
                            *map.get_mut(lhs).expect("lhs should exist in var map") = Value::Top; 
                        } else {
                            map.insert(*lhs, Value::Var(*var));
                            rev_map.insert(*var, *lhs);  
                        }
                        /* 
                        if let Some(replacement_var) = check(*var) {
                            res.push(Stmt::Assign{
                                lhs: *lhs,
                                type_: stmt_type,
                                rhs: Expr::VarRef(replacement_var),                                
                            })
                        } else {
                            res.push(stmt.clone()); 
                        }
                        */
                        res.push(stmt.clone()); 
                    },

                    Expr::Const(_var_) => res.push(stmt.clone()), 

                    Expr::Load { op, memarg, addr } => {
                        if let Some(replacement_var) = check(*addr, map, replaced_vars) {
                            res.push(Stmt::Assign{
                                lhs: *lhs,
                                type_: stmt_type,
                                rhs: Expr::Load{
                                    op: *op,
                                    memarg: *memarg,
                                    addr: replacement_var,
                                },
                            })
                        } else {
                            res.push(stmt.clone()); 
                        } 
                    },

                    Expr::MemoryGrow { pages } => {
                        if let Some(replacement_var) = check(*pages, map, replaced_vars) {
                            res.push(Stmt::Assign{
                                lhs: *lhs,
                                type_: stmt_type,
                                rhs: Expr::MemoryGrow {
                                    pages: replacement_var,
                                },
                            })
                        } else {
                            res.push(stmt.clone()); 
                        } 
                    },
                    
                    Expr::Numeric { op, args } => {
                        res.push(Stmt::Assign{
                            lhs: *lhs,
                            type_: stmt_type,
                            rhs: Expr::Numeric{
                                op: *op, 
                                args: check_vec(args.clone(), map, replaced_vars) 
                            },
                        })                        
                    } 

                    Expr::Call { func, args } => {
                        res.push(Stmt::Assign{
                            lhs: *lhs,
                            type_: stmt_type, 
                            rhs: Expr::Call{
                                func: func.clone(), 
                                args: check_vec(args.clone(), map, replaced_vars) 
                            },
                        })
                    }

                    Expr::CallIndirect { type_, table_idx, args } => {
                        res.push(Stmt::Assign{
                            lhs: *lhs,
                            type_: stmt_type,
                            rhs: Expr::CallIndirect{
                                args: check_vec(args.clone(), map, replaced_vars),
                                type_: type_.clone(),
                                table_idx: *table_idx, 
                            },
                        })
                    },
                    
                    Expr::MemorySize => res.push(stmt.clone()),

                }
            },
            
            Stmt::Block { body, end_label: _ } | 
            Stmt::Loop { begin_label: _, body } => {
                let mut body_stmts = body.0.iter();
                let body_stmts = populate_var_map(&mut body_stmts, map, rev_map, replaced_vars);
                res.push(match stmt {
                    
                    Stmt::Block { body: _, end_label } => {
                        Stmt::Block{
                            body: Body(body_stmts),
                            end_label: *end_label,
                        }
                    },

                    Stmt::Loop { begin_label, body: _ } => {
                        Stmt::Loop{
                            begin_label: *begin_label,
                            body: Body(body_stmts),
                        }
                    },
                    
                    _ => panic!("no statements other than block and loop expected")  
                })
            }, 

            Stmt::If { condition , if_body, else_body } => {
                let mut if_body_stmts = if_body.0.iter();
                let if_body = Body(populate_var_map(&mut if_body_stmts, map, rev_map, replaced_vars));

                let else_body = if let Some(else_body) = else_body {
                    let mut else_body_stmts = else_body.0.iter();
                    Some(Body(populate_var_map(&mut else_body_stmts, map, rev_map, replaced_vars)))    
                } else {
                    None
                }; 

                res.push(Stmt::If{
                    condition: *condition,
                    if_body,
                    else_body,
                })
            },

            Stmt::Expr(expr) => {
                match expr {
                    
                    Expr::VarRef(_) => panic!("dangling variable"),
                    
                    Expr::Const(_) => res.push(stmt.clone()),
                    Expr::MemorySize => res.push(stmt.clone()),

                    Expr::Load { op, memarg, addr } => {
                        if let Some(replacement_var) = check(*addr, map, replaced_vars) {
                            res.push(Stmt::Expr(Expr::Load{
                                op: *op,
                                memarg: *memarg,
                                addr: replacement_var,
                                })
                            )
                        } else {
                            res.push(stmt.clone()); 
                        } 
                    },

                    Expr::MemoryGrow { pages } => {
                        if let Some(replacement_var) = check(*pages, map, replaced_vars) {
                            res.push(Stmt::Expr(Expr::MemoryGrow {
                                    pages: replacement_var,
                                }
                            ))
                        } else {
                            res.push(stmt.clone()); 
                        } 
                    },
                    
                    Expr::Numeric { op, args } => {
                        res.push(Stmt::Expr(Expr::Numeric{
                                op: *op, 
                                args: check_vec(args.clone(), map, replaced_vars) 
                            },
                        ))                        
                    } 

                    Expr::Call { func, args } => {
                        res.push(Stmt::Expr(Expr::Call{
                                func: func.clone(), 
                                args: check_vec(args.clone(), map, replaced_vars) 
                            },
                        ))
                    }

                    Expr::CallIndirect { type_, table_idx, args } => {
                        res.push(Stmt::Expr(Expr::CallIndirect{
                                args: check_vec(args.clone(), map, replaced_vars),
                                type_: type_.clone(),
                                table_idx: *table_idx, 
                            },
                        ))
                    },
                }
            }
            
            _ => res.push(stmt.clone()) 
        }
    }; 
    res
}

fn cleanup (stmts: &[Stmt], replaced_vars: &[Var]) -> Vec<Stmt>{
    let  mut cleaned_stmts = Vec::new(); 
    for stmt in stmts {
        match stmt {
            Stmt::Assign { lhs, type_: _, rhs: Expr::VarRef(_) } => {
                if !replaced_vars.contains(lhs) {
                    cleaned_stmts.push(stmt.clone());    
                } 
            },

            Stmt::Loop {begin_label: _, body} |
            Stmt::Block{ body, end_label: _} => {
                let body_stmts = &body.0;
                let body = Body(cleanup(body_stmts, replaced_vars));
                cleaned_stmts.push(match stmt {

                    Stmt::Loop {begin_label, body: _} => {
                        Stmt::Loop{
                            begin_label: *begin_label,
                            body,
                        }
                    }, 
                    
                    Stmt::Block{ body: _, end_label} => {
                        Stmt::Block{
                            body,
                            end_label: *end_label,
                        }
                    }, 
                    _ => panic!("should not get anything other than block or loop here")
                }
                )
            },
            
            Stmt::If{ condition, if_body, else_body} => {
                let if_body_stmts = &if_body.0;
                let if_body = Body(cleanup(if_body_stmts, replaced_vars));

                let else_body = if let Some(else_body) = else_body {
                    let else_body_stmts = &else_body.0;
                    Some(Body(cleanup(else_body_stmts, replaced_vars)))
                } else {
                    None
                }; 

                cleaned_stmts.push(Stmt::If{
                    condition: *condition,
                    if_body,
                    else_body,
                })
                
            }, 

            _ => cleaned_stmts.push(stmt.clone()),
        }
    }
    cleaned_stmts 
}   

fn constant_propogation (module: &mut Module) -> Module {
    let mut opt_funcs = Vec::new();  
    for func in module.functions.clone() { //TODO dont want to clone and yet 
        let mut stmts = func.body.0.iter();
        let mut map = HashMap::new(); 
        let mut rev_map = HashMap::new(); 
        let mut replaced_vars = Vec::new(); 
        let opt_stmts = populate_var_map(&mut stmts, &mut map, &mut rev_map, &mut replaced_vars); 
        let cleaned_stmts = cleanup(&opt_stmts, &replaced_vars);                
        opt_funcs.push(Function{
            name: func.name,
            type_: func.type_,
            body: Body(cleaned_stmts),
            export: func.export,
        })
    }; 
    Module { functions: opt_funcs, globals: module.globals.clone(), tables: module.tables.clone() }
}

pub fn wimpl_optimize (path: impl AsRef<Path>) -> Result<Module, String> {
    let mut module = Module::from_wasm_file(path)?;
    let module = constant_propogation(&mut module);   
    Ok(module)   
}

#[test]
fn opt_test() {
    let module = wimpl_optimize("tests/wimpl/wimplify_expected/updated_local/local.wasm");
    //let module = wimpl_optimize("tests/wimpl/wimplify_expected/global/global.wasm");
    println!("{}", module.unwrap()); 
}