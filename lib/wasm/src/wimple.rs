
use crate::{highlevel::{self, Module, Function}, types::types};
use crate::highlevel::Instr::*;
use crate::Val;
use crate::highlevel::NumericOp::*;



#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Var (usize); 

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instr { 
    pub lhs : Vec<Var>,
    pub op : highlevel::Instr, 
    pub args : Vec<Var>,
} 

impl Instr { 
    pub fn new(lhs: Vec<Var>, op : highlevel::Instr, args: Vec<Var>) 
    -> Self {
        Instr {
            lhs,
            op,
            args,
        }
    }
}

pub fn wimplify (
    instrs: &[highlevel::Instr],
    function: &Function,
    module: &Module,
) -> Result<Vec<Instr>, String> {
    let mut var_stack = Vec::new();
    let mut var_count = 0; 
    let mut result_instrs = Vec::new();
    let tys = types(instrs, function, module).map_err(|e| format!("{:?}", e))?;
    
    for (instr, ty) in instrs.iter().zip(tys.into_iter()) {
        println!("{:?}, {:?}", instr, ty);
        let n_inputs = ty.inputs.len();
        let n_results = ty.results.len();

        let result_instr = {
            
            let mut args = Vec::new(); 
            for _ in 0..n_inputs {
                args.push(var_stack.pop().unwrap()); 
            }
            
            let mut lhs = Vec::new(); 
            for _ in 0..n_results {
                lhs.push(Var(var_count));
                var_stack.push(Var(var_count)); 
                var_count += 1; 
            }  
            
            Instr {
                lhs : lhs, 
                op : instr.clone(), 
                args : args, 
            }
        }; 
        result_instrs.push(result_instr);
    } 

    Ok(result_instrs)
}

#[test]
fn constant() {
    let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..1];
    let actual = wimplify(instrs, func, &module).unwrap();
    //println!("actual {:?}",actual); 
    let expected = vec![
        Instr {
            lhs : vec![Var(0)], 
            op : Const(Val::I32(3)), 
            args : Vec::new(), 
        }
        ];
    assert_eq!(actual, expected);
}


#[test]
fn drop() {
    let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..2];
    let actual = wimplify(instrs, func, &module).unwrap();
    println!("actual {:?}",actual); 
    let expected = vec![
        Instr {
            lhs : vec![Var(0)], 
            op : Const(Val::I32(3)), 
            args : Vec::new(), 
        },
        Instr {
            lhs : Vec::new(), 
            op : Drop, 
            args : vec![Var(0)], 
        }        
    ];
    assert_eq!(actual, expected);
}

#[test]
fn add() {
    let module = Module::from_file("../../tests/inputs/folding/add.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..3];
    let actual = wimplify(instrs, func, &module).unwrap();
    let expected = vec![
        Instr::new(vec![Var(0)], Const(Val::I32(3)) , Vec::new()),
        Instr::new(vec![Var(1)], Const(Val::I32(4)) , Vec::new()),
        Instr::new(vec![Var(2)], Numeric(I32Add), vec![Var(1), Var(0)]),  
    ];
    println!("{:?}", expected);
    assert_eq!(actual, expected);
}

#[test]
fn call_ind() {
    let module = Module::from_file("../../tests/inputs/folding/call_ind.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..2];
    let actual = wimplify(instrs, func, &module).unwrap();
    println!("{:?}",actual);
    // let expected = vec![FoldedExpr(
    //     CallIndirect(FunctionType::new(&[], &[]), Idx::from(0)),
    //     vec![FoldedExpr::new(Const(Val::I32(0)))],
    // )];
    // assert_eq!(actual, expected);
}

#[test]
fn block_br() {
    let module = Module::from_file("../../tests/inputs/folding/block-br.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body;
    let actual = wimplify(instrs, func, &module).unwrap();
    println!("{:?}",actual);
    // let expected = vec![FoldedExpr(
    //     CallIndirect(FunctionType::new(&[], &[]), Idx::from(0)),
    //     vec![FoldedExpr::new(Const(Val::I32(0)))],
    // )];
    // assert_eq!(actual, expected);
}

/*
#[test]
fn multiple_expr() {
    let module = Module::from_file("../../tests/inputs/folding/multiple.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = func.code().unwrap().body.as_slice();
    // let instrs = &func.code().unwrap().body[0..2];
    let actual = wimplify(instrs, func, &module).unwrap();
    let expected = vec![
        FoldedExpr(
            Numeric(I32Add),
            vec![
                FoldedExpr::new(Const(Val::I32(0))),
                FoldedExpr::new(Const(Val::I32(1))),
            ],
        ),
        FoldedExpr(
            Drop,
            vec![FoldedExpr(
                Numeric(I32Add),
                vec![
                    FoldedExpr::new(Const(Val::I32(2))),
                    FoldedExpr::new(Const(Val::I32(3))),
                ],
            )],
        ),
        FoldedExpr::new(Drop),
    ];
    println!("{}", actual.iter().map(|e| format!("{}", e)).collect::<Vec<_>>().join("\n"));
    assert_eq!(actual, expected);
}

*/