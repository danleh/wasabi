use std::{
    fmt::{self, Write},
    io::{self, ErrorKind},
    path::Path,
};

use crate::{highlevel::MemoryOp, Val, ValType, BlockType};
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

/// Wimpl instructions make the following major changes over high-level Wasm:
/// - Remove the evaluation/operand stack completely, every instruction takes
/// explicit arguments and optionally produces a (in the Wasm MVP) single LHS.
/// - Stratify, i.e., express instructions that add no expressiveness as
/// combinations of simple instructions, e.g., br_if and select.
/// - Resolve relative, numerical branch targets to explicitly labeled blocks.
/// - Represent stack variables, locals, globals, and function parameters with
/// a single `Variable` construct. As a side-effect of this replaces all
/// local.* and global.* instructions with a single `Assign` instruction.
// TODO Optimize this representation, in particular remove redundant assignments
// between stack variables and locals/globals.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Instr {
    // Simplify: nop is not necessary for analysis.
    Unreachable,

    Block {
        lhs: Option<Var>,
        label: Label,
        body: Body,
    },
    Loop {
        lhs: Option<Var>,
        label: Label,
        body: Body,
    },
    If {
        lhs: Option<Var>,
        // No label for an if generated from select or br_if
        // (where this if block is never the target of a branch).
        label: Option<Label>,
        condition: Var,
        // Invariant: if and else must either both return value or not.
        if_body: Body,
        else_body: Option<Body>,
    },

    Br {
        target: Label,
        value: Option<Var>,
    },
    // Simplify: Represent br_if as an if (cond) { br }.
    BrTable {
        idx: Var,
        table: Vec<Label>,
        default: Label,
        value: Option<Var>,
    },

    Return {
        value: Option<Var>,
    },

    Call {
        lhs: Option<Var>,
        func: Func,
        args: Vec<Var>,
    },
    CallIndirect {
        lhs: Option<Var>,
        type_: FunctionType,
        table_idx: Var,
        args: Vec<Var>,
    },

    // Simplify: No instruction for drop, this is just a dead variable.
    // TODO Remove dead variables recursively.

    // Simplify: Encode select as result = if (arg0) { arg1 } else { arg2 }.

    // Simplify: Handles all of local.set, global.set, local.tee, local.get, global.get.
    Assign {
        lhs: Var,
        rhs: Var,
    },

    Load {
        lhs: Var,
        op: LoadOp,
        memarg: Memarg,
        addr: Var,
    },
    Store {
        op: StoreOp,
        memarg: Memarg,
        value: Var,
        addr: Var,
    },

    MemorySize {
        lhs: Var,
    },
    MemoryGrow {
        lhs: Var,
        pages: Var,
    },

    Const {
        lhs: Var,
        val: Val,
    },

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
    T::Item: fmt::Display,
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

// Pretty-prints instructions, including indenting nested blocks.
// Comments show examples.
// Conventions for the text format:
// - Things in parentheses (x, y) signify runtime arguments.
// - Everything outside of the parentheses is statically encoded into the instruction.
// - Curly brances { ... } signify block bodies (i.e., instructions and an optional return variable).
impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(lhs) = self.lhs() {
            write!(f, "{} = ", lhs)?;
        }

        use Instr::*;
        match self {
            Unreachable => f.write_str("unreachable")?,

            // s0 = @label0: block {
            //   s1 = i32.const 3
            //   s1
            // }
            Block {
                lhs: _,
                label,
                body,
            } => write!(f, "{}: block {}", label, body)?,
            // s0 = @label0: loop {
            //     s1 = i32.const 3
            //     br @label0 (s1)
            //     s1
            // }
            Loop {
                lhs: _,
                label,
                body,
            } => write!(f, "{}: loop {}", label, body)?,
            // s0 = @label0: if {
            //     s1 = i32.const 3
            //     s1
            // } else {
            //     s2 = i32.const 6
            //     s2
            // }
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

            // br @label0 (s1)
            Br { target, value } => {
                write!(f, "br {}", target)?;
                display_delim(f, value, " (", ")", ", ")?;
            }
            // br-table @label0, @label1, @label2, default=label@3 (s0)
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

            // return (s1)
            Return { value } => {
                f.write_str("return")?;
                display_delim(f, value, " (", ")", ", ")?;
            }

            // s0 = call f1 (s1)
            Call { lhs: _, func, args } => {
                // Always print the parentheses, even if `args` is empty.
                write!(f, "call {} (", func)?;
                display_delim(f, args, "", "", ", ")?;
                f.write_str(")")?;
            }
            // s2 = call_indirect [] -> [i32] (s0) (s1, s2, s3...)
            CallIndirect {
                lhs: _,
                type_,
                table_idx: table_index,
                args,
            } => {
                // Always print the parentheses, even if `args` is empty.
                write!(f, "call_indirect {} ({}) (", type_, table_index)?;
                display_delim(f, args, "", "", ", ")?;
                f.write_str(")")?;
            }

            // s2 = s1
            Assign { lhs: _, rhs } => write!(f, "{}", rhs)?,

            // s1 = i32.load offset=3 align=4 (s0)
            Load {
                lhs: _,
                addr,
                op,
                memarg,
            } => {
                write!(f, "{}", op)?;
                if !memarg.is_default(*op) {
                    f.write_str(" ")?;
                    memarg.fmt(f, *op)?;
                    f.write_str(" ")?;
                }
                write!(f, "({})", addr)?;
            }
            // i32.store offset=3 align=4 (s0//addr) (s1//value)
            Store {
                value,
                addr,
                op,
                memarg,
            } => {
                write!(f, "{}", op)?;
                if !memarg.is_default(*op) {
                    f.write_str(" ")?;
                    memarg.fmt(f, *op)?;
                    f.write_str(" ")?;
                }
                write!(f, "({}) ({})", addr, value)?;
            }

            // s1 = memory.size
            MemorySize { lhs: _ } => write!(f, "memory.size")?,
            // s1 = memory.grow(s0)
            MemoryGrow { lhs: _, pages } => write!(f, "memory.grow({})", pages)?,

            // s1 = i32.const 3
            Const { lhs: _, val } => write!(f, "{}.const {}", val.to_type(), val)?,

            // s2 = i32.add(s0, s1)
            // s1 = f32.neg(s0)
            Numeric { lhs: _, op, rhs } => {
                write!(f, "{}", op)?;
                display_delim(f, rhs, "(", ")", ", ")?;
            }
        };

        Ok(())
    }
}

pub fn wimplify(
    instrs: &[highlevel::Instr],
    function: &Function,
    module: &Module,
    label_count: usize, 
) -> Result<Vec<Instr>, String> {
    
    // Convenience:
    use Instr::*;
    use Var::*;
    
    let mut var_stack = Vec::new();
    let mut var_count = 0;
    let mut result_instrs = Vec::new();
    let tys = types(instrs, function, module).map_err(|e| format!("{:?}", e))?;

    for (instr, ty) in instrs.iter().zip(tys.into_iter()) {
    
        println!("{:?}, {:?}", instr, ty);
        let n_inputs = ty.inputs.len();
        let n_results = ty.results.len();

        let lhs : Option<Var>; 
        if n_results == 0 {
            lhs = None;  
        } else if n_results == 1 {
            lhs = Some(Var::Stack(var_count));
        } else {
            todo!(); // ERROR! 
        }
        
        let mut rhs = Vec::new();
        for _ in 0..n_inputs {
            rhs.push(var_stack.pop().unwrap());
        }
        
        // we can only push the new variable onto the stack once we have popped the required rhs values
        if lhs != None {
            var_stack.push(Var::Stack(var_count));
            var_count += 1;
        }

        let result_instr: Option<Instr> = match instr {
            highlevel::Instr::Unreachable => Some(Unreachable),
            highlevel::Instr::Nop => None,
            
            highlevel::Instr::Block(blocktype) => {
                // collect block body instructions 
                // FIXME: technically should not be till end since you can have nested blocks 
                
                // let mut block_body : Vec<highlevel::Instr> = Vec::new(); 
                // while instrs[ind] != highlevel::Instr::End && ind != instrs.len()-1{
                //     block_body.push(instrs[ind].clone()); 
                //     ind = ind+1; 
                // }
                // println!("{}", ind);
                // println!("{:?}", block_body); 
                
                // let btype = blocktype.0; 
                // Some(Block{
                //     lhs,
                //     label: Label(label_count),
                //     body: Body{
                //         instrs: wimplify(&block_body, function, module, label_count+1)?,
                //         result: if let Some(_btype) = btype {
                //             Some(rhs.pop().unwrap())
                //         } else {
                //             None
                //         },

                //     },
                // })
                todo!()
            },
            highlevel::Instr::Loop(_) => todo!(),
            highlevel::Instr::If(_) => todo!(),
            highlevel::Instr::Else => todo!(),
            highlevel::Instr::End => todo!(),
            highlevel::Instr::Br(_) => todo!(),
            highlevel::Instr::BrIf(lab) => {
                Some(If{
                    lhs,
                    label: None, 
                    condition: var_stack.pop().unwrap(),
                    if_body: Body{ 
                        instrs: vec![Br{ 
                                    target: Label(lab.0 as usize), 
                                    value: None,  
                                }],
                        result: None, 
                    },
                    else_body: None,
                })
            },
            highlevel::Instr::BrTable { table, default } => todo!(),
            highlevel::Instr::Return => todo!(),
            highlevel::Instr::Call(_) => todo!(),
            highlevel::Instr::CallIndirect(fn_type, index) => {
                // in call_indirect, 
                // the last variable on the stack is the index value
                // the rest (till you collect all the needed parameters are arguments  
                // then what is index?? do we need it here???
                Some(CallIndirect{
                    lhs, 
                    type_: fn_type.clone(), //do we need to clone??
                    table_idx: rhs.pop().unwrap(), 
                    args: rhs, 
                })
            },
            highlevel::Instr::Drop => {
                var_stack.pop(); 
                None
            },
            highlevel::Instr::Select => todo!(),
            highlevel::Instr::Local(_, _) => todo!(),
            highlevel::Instr::Global(_, _) => todo!(),
            highlevel::Instr::Load(_, _) => todo!(),
            highlevel::Instr::Store(_, _) => todo!(),
            highlevel::Instr::MemorySize(_) => todo!(),
            highlevel::Instr::MemoryGrow(_) => todo!(),
            highlevel::Instr::Const(val) => { 
                if let Some(lhs) = lhs {
                    Some(Const{
                        lhs,
                        val: *val,
                    })
                } else {
                    todo!(); //ERROR
                }
            },
            highlevel::Instr::Numeric(numop) => {
                if let Some(lhs) = lhs { 
                    Some(Numeric{
                        lhs,
                        op: *numop,
                        rhs,
                    })
                } else {
                    todo!() //ERROR
                }
            }, 
        }; 
        if let Some(result_instr) = result_instr {
            result_instrs.push(result_instr);
            
        } 
    }
    Ok(result_instrs)
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
    use highlevel::LoadOp::*;
    use highlevel::NumericOp::*;
    use highlevel::StoreOp::*;
    use Instr::*;
    use Val::*;
    use Var::*;
    fn test(wimpl: Instr, str: &str) {
        assert_eq!(wimpl.to_string(), str);
    }

    test(Unreachable, "unreachable");

    test(
        Assign {
            lhs: Global(0),
            rhs: Local(0),
        },
        "g0 = l0",
    );

    test(
        Const {
            lhs: Stack(0),
            val: I32(1337),
        },
        "s0 = i32.const 1337",
    );
    test(
        Numeric {
            lhs: Stack(1),
            op: I32Add,
            rhs: vec![Stack(2), Stack(3)],
        },
        "s1 = i32.add(s2, s3)",
    );

    test(
        Load {
            lhs: Stack(1),
            op: I32Load,
            memarg: Memarg::default(I32Load),
            addr: Stack(0),
        },
        "s1 = i32.load(s0)",
    );
    // Non-default alignment:
    test(
        Store {
            op: I64Store8,
            value: Stack(1),
            addr: Stack(2),
            memarg: Memarg {
                offset: 0,
                alignment_exp: 4,
            },
        },
        "i64.store8 align=16 (s2) (s1)",
    );

    test(
        Br {
            target: Label(0),
            value: None,
        },
        "br @label0",
    );
    // With index and value.
    test(
        BrTable {
            idx: Stack(0),
            table: vec![Label(1), Label(2)],
            default: Label(0),
            value: Some(Stack(1)),
        },
        "br_table @label1 @label2 default=@label0 (s0) (s1)",
    );

    // Always print the argument parentheses, even if no arguments passed.
    test(
        Call {
            lhs: None,
            func: Func(7),
            args: Vec::new(),
        },
        "call f7 ()",
    );
    test(
        CallIndirect {
            lhs: Some(Stack(1)),
            type_: FunctionType::new(&[ValType::I32], &[ValType::I32]),
            table_idx: Stack(0),
            args: vec![Stack(2), Stack(3)],
        },
        "s1 = call_indirect [i32] -> [i32] (s0) (s2, s3)",
    );

    // Single if + br (which is our form of br_if).
    test(
        If {
            lhs: None,
            condition: Stack(0),
            label: None,
            if_body: Body {
                instrs: vec![Br {
                    target: Label(0),
                    value: None,
                }],
                result: None,
            },
            else_body: None,
        },
        "if (s0) { br @label0 }",
    );
    // Multi-line and nested loop/block.
    test(
        Loop {
            lhs: None,
            label: Label(1),
            body: Body {
                instrs: vec![
                    Block {
                        lhs: Some(Stack(0)),
                        label: Label(2),
                        body: Body {
                            instrs: vec![Const {
                                lhs: Stack(1),
                                val: I32(7),
                            }],
                            result: Some(Stack(1)),
                        },
                    },
                    Br {
                        target: Label(1),
                        value: None,
                    },
                ],
                result: None,
            },
        },
        r"@label1: loop {
  s0 = @label2: block {
    s1 = i32.const 7
    s1
  }
  br @label1
}",
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

#[test]
fn constant() {
    let module = Module::from_file("../../tests/inputs/folding/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..1];
    let actual = wimplify(instrs, func, &module, 0).unwrap();
    //println!("actual {:?}",actual);
    for ins in &actual {
        println!("{}", ins);
    }
    let expected = vec![
        Instr::Const {
            lhs: Var::Stack(0), 
            val: Val::I32(3), 
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
    let actual = wimplify(instrs, func, &module, 0).unwrap();
    //println!("actual {:?}",actual);
    for ins in &actual {
        println!("{}", ins);
    }
    let expected = vec![
        Instr::Const {
            lhs: Var::Stack(0), 
            val: Val::I32(3), 
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
    let actual = wimplify(instrs, func, &module, 0).unwrap();
    for ins in &actual {
        println!("{}", ins);
    }
    let expected = vec![
        Instr::Const {
            lhs: Var::Stack(0), 
            val: Val::I32(3), 
        },
        Instr::Const {
            lhs: Var::Stack(1), 
            val: Val::I32(4), 
        }, 
        Instr::Numeric { 
            lhs: Var::Stack(2), 
            op: highlevel::NumericOp::I32Add, 
            rhs: vec![Var::Stack(1), Var::Stack(0)] 
        }, 
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
    let actual = wimplify(instrs, func, &module, 0).unwrap();
    //println!("{:?}",actual);
    for ins in &actual {
        println!("{}", ins);
    }
    let expected = vec![
        Instr::Const {
            lhs: Var::Stack(0), 
            val: Val::I32(0), 
        },
        Instr::CallIndirect{
            lhs : None,
            type_ : FunctionType { params: Box::new([]), results: Box::new([]) }, 
            table_idx : Var::Stack(0), 
            args: Vec::new(),   
        }    
    ];
    assert_eq!(actual, expected);
}

#[test]
fn block_br() {
    let module = Module::from_file("../../tests/inputs/folding/block-br.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body;
    
    let actual = wimplify(instrs, func, &module, 0).unwrap();
    println!("{:?}",actual);
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
