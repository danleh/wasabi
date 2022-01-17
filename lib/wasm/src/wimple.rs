use std::{fmt, path::Path, io::{self, ErrorKind}};

use logos::Logos;
use regex::Regex;

use crate::{highlevel::{self, Module, Function}, types::types};
use crate::highlevel::Instr::*;
use crate::Val;
use crate::highlevel::NumericOp::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum WimpleRhs {
    InstVec(Vec<Instr>), 
    VarVec(Vec<Var>)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Var(usize); 

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instr { 
    pub lhs : Vec<Var>,
    pub op : highlevel::Instr, 
    pub rhs : WimpleRhs,
} 

impl Instr { 
    pub fn new(lhs: Vec<Var>, op : highlevel::Instr, rhs: WimpleRhs) 
    -> Self {
        Instr {
            lhs,
            op,
            rhs,
        }
    }

    pub fn parse(str: &str) -> io::Result<Vec<Self>> {
        for (token, span) in WimplTextToken::lexer(str).spanned() {
            let span = &str[span];
            println!("{:?} {:?}", token, span);
        }
        todo!()
    }

    /// Convenience function to parse Wimpl from a filename.
    pub fn from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Self>> {
        let str = std::fs::read_to_string(filename)?;
        Ok(Self::parse(&str)?)
    }
}

// https://crates.io/crates/logos
#[derive(Logos, Debug, PartialEq)]
pub enum WimplTextToken {
    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,
    
    #[token(",")]
    Comma,

    #[token("=")]
    Equals,
    
    #[regex(r"v\d+")]
    Variable,

    #[regex(r"\s+", logos::skip)]
    Whitespace,

    #[regex("[a-z][a-z0-9.]+")]
    Token,
    
    // #[token("(")]
    // Immediate(String),

    #[error]
    Error,
}

// fn tokenize_wimpl(str: &str) -> io::Result<Vec<WimplTextToken>> {
//     let var_re = Regex::new(r"^v\d+$").unwrap();

//     for token in str.split_inclusive(&['\n', ' ', '(', ')', ','][..]) {
//         // DEBUG
//         println!("{:?}", token);
        
//         use WimplTextToken::*;
//         match token {
//             "=" => Equals,
//             "(" => LParen,
//             ")" => RParen,
//             "," => Comma,
//             " " | "\n" => continue,
//             _ if var_re.is_match(token) => Variable(token.to_string()),

//             _ => return Err(io::Error::new(ErrorKind::Other, format!("unknown token: {:?}", token))),
//         };
//     }
//     todo!()
// }

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // handle the LHS 
        if !self.lhs.is_empty() {
            write!(f, "v{} = ", self.lhs[0].0)?;
        } 
        // op is always printed 
        write!(f, "{}", self.op)?;
        // handle the rhs
        match &self.rhs {
            WimpleRhs::InstVec(inst_vec) => {
                write!(f, "{{\n")?; 
                for inst in inst_vec {
                    write!(f, "{}", inst); 
                }
                write!(f, "\n}}") 
            },
            WimpleRhs::VarVec(var_vec) => {
                if !var_vec.is_empty() {
                    write!(f, "(")?; 
                    for (ind, var) in var_vec.iter().enumerate() {
                        if ind == var_vec.len()-1 {
                            write!(f, "v{}", var.0)?;
                        } else {
                            write!(f, "v{}, ", var.0)?;
                        }
                    }
                    write!(f, ")",)
                } else {
                    write!(f, "")
                }
            },
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
                rhs : WimpleRhs::VarVec(args), 
            }
            
        }; 
        result_instrs.push(result_instr);
    } 

    Ok(result_instrs)
}

#[test]
fn constant_file() {
    let path = "tests/wimpl/const.wimpl"; 
    let expected = Instr::from_file(path).unwrap(); 

    let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = &func.code().unwrap().body[0..1];
    let actual = wimplify(instrs, func, &module).unwrap();
    
    assert_eq!(actual, expected);
}

#[test]
fn constant() {
    let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..1];
    let actual = wimplify(instrs, func, &module).unwrap();
    //println!("actual {:?}",actual); 
    for ins in &actual {
        println!("{}", ins);
    }
    let expected = vec![
        Instr {
            lhs : vec![Var(0)], 
            op : Const(Val::I32(3)), 
            rhs : WimpleRhs::VarVec(Vec::new()), 
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
    //println!("actual {:?}",actual); 
    for ins in &actual {
        println!("{}", ins);
    }
    let expected = vec![
        Instr {
            lhs : vec![Var(0)], 
            op : Const(Val::I32(3)), 
            rhs : WimpleRhs::VarVec(Vec::new()), 
        },
        Instr {
            lhs : Vec::new(), 
            op : Drop, 
            rhs : WimpleRhs::VarVec(vec![Var(0)]), 
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
    for ins in &actual {
        println!("{}", ins);
    }
    let expected = vec![
        Instr::new(vec![Var(0)], Const(Val::I32(3)) , WimpleRhs::VarVec(Vec::new())),
        Instr::new(vec![Var(1)], Const(Val::I32(4)) , WimpleRhs::VarVec(Vec::new())),
        Instr::new(vec![Var(2)], Numeric(I32Add), WimpleRhs::VarVec(vec![Var(1), Var(0)])),  
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
    //println!("{:?}",actual);
    for ins in &actual {
        println!("{}", ins);
    }
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
    //println!("{:?}",actual);
    for ins in &actual {
        println!("{}", ins);
    }
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