use std::{
    fmt::{self, Write},
    io::{self, ErrorKind},
    path::Path,
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_until, take_while},
    character::complete::{alphanumeric0, alphanumeric1, digit1, multispace0},
    combinator::{map, map_res, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    AsChar, Finish, IResult,
};

use crate::{highlevel::MemoryOp, BlockType, Val, ValType};
use crate::{
    highlevel::{self, Function, LoadOp, Module, NumericOp, StoreOp},
    types::types,
    FunctionType, Memarg,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct ParseError;

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

impl FromStr for Var {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // `split_at` can panic, so ensure `s` has at least len >= 1.
        if s.is_empty() {
            return Err(ParseError);
        }
        let (letter, i) = s.split_at(1);
        let i = i.parse().map_err(|_| ParseError)?;
        use Var::*;
        Ok(match letter {
            "s" => Stack(i),
            "l" => Local(i),
            "g" => Global(i),
            "p" => Param(i),
            _ => return Err(ParseError),
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Func(usize);

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "f{}", self.0)
    }
}

impl FromStr for Func {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.strip_prefix("f").ok_or(ParseError)?;
        let i = i.parse().map_err(|_| ParseError)?;
        Ok(Func(i))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Label(usize);

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@label{}", self.0)
    }
}

impl FromStr for Label {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.strip_prefix("@label").ok_or(ParseError)?;
        let i = i.parse().map_err(|_| ParseError)?;
        Ok(Label(i))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
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

/// A nom parser combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn ws_begin<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(multispace0, inner)
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

    /// Top-level entry into the nom-parser of Wimpl instructions.
    fn parse_nom(input: &str) -> IResult<&str, Instr> {
        use Instr::*;

        // Utility parsers, remove whitespace.
        // NOTE Cannot be closures because type inference makes them FnMut,
        // which then cannot be re-used multiple times :/
        fn var(input: &str) -> IResult<&str, Var> {
            ws(map_res(alphanumeric1, Var::from_str))(input)
        }
        fn arg_single(input: &str) -> IResult<&str, Var> {
            ws(delimited(tag("("), var, tag(")")))(input)
        }
        fn arg_list(input: &str) -> IResult<&str, Vec<Var>> {
            ws_begin(delimited(
                tag("("),
                separated_list0(tag(","), var),
                tag(")"),
            ))(input)
        }
        fn lhs(input: &str) -> IResult<&str, Var> {
            terminated(var, ws(tag("=")))(input)
        }
        fn label(input: &str) -> IResult<&str, Label> {
            ws(map_res(
                take_while(|c: char| c.is_alphanum() || c == '@'),
                Label::from_str,
            ))(input)
        }
        let func = ws(map_res(alphanumeric1, Func::from_str));
        let func_ty = ws(map_res(take_until("("), FunctionType::from_str));
        fn op(input: &str) -> IResult<&str, &str> {
            take_while(|c: char| c.is_alphanum() || c == '.' || c == '_')(input)
        }
        fn memarg<'a>(op: impl MemoryOp + 'a) -> impl FnMut(&'a str) -> IResult<&'a str, Memarg> {
            let op = op.clone();
            ws(map_res(take_until("("), move |s| Memarg::from_str(s, op)))
        }
        fn body(input: &str) -> IResult<&str, Body> {
            map(
                delimited(
                    ws(tag("{")),
                    pair(many0(ws(Instr::parse_nom)), opt(var)),
                    ws(tag("}")),
                ),
                |(instrs, result)| Body { instrs, result },
            )(input)
        }

        // Each indivudual instruction, assumes without outer whitespace.
        let unreachable = map(tag("unreachable"), |_| Unreachable);

        let block = map(
            tuple((opt(lhs), label, tag(":"), ws_begin(tag("block")), body)),
            |(lhs, label, _, _, body)| Block { lhs, label, body },
        );
        let loop_ = map(
            tuple((opt(lhs), label, tag(":"), ws_begin(tag("loop")), body)),
            |(lhs, label, _, _, body)| Loop { lhs, label, body },
        );
        let if_ = map(
            tuple((
                opt(lhs),
                opt(terminated(label, tag(":"))),
                ws_begin(tag("if")),
                arg_single,
                body,
                opt(preceded(ws(tag("else")), body)),
            )),
            |(lhs, label, _, condition, if_body, else_body)| If {
                lhs,
                label,
                condition,
                if_body,
                else_body,
            },
        );

        let br = map(
            preceded(tag("br"), pair(label, opt(arg_single))),
            |(target, value)| Br { target, value },
        );
        let br_table = map(
            preceded(
                tag("br_table"),
                tuple((
                    many0(label),
                    preceded(tag("default="), label),
                    arg_single,
                    opt(arg_single),
                )),
            ),
            |(table, default, idx, value)| BrTable {
                table,
                default,
                idx,
                value,
            },
        );

        let return_ = map(preceded(tag("return"), opt(arg_single)), |value| Return {
            value,
        });

        let call = map(
            tuple((opt(lhs), tag("call"), func, arg_list)),
            |(lhs, _, func, args)| Call { lhs, func, args },
        );
        let call_indirect = map(
            tuple((
                opt(lhs),
                tag("call_indirect"),
                ws(func_ty),
                arg_single,
                arg_list,
            )),
            |(lhs, _, type_, table_idx, args)| CallIndirect {
                lhs,
                type_,
                table_idx,
                args,
            },
        );

        let assign = map(pair(lhs, var), |(lhs, rhs)| Assign { lhs, rhs });

        // Memarg parsing depends on result of previous LoadOp/StoreOp parsing.
        // This is easier to write in direct than in point-free style, so we do.
        fn load(input: &str) -> IResult<&str, Instr> {
            let (input, lhs) = lhs(input)?;
            let (input, op) = map_res(op, LoadOp::from_str)(input)?;
            let (input, memarg) = memarg(op)(input)?;
            let (input, addr) = arg_single(input)?;
            Ok((
                input,
                Load {
                    lhs,
                    op,
                    memarg,
                    addr,
                },
            ))
        }
        fn store(input: &str) -> IResult<&str, Instr> {
            let (input, op) = map_res(op, StoreOp::from_str)(input)?;
            let (input, memarg) = memarg(op)(input)?;
            let (input, addr) = arg_single(input)?;
            let (input, value) = arg_single(input)?;
            Ok((
                input,
                Store {
                    op,
                    memarg,
                    addr,
                    value,
                },
            ))
        }

        let memory_size = map(terminated(lhs, tag("memory.size")), |lhs| MemorySize {
            lhs,
        });
        let memory_grow = map(
            separated_pair(lhs, tag("memory.grow"), arg_single),
            |(lhs, pages)| MemoryGrow { lhs, pages },
        );

        let const_ = map_res(
            tuple((
                lhs,
                map_res(take(3usize), ValType::from_str),
                tag(".const"),
                // FIXME Doesn't work with floats
                ws(digit1),
            )),
            |(lhs, ty, _, number)| Val::from_str(number, ty).map(|val| Const { lhs, val }),
        );

        let numeric = map(
            tuple((lhs, map_res(op, NumericOp::from_str), arg_list)),
            |(lhs, op, rhs)| Numeric { lhs, op, rhs },
        );

        alt((
            unreachable,
            block,
            loop_,
            if_,
            br,
            br_table,
            return_,
            call,
            call_indirect,
            assign,
            load,
            store,
            memory_size,
            memory_grow,
            const_,
            numeric,
        ))(input)
    }

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

impl FromStr for Instr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Instr::parse_nom(s).finish() {
            Ok((_nothing_remaining, instr)) => Ok(instr),
            // TODO Output byte offset/line/column of parse error.
            Err(_err) => Err(ParseError),
        }
    }
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
            // br-table @label0 @label1 @label2 default=@label3 (s0)
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
                op,
                memarg,
                addr,
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
                op,
                memarg,
                addr,
                value,
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

        let lhs: Option<Var>;
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
            }
            highlevel::Instr::Loop(_) => todo!(),
            highlevel::Instr::If(_) => todo!(),
            highlevel::Instr::Else => todo!(),
            highlevel::Instr::End => todo!(),
            highlevel::Instr::Br(_) => todo!(),
            highlevel::Instr::BrIf(lab) => Some(If {
                lhs,
                label: None,
                condition: var_stack.pop().unwrap(),
                if_body: Body {
                    instrs: vec![Br {
                        target: Label(lab.0 as usize),
                        value: None,
                    }],
                    result: None,
                },
                else_body: None,
            }),
            highlevel::Instr::BrTable { table, default } => todo!(),
            highlevel::Instr::Return => todo!(),
            highlevel::Instr::Call(_) => todo!(),
            highlevel::Instr::CallIndirect(fn_type, index) => {
                // in call_indirect,
                // the last variable on the stack is the index value
                // the rest (till you collect all the needed parameters are arguments
                // then what is index?? do we need it here???
                Some(CallIndirect {
                    lhs,
                    type_: fn_type.clone(), //do we need to clone??
                    table_idx: rhs.pop().unwrap(),
                    args: rhs,
                })
            }
            highlevel::Instr::Drop => {
                var_stack.pop();
                None
            }
            highlevel::Instr::Select => todo!(),
            highlevel::Instr::Local(_, _) => todo!(),
            highlevel::Instr::Global(_, _) => todo!(),
            highlevel::Instr::Load(_, _) => todo!(),
            highlevel::Instr::Store(_, _) => todo!(),
            highlevel::Instr::MemorySize(_) => todo!(),
            highlevel::Instr::MemoryGrow(_) => todo!(),
            highlevel::Instr::Const(val) => {
                if let Some(lhs) = lhs {
                    Some(Const { lhs, val: *val })
                } else {
                    todo!(); //ERROR
                }
            }
            highlevel::Instr::Numeric(numop) => {
                if let Some(lhs) = lhs {
                    Some(Numeric {
                        lhs,
                        op: *numop,
                        rhs,
                    })
                } else {
                    todo!() //ERROR
                }
            }
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

#[cfg(test)]
mod test {
    // Convenience imports:
    use super::Body;
    use super::Func;
    use super::Instr::{self, *};
    use super::Label;
    use super::Var::{self, *};
    use crate::highlevel::LoadOp::*;
    use crate::highlevel::NumericOp::*;
    use crate::highlevel::StoreOp::*;
    use crate::FunctionType;
    use crate::Memarg;
    use crate::Val::*;
    use crate::ValType;

    use lazy_static::lazy_static;
    use std::str::FromStr;

    lazy_static! {
        /// Pairs of Wimpl AST with concrete syntax, and optionally a comment what is
        /// "special" about this testcase. This is used for testing both parsing and
        /// pretty-printing of Wimpl, just in different directions.
        ///
        /// For these examples, the concrete syntax is in the "canonical pretty"
        /// format, i.e., with "standard" whitespace.
        static ref WIMPL_CANONICAL_SYNTAX_TESTCASES: Vec<(Instr, &'static str, &'static str)> = vec![
            (Unreachable, "unreachable", ""),
            (Return { value: None }, "return", "return without value"),
            (Return { value: Some(Stack(0)) }, "return (s0)", "with value, with whitespace"),
            (MemorySize { lhs: Stack(0) }, "s0 = memory.size", "with lhs"),
            (Assign { lhs: Global(0), rhs: Local(0) }, "g0 = l0", ""),
            (
                Const {
                    lhs: Stack(0),
                    val: I32(1337),
                },
                "s0 = i32.const 1337",
                ""
            ),
            (
                Numeric {
                    lhs: Stack(1),
                    op: I32Add,
                    rhs: vec![Stack(2), Stack(3)],
                },
                "s1 = i32.add(s2, s3)",
                ""
            ),
            (
                Load {
                    lhs: Stack(1),
                    op: I32Load,
                    memarg: Memarg::default(I32Load),
                    addr: Stack(0),
                },
                "s1 = i32.load(s0)",
                ""
            ),
            (
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
                "memory operation with non-default alignment"
            ),
            (
                Br {
                    target: Label(0),
                    value: None,
                },
                "br @label0",
                "br without value"
            ),
            (Br { target: Label(1), value: Some(Stack(0)) }, "br @label1 (s0)", "br with value"),
            (
                BrTable {
                    idx: Stack(0),
                    table: vec![Label(1), Label(2)],
                    default: Label(0),
                    value: Some(Stack(1)),
                },
                "br_table @label1 @label2 default=@label0 (s0) (s1)",
                "br_table with index argument and passed value"
            ),
            (
                Call {
                    lhs: None,
                    func: Func(7),
                    args: Vec::new(),
                },
                "call f7 ()",
                "call argument list is always printed, even if empty"
            ),
            (
                CallIndirect {
                    lhs: Some(Stack(1)),
                    type_: FunctionType::new(&[ValType::I32], &[ValType::I32]),
                    table_idx: Stack(0),
                    args: vec![Stack(2), Stack(3)],
                },
                "s1 = call_indirect [i32] -> [i32] (s0) (s2, s3)",
                ""
            ),
            (
                Block {
                    lhs: None,
                    label: Label(0),
                    body: Body {
                        instrs: vec![],
                        result: None,
                    },
                },
                "@label0: block {}", ""
            ),
            (
                Block {
                    lhs: Some(Stack(1)),
                    label: Label(0),
                    body: Body {
                        instrs: vec![],
                        result: Some(Stack(0)),
                    },
                },
                "s1 = @label0: block { s0 }",
                "no block instructions, only a result"
            ),
            (
                Block {
                    lhs: None,
                    label: Label(1),
                    body: Body {
                        instrs: vec![Assign { lhs: Stack(1), rhs: Stack(0) }],
                        result: None,
                    },
                },
                "@label1: block { s1 = s0 }",
                "block with a single instruction, no result; on one line"
            ),
            (
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
                "if + br (which is our form of br_if)"
            ),
            (
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
            "nested and multi-line loop/block")
        ];

        /// The following examples are NOT in the canonical text format, e.g.,
        /// because they contain too little or too much whitespace.
        /// They are only used for testing parsing, not pretty-printing.
        static ref WIMPL_ALTERNATIVE_SYNTAX_TESTCASES: Vec<(Instr, &'static str, &'static str)> = vec![
            (Return { value: Some(Stack(0)) }, "return(s0)", "no space between op and arguments"),
            (MemoryGrow { lhs: Stack(1), pages: Stack(0) }, "s1 = memory.grow ( s0 )", "extra space around arguments"),
            (
                Call {
                    lhs: None,
                    func: Func(2),
                    args: vec![Stack(2), Stack(3)],
                },
                "call f2 ( s2, s3 )",
                "extra space around call arguments"
            ),
            (
                CallIndirect {
                    lhs: Some(Stack(1)),
                    type_: FunctionType::new(&[ValType::I32], &[ValType::I32]),
                    table_idx: Stack(0),
                    args: vec![],
                },
                "s1 = call_indirect [  i32  ] ->[i32] (s0) ()",
                "non-standard spacing around function type"
            ),
            (
                Numeric {
                    lhs: Stack(1),
                    op: I32Add,
                    rhs: vec![Stack(2), Stack(3)],
                },
                "s1 = i32.add (s2,s3)",
                "space before arguments, no space after comma"
            ),
            (
                Store {
                    op: I64Store8,
                    value: Stack(1),
                    addr: Stack(2),
                    memarg: Memarg {
                        offset: 0,
                        alignment_exp: 4,
                    },
                },
                "i64.store8 align=16(s2)(s1)",
                "minimal spacing around arguments"
            ),
            (
                Block {
                    lhs: None,
                    label: Label(0),
                    body: Body {
                        instrs: vec![],
                        result: None,
                    },
                },
                "@label0:block{}",
                "minimal space in block"
            ),
            (
                Block {
                    lhs: None,
                    label: Label(2),
                    body: Body {
                        instrs: vec![Assign { lhs: Stack(1), rhs: Stack(0) }],
                        result: Some(Stack(1)),
                    },
                },
                "@label2: block { s1 = s0 s1 }",
                "no linebreak between block instructions and result"
            )
        ];
    }

    #[test]
    fn pretty_print() {
        for (i, (wimpl, text, msg)) in WIMPL_CANONICAL_SYNTAX_TESTCASES.iter().enumerate() {
            assert_eq!(&wimpl.to_string(), text, "\ntest #{}\n{}", i, msg);
        }
    }

    #[test]
    fn parse_var() {
        assert_eq!(Ok(Stack(0)), "s0".parse());
        assert_eq!(Ok(Global(0)), "g0".parse());

        // Negative tests:
        assert!(" s0 \n ".parse::<Var>().is_err(), "whitespace not allowed");
        assert!(
            "sABC".parse::<Var>().is_err(),
            "characters instead of number"
        );
        assert!("x123".parse::<Var>().is_err(), "invalid variable type");
    }

    #[test]
    fn parse_instr() {
        let parse_testcases = WIMPL_CANONICAL_SYNTAX_TESTCASES
            .iter()
            .chain(WIMPL_ALTERNATIVE_SYNTAX_TESTCASES.iter());
        for (i, (wimpl, text, msg)) in parse_testcases.enumerate() {
            let parsed = Instr::from_str(text);
            assert!(
                parsed.is_ok(),
                "\ntest #{} could not be parsed\ninput: `{}`\n{}",
                i,
                text,
                msg
            );
            assert_eq!(
                &parsed.unwrap(),
                wimpl,
                "\ntest #{}\ninput: `{}`\n{}",
                i,
                text,
                msg
            );
        }
    }

    #[test]
    fn parse_pretty_print_roundtrips() {
        // For the text inputs in the canonical format, parsing then pretty-printing
        // the AST should round-trip, i.e., pretty(parse(text)) == text.
        for (i, (_, text, msg)) in WIMPL_CANONICAL_SYNTAX_TESTCASES.iter().enumerate() {
            let parsed = Instr::from_str(text).unwrap();
            let pretty = parsed.to_string();
            assert_eq!(
                text, &pretty,
                "\nleft is input, right is pretty-printed\ntest #{}\n{}",
                i, msg
            );
        }

        // For the text inputs in non-canonical format, first parse and pretty-print
        // to canonicalize, then parse should be equal to non-canonicalized, i.e.
        // parse(pretty(parse(text))) == parse(text).
        for (i, (_, text, msg)) in WIMPL_CANONICAL_SYNTAX_TESTCASES.iter().enumerate() {
            let parsed = Instr::from_str(text).unwrap();
            let pretty = parsed.to_string();
            let parsed_pretty = Instr::from_str(&pretty).unwrap();
            assert_eq!(parsed, parsed_pretty, "\ntest #{}\n{}", i, msg);
        }
    }
}

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
    let expected = vec![Instr::Const {
        lhs: Var::Stack(0),
        val: Val::I32(3),
    }];
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
    let expected = vec![Instr::Const {
        lhs: Var::Stack(0),
        val: Val::I32(3),
    }];
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
            rhs: vec![Var::Stack(1), Var::Stack(0)],
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
        Instr::CallIndirect {
            lhs: None,
            type_: FunctionType {
                params: Box::new([]),
                results: Box::new([]),
            },
            table_idx: Var::Stack(0),
            args: Vec::new(),
        },
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
    println!("{:?}", actual);
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
