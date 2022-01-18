use std::{
    fmt::{self, Write},
    io::{self, ErrorKind},
    path::Path,
};

use logos::{Lexer, Logos};
use regex::Regex;

use crate::{Val, highlevel::MemoryOp, ValType};
use crate::{
    highlevel::{self, Function, LoadOp, Module, NumericOp, StoreOp},
    types::types,
    FunctionType, Memarg,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Var {
    Stack(usize),
    Local(usize),
    Global(usize),
    Param(usize),
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Var::*;
        match self {
            Stack(i) => write!(f, "s{}", i),
            Local(i) => write!(f, "l{}", i),
            Global(i) => write!(f, "g{}", i),
            Param(i) => write!(f, "p{}", i),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Func(usize);

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Label(usize);

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@label{}", self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Body {
    instrs: Vec<Instr>,
    result: Option<Var>,
}

const BLOCK_INDENT: &'static str = "  ";

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Put each inner instruction and the result on a separate line.
        let mut inner = String::new();
        for instr in &self.instrs {
            writeln!(inner, "{}", instr)?;
        }
        if let Some(result) = self.result {
            writeln!(inner, "{}", result)?;
        }
        // Pop-off the last superfluous newline (or do nothing if empty).
        inner.pop();

        // If there is only a single result without instructions, or only a
        // single-line instructions (such as br @label), format on one line.
        match inner.as_ref() {
            "" => f.write_str("{}"),
            single_line if single_line.lines().count() == 1 => write!(f, "{{ {} }}", single_line),
            multi_line => {
                // Recursively indent the inner blocks.
                let multi_line = multi_line.replace("\n", &format!("\n{}", BLOCK_INDENT));
                write!(f, "{{\n{}{}\n}}", BLOCK_INDENT, multi_line)
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Instr {
    // no nop

    // unreachable
    Unreachable,

    // s0 = @label0: block {
    //     s1 = i32.const 3
    //     s1
    // }
    Block {
        lhs: Option<Var>,
        label: Label,
        body: Body,
    },
    // s0 = @label0: loop {
    //     s1 = i32.const 3
    //     br @label0 (s1)
    //     s1
    // }
    Loop {
        lhs: Option<Var>,
        label: Label,
        body: Body,
    },
    // s0 = @label0: if {
    //     s1 = i32.const 3
    //     s1
    // } else {
    //     s2 = i32.const 6
    //     s2
    // }
    If {
        lhs: Option<Var>,
        // No label for an if generated from select or br_if.
        label: Option<Label>,
        condition: Var,
        // Invariant: if and else must either both return value or none
        if_body: Body,
        else_body: Option<Body>,
    },

    // br @label0 (s1)
    Br {
        target: Label,
        value: Option<Var>,
    },
    // br-table @label0, @label1, @label2, default=label@3 (s0)
    BrTable {
        idx: Var,
        table: Vec<Label>,
        default: Label,
        value: Option<Var>,
    },

    // return (s1)
    Return {
        value: Option<Var>,
    },

    // s0 = call f1 (s1)
    Call {
        lhs: Option<Var>,
        func: Func,
        args: Vec<Var>,
    },
    // s2 = call_indirect []->[i32] (s0) (s1, s2, s3...)
    CallIndirect {
        lhs: Option<Var>,
        type_: FunctionType,
        table_idx: Var,
        args: Vec<Var>,
    },

    // don't generate Wimpl Instr for Drop, but remove stack variable
    // encode Select -> If { lhs : s0, label = fresh(), condition=pop stack, if-branch=vec[pop stack], else-branch=vec[pop-stack], }

    // local.set, global.set, local.tee, local.get, global.get
    // s2 = s1
    Assign {
        lhs: Var,
        rhs: Var,
    },

    // s1 = i32.load offset=3 align=4 (s0)
    Load {
        lhs: Var,
        op: LoadOp,
        memarg: Memarg,
        addr: Var,
    },
    // i32.store offset=3 align=4 (s0//addr) (s1//value)
    Store {
        op: StoreOp,
        memarg: Memarg,
        value: Var,
        addr: Var,
    },

    // s1 = memory.size
    MemorySize {
        lhs: Var,
    },
    // s1 = memory.grow(s0)
    MemoryGrow {
        lhs: Var,
        new_size: Var,
    },

    // s1 = i32.const 3
    Const {
        lhs: Var,
        val: Val,
    },

    // s2 = i32.add(s0, s1)
    // s1 = f32.neg(s0)
    Numeric {
        lhs: Var,
        op: NumericOp,
        rhs: Vec<Var>,
    },
}

impl Instr {
    /// Convenience accessor for all instructions that have a LHS.
    pub fn lhs(&self) -> Option<Var> {
        use Instr::*;
        match self {
            Unreachable => None,

            Block { lhs, .. } => *lhs,
            Loop { lhs, .. } => *lhs,
            If { lhs, .. } => *lhs,

            Br { .. } => None,
            BrTable { .. } => None,
            Return { .. } => None,

            Call { lhs, .. } => *lhs,
            CallIndirect { lhs, .. } => *lhs,

            Assign { lhs, .. } => Some(*lhs),

            Load { lhs, .. } => Some(*lhs),
            Store { .. } => None,

            MemorySize { lhs } => Some(*lhs),
            MemoryGrow { lhs, .. } => Some(*lhs),

            Const { lhs, .. } => Some(*lhs),
            Numeric { lhs, .. } => Some(*lhs),
        }
    }

    // pub fn new(lhs: Vec<Var>, op: highlevel::Instr, rhs: WimpleRhs) -> Self {
    //     Instr { lhs, op, rhs }
    // }

    // pub fn parse_instr(mut lexer: Lexer<WimplTextToken>) -> io::Result<Self> {
    //     let mut instr = Instr {
    //         lhs: Vec::new(),
    //         op: highlevel::Instr::Nop,
    //         rhs: WimpleRhs::VarVec(Vec::new()),
    //     };

    //     match lexer.next() {
    //         Some(WimplTextToken::Variable(i)) => instr.lhs.push(Var(i)),
    //         None => {
    //             return Err(io::Error::new(
    //                 ErrorKind::UnexpectedEof,
    //                 "missing Wimpl instruction",
    //             ))
    //         }
    //         Some(WimplTextToken::AlphaNum) => {
    //             let start_instr = lexer.span().start;
    //             println!("in AlphaNum: start={}", start_instr);

    //             let mut current = lexer.next().unwrap();
    //             while current != WimplTextToken::LParen && current != WimplTextToken::Linebreak {
    //                 current = lexer.next().unwrap();
    //             }
    //             let end_instr = lexer.span().end - 1;
    //             println!("in AlphaNum: end={}", end_instr);

    //             let op_str = &lexer.source()[start_instr..end_instr].trim();
    //             println!("in AlphaNum: op_str={:?}", op_str);

    //             instr.op = highlevel::Instr::parse_text(op_str)
    //                 .map_err(|_| io::Error::new(ErrorKind::Other, "unknown"))?;

    //             // TODO parse argument list
    //         }
    //         _ => todo!(),
    //     }

    //     Ok(instr)
    // }

    // pub fn parse_args_list(lexer) -> ... {
    //     expect_token(LParen)
    //     while next_token != RParen
    //         if Comma => Ignore
    //         if Variable => add to result
    //         else ERROR
    //     return result vec
    // }

    // pub fn parse(str: &str) -> io::Result<Vec<Self>> {
    //     let mut lexer = WimplTextToken::lexer(str);
    //     Ok(vec![Self::parse_instr(lexer)?])

    //     // program = instr*
    //     // instr = (variable '=')? operator args?
    //     // args = '(' (variable ',')* variable ')'
    // }

    // /// Convenience function to parse Wimpl from a filename.
    // pub fn from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Self>> {
    //     let str = std::fs::read_to_string(filename)?;
    //     Ok(Self::parse(&str)?)
    // }
}

/// Helper function for `fmt::Display`-ing an arbitrary iterator of `values`,
/// where each element is separated by `delim` and if the iterator is non-empty
/// surrounded by `begin` and `end`.
fn display_delim<T>(
    f: &mut fmt::Formatter<'_>,
    values: T,
    begin: &str,
    end: &str,
    delim: &str,
) -> fmt::Result
where
    T: IntoIterator,
    T::IntoIter: DoubleEndedIterator,
    T::Item: fmt::Display
{
    let mut iter = values.into_iter();
    match iter.next_back() {
        // Empty iterator, don't format anything.
        None => Ok(()),
        Some(last) => {
            f.write_str(begin)?;
            for item in iter {
                write!(f, "{}{}", item, delim)?;
            }
            write!(f, "{}", last)?;
            f.write_str(end)
        }
    }
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(lhs) = self.lhs() {
            write!(f, "{} = ", lhs)?;
        }

        use Instr::*;
        match self {
            Unreachable => f.write_str("unreachable")?,

            Block { lhs: _, label, body } => write!(f, "{}: block {}", label, body)?,
            Loop { lhs: _, label, body } => write!(f, "{}: loop {}", label, body)?,
            If {
                lhs: _,
                condition,
                label,
                if_body,
                else_body,
            } => {
                if let Some(label) = label {
                    write!(f, "{}: ", label)?;
                }
                write!(f, "if ({}) {}", condition, if_body)?;
                if let Some(else_branch) = else_body {
                    write!(f, "\nelse {}", else_branch)?;
                }
            }

            Br { target, value } => {
                write!(f, "br {}", target)?;
                display_delim(f, value, " (", ")", ", ")?;
            }
            BrTable {
                idx: index,
                table,
                default,
                value,
            } => {
                f.write_str("br_table")?;
                display_delim(f, table, " ", "", " ")?;
                write!(f, " default={} ({})", default, index)?;
                display_delim(f, value, " (", ")", ", ")?;
            }
            Return { value } => {
                f.write_str("return")?;
                display_delim(f, value, " (", ")", ", ")?;
            },

            Call { lhs: _, func, args } => {
                write!(f, "call {} (", func)?;
                display_delim(f, args, "", "", ", ")?;
                f.write_str(")")?;
            }

            CallIndirect {
                lhs: _,
                type_,
                table_idx: table_index,
                args,
            } => {
                write!(f, "call_indirect {} ({}) (", type_, table_index)?;
                display_delim(f, args, "", "", ", ")?;
                f.write_str(")")?;
            }

            Assign { lhs: _, rhs } => write!(f, "{}", rhs)?,

            Load {
                lhs: _,
                addr,
                op,
                memarg,
            } => {
                write!(f, "{}", op)?;
                if !memarg.is_default(*op) {
                    f.write_str(" ")?;
                }
                memarg.fmt(f, *op)?;
                if !memarg.is_default(*op) {
                    f.write_str(" ")?;
                }
                write!(f, "({})", addr)?;
            }
            Store {
                value,
                addr,
                op,
                memarg,
            } => {
                write!(f, "{}", op)?;
                if !memarg.is_default(*op) {
                    f.write_str(" ")?;
                }
                memarg.fmt(f, *op)?;
                if !memarg.is_default(*op) {
                    f.write_str(" ")?;
                }
                write!(f, "({}) ({})", addr, value)?;
            }
            
            MemorySize { lhs: _ } => write!(f, "memory.size")?,
            MemoryGrow { lhs: _, new_size } => write!(f, "memory.grow({})", new_size)?,

            Const { lhs: _, val } => write!(f, "{}.const {}", val.to_type(), val)?,
            Numeric { lhs: _, op, rhs } => {
                write!(f, "{}", op)?;
                display_delim(f, rhs, "(", ")", ", ")?;
            }
        };

        Ok(())
    }
}

// https://crates.io/crates/logos
#[derive(Logos, Debug, PartialEq, Eq)]
pub enum WimplTextToken {
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("->")]
    Arrow,

    #[token(",")]
    Comma,

    #[token("=")]
    Equals,

    #[regex(r"v\d+", |str| str.slice()[1..].parse())]
    Variable(usize),

    #[token("\n")]
    Linebreak,

    #[regex(r"\s+", logos::skip)]
    Whitespace,

    #[regex(r"[a-zA-Z0-9_\.]+")]
    AlphaNum,

    #[error]
    Error,
}

// pub fn wimplify(
//     instrs: &[highlevel::Instr],
//     function: &Function,
//     module: &Module,
// ) -> Result<Vec<Instr>, String> {
//     let mut var_stack = Vec::new();
//     let mut var_count = 0;
//     let mut result_instrs = Vec::new();
//     let tys = types(instrs, function, module).map_err(|e| format!("{:?}", e))?;

//     for (instr, ty) in instrs.iter().zip(tys.into_iter()) {
//         println!("{:?}, {:?}", instr, ty);
//         let n_inputs = ty.inputs.len();
//         let n_results = ty.results.len();

//         let result_instr = {
//             let mut args = Vec::new();
//             for _ in 0..n_inputs {
//                 args.push(var_stack.pop().unwrap());
//             }

//             let mut lhs = Vec::new();
//             for _ in 0..n_results {
//                 lhs.push(Var(var_count));
//                 var_stack.push(Var(var_count));
//                 var_count += 1;
//             }

//             Instr {
//                 lhs: lhs,
//                 op: instr.clone(),
//                 rhs: WimpleRhs::VarVec(args),
//             }
//         };
//         result_instrs.push(result_instr);
//     }

//     Ok(result_instrs)
// }

#[test]
fn pretty_print() {
    // Convenience:
    use Instr::*;
    use Var::*;
    use Val::*;
    use highlevel::NumericOp::*;
    use highlevel::StoreOp::*;
    use highlevel::LoadOp::*;
    fn test(wimpl: Instr, str: &str) {
        assert_eq!(wimpl.to_string(), str);
    }

    test(Unreachable, "unreachable");

    test(Assign { lhs: Global(0), rhs: Local(0) }, "g0 = l0");

    test(Const { lhs: Stack(0), val: I32(1337) }, "s0 = i32.const 1337");
    test(Numeric { lhs: Stack(1), op: I32Add, rhs: vec![Stack(2), Stack(3)] }, "s1 = i32.add(s2, s3)");

    test(
        Load { lhs: Stack(1), op: I32Load, memarg: Memarg::default(I32Load), addr: Stack(0) },
        "s1 = i32.load(s0)"
    );
    // Non-default alignment:
    test(
        Store { op: I64Store8, value: Stack(1), addr: Stack(2), memarg: Memarg { offset: 0, alignment_exp: 4 } }, 
        "i64.store8 align=16 (s2) (s1)"
    );

    test(Br { target: Label(0), value: None }, "br @label0");
    // With index and value.
    test(
        BrTable { idx: Stack(0), table: vec![Label(1), Label(2)], default: Label(0), value: Some(Stack(1)) },
        "br_table @label1 @label2 default=@label0 (s0) (s1)"
    );

    // Always print the argument parentheses, even if no arguments passed.
    test(
        Call { lhs: None, func: Func(7), args: Vec::new() },
        "call f7 ()"
    );
    test(
        CallIndirect { 
            lhs: Some(Stack(1)), 
            type_: FunctionType::new(&[ValType::I32], &[ValType::I32]), 
            table_idx: Stack(0),
            args: vec![Stack(2), Stack(3)]
        },
        "s1 = call_indirect [i32] -> [i32] (s0) (s2, s3)"
    );

    // Single if + br (which is our form of br_if).
    test(
        If { lhs: None, condition: Stack(0), label: None, if_body: Body {
            instrs: vec![Br { target: Label(0), value: None }],
            result: None,
        }, else_body: None }, 
        "if (s0) { br @label0 }"
    );
    // Multi-line and nested loop/block.
    test(
        Loop { lhs: None, label: Label(1), body: Body { 
            instrs: vec![
                Block { lhs: Some(Stack(0)), label: Label(2), body: Body {
                    instrs: vec![Const { lhs: Stack(1), val: I32(7) }],
                    result: Some(Stack(1))
                }},
                Br { target: Label(1), value: None }
            ],
            result: None
        } },
        r"@label1: loop {
  s0 = @label2: block {
    s1 = i32.const 7
    s1
  }
  br @label1
}"
    );
}

// #[test]
// fn wimpl_text_syntax() {
//     let parsed = Instr::from_file("tests/wimpl/syntax.wimpl").unwrap();

//     let expected = Instr::new(
//         Vec::new(),
//         Const(Val::I32(2)),
//         WimpleRhs::VarVec(Vec::new()),
//     );
//     assert_eq!(parsed[0], expected);

//     let expected = Instr::new(
//         vec![Var(0)],
//         Const(Val::I32(3)),
//         WimpleRhs::VarVec(Vec::new()),
//     );
//     assert_eq!(parsed[1], expected);
// }

// #[test]
// fn constant_file() {
//     let path = "tests/wimpl/const.wimpl";
//     let expected = Instr::from_file(path).unwrap();

//     let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
//     let func = module.functions().next().unwrap().1;
//     let instrs = &func.code().unwrap().body[0..1];
//     let actual = wimplify(instrs, func, &module).unwrap();

//     assert_eq!(actual, expected);
// }

// #[test]
// fn constant() {
//     let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
//     let func = module.functions().next().unwrap().1;
//     // let instrs = func.code().unwrap().body.as_slice();
//     let instrs = &func.code().unwrap().body[0..1];
//     let actual = wimplify(instrs, func, &module).unwrap();
//     //println!("actual {:?}",actual);
//     for ins in &actual {
//         println!("{}", ins);
//     }
//     let expected = vec![Instr {
//         lhs: vec![Var(0)],
//         op: Const(Val::I32(3)),
//         rhs: WimpleRhs::VarVec(Vec::new()),
//     }];
//     assert_eq!(actual, expected);
// }

// #[test]
// fn drop() {
//     let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
//     let func = module.functions().next().unwrap().1;
//     // let instrs = func.code().unwrap().body.as_slice();
//     let instrs = &func.code().unwrap().body[0..2];
//     let actual = wimplify(instrs, func, &module).unwrap();
//     //println!("actual {:?}",actual);
//     for ins in &actual {
//         println!("{}", ins);
//     }
//     let expected = vec![
//         Instr {
//             lhs: vec![Var(0)],
//             op: Const(Val::I32(3)),
//             rhs: WimpleRhs::VarVec(Vec::new()),
//         },
//         Instr {
//             lhs: Vec::new(),
//             op: Drop,
//             rhs: WimpleRhs::VarVec(vec![Var(0)]),
//         },
//     ];
//     assert_eq!(actual, expected);
// }

// #[test]
// fn add() {
//     let module = Module::from_file("../../tests/inputs/folding/add.wasm").unwrap();
//     let func = module.functions().next().unwrap().1;
//     // let instrs = func.code().unwrap().body.as_slice();
//     let instrs = &func.code().unwrap().body[0..3];
//     let actual = wimplify(instrs, func, &module).unwrap();
//     for ins in &actual {
//         println!("{}", ins);
//     }
//     let expected = vec![
//         Instr::new(
//             vec![Var(0)],
//             Const(Val::I32(3)),
//             WimpleRhs::VarVec(Vec::new()),
//         ),
//         Instr::new(
//             vec![Var(1)],
//             Const(Val::I32(4)),
//             WimpleRhs::VarVec(Vec::new()),
//         ),
//         Instr::new(
//             vec![Var(2)],
//             Numeric(I32Add),
//             WimpleRhs::VarVec(vec![Var(1), Var(0)]),
//         ),
//     ];
//     println!("{:?}", expected);
//     assert_eq!(actual, expected);
// }

// #[test]
// fn call_ind() {
//     let module = Module::from_file("../../tests/inputs/folding/call_ind.wasm").unwrap();
//     let func = module.functions().next().unwrap().1;
//     // let instrs = func.code().unwrap().body.as_slice();
//     let instrs = &func.code().unwrap().body[0..2];
//     let actual = wimplify(instrs, func, &module).unwrap();
//     //println!("{:?}",actual);
//     for ins in &actual {
//         println!("{}", ins);
//     }
//     // let expected = vec![FoldedExpr(
//     //     CallIndirect(FunctionType::new(&[], &[]), Idx::from(0)),
//     //     vec![FoldedExpr::new(Const(Val::I32(0)))],
//     // )];
//     // assert_eq!(actual, expected);
// }

// #[test]
// fn block_br() {
//     let module = Module::from_file("../../tests/inputs/folding/block-br.wasm").unwrap();
//     let func = module.functions().next().unwrap().1;
//     // let instrs = func.code().unwrap().body.as_slice();
//     let instrs = &func.code().unwrap().body;
//     let actual = wimplify(instrs, func, &module).unwrap();
//     //println!("{:?}",actual);
//     for ins in &actual {
//         println!("{}", ins);
//     }
//     // let expected = vec![FoldedExpr(
//     //     CallIndirect(FunctionType::new(&[], &[]), Idx::from(0)),
//     //     vec![FoldedExpr::new(Const(Val::I32(0)))],
//     // )];
//     // assert_eq!(actual, expected);
// }

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
