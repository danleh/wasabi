use std::fmt;

use crate::highlevel::Instr::*;
use crate::highlevel::NumericOp::*;
use crate::highlevel::{Function, Instr, Module};
use crate::types::InferredInstructionType;
use crate::types::TypeChecker;
use crate::{FunctionType, Idx, Val};

#[derive(Debug, Eq, PartialEq)]
pub struct FoldedExpr(Instr, Vec<FoldedExpr>);

impl FoldedExpr {
    pub fn new(instr: Instr) -> Self {
        FoldedExpr(instr, Vec::new())
    }
}

impl fmt::Display for FoldedExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print leaf instructions without parentheses.
        if self.1.is_empty() {
            write!(f, "{}", self.0)
        } else {
            write!(
                f,
                "({}, {})",
                self.0,
                self.1
                    .iter()
                    .map(|expr| format!("{}", expr))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

pub fn folds(
    instrs: &[Instr],
    function: &Function,
    module: &Module,
) -> Result<Vec<FoldedExpr>, String> {
    let mut type_checker = TypeChecker::begin_function(function, module);
    let mut stack = Vec::new();
    for instr in instrs {
        let ty = type_checker.check_next_instr(instr).map_err(|e| e.to_string())?;
        println!("{:?}, {:?}", instr, ty);
        if let InferredInstructionType::Reachable(ty) = ty {
            let n_inputs = ty.params.len();
            let n_results = ty.results.len();
            let expr = if n_inputs == 0 {
                FoldedExpr(instr.clone(), Vec::new())
            } else {
                let mut args = Vec::new();
                for _ in 0..n_inputs {
                    let x = stack.pop().unwrap();
                    println!("{:?}", x);
                    args.push(x);
                }
                args.reverse();
                FoldedExpr(instr.clone(), args)
            };
            stack.push(expr);
        }
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
    let expected = vec![FoldedExpr(Drop, vec![FoldedExpr::new(Const(Val::I32(3)))])];
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
        vec![
            FoldedExpr::new(Const(Val::I32(3))),
            FoldedExpr::new(Const(Val::I32(4))),
        ],
    )];
    println!("{}", expected[0]);
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
        vec![FoldedExpr::new(Const(Val::I32(0)))],
    )];
    assert_eq!(actual, expected);
}

#[test]
fn multiple_expr() {
    let module = Module::from_file("../../tests/inputs/folding/multiple.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = func.code().unwrap().body.as_slice();
    // let instrs = &func.code().unwrap().body[0..2];
    let actual = folds(instrs, func, &module).unwrap();
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
