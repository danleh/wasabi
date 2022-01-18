use std::{
    fmt,
    io::{self, ErrorKind},
    path::Path,
};

use logos::{Lexer, Logos};
use regex::Regex;

use crate::Val;
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
            Stack(num) => write!(f, "s{}", num),
            Local(num) => write!(f, "l{}", num),
            Global(num) => write!(f, "g{}", num),
            Param(num) => write!(f, "p{}", num),
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
pub struct Block {
    instrs: Vec<Instr>,
    result: Option<Var>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let single_line = self.instrs.is_empty();
        if single_line {
            if let Some(result) = self.result {
                writeln!(f, "{{ {} }}", result)
            } else {
                f.write_str("{}")
            }
        } else {
            // TODO Indent blocks recursively, needs level.
            f.write_str("{\n")?;
            for instr in self.instrs {
                writeln!(f, "{}", instr)?;
            }
            if let Some(result) = self.result {
                writeln!(f, "{}", result);
            }
            f.write_str("\n}")
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
        body: Block,
    },
    // s0 = @label0: loop {
    //     s1 = i32.const 3
    //     br @label0 (s1)
    //     s1
    // }
    Loop {
        lhs: Option<Var>,
        label: Label,
        body: Block,
    },
    // s0 = @label0: if {
    //     s1 = i32.const 3
    //     s1
    // } else {
    //     s2 = i32.const 6
    //     s2
    // }
    If {
        //if, if-else, select, br_if
        lhs: Option<Var>,
        condition: Var,
        // no label for select or br_if
        // br_if 3 (s0)
        // ->
        // if (s0) { br @label3 }
        label: Option<Label>,
        // invariant: if and else must either both return value or none
        if_body: Block,
        else_body: Option<Block>,
    },

    // br @label0 (s1)
    Br {
        target: Label,
        values: Vec<Var>,
    },
    // br-table (s0) @label0, @label1, @label2, default=label@3
    BrTable {
        index: Var,
        table: Vec<Label>,
        default: Label,
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
        table_index: Var,
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
        addr: Var,
        op: LoadOp,
        memarg: Memarg,
    },
    // i32.store offset=3 align=4 (s0//addr) (s1//value)
    Store {
        value: Var,
        addr: Var,
        op: StoreOp,
        memarg: Memarg,
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
        op: NumericOp,
        lhs: Var,
        rhs: Vec<Var>,
    },
}

impl Instr {
    pub fn lhs(&self) -> Option<Var> {
        use Instr::*;
        match self {
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

    pub fn new(lhs: Vec<Var>, op: highlevel::Instr, rhs: WimpleRhs) -> Self {
        Instr { lhs, op, rhs }
    }

    pub fn parse_instr(mut lexer: Lexer<WimplTextToken>) -> io::Result<Self> {
        let mut instr = Instr {
            lhs: Vec::new(),
            op: highlevel::Instr::Nop,
            rhs: WimpleRhs::VarVec(Vec::new()),
        };

        match lexer.next() {
            Some(WimplTextToken::Variable(i)) => instr.lhs.push(Var(i)),
            None => {
                return Err(io::Error::new(
                    ErrorKind::UnexpectedEof,
                    "missing Wimpl instruction",
                ))
            }
            Some(WimplTextToken::AlphaNum) => {
                let start_instr = lexer.span().start;
                println!("in AlphaNum: start={}", start_instr);

                let mut current = lexer.next().unwrap();
                while current != WimplTextToken::LParen && current != WimplTextToken::Linebreak {
                    current = lexer.next().unwrap();
                }
                let end_instr = lexer.span().end - 1;
                println!("in AlphaNum: end={}", end_instr);

                let op_str = &lexer.source()[start_instr..end_instr].trim();
                println!("in AlphaNum: op_str={:?}", op_str);

                instr.op = highlevel::Instr::parse_text(op_str)
                    .map_err(|_| io::Error::new(ErrorKind::Other, "unknown"))?;

                // TODO parse argument list
            }
            _ => todo!(),
        }

        Ok(instr)
    }

    // pub fn parse_args_list(lexer) -> ... {
    //     expect_token(LParen)
    //     while next_token != RParen
    //         if Comma => Ignore
    //         if Variable => add to result
    //         else ERROR
    //     return result vec
    // }

    pub fn parse(str: &str) -> io::Result<Vec<Self>> {
        let mut lexer = WimplTextToken::lexer(str);
        Ok(vec![Self::parse_instr(lexer)?])

        // program = instr*
        // instr = (variable '=')? operator args?
        // args = '(' (variable ',')* variable ')'
    }

    /// Convenience function to parse Wimpl from a filename.
    pub fn from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Self>> {
        let str = std::fs::read_to_string(filename)?;
        Ok(Self::parse(&str)?)
    }
}

fn display_list<T: fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    list: &[T],
    begin: &str,
    end: &str,
    delim: &str,
) -> fmt::Result {
    match list.split_last() {
        None => Ok(()),
        Some((last, list)) => {
            f.write_str(begin)?;
            for item in list {
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

            Block { lhs, label, body } => write!(f, "{}: block {}", label, body)?,
            Loop { lhs, label, body } => write!(f, "{}: loop {}", label, body)?,
            If {
                lhs,
                condition,
                label,
                if_body,
                else_body,
            } => {
                if let Some(label) = label {
                    write!(f, "{}: ", label);
                }
                write!(f, "if ({}) {}", condition, if_body)?;
                if let Some(else_branch) = else_body {
                    write!(f, "\nelse {}", else_branch)?;
                }
            }

            Br { target, values } => {
                write!(f, "br {}", target)?;
                display_list(f, values, "(", ")", ",");
            }
            BrTable {
                index,
                table,
                default,
            } => {
                write!(f, "br_table ({})", index)?;
                display_list(f, table, "", "", ",")?;
                write!(f, ", default={}", default)?;
            }
            Return { value } => match value {
                Some(value) => write!(f, "return ({})", value)?,
                None => write!(f, "return")?,
            },

            Call { lhs, func, args } => {
                write!(f, "call {} ", func)?;
                display_list(f, args, "(", ")", ",")?;
            }

            CallIndirect {
                lhs,
                type_,
                table_index,
                args,
            } => {
                write!(f, "call_indirect {} ({}) ", type_, table_index)?;
                display_list(f, args, "(", ")", ",")?;
            }

            Assign { lhs, rhs } => write!(f, "{} = {}", lhs, rhs)?,

            Load {
                lhs,
                addr,
                op,
                memarg,
            } => write!(f, "{} ({})", op, addr)?,
            Store {
                value,
                addr,
                op,
                memarg,
            } => write!(f, "{} ({}) ({})", op, addr, value)?,
            MemorySize { lhs } => write!(f, "memory.size")?,
            MemoryGrow { lhs, new_size } => write!(f, "memory.grow({})", new_size)?,
            Const { lhs, val } => write!(f, "{}.const {}", val.to_type(), val)?,
            Numeric { op, lhs, rhs } => {
                write!(f, "{}", op)?;
                display_list(f, rhs, "(", ")", ",")?;
            }
        };
        write!(f, "\n");
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

pub fn wimplify(
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
                lhs: lhs,
                op: instr.clone(),
                rhs: WimpleRhs::VarVec(args),
            }
        };
        result_instrs.push(result_instr);
    }

    Ok(result_instrs)
}

#[test]
fn wimpl_text_syntax() {
    let parsed = Instr::from_file("tests/wimpl/syntax.wimpl").unwrap();

    let expected = Instr::new(
        Vec::new(),
        Const(Val::I32(2)),
        WimpleRhs::VarVec(Vec::new()),
    );
    assert_eq!(parsed[0], expected);

    let expected = Instr::new(
        vec![Var(0)],
        Const(Val::I32(3)),
        WimpleRhs::VarVec(Vec::new()),
    );
    assert_eq!(parsed[1], expected);
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
    let expected = vec![Instr {
        lhs: vec![Var(0)],
        op: Const(Val::I32(3)),
        rhs: WimpleRhs::VarVec(Vec::new()),
    }];
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
            lhs: vec![Var(0)],
            op: Const(Val::I32(3)),
            rhs: WimpleRhs::VarVec(Vec::new()),
        },
        Instr {
            lhs: Vec::new(),
            op: Drop,
            rhs: WimpleRhs::VarVec(vec![Var(0)]),
        },
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
        Instr::new(
            vec![Var(0)],
            Const(Val::I32(3)),
            WimpleRhs::VarVec(Vec::new()),
        ),
        Instr::new(
            vec![Var(1)],
            Const(Val::I32(4)),
            WimpleRhs::VarVec(Vec::new()),
        ),
        Instr::new(
            vec![Var(2)],
            Numeric(I32Add),
            WimpleRhs::VarVec(vec![Var(1), Var(0)]),
        ),
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
