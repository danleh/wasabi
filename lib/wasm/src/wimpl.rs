use std::{
    collections::VecDeque,
    fmt::{self, Write},
    io::{self, ErrorKind},
    iter::FromIterator,
    path::Path,
    str::FromStr,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{alphanumeric1, multispace1, not_line_ending},
    combinator::{all_consuming, map, map_res, opt, value},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    AsChar, Finish, IResult,
};

use crate::{highlevel::MemoryOp, types::InstructionType, Val, ValType};
use crate::{
    highlevel::{self, LoadOp, Module, NumericOp, StoreOp},
    types::types,
    FunctionType, Memarg,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Function {
    pub type_: FunctionType,
    pub instrs: Body, //want to reuse 
    //pub export: Vec<String>,
    pub name: String,
    //pub param_names: Vec<Option<String>>,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "func f{} {} ", self.name, self.type_); 
        write!(f, "{}", self.instrs); 
        Ok(())
    }
}

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
        // `split_at` can panic, so ensure `s` has at least one byte.
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
        let i = s.strip_prefix('f').ok_or(())?;
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
    instrs: Vec<Instr>,
    result: Option<Var>,
}

const BLOCK_INDENT: &str = "  ";

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

/// User-facing error for parsing the Wimpl text format.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParseError {
    /// Line number in the full input where the error occurred (1-indexed).
    line: usize,

    /// Byte offset inside the line where the error occurred (0-indexed).
    column: usize,

    /// Erroneous input line text.
    input_line: String,
}

impl std::error::Error for ParseError {}

// Conversion from nom error to my own.
impl ParseError {
    fn from<'input>(error: nom::error::Error<&'input str>, full_input: &'input str) -> Self {
        let full_input_ptr = full_input.as_ptr() as usize;
        let error_input_ptr = error.input.as_ptr() as usize;
        let error_offset = error_input_ptr - full_input_ptr;

        for (line, input_line) in full_input.lines().enumerate() {
            let line_start_offset = input_line.as_ptr() as usize - full_input.as_ptr() as usize;
            let line_end_offset = line_start_offset + input_line.len();
            if error_offset >= line_start_offset && error_offset < line_end_offset {
                let line = line + 1;
                let column = error_offset - line_start_offset;
                let input_line = input_line.to_string();
                return Self {
                    line,
                    column,
                    input_line,
                };
            }
        }
        unreachable!()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "parse error in line {}, column {}:",
            self.line, self.column
        )?;
        let line_header = format!("\nline {}: ", self.line);
        writeln!(f, "{}{}", line_header, self.input_line)?;
        let indent = " ".repeat(line_header.len() + self.column - 1);
        write!(f, "{}^ error starts around here or later", indent)
    }
}

/// Type abbreviation for internal nom parser result.
type NomResult<'input, O> = IResult<&'input str, O>;

/// Whitespace and comment parser, both of which are removed (which is why it returns unit).
fn ws(input: &str) -> NomResult<()> {
    value(
        (),
        // NOTE `many0` may never wrap an inner parser that potentially matches
        // the empty word. Because of this, use `multispace1`.
        many0(alt((
            multispace1,
            // Line comment, i.e., from // until newline.
            preceded(tag("//"), not_line_ending),
        ))),
    )(input)
}

/// Adapt nom parser such that it returns a standard Rust `Result`.
fn adapt_nom_parser<'input, O>(
    parser: impl Fn(&'input str) -> NomResult<'input, O>,
    input: &'input str,
) -> Result<O, ParseError> {
    match all_consuming(parser)(input).finish() {
        Ok(("", instr)) => Ok(instr),
        Ok(_) => unreachable!(
            "successful parse should hould have consumed full input because of all_consuming()"
        ),
        Err(err) => Err(ParseError::from(err, input)),
    }
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

    /// Parse multiple instructions, with possibly preceding and trailing whitespace.
    fn parse_nom_multiple_ws(input: &str) -> NomResult<Vec<Instr>> {
        preceded(ws, many0(terminated(Instr::parse_nom_single, ws)))(input)
    }

    /// Parse a single instruction, without surrounding whitespace.
    /// Main entry point into the nom-based parser.
    fn parse_nom_single(input: &str) -> NomResult<Instr> {
        use Instr::*;

        // Utility parsers, reused in the different instruction parsers below.
        //
        // They all assume only "internal" whitespace, i.e., they assume initial
        // whitespace is already consumed by the outer parser and the input
        // directly starts with the first non-whitespace token.
        //
        // They can unfortunately not be written with let + point-free
        // style, because type inference makes them FnMut, which then cannot be
        // re-used multiple times :/. For this reason, most are written as normal
        // functions with explicit type signatures.

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
        // HACK Accept any non-whitespace character for the integer/float
        // immediate, the rest of the parsing is done by Val::from_str.
        let number = take_while1(|c: char| !c.is_ascii_whitespace());

        // The defaults of a memarg (if not given) depend on the natural alignment
        // of the memory instruction, hence this higher-order combinator.
        fn memarg<'a>(op: impl MemoryOp + 'a) -> impl FnMut(&'a str) -> NomResult<'a, Memarg> {
            // Same trick as for function types in call_indirect: Consume until beginning of argument list.
            map_res(take_until("("), move |s| Memarg::from_str(s, op))
        }
        fn body(input: &str) -> NomResult<Body> {
            map(
                delimited(
                    tag("{"),
                    pair(Instr::parse_nom_multiple_ws, opt(terminated(var, ws))),
                    tag("}"),
                ),
                |(instrs, result)| Body {
                    instrs,
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
                number,
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

    pub fn from_str_multiple(input: &str) -> Result<Vec<Self>, ParseError> {
        adapt_nom_parser(Self::parse_nom_multiple_ws, input)
    }

    /// Convenience function to parse Wimpl from a filename.
    pub fn from_file(filename: impl AsRef<Path>) -> io::Result<Vec<Self>> {
        let str = std::fs::read_to_string(filename)?;
        Self::from_str_multiple(&str).map_err(|e| io::Error::new(ErrorKind::Other, e))
    }
}

/// Adapt nom parser for use with Rust `parse()` / `from_str`.
impl FromStr for Instr {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        adapt_nom_parser(Self::parse_nom_single, input)
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
    // Get last element:
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
                //println!("{:?}", self);
                if let Some(label) = label {
                    write!(f, "{}: ", label)?;
                }
                write!(f, "if ({}) {}", condition, if_body)?;
                if let Some(else_branch) = else_body {
                    write!(f, " else {}", else_branch)?;
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

macro_rules! wimpl {
    ($($tokens:tt)+) => {
        {
            let mut instrs = wimpls!($($tokens)+);
            match (instrs.pop(), instrs.is_empty()) {
                (Some(instr), true) => instr,
                _ => panic!("The wimpl! macro accepts only a single instruction, use wimpls! instead.")
            }
        }
    }
}

/// Convenience macro to write Wimpl instructions in Rust.
macro_rules! wimpls {
    ($($tokens:tt)*) => {
        {
            let input_str = std::stringify!($($tokens)*)
                // std::stringify somehow re-wraps the input tokens to 80 columns.
                // Replace those inserted newlines, also to make the following easier.
                .replace("\n", " ")
                // HACK Because the input `tokens` are tokenized by rustc's 
                // lexer, it inserts whitespace sometimes where Wimpl/Wasm 
                // syntax doesn't accept it. Fix those cases here.
                .replace("offset = ", "offset=")
                .replace("align = ", "align=")
                .replace("@ label", "@label");
            match Instr::from_str_multiple(&input_str) {
                Ok(instrs) => instrs,
                Err(err) => panic!("Invalid Wimpl instriction(s).\n{}\n(Note: whitespace might be different from your input.)", err)
            }
        }
    }
}

// Export macros.
pub(crate) use wimpl;
pub(crate) use wimpls;

#[derive(Default)]
pub struct State {
    pub label_count: usize,
    pub stack_var_count: usize,
    pub var_stack: Vec<Var>,
    pub label_stack: Vec<usize>, 
    pub else_taken: bool, 
}

fn panic_if_size_lt (vec : &[Var], size : usize, error: &str) {
    if vec.len() < size { 
        panic!("{}", error); 
    }; 
}

fn panic_if_size_gt (vec : &[Var], size : usize, error: &str) {
    if vec.len() > size { 
        panic!("{}", error); 
    }; 
}

fn wimplify_instrs(
    instrs: &mut VecDeque<&highlevel::Instr>,
    tys: &mut VecDeque<InstructionType>,
    state: &mut State,
) -> Result<Vec<Instr>, String> {
    use Instr::*;
    use Var::*;

    let instr = instrs.pop_front().unwrap();
    let ty = tys.pop_front().unwrap();

    //println!("{}, {:?}, {:?}", instr, ty, state.var_stack); 

    let n_inputs = ty.inputs.len();
    let n_results = ty.results.len();
    let mut result_instrs = Vec::new();

    let mut lhs = if n_results == 0 {
        None
    } else if n_results == 1 {
        match instr {
            highlevel::Instr::Local(highlevel::LocalOp::Tee,_) => None, //tee lhs is the argument itself, ie, rhs which is handled in the match arm for it below
            _ => Some(Var::Stack(state.stack_var_count)) 
        }
    } else {
        panic!("cannot return more than one value in wasm1.0")
    };

    let mut rhs = Vec::new();
    for _ in 0..n_inputs {
        rhs.push(state.var_stack.pop().expect("type correct wasm programs should not have empty stack"));
    }
    //FIXME: create rhs using rsplit_at so that numeric instrs and calls have arguments in the right (intuitive) order
        // rsplit_at does not exist
        // you could reverse state.var_stack and then split_at but that seems to be more expensive 
        // instead we just reverse here 
    rhs.reverse(); 

    let result_instr: Option<Instr> = match instr {

        highlevel::Instr::Unreachable => Some(Unreachable),

        highlevel::Instr::Nop => None,

        highlevel::Instr::Block(blocktype) => {
            
            let curr_label_count = state.label_count;
            state.label_count += 1;
            state.label_stack.push(curr_label_count); 

            let mut block_state = State {
                label_count: state.label_count,
                stack_var_count: state.stack_var_count,
                var_stack: Vec::new(),
                label_stack: state.label_stack.clone(),
                else_taken: false,
            }; 

            let block_body = wimplify_instrs(instrs, tys, &mut block_state).unwrap();
            
            // the variable returned by the block, if any is on top of the stack 
            // and was pushed there by the end instruction 
            let mut result = None; 
            if let Some(_btype) = blocktype.0 {
                panic_if_size_lt(&block_state.var_stack, 2, "block expects a value to be returned, which is not on the stack"); 
                lhs = block_state.var_stack.pop();
                result = Some(block_state.var_stack.pop().unwrap());             
            }

            state.label_count = block_state.label_count;
            state.stack_var_count = block_state.stack_var_count; 

            Some(Block {
                lhs, 
                label: Label(curr_label_count),
                body: Body {
                    instrs: block_body,
                    result,
                },
            })
        }

        highlevel::Instr::Loop(blocktype) => {
            
            let curr_label_count = state.label_count;
            state.label_count += 1;
            state.label_stack.push(curr_label_count); 

            let loop_body = wimplify_instrs(instrs, tys, state).unwrap();
            // end of loop -> check this
            //FIXME

            // the variable returned by the block, if any is on top of the stack 
            // and was pushed there by the end instruction 
            let mut result = None; 
            if let Some(_btype) = blocktype.0 {
                panic_if_size_lt(&state.var_stack, 2, "block expects a value to be returned, which is not on the stack"); 
                lhs = state.var_stack.pop();
                result = Some(state.var_stack.pop().unwrap());             
            }

            Some(Loop {
                lhs, 
                label: Label(curr_label_count),
                body: Body {
                    instrs: loop_body,
                    result,
                },
            })
        },

        highlevel::Instr::If(blocktype) => {
            panic_if_size_lt(&rhs, 1, "if consumes one value from stack as condition"); 

            let curr_label_count = state.label_count;
            state.label_count += 1;
            state.label_stack.push(curr_label_count); 

            let if_body = wimplify_instrs(instrs, tys, state).unwrap();            
            let mut if_return = None;
            if let Some(_btype) = blocktype.0 {
                panic_if_size_lt(&state.var_stack, 2, "block expects a value to be returned, which is not on the stack"); 
                lhs = state.var_stack.pop();
                if_return = Some(state.var_stack.pop().unwrap());             
            }

            if state.else_taken {
                
                state.else_taken = false; 

                // the lhs produced in else is actually the same as in the if branch
                // hence, throw away the variable and decrement the stack variable counter
                let else_body = wimplify_instrs(instrs, tys, state).unwrap(); 
                let mut else_return = None;
                if let Some(_btype) = blocktype.0 {
                    panic_if_size_lt(&state.var_stack, 2, "block expects a value to be returned, which is not on the stack"); 
                    let _ = state.var_stack.pop();
                    else_return = Some(state.var_stack.pop().unwrap());             
                    state.stack_var_count -= 1; 
                }

                // validation that if-body and else-body have the same number of returns
                // type checking validation already done 
                match (if_return, else_return) {
                    (None, Some(_)) | (Some(_), None) => {
                        panic!("if and else branch should either both return a value or None");
                    },
                    (None, None) | (Some(_), Some(_)) => (),
                }; 

                Some(If{
                    lhs,
                    label: Some(Label(curr_label_count)),
                    condition: rhs.pop().unwrap(),
                    if_body: Body { instrs: if_body, result: if_return },
                    else_body: Some(Body{ instrs: else_body, result: else_return}),
                })

            } else {

                Some(If{
                    lhs,
                    label: Some(Label(curr_label_count)),
                    condition: rhs.pop().unwrap(),
                    if_body: Body { instrs: if_body, result: if_return },
                    else_body: None,
                })
            }
        },

        highlevel::Instr::Else => {
            state.else_taken = true; 
            return Ok(result_instrs); 
        },

        highlevel::Instr::End => {
            // if end has a return, push it to the stack since 
            // the enclosing function will have to pop it back out
            state.label_stack.pop(); 
            if let Some(lhs) = lhs {
                state.var_stack.push(lhs); 
            }
            return Ok(result_instrs);
        }

        highlevel::Instr::Br(lab) => {                        
            let target = Label(state.label_stack[state.label_stack.len()-lab.into_inner()-1]); 
            
            let mut value = None; 
            if n_inputs > 0 {
                // value to be returned is already in rhs
                // push it back on the stack
                value = rhs.pop(); 
                //state.var_stack.push(value.unwrap()); 
                    // hacky and wrong FIXME - br_simple testcase still fails 
                    // because the block expects a variable to be returned 
            }
            
            Some(Br {
                target,
                value, //variable returned with the br
            })
        },

        highlevel::Instr::BrIf(lab) => {
            //println!("{:?}", rhs);
            //println!("{:?}", n_inputs); 
            panic_if_size_lt(&rhs, 1, "if required a conditional statement"); 
            
            let target = Label(state.label_stack[state.label_stack.len()-lab.into_inner()-1]); 
            
            let condition = rhs.pop().unwrap(); 
            
            // pop the return but then push it back in (if any) 
            // because the stack remains unchanged by br  
            let value = rhs.pop(); 
            if let Some(val) = value {
                state.var_stack.push(val); 
            }
            
            Some(If {
                lhs,
                label: None,
                condition, 
                if_body: Body {
                    instrs: vec![Br {
                        target, 
                        value, //variable returned with br if any 
                    }],
                    result: None,
                },
                else_body: None,
            })
        }

        highlevel::Instr::BrTable { table, default } => { 
            //println!("{:?}", rhs); 
            panic_if_size_lt(&rhs, 1, "br_table requires a condition"); 
            
            let mut lab_table = Vec::new();
            for lab in table {
                lab_table.push(Label(state.label_stack[state.label_stack.len()-lab.into_inner()-1])); 
            }

            let default = Label(state.label_stack[state.label_stack.len()-default.into_inner()-1]);
            
            Some(BrTable {
                idx: rhs.pop().unwrap(), 
                table: lab_table,
                default,
                value: rhs.pop(), //variable returned with brtable if any
            })
        }

        highlevel::Instr::Return => {
            panic_if_size_gt(&rhs, 1, "multiple returns not yet allowed");
            Some(Return {
                value: rhs.pop(),
            })
        }

        highlevel::Instr::Call(idx) => Some(Call {
            lhs,
            func: Func(idx.into_inner()), 
            args: rhs,
        }),

        highlevel::Instr::CallIndirect(fn_type, index) => {
            assert_eq!(index.into_inner(), 0, "wasm mvp must always have a single table"); 
            // in call_indirect,
            // the last variable on the stack is the index value
            // the rest (till you collect all the needed parameters are arguments
            panic_if_size_lt(&rhs, 1, "call_indirect requires an index"); 
            Some(CallIndirect {
                lhs,
                type_: fn_type.clone(), 
                table_idx: rhs.pop().unwrap(),
                args: rhs,
            })
        }

        highlevel::Instr::Drop => None, 

        highlevel::Instr::Select => { 
            panic_if_size_lt(&rhs, 3, "select requires that there is a condition and two other values on the stack"); 
            let arg1 = rhs.pop().unwrap(); //cond  //wasm spec pg 71/155
            let arg2 = rhs.pop().unwrap(); //if
            let arg3 = rhs.pop().unwrap(); //else
            Some(If {
                lhs,
                label: None,
                condition: arg1,
                if_body: Body {
                    instrs: Vec::new(),
                    result: Some(arg2),
                },
                else_body: Some(Body {
                    instrs: Vec::new(),
                    result: Some(arg3),
                }),
            })
        }

        //TODO: flatten double match
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
                        panic!("local.get requires a local variable to save a value into");
                    }
                }
                // set local variable so LHS is local variable
                highlevel::LocalOp::Set => {
                    panic_if_size_lt(&rhs, 1, "local.set expects a value on the stack"); 
                    Some(Assign {
                        lhs: local_var,
                        rhs: rhs.pop().unwrap(),
                    })
                }
                // like local set but also return argument -> top of stack
                highlevel::LocalOp::Tee => {
                    panic_if_size_lt(&rhs, 1, "local.tee expects a value on the stack"); 
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
                        panic!("global.get requires a local variable to save a value into");
                    }
                }
                highlevel::GlobalOp::Set => {
                    panic_if_size_lt(&rhs, 1, "global.set expects a value on the stack"); 
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
                panic!("Every load produces a value"); 
            }
            if rhs.len() != 1 {
                panic!("Every load consumes a value"); 
            }
            let lhs = lhs.unwrap();
            let rhs = rhs.pop().unwrap();
            Some(Load {
                lhs,
                op: *loadop,
                memarg: *memarg,
                addr: rhs,
            })
        }

        highlevel::Instr::Store(storeop, memarg) => {
            panic_if_size_lt(&rhs, 2, "store consumes two values from the stack"); 
            Some(Store {
                op: *storeop,
                memarg: *memarg,
                addr: rhs.pop().unwrap(),
                value: rhs.pop().unwrap(),
            })
        }

        highlevel::Instr::MemorySize(_) => {
            if let Some(lhs) = lhs {
                Some(MemorySize { lhs })
            } else {
                panic!("memory size has to produce a value"); 
            }
        }

        highlevel::Instr::MemoryGrow(ind) => {
            assert_eq!(ind.into_inner(), 0, "wasm mvp only has single memory");
            panic_if_size_lt(&rhs, 1, "memory_grow has to consume a value from stack"); 
            if let Some(lhs) = lhs {
                Some(MemoryGrow {
                    lhs,
                    pages: rhs.pop().unwrap(), 
                })
            } else {
                panic!("memory grow has to produce a value"); 
            }
        }

        highlevel::Instr::Const(val) => {
            if let Some(lhs) = lhs {
                Some(Const { lhs, val: *val })
            } else {
                panic!("const has to produce a value "); 
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
                panic!("numeric op has to produce a value "); 
            }
        }
    };

    if let Some(result_instr) = result_instr {
        result_instrs.push(result_instr);
    }

    if !instrs.is_empty() {
        if let Some(lhs) = lhs { 
            state.var_stack.push(Var::Stack(state.stack_var_count));
            if let Stack(_) = lhs {
                state.stack_var_count += 1;
            }
        }
        let res = wimplify_instrs(instrs, tys, state).unwrap();
        for inst in res {
            result_instrs.push(inst); 
        }
    }
    Ok(result_instrs)
}

static mut FN_COUNTER : usize = 0; 

pub fn wimplify(
    instrs: &[highlevel::Instr],
    function: &highlevel::Function,
    module: &Module,
) -> Result<Function, String> {

    let tys = types(instrs, function, module).map_err(|e| format!("{:?}", e))?;
    let mut instrs = VecDeque::from_iter(instrs); //pass in iterator instead of vecdeque
    let mut tys = VecDeque::from_iter(tys);
    let result_instrs = wimplify_instrs(&mut instrs, &mut tys, &mut State::default()).unwrap();
    
    let name = if let Some(name) = &function.name {
        name.clone()
    } else {
        unsafe{
            FN_COUNTER += 1;    
            FN_COUNTER.to_string()
        }
    }; 

    Ok(Function{
        type_: function.type_.clone(),
        instrs: Body{
            instrs: result_instrs,
            result: None,
        }, 
        name,
    })
}

#[cfg(test)]
mod test {
    // Convenience imports:
    use super::Body;
    use super::Func;
    use super::Instr::{self, *};
    use super::Label;
    use super::Var::{self, *};
    use super::{wimpl, wimpls};
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
        assert!(
            " s0 \n ".parse::<Var>().is_err(),
            "whitespace is not allowed"
        );
        assert!(
            "sABC".parse::<Var>().is_err(),
            "characters instead of number"
        );
        assert!(
            "x123".parse::<Var>().is_err(),
            "invalid variable type/prefix"
        );
    }

    #[test]
    fn parse_instr() {
        let parse_testcases = WIMPL_CANONICAL_SYNTAX_TESTCASES
            .iter()
            .chain(WIMPL_ALTERNATIVE_SYNTAX_TESTCASES.iter());
        for (i, (wimpl, text, msg)) in parse_testcases.enumerate() {
            let parsed = Instr::from_str(text);
            match parsed {
                Err(err) => panic!(
                    "\ntest #{} could not be parsed\ninput: `{}`\nerr: `{}`\n{}",
                    i, text, err, msg
                ),
                Ok(parsed) => {
                    assert_eq!(&parsed, wimpl, "\ntest #{}\ninput: `{}`\n{}", i, text, msg)
                }
            };
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

    #[test]
    fn parse_file() {
        let instrs = Instr::from_file("tests/wimpl/syntax.wimpl");
        assert!(instrs.is_ok());
    }

    #[test]
    fn macros() {
        let _ = wimpl!(g0 = f32.const 1.1);
        let _ = wimpl!(s2 = i32.add (s0, s1));
        let _ = wimpl!(call_indirect [ ] ->[] (s1) ());

        // Tricky, because rustc lexes these tokens differently than we need to.
        let _ = wimpl!(s3 = i32.load offset=3 (s0));
        let _ = wimpl!(@label0: block {});

        // Multiple instructions:
        let _ = wimpls! {};
        let _ = wimpls! {
            s4 = @label2: loop {
                s5 = i32.const 3
                br @label2 (s5)
            }
            l0 = g0
        };
    }
}

fn test (path_wimpl: &str, path_wasm: &str) {
    let expected = Instr::from_file(path_wimpl).unwrap();
    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

    let module = Module::from_file(path_wasm).unwrap();
    let func = module.functions().next().unwrap().1;
    let instrs = &func.code().unwrap().body.as_slice();
    let actual = wimplify(instrs, func, &module).unwrap();

    println!("\nACTUAL");
    println!("{}", actual);

    assert_eq!(actual.instrs.instrs, expected);
}

#[test]
fn constant() {
    test("tests/wimpl/const/const.wimpl", "tests/wimpl/const/const.wasm"); 
}

#[test]
fn add() {
    test("tests/wimpl/add/add.wimpl", "tests/wimpl/add/add.wasm");
}

#[test]
fn call_ind() {
    test("tests/wimpl/call_ind/call_ind.wimpl", "tests/wimpl/call_ind/call_ind.wasm");
}

#[test]
fn multiple_expr() {
    test("tests/wimpl/multiple_expr/multiple_expr.wimpl", "tests/wimpl/multiple_expr/multiple_expr.wasm");
}

#[test]
fn call() {
    test("tests/wimpl/call/call.wimpl","tests/wimpl/call/call.wasm");
}

#[test]
fn local() {
    test("tests/wimpl/local/local.wimpl", "tests/wimpl/local/local.wasm");
}

#[test]
fn global() {
    test("tests/wimpl/global/global.wimpl", "tests/wimpl/global/global.wasm");
}

#[test]
fn load_store() {
    test("tests/wimpl/load_store/load_store.wimpl", "tests/wimpl/load_store/load_store.wasm"); 
}

#[test]
fn load_store_qs() {
    test("tests/wimpl/qs1/qs1.wimpl", "tests/wimpl/qs1/qs1.wasm");
}

#[test]
fn memory() {
    test("tests/wimpl/memory/memory.wimpl", "tests/wimpl/memory/memory.wasm");
}

#[test]
fn select() {
    test("tests/wimpl/select/select.wimpl", "tests/wimpl/select/select.wasm");
}

#[test]
fn block_nested() {
    test("tests/wimpl/block_nested/block.wimpl", "tests/wimpl/block_nested/block.wasm");
}

#[test]
fn br_simple() {
    test("tests/wimpl/br_simple/br.wimpl", "tests/wimpl/br_simple/br.wasm");
}

#[test]
fn br_nested_simple() {  
    test("tests/wimpl/br_nested_simple/br.wimpl", "tests/wimpl/br_nested_simple/br.wasm");
}

#[test]
fn br_nested() {
    test("tests/wimpl/br_nested/br.wimpl", "tests/wimpl/br_nested/br.wasm");
}

#[test]
fn br_triple_nested() {
    test("tests/wimpl/br_triple_nested/br.wimpl", "tests/wimpl/br_triple_nested/br.wasm");
}

#[test]
fn br_if() {  
    test("tests/wimpl/br_if/br_if.wimpl", "tests/wimpl/br_if/br_if.wasm");
}

#[test]
fn br_if_2() {
    test("tests/wimpl/br_if_2/br_if.wimpl", "tests/wimpl/br_if_2/br_if.wasm");
}

#[test]
fn br_table() {  
    test("tests/wimpl/br_table/br_table.wimpl", "tests/wimpl/br_table/br_table.wasm");
}

#[test]
fn if_() {
    test("tests/wimpl/if/if.wimpl", "tests/wimpl/if/if.wasm");
}

#[test]
fn if_else() { 
    test("tests/wimpl/if_else/if_else.wimpl", "tests/wimpl/if_else/if_else.wasm");
}

