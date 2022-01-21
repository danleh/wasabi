use std::{
    collections::VecDeque,
    fmt::{self, Write},
    fs::File,
    io::{self, BufRead, ErrorKind},
    iter::FromIterator,
    path::Path,
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{alphanumeric1, multispace0, not_line_ending},
    combinator::{map, map_res, opt, value},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    AsChar, Finish, IResult,
};

use crate::{highlevel::MemoryOp, types::InstructionType, BlockType, Val, ValType};
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

impl FromStr for Var {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // `split_at` can panic, so ensure `s` has at least len >= 1.
        if s.is_empty() {
            return Err(());
        }
        let (letter, i) = s.split_at(1);
        let i = i.parse().map_err(|_| ())?;
        use Var::*;
        Ok(match letter {
            "s" => Stack(i),
            "l" => Local(i),
            "g" => Global(i),
            "p" => Param(i),
            _ => return Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.strip_prefix("f").ok_or(())?;
        let i = i.parse().map_err(|_| ())?;
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.strip_prefix("@label").ok_or(())?;
        let i = i.parse().map_err(|_| ())?;
        Ok(Label(i))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Body {
    instrs: Option<Vec<Instr>>, //when writing select as if, the bodies will not have any instructions just returns
    result: Option<Var>,
}

const BLOCK_INDENT: &'static str = "  ";

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Put each inner instruction and the result on a separate line.
        let mut inner = String::new();
        if let Some(instrs) = &self.instrs {
            for instr in instrs {
                writeln!(inner, "{}", instr)?;
            }
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

/// Type abbreviation for internal nom parser result.
type NomResult<'input, O> = IResult<&'input str, O>;

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

    /// Top-level entry into the nom-parser of Wimpl instructions.
    fn parse_nom(input: &str) -> NomResult<Instr> {
        use Instr::*;

        // Utility parsers, reused in the different instruction parsers below.
        // NOTE They can unfortunately not be written with let + point-free
        // style, because type inference makes them FnMut, which then cannot be
        // re-used multiple times :/. For this reason, most are written as normal
        // functions with explicit type signatures.

        /// Whitespace and comment parser, which are removed (which is why it returns unit).
        // All other parsers below only handle internal whitespace, i.e., they
        // assume initial whitespace is already consumed by the outer parser and
        // the input directly starts with the first non-whitespace token.
        fn ws(input: &str) -> NomResult<()> {
            value(
                // Always return ().
                (),
                alt((
                    multispace0,
                    // Line comment, i.e., from // until newline.
                    preceded(tag("//"), not_line_ending),
                )),
            )(input)
        }

        fn var(input: &str) -> NomResult<Var> {
            map_res(alphanumeric1, Var::from_str)(input)
        }
        fn arg_single(input: &str) -> NomResult<Var> {
            delimited(pair(tag("("), ws), var, pair(ws, tag(")")))(input)
        }
        fn arg_list(input: &str) -> NomResult<Vec<Var>> {
            delimited(
                pair(tag("("), ws),
                separated_list0(tuple((ws, tag(","), ws)), var),
                pair(ws, tag(")")),
            )(input)
        }
        fn lhs(input: &str) -> NomResult<Var> {
            // Include trailing whitespace in this parser, since LHS is always
            // followed by something else, so we don't need to put ws there.
            terminated(var, tuple((ws, tag("="), ws)))(input)
        }
        fn label(input: &str) -> NomResult<Label> {
            map_res(
                take_while(|c: char| c == '@' || c.is_alphanum()),
                Label::from_str,
            )(input)
        }
        fn label_colon(input: &str) -> NomResult<Label> {
            // Same as with lhs: include trailing whitespace here.
            terminated(label, tuple((ws, tag(":"), ws)))(input)
        }
        fn op(input: &str) -> NomResult<&str> {
            take_while(|c: char| c.is_alphanum() || c == '.' || c == '_')(input)
        }

        let func = map_res(alphanumeric1, Func::from_str);
        // HACK For call_indirect, we know nothing besides the argument list is following
        // the function type, so consume up to the opening parenthesis.
        // However, this will fail to recognize comments after the function type!
        let func_ty = map_res(take_until("("), FunctionType::from_str);

        // The defaults of a memarg (if not given) depend on the natural alignment
        // of the memory instruction, hence this higher-order combinator.
        fn memarg<'a>(op: impl MemoryOp + 'a) -> impl FnMut(&'a str) -> NomResult<'a, Memarg> {
            let op = op.clone();
            // Same trick as for function types in call_indirect: Consume until beginning of argument list.
            map_res(take_until("("), move |s| Memarg::from_str(s, op))
        }
        fn body(input: &str) -> NomResult<Body> {
            map(
                delimited(
                    pair(tag("{"), ws),
                    separated_pair(separated_list0(ws, Instr::parse_nom), ws, opt(var)),
                    pair(ws, tag("}")),
                ),
                |(instrs, result)| Body {
                    instrs: Some(instrs),
                    result,
                },
            )(input)
        }

        // One parser for each instruction, using the utility parsers from above.
        let unreachable = map(tag("unreachable"), |_| Unreachable);

        let block = map(
            tuple((opt(lhs), label_colon, tag("block"), ws, body)),
            |(lhs, label, _, (), body)| Block { lhs, label, body },
        );
        let loop_ = map(
            tuple((opt(lhs), label_colon, tag("loop"), ws, body)),
            |(lhs, label, _, (), body)| Loop { lhs, label, body },
        );
        let if_ = map(
            tuple((
                opt(lhs),
                opt(label_colon),
                tag("if"),
                ws,
                arg_single,
                ws,
                body,
                opt(preceded(tuple((ws, tag("else"), ws)), body)),
            )),
            |(lhs, label, _, (), condition, (), if_body, else_body)| If {
                lhs,
                label,
                condition,
                if_body,
                else_body,
            },
        );

        let br = map(
            tuple((tag("br"), ws, label, opt(preceded(ws, arg_single)))),
            |(_, (), target, value)| Br { target, value },
        );
        let br_table = map(
            tuple((
                tag("br_table"),
                ws,
                separated_list0(ws, label),
                ws,
                tag("default"),
                ws,
                tag("="),
                ws,
                label,
                ws,
                arg_single,
                opt(preceded(ws, arg_single)),
            )),
            |(_, (), table, (), _, (), _, (), default, (), idx, value)| BrTable {
                table,
                default,
                idx,
                value,
            },
        );

        let return_ = map(
            preceded(tag("return"), opt(preceded(ws, arg_single))),
            |value| Return { value },
        );

        let call = map(
            tuple((opt(lhs), tag("call"), ws, func, ws, arg_list)),
            |(lhs, _, (), func, (), args)| Call { lhs, func, args },
        );
        let call_indirect = map(
            tuple((
                opt(lhs),
                tag("call_indirect"),
                ws,
                func_ty,
                ws,
                arg_single,
                ws,
                arg_list,
            )),
            |(lhs, _, (), type_, (), table_idx, (), args)| CallIndirect {
                lhs,
                type_,
                table_idx,
                args,
            },
        );

        let assign = map(pair(lhs, var), |(lhs, rhs)| Assign { lhs, rhs });

        // Memarg parsing depends on result of previous LoadOp/StoreOp parsing.
        // This is easier to write in direct than in point-free style, so we do.
        fn load(input: &str) -> NomResult<Instr> {
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
        fn store(input: &str) -> NomResult<Instr> {
            let (input, op) = map_res(op, StoreOp::from_str)(input)?;
            let (input, memarg) = memarg(op)(input)?;
            let (input, addr) = arg_single(input)?;
            let (input, ()) = ws(input)?;
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
            tuple((lhs, tag("memory.grow"), ws, arg_single)),
            |(lhs, _, (), pages)| MemoryGrow { lhs, pages },
        );

        let const_ = map_res(
            tuple((
                lhs,
                map_res(take_until("."), ValType::from_str),
                tag(".const"),
                ws,
                // HACK Accept any non-whitespace character for a number, rest
                // is done by Val::from_str.
                take_while(|c: char| !c.is_ascii_whitespace()),
            )),
            |(lhs, ty, _, (), number)| Val::from_str(number, ty).map(|val| Const { lhs, val }),
        );

        let numeric = map(
            tuple((lhs, map_res(op, NumericOp::from_str), ws, arg_list)),
            |(lhs, op, (), rhs)| Numeric { lhs, op, rhs },
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Instr::parse_nom(s).finish() {
            Ok((_nothing_remaining, instr)) => Ok(instr),
            // TODO Output byte offset/line/column of parse error.
            Err(_err) => Err(()),
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

pub struct State {
    pub label_count: usize,
    pub var_count: usize,
    pub var_stack: Vec<Var>,
}

impl State {
    pub fn new() -> Self {
        State {
            label_count: 0,
            var_count: 0,
            var_stack: Vec::new(),
        }
    }
}

pub fn wimplify_helper(
    mut instrs: VecDeque<&highlevel::Instr>,
    mut tys: VecDeque<InstructionType>,
    mut result_instrs: Vec<Instr>,
    mut state: State,
) -> Result<(Vec<Instr>, State, VecDeque<&highlevel::Instr>), String> {
    use Instr::*;
    use Var::*;

    let instr = instrs.pop_front().unwrap();
    let ty = tys.pop_front().unwrap();

    let n_inputs = ty.inputs.len();
    let n_results = ty.results.len();

    let mut lhs: Option<Var> = None;
    if n_results == 0 {
        lhs = None;
    } else if n_results == 1 {
        lhs = Some(Var::Stack(state.var_count));
    } else {
        //Err("cannot return more than one value in wasm1.0"); //ERROR
    }

    let mut rhs = Vec::new();
    for _ in 0..n_inputs {
        rhs.push(state.var_stack.pop().unwrap());
    }

    let result_instr: Option<Instr> = match instr {
        highlevel::Instr::Unreachable => Some(Unreachable),
        highlevel::Instr::Nop => None,

        highlevel::Instr::Block(blocktype) => {
            let curr_label_count = state.label_count;
            state.label_count += 1;
            let result = wimplify_helper(instrs.clone(), tys.clone(), Vec::new(), state).unwrap();
            let block_body = result.0;

            state = result.1;
            instrs = result.2;

            let btype = blocktype.0;
            Some(Block {
                lhs,
                label: Label(curr_label_count),
                body: Body {
                    instrs: Some(block_body),
                    result: if let Some(_btype) = btype {
                        Some(state.var_stack.pop().unwrap())
                    } else {
                        None
                    },
                },
            })
        }

        highlevel::Instr::Loop(_) => todo!(),
        highlevel::Instr::If(_) => todo!(),
        highlevel::Instr::Else => todo!(),

        highlevel::Instr::End => {
            return Ok((result_instrs, state, instrs));
        }

        highlevel::Instr::Br(lab) => Some(Br {
            target: Label(lab.into_inner()),
            value: lhs,
        }),

        highlevel::Instr::BrIf(lab) => {
            Some(If {
                lhs,
                label: None,
                condition: rhs.pop().unwrap(), //var_stack.pop().unwrap(),
                if_body: Body {
                    instrs: Some(vec![Br {
                        target: Label(lab.into_inner()),
                        value: None,
                    }]),
                    result: None,
                },
                else_body: None,
            })
        }

        highlevel::Instr::BrTable { table, default } => {
            Some(BrTable {
                idx: rhs.pop().unwrap(), //var_stack.pop().unwrap(),
                table: table.iter().map(|x| Label(x.into_inner())).collect(),
                default: Label(default.into_inner()),
                value: lhs,
            })
        }

        highlevel::Instr::Return => {
            if rhs.len() > 1 {
                println!("am i here");
                todo!() //ERROR: multiple returns not yet allowed
            } else {
                Some(Return {
                    value: rhs.first().cloned(), //do we need to clone this??
                })
            }
        }

        highlevel::Instr::Call(idx) => Some(Call {
            lhs,
            func: Func(idx.into_inner()),
            args: rhs,
        }),

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
            //var_stack.pop();
            None
        }

        highlevel::Instr::Select => {
            let arg1 = rhs.pop().unwrap(); //cond  //wasm spec pg 71/155
            let arg2 = rhs.pop().unwrap(); //if
            let arg3 = rhs.pop().unwrap(); //else
            Some(If {
                lhs,
                label: None,
                condition: arg1,
                if_body: Body {
                    instrs: None,
                    result: Some(arg2),
                },
                else_body: Some(Body {
                    instrs: None,
                    result: Some(arg3),
                }),
            })
        }

        highlevel::Instr::Local(localop, local_ind) => {
            let local_var = Local(local_ind.into_inner());
            match localop {
                // fetch value from local variable so RHS is local variable
                highlevel::LocalOp::Get => {
                    if let Some(lhs) = lhs {
                        Some(Assign {
                            lhs,
                            rhs: local_var,
                        })
                    } else {
                        todo!() // ERROR: LHS should not be None
                    }
                }
                // set local variable so LHS is local variable
                highlevel::LocalOp::Set => {
                    if rhs.len() != 1 {
                        todo!() // ERROR!
                    }
                    Some(Assign {
                        lhs: local_var,
                        rhs: rhs.pop().unwrap(),
                    })
                }
                // like local set but also return argument -> top of stack
                highlevel::LocalOp::Tee => {
                    if rhs.len() != 1 {
                        todo!() // ERROR!
                    }
                    let rhs = rhs.pop().unwrap();
                    state.var_stack.push(rhs);
                    Some(Assign {
                        lhs: local_var,
                        rhs,
                    })
                }
            }
        }

        highlevel::Instr::Global(globalop, global_ind) => {
            // same as above
            let global_var = Global(global_ind.into_inner());
            match globalop {
                highlevel::GlobalOp::Get => {
                    if let Some(lhs) = lhs {
                        Some(Assign {
                            lhs,
                            rhs: global_var,
                        })
                    } else {
                        todo!() // ERROR
                    }
                }
                highlevel::GlobalOp::Set => {
                    if rhs.len() != 1 {
                        todo!() // ERROR!
                    }
                    Some(Assign {
                        lhs: global_var,
                        rhs: rhs.pop().unwrap(),
                    })
                }
            }
        }

        highlevel::Instr::Load(loadop, memarg) => {
            //lhs needs to have one variable and so does rhs
            if lhs == None {
                todo!(); //ERROR!
            }
            if rhs.len() != 1 {
                todo!(); // ERROR!
            }
            let lhs = lhs.unwrap();
            let rhs = rhs.pop().unwrap();
            Some(Load {
                lhs: lhs,
                op: *loadop,
                memarg: *memarg,
                addr: rhs,
            })
        }

        highlevel::Instr::Store(storeop, memarg) => {
            // two values need to be popped from the stack so len == 2
            if rhs.len() != 2 {
                todo!(); // ERROR!
            }
            Some(Store {
                op: *storeop,
                memarg: *memarg,
                value: rhs.pop().unwrap(),
                addr: rhs.pop().unwrap(),
            })
        }

        highlevel::Instr::MemorySize(_) => {
            if let Some(lhs) = lhs {
                Some(MemorySize { lhs })
            } else {
                todo!() // ERROR: lhs has to be a variable
            }
        }

        highlevel::Instr::MemoryGrow(ind) => {
            if let Some(lhs) = lhs {
                Some(MemoryGrow {
                    lhs,
                    pages: Stack(ind.into_inner()),
                })
            } else {
                todo!() // ERROR: lhs has to be a variable
            }
        }

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

    if instrs.is_empty() {
        Ok((result_instrs, state, instrs))
    } else {
        if lhs != None {
            state.var_stack.push(Var::Stack(state.var_count));
            state.var_count += 1;
        }
        wimplify_helper(instrs, tys, result_instrs, state)
    }
}

pub fn wimplify(
    instrs: &[highlevel::Instr],
    function: &Function,
    module: &Module,
) -> Result<Vec<Instr>, String> {
    let tys = types(instrs, function, module).map_err(|e| format!("{:?}", e))?;

    let instrs = VecDeque::from_iter(instrs);
    let tys = VecDeque::from_iter(tys);
    let result = wimplify_helper(instrs, tys, Vec::new(), State::new()).unwrap();
    let result_instrs = result.0;

    Ok(result_instrs)
}

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
                        instrs: Some(vec![]),
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
                        instrs: Some(vec![]),
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
                        instrs: Some(vec![Assign { lhs: Stack(1), rhs: Stack(0) }]),
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
                        instrs: Some(vec![Br {
                            target: Label(0),
                            value: None,
                        }]),
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
                        instrs: Some(vec![
                            Block {
                                lhs: Some(Stack(0)),
                                label: Label(2),
                                body: Body {
                                    instrs: Some(vec![Const {
                                        lhs: Stack(1),
                                        val: I32(7),
                                    }]),
                                    result: Some(Stack(1)),
                                },
                            },
                            Br {
                                target: Label(1),
                                value: None,
                            },
                        ]),
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
                        instrs: Some(vec![]),
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
                        instrs: Some(vec![Assign { lhs: Stack(1), rhs: Stack(0) }]),
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn constant() {
    let path = "tests/wimpl/const/const.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/const/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = &func.code().unwrap().body[0..1];
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn add() {
    let path = "tests/wimpl/add/add.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/add/add.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = &func.code().unwrap().body[0..3];
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn call_ind() {
    let path = "tests/wimpl/call_ind/call_ind.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/call_ind/call_ind.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = &func.code().unwrap().body[0..2];
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn multiple_expr() {
    let path = "tests/wimpl/multiple_expr/multiple_expr.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/multiple_expr/multiple_expr.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = func.code().unwrap().body.as_slice();
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn call() {
    let path = "tests/wimpl/call/call.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/call/call.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = func.code().unwrap().body.as_slice();
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn local() {
    let path = "tests/wimpl/local/local.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/local/local.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = func.code().unwrap().body.as_slice();
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn global() {
    let path = "tests/wimpl/global/global.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/global/global.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = func.code().unwrap().body.as_slice();
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn load_store() {
    let path = "tests/wimpl/load_store/load_store.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/load_store/load_store.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = func.code().unwrap().body.as_slice();
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn memory() {
    let path = "tests/wimpl/memory/memory.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/memory/memory.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = func.code().unwrap().body.as_slice();
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn select() {
    let path = "tests/wimpl/select/select.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/select/select.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = func.code().unwrap().body.as_slice();
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn block() {
    let path = "tests/wimpl/block/block.wimpl";
    let mut expected = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                let result = Instr::parse_nom(&line);
                if let Ok(res) = result {
                    expected.push(res.1);
                }
            }
        }
    }

    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file("tests/wimpl/block/block.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = func.code().unwrap().body.as_slice();
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    for instr in &actual {
        println!("{}", instr);
    }
    println!();

    assert_eq!(actual, expected);
}

#[test]
fn constant_wasm() {
    let module = Module::from_file("tests/wimpl/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..1];
    let actual = wimplify(instrs, func, &module).unwrap();
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
fn drop_wasm() {
    let module = Module::from_file("tests/wimpl/const.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..2];
    let actual = wimplify(instrs, func, &module).unwrap();
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
fn add_wasm() {
    let module = Module::from_file("tests/wimpl/add.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..3];
    let actual = wimplify(instrs, func, &module).unwrap();
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
fn call_ind_wasm() {
    let module = Module::from_file("tests/wimpl/call_ind.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body[0..2];
    let actual = wimplify(instrs, func, &module).unwrap();
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
    let module = Module::from_file("tests/wimpl/block-br.wasm").unwrap();
    let func = module.functions().next().unwrap().1;
    // let instrs = func.code().unwrap().body.as_slice();
    let instrs = &func.code().unwrap().body;

    let actual = wimplify(instrs, func, &module).unwrap();
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
