use crate::{Val, FunctionType, Idx};
use crate::highlevel::{Instr, Function, Module};
use crate::highlevel::Instr::*;
use crate::highlevel::NumericOp::*;
use crate::types::types;

#[derive(Debug, Eq, PartialEq)]
pub struct FoldedExpr(Instr, Vec<FoldedExpr>);

impl FoldedExpr {
    pub fn new(instr: Instr) -> Self {
        FoldedExpr(instr, Vec::new())
    }
}

pub fn folds(instrs: &[Instr], function: &Function, module: &Module) -> Result<Vec<FoldedExpr>, String> {
    let tys = types(instrs, function, module).map_err(|e| format!("{:?}", e))?;
    let mut stack = Vec::new();
    for (instr, ty) in instrs.iter().zip(tys.into_iter()) {
        println!("{:?}, {:?}",instr, ty);
        let n_inputs = ty.inputs.len();
        let n_results = ty.results.len();
        let expr = if n_inputs == 0 {
            FoldedExpr(instr.clone(), Vec::new())
        } else {
            let mut args = Vec::new();
            for _ in 0..n_inputs {
                let x = stack.pop().unwrap();
                println!("{:?}",x);
                args.push(x);
            }
            args.reverse();
            FoldedExpr(instr.clone(), args)
        };
        stack.push(expr);
    }
    Ok(stack)
}

#[test]
fn constant() {
    let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..1];
    let actual = folds(instrs, func, &module).unwrap();
    let expected = vec![FoldedExpr::new(Const(Val::I32(3)))];
    assert_eq!(actual, expected);
}

#[test]
fn drop() {
    let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..2];
    let actual = folds(instrs, func, &module).unwrap();
    let expected = vec![FoldedExpr(
        Drop,
        vec![FoldedExpr::new(Const(Val::I32(3)))])];
    assert_eq!(actual, expected);
}

#[test]
fn add() {
    let module = Module::from_file("../../tests/inputs/folding/add.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..3];
    let actual = folds(instrs, func, &module).unwrap();
    let expected = vec![FoldedExpr(
        Numeric(I32Add),
        vec![FoldedExpr::new(Const(Val::I32(3))), 
            FoldedExpr::new(Const(Val::I32(4)))]
        )];
    assert_eq!(actual, expected);
}

#[test]
fn call_ind() {
    let module = Module::from_file("../../tests/inputs/folding/call_ind.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..2];
    let actual = folds(instrs, func, &module).unwrap();
    let expected = vec![FoldedExpr(
        CallIndirect(FunctionType::new(&[], &[]), Idx::from(0)),
        vec![
            FoldedExpr::new(Const(Val::I32(0)))
        ]
        )];
    assert_eq!(actual, expected);
}
