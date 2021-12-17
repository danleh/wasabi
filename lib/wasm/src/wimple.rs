
use crate::{highlevel::{self, Module, Function}, types::types};

#[derive(Debug, Eq, PartialEq)]
pub struct Var (usize); 

#[derive(Debug, Eq, PartialEq)]
pub struct Instr { 
    pub lhs : Option<Var>,
    pub op : highlevel::Instr, 
    pub args : Vec<Var>,
} 

pub fn wimplify (
    instrs: &[highlevel::Instr],
    function: &Function,
    module: &Module,
) -> Result<Vec<Instr>, String> {
    // let mut var_stack = Vec::new();
    let mut result = Vec::new();
    let mut var_count = 0; 
    
    let tys = types(instrs, function, module).map_err(|e| format!("{:?}", e))?;
    for (instr, ty) in instrs.iter().zip(tys.into_iter()) {
        println!("{:?}, {:?}", instr, ty);
        let n_inputs = ty.inputs.len();
        let n_results = ty.results.len();
/*
        if n_inputs == 0 { 
            Instr {
                lhs : Var() 
            }
        }
*/
    } 

    Ok(result)
}

#[test]
fn constant() {
    let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..1];
    let actual = wimplify(instrs, func, &module).unwrap();
    
    //let expected = vec![FoldedExpr::new(Const(Val::I32(3)))];
    // assert_eq!(actual, expected);
}

/*
#[test]
fn drop() {
    let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..2];
    let actual = wimplify(instrs, func, &module).unwrap();
    let expected = vec![FoldedExpr(Drop, vec![FoldedExpr::new(Const(Val::I32(3)))])];
    assert_eq!(actual, expected);
}

#[test]
fn add() {
    let module = Module::from_file("../../tests/inputs/folding/add.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..3];
    let actual = wimplify(instrs, func, &module).unwrap();
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
    let actual = wimplify(instrs, func, &module).unwrap();
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