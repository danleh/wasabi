use std::{
    fmt::{self, Write},
    io::{self, ErrorKind},
    path::Path,
    str::FromStr,
};

use crate::{highlevel::{MemoryOp, Global, Table}, types::{InferredInstructionType, TypeChecker}, Val, ValType, Idx, BlockType};
use crate::{
    highlevel::{self, LoadOp, NumericOp, StoreOp},
    FunctionType, Memarg,
};


#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
    pub globals: Vec<Global>,
    pub tables: Vec<Table>,

    // From the name section, if present, e.g., compiler-generated debug info.
    // pub name: Option<String>,
    // pub memories: Vec<Memory>,
    // pub start: Option<Idx<Function>>,
    // pub custom_sections: Vec<RawCustomSection>,
}

impl Module {
    pub fn function(&self, name: Func) -> Option<&Function> {
        let mut functions = self.functions.iter().filter(|f| f.name == name);
        let function = functions.next();
        assert!(functions.next().is_none(), "more than one matching function for name {}", name);
        function
    }

    pub fn function_by_idx(&self, idx: Idx<highlevel::Function>) -> &Function {
        &self.functions[idx.into_inner()]
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Function {
    /// Either the name of a function (from debug info originally), or a
    /// numerical index.
    pub name: Func,
    pub type_: FunctionType,
    // TODO what about imported functions? I think we should make body an Option.
    pub body: Body,

    //pub export: Vec<String>,
    //pub param_names: Vec<Option<String>>,
}

impl Function {
    pub fn name(&self) -> Func {
        self.name.clone()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Func {
    /// If the function had a debug name attached to it (from the `name` custom section).
    Named(String),
    /// Otherwise, just refer to the function via its index, which is the same as in the original
    /// WebAssembly module.
    Idx(usize),
}

impl Func {
    pub fn from_idx(idx: Idx<highlevel::Function>, module: &highlevel::Module) -> Self {
        match module.function(idx).name.clone() {
            Some(name) => Func::Named(name),
            None => Func::Idx(idx.into_inner()),
        }
    }
}

/// A sequence of instructions, typically as the body of a function or block.
#[derive(Debug, Eq, PartialEq, Clone, Default, Hash)]
pub struct Body(pub Vec<Stmt>);

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Var {
    // These two correspond to the WebAssembly constructs of the same name.
    // Note that the index of locals in WebAssembly does not have to match the local numbering here,
    // because the "index space" of locals contains also the function parameters.
    Local(usize),
    Global(usize),

    /// Originally an (implicit) stack slot in the WebAssembly operand stack.
    Stack(usize),
    /// Originally a parameter to the current function (which would have been accessed via
    /// `local.get` and was in the same index space as locals).
    Param(usize),
    /// Originally the result value of a block with non-empty block type.
    BlockResult(usize),
    /// Originally the return value of the function.
    /// In the Wasm MVP, there is always only a single one.
    Return(usize),
}

/// An absolute block label, NOT to be confused with the relative branch labels of WebAssembly!
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Default)]
pub struct Label(usize);

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
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Stmt {

    // Simplify nop: Not necessary for analysis.

    Unreachable,

    /// Expression statement, expression is executed for side-effects only.
    Expr(Expr),

    /// This unifies all local.set, global.set, local.tee, local.get, and global.get,
    /// and data-flow before branches ("br with value" in Wasm).
    Assign {
        lhs: Var,
        type_ : ValType,
        rhs: Expr,
    },

    Store {
        op: StoreOp,
        memarg: Memarg,
        addr: Var,
        value: Var,
    },

    // Simplify drop: This is just a dead variable, no instruction needed.
    // Simplify select: Encode as `if (arg0) { s0 = arg1 } else { s0 = arg2 }`.

    /// `br` is the only branching construct in Wimpl.
    /// `br_if` is represented as `if (cond) { .. , br @label }`.
    /// `br_table` as `switch (idx) { case 0: { br @label } ... }`.
    /// `return` is represented as `br @label_body`.
    /// Where for all cases the carried value is assigned explicitly beforehand.
    Br {
        target: Label,
    },

    Block {
        body: Body,
        end_label: Label,
    },

    Loop {
        begin_label: Label,
        body: Body,
    },

    // If the if was target of a branch in the original WebAssembly, wrap
    // this in a Wimpl block to have a branch target label available.
    // TODO technically, we could simplify/represent If by Switch as well...
    // Downside: br_if (which are frequent) would be expanded into a quite complex switch:
    // br_if (cond) (value) @label
    // -> if (cond) { br (value) @label }
    // -> if (cond) { b0 = value; br @label }
    // -> switch (cond) { case 0: {} default: { b0 = value; br @label } }
    If {
        condition: Var,
        if_body: Body,
        else_body: Option<Body>,
    },

    /// Similar to C switch statement, but doesn't fallthrough from one case to the next.
    Switch {
        index: Var,
        cases: Vec<Body>,
        default: Body,
    },

}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Expr {

    VarRef(Var),

    Const(Val),

    // TODO Make Expr recursive (i.e., allow for folded expressions) by replacing all occurrences of
    // `Var` below with `Box<Expr>`.

    Load {
        op: LoadOp,
        memarg: Memarg,
        addr: Var,
    },

    MemorySize,
    MemoryGrow { pages: Var },

    Numeric {
        op: NumericOp,
        args: Vec<Var>,
    },

    Call {
        func: Func,
        args: Vec<Var>,
    },

    CallIndirect {
        type_: FunctionType,
        table_idx: Var,
        args: Vec<Var>,
    },

}


// Pretty-printing of Wimpl:

const PRETTY_PRINT_INDENT: &str = "  ";
const PRETTY_PRINT_NEWLINE_INDENT: &str = "\n  ";

/// Helper function that indents each line of the `Display` output of `x`.
fn indent_once(x: &dyn fmt::Display) -> String {
    // This allocates a new String just for indentation, which seems wasteful.
    // But since we pretty-print Wimpl fairly seldom, and even if, it's not performance criticial,
    // and there are also other places below where we allocate, let's not optimize this for now.
    // If it ever really becomes a problem, use the `pad-adapter` crate.
    format!("{}{}", PRETTY_PRINT_INDENT, x).replace("\n", PRETTY_PRINT_NEWLINE_INDENT)
}

/// Helper function for `fmt::Display`-ing an arbitrary iterator of `values`, where each element is
/// separated by `delim` and if the iterator is non-empty surrounded by `begin` and `end`.
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

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "module {{").expect("");
        for func in &self.functions {
            writeln!(f, "{}", indent_once(func))?;
        }
        writeln!(f, "}}")
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "func {}", self.name)?;
        write!(f, " (")?;
        for (i, ty) in self.type_.params.iter().enumerate() {
            write!(f, "{}: {}", Var::Param(i), ty)?;
            if i < self.type_.params.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ") -> (")?;
        for (i, ty) in self.type_.results.iter().enumerate() {
            write!(f, "{}: {}", Var::Return(i), ty)?;
            if i < self.type_.results.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ") {}: ", Label(0))?;
        write!(f, "{}", self.body)
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return f.write_str("{}");
        }

        // Put each instruction on a separate line.
        let mut inner = String::new();
        for instr in &self.0 {
            writeln!(inner, "{}", instr)?;
        }
        // Pop-off the last superfluous newline.
        inner.pop();

        // If the inner part (inside the curly braces) is only a single line, e.g., a single br
        // instruction, then print the whole body as a single line as well.
        if inner.lines().count() == 1 {
            write!(f, "{{ {} }}", inner)
        } else {
            write!(f, "{{\n{}\n}}", indent_once(&inner))
        }
    }
}

impl fmt::Display for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Func::Named(s) => write!(f, "{}", s),
            Func::Idx(i) => write!(f, "f{}", i),
        }
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Var::*;
        match self {
            Local(i) => write!(f, "l{}", i),
            Global(i) => write!(f, "g{}", i),
            Stack(i) => write!(f, "s{}", i),
            Param(i) => write!(f, "p{}", i),
            BlockResult(i) => write!(f, "b{}", i),
            Return(i) => write!(f, "r{}", i),
        }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@label{}", self.0)
    }
}

// Pretty-prints instructions, including indenting nested blocks.
// Comments show examples.
// Conventions for the text format:
// - Things in parentheses (x, y) signify runtime arguments.
// - Everything outside of the parentheses is statically encoded into the
//   instruction.
// - Curly braces { ... } for nesting.
impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Stmt::*;
        match self {

            Unreachable => f.write_str("unreachable")?,

            Expr(expr) => write!(f, "{}", expr)?,

            Assign { lhs, rhs , type_} => {
                write!(f, "{}: {} = {}", lhs, type_, rhs)?;
            },

            // i32.store offset=3 align=4 (s0) (s1)
            // The first argument is addr, second is value.
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
            },

            Br { target } => write!(f, "br {}", target)?,

            // @label0: block {
            //   s1 = i32.const 3
            // }
            Block {
                end_label,
                body
            } => write!(f, "{}: block {}", end_label, body)?,

            Loop {
                begin_label,
                body
            } => write!(f, "{}: loop {}", begin_label, body)?,

            If {
                condition,
                if_body,
                else_body
            } => {
                write!(f, "if ({}) {}", condition, if_body)?;
                if let Some(else_body) = else_body {
                    write!(f, " else {}", else_body)?;
                }
            },

            // switch (s0) {
            //   case 0: {
            //     b0 = i32.const 42
            //     br @label0
            //   }
            //   ...
            // }
            Switch {
                index,
                cases,
                default,
            } => {
                writeln!(f, "switch ({}) {{", index)?;
                for (i, case) in cases.iter().enumerate() {
                    writeln!(f, "{}", indent_once(&format!("case {}: {}", i, case)))?;
                }
                writeln!(f, "{}", indent_once(&format!("default: {}", default)))?;
                f.write_str("}")?;
            },

        }
        Ok(())
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expr::*;
        match self {

            VarRef(var) => write!(f, "{}", var),

            // i32.const 3
            Const(val) => write!(f, "{}.const {}", val.to_type(), val),

            // i32.load offset=3 align=4 (s0)
            Load {
                op,
                memarg,
                addr
            } => {
                write!(f, "{}", op)?;
                if !memarg.is_default(*op) {
                    f.write_str(" ")?;
                    memarg.fmt(f, *op)?;
                    f.write_str(" ")?;
                }
                write!(f, "({})", addr)
            },

            MemorySize => write!(f, "memory.size"),
            MemoryGrow { pages } => write!(f, "memory.grow({})", pages),

            // i32.add(s0, s1)
            // f32.neg(s0)
            Numeric { op, args } => {
                write!(f, "{}", op)?;
                display_delim(f, args, "(", ")", ", ")
            },

            // call f1 (s1, s2)
            Call {
                func,
                args
            } => {
                // Always print the parentheses, even if `args` is empty.
                write!(f, "call {} (", func)?;
                display_delim(f, args, "", "", ", ")?;
                f.write_str(")")
            },

            // call_indirect [] -> [i32] (s0) (s1, s2, s3...)
            // The first argument is the table index, the others the args.
            CallIndirect {
                type_,
                table_idx,
                args
            } => {
                write!(f, "call_indirect {} ({}) (", type_, table_idx)?;
                display_delim(f, args, "", "", ", ")?;
                f.write_str(")")
            },

        }
    }
}


// Parsing of the Wimpl text format:

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
            "l" => Local(i),
            "g" => Global(i),
            "s" => Stack(i),
            "p" => Param(i),
            "b" => BlockResult(i),
            "r" => Return(i),
            _ => return Err(()),
        })
    }
}

impl FromStr for Func {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(idx) = s.strip_prefix('f') {
            let idx = idx.parse().map_err(|_| ())?;
            Func::Idx(idx)
        } else {
            // Functions must start with an alphabetic character, to avoid confusion with constants.
            match s.chars().next() {
                Some(c) if c.is_alphabetic() => Func::Named(s.to_string()),
                _ => return Err(()),
            }
        })
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

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_until, take_while, take_while1},
        character::complete::{alphanumeric1, multispace1, not_line_ending, u32},
        combinator::{all_consuming, map, map_res, opt, value},
        multi::{many0, separated_list0},
        sequence::{delimited, pair, preceded, terminated, tuple},
        AsChar, Finish, IResult,
    };

    use super::*;

    /// Type abbreviation for nom's own parser result type.
    pub type NomResult<'input, O> = IResult<&'input str, O>;

    /// Adapt a nom parser such that it returns a standard Rust `Result`.
    pub fn adapt_nom_parser<'input, O>(
        parser: impl Fn(&'input str) -> NomResult<'input, O>,
        input: &'input str,
    ) -> Result<O, ParseError> {
        match all_consuming(parser)(input).finish() {
            Ok(("", instr)) => Ok(instr),
            Ok(_) => unreachable!("successful parse should hould have consumed full input because of all_consuming()"),
            Err(err) => Err(ParseError::from(err, input)),
        }
    }

    // Utility parsers, used to build more complex parsers below.

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

    // The following utility parsers all assume only "internal" whitespace, i.e., they assume the
    // initial whitespace is already consumed by the outer parser and the input directly starts with
    // the first non-whitespace token.
    fn var(input: &str) -> NomResult<Var> {
        map_res(alphanumeric1, Var::from_str)(input)
    }
    fn func(input: &str) -> NomResult<Func> {
        map_res(alphanumeric1, Func::from_str)(input)
    }
    fn label(input: &str) -> NomResult<Label> {
        map_res(
            take_while(|c: char| c == '@' || c.is_alphanum()),
            Label::from_str,
        )(input)
    }
    fn val_type(input: &str) -> NomResult<ValType> {
        map_res(alphanumeric1, ValType::from_str)(input)
    }
    fn op<T: FromStr>(input: &str) -> NomResult<T> {
        map_res(
            take_while(|c: char| c.is_alphanum() || c == '.' || c == '_'),
            T::from_str
        )(input)
    }

    fn arg_single(input: &str) -> NomResult<Var> {
        delimited(
            pair(tag("("), ws), var, pair(ws, tag(")"))
        )(input)
    }
    fn arg_list(input: &str) -> NomResult<Vec<Var>> {
        delimited(
            pair(tag("("), ws),
            separated_list0(tuple((ws, tag(","), ws)), var),
            pair(ws, tag(")")),
        )(input)
    }

    // The defaults of a memarg (if not given) depend on the natural alignment
    // of the memory instruction, hence this higher-order combinator.
    fn memarg<'a>(op: impl MemoryOp + 'a) -> impl FnMut(&'a str) -> NomResult<'a, Memarg> {
        // Same trick as for function types in call_indirect: Consume until beginning of argument list.
        map_res(take_until("("), move |s| Memarg::from_str(s, op))
    }

    // Memarg parsing depends on result of previous LoadOp/StoreOp parsing.
    // This is easier to write in direct than in point-free style, so we do.
    fn load(input: &str) -> NomResult<Expr> {
        let (input, op) = op::<LoadOp>(input)?;
        let (input, memarg) = memarg(op)(input)?;
        let (input, addr) = arg_single(input)?;
        Ok((
            input,
            Expr::Load {
                op,
                memarg,
                addr,
            },
        ))
    }

    fn store(input: &str) -> NomResult<Stmt> {
        let (input, op) = op::<StoreOp>(input)?;
        let (input, memarg) = memarg(op)(input)?;
        let (input, addr) = arg_single(input)?;
        let (input, ()) = ws(input)?;
        let (input, value) = arg_single(input)?;
        Ok((
            input,
            Stmt::Store {
                op,
                memarg,
                addr,
                value,
            },
        ))
    }

    pub fn expr(input: &str) -> NomResult<Expr> {
        use Expr::*;

        let var_ref = map(var, VarRef);

        // This is a bit tricky to parse, because `Val::from_str` requires a tyle as input,
        // so parse the type first, the number only approximately, and then pass both later.
        let const_ = map_res(
            tuple((
                val_type,
                tag(".const"),
                ws,
                // HACK Accept any non-whitespace character for the integer/float
                // immediate, the rest of the parsing is done by Val::from_str.
                take_while1(|c: char| !c.is_ascii_whitespace()),
            )),
            |(ty, _, (), number)| Val::from_str(number, ty).map(Const),
        );

        let memory_size = map(tag("memory.size"), |_| MemorySize);
        let memory_grow = map(
            tuple((tag("memory.grow"), ws, arg_single)),
            |(_, (), pages)| MemoryGrow { pages },
        );

        let numeric = map(
            tuple((op::<NumericOp>, ws, arg_list)),
            |(op, (), args)| Numeric { op, args },
        );

        let call = map(
            tuple((tag("call"), ws, func, ws, arg_list)),
            |(_, (), func, (), args)| Call { func, args }
        );

        // HACK For call_indirect, we know nothing besides the argument list is following
        // the function type, so consume up to the opening parenthesis.
        // However, this will fail to recognize comments after the function type!
        let func_ty = map_res(take_until("("), FunctionType::from_str);
        let call_indirect = map(
            tuple((
                tag("call_indirect"),
                ws,
                func_ty,
                ws,
                arg_single,
                ws,
                arg_list,
            )),
            |(_, (), type_, (), table_idx, (), args)|
                CallIndirect { type_, table_idx, args }
        );

        alt((var_ref, const_, load, memory_size, memory_grow, numeric, call, call_indirect))(input)
    }

    /// Parse multiple statements, with possibly preceding and trailing whitespace.
    pub fn stmts_ws(input: &str) -> NomResult<Vec<Stmt>> {
        preceded(ws, many0(terminated(stmt, ws)))(input)
    }

    fn body(input: &str) -> NomResult<Body> {
        map(delimited(tag("{"), stmts_ws, tag("}")), Body)(input)
    }

    /// Parse a single statement, without surrounding whitespace.
    pub fn stmt(input: &str) -> NomResult<Stmt> {
        use Stmt::*;

        let unreachable = map(tag("unreachable"), |_| Unreachable);

        let expr_stmt = map(expr, Expr);

        let assign = map(
            tuple((var, ws, tag(":"), ws, val_type, ws, tag("="), ws, expr)),
            |(lhs, (), _, (), type_, (), _, (), rhs)| Assign { lhs, type_, rhs }
        );

        let br = map(
            tuple((tag("br"), ws, label)),
            |(_, (), target)| Br { target },
        );

        let block = map(
            tuple((label, ws, tag(":"), ws, tag("block"), ws, body)),
            |(end_label, (), _, (), _, (), body)| Block { end_label, body }
        );
        let loop_ = map(
            tuple((label, ws, tag(":"), ws, tag("loop"), ws, body)),
            |(begin_label, (), _, (), _, (), body)| Loop { begin_label, body }
        );

        let if_ = map(
            tuple((
                tag("if"),
                ws,
                arg_single,
                ws,
                body,
                opt(preceded(tuple((ws, tag("else"), ws)), body)),
            )),
            |(_, (), condition, (), if_body, else_body)| If { condition, if_body, else_body }
        );

        // Also parse the case number, but only for validation, then thrown away.
        let case = map(
            tuple((tag("case"), ws, u32, ws, tag(":"), ws, body)),
            |(_, (), i, (), _, (), body)| (i, body)
        );
        let default = map(
            tuple((tag("default"), ws, tag(":"), ws, body)),
            |(_, (), _, (), body)| body
        );
        let switch = map_res(
            tuple((
                tag("switch"),
                ws,
                arg_single,
                ws,
                tag("{"),
                ws,
                many0(terminated(case, ws)),
                default,
                ws,
                tag("}"),
            )),
            |(_, (), index, (), _, (), cases, default, (), _)| {
                let (case_i, cases): (Vec<_>, Vec<_>) = cases.into_iter().unzip();
                // Ensure that the case numbers are ascending and the same length as the cases.
                if !(0..cases.len()).eq(case_i.into_iter().map(|i| i as usize)) {
                    return Err(())
                }

                Ok(Switch { index, cases, default })
            }
        );

        alt((
            unreachable,
            assign,
            store,
            br,
            block,
            loop_,
            if_,
            switch,

            // HACK Order this case below assign, because otherwise the parser parses a LHS
            // variable as an `Stmt::Expr(Expr::VarRef(...))`, continues, and fails with the rest.
            // Otherwise
            expr_stmt,
        ))(input)
    }

}

// Adapt the nom parsers above such that they can be used with Rust `parse()` / `from_str`.

impl Stmt {
    pub fn from_str_multiple(input: &str) -> Result<Vec<Self>, ParseError> {
        parser::adapt_nom_parser(parser::stmts_ws, input)
    }

    /// Convenience function to parse Wimpl from a filename.
    // TODO Implement parser for wimpl::Module, Function.
    // TODO Once parsing is implemented, add from_text_file to wimpl::Module and return Module there.
    pub fn from_text_file(filename: impl AsRef<Path>) -> io::Result<Vec<Self>> {
        let str = std::fs::read_to_string(filename)?;
        Self::from_str_multiple(&str).map_err(|e| io::Error::new(ErrorKind::Other, e))
    }
}

impl FromStr for Expr {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::adapt_nom_parser(parser::expr, input)
    }
}

impl FromStr for Stmt {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::adapt_nom_parser(parser::stmt, input)
    }
}

/// Convenience macro to write Wimpl statements in Rust.
#[cfg(test)]
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
            match Stmt::from_str_multiple(&input_str) {
                Ok(instrs) => instrs,
                Err(err) => panic!("Invalid Wimpl instriction(s).\n{}\n(Note: whitespace might be different from your input.)", err)
            }
        }
    }
}

/// Macro for a single Wimpl statement, see `wimpls!`.
#[cfg(test)]
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

pub struct State<'module> {
    type_checker: TypeChecker<'module>,

    label_count: usize,
    stack_var_count: usize,

    // The bool is `true` if the label is for a `loop` block, and false for `block` and `if`.
    #[allow(clippy::type_complexity)]
    label_stack: Vec<(Label, Option<(Var, ValType, bool)>)>,
}

#[derive(Clone, Copy)]
pub struct Context<'module> {
    func_ty: &'module FunctionType,
    module: &'module highlevel::Module,
}

fn wimplify_instrs<'module>(
    instrs: &mut impl Iterator<Item=&'module highlevel::Instr>,
    context: Context<'module>,
    state: &mut State,
) -> Result<(Vec<Stmt>, /* was_else */ bool), String> {

    use Expr::*;
    use Var::*;

    // State that is "local" to this nested block (unlike `state`, which is for the whole function).
    let mut var_stack = Vec::new();
    let mut result_instrs = Vec::new();

    while let Some(instr) = instrs.next() {
        let ty = state.type_checker.check_next_instr(instr).map_err(|e| e.to_string())?;

        // DEBUG
        // println!("{}, {}, {:?}", instr, ty, var_stack);

        let ty = match ty {
            // If this code is unreachable, we don't generate any Wimpl code from it, so exit early.
            InferredInstructionType::Unreachable => {
                match instr {
                    highlevel::Instr::End => {
                        state.label_stack.pop();
                        return Ok((result_instrs, false))
                    },
                    highlevel::Instr::Else => {
                        return Ok((result_instrs, true))
                    }
                    _ => continue
                }
            },
            InferredInstructionType::Reachable(ty) => ty,
        };


        // Utility functions for the conversion:

        // Only call this function when you have finished translating the instructions, i.e., after
        // you have popped all inputs from the `var_stack`, since this changes `var_stack`.
        fn create_fresh_stack_var(state: &mut State, var_stack: &mut Vec<Var>) -> Var {
            let var = Stack(state.stack_var_count);
            state.stack_var_count += 1;
            var_stack.push(var);
            var
        }

        fn local_idx_to_var(context: Context, local_idx: Idx<highlevel::Local>) -> Var {
            let local_idx = local_idx.into_inner();
            let num_params = context.func_ty.params.len();
            if local_idx < num_params {
                Param(local_idx)
            } else {
                Local(local_idx - num_params)
            }
        }

        fn blocktype_to_var_and_init(blocktype: BlockType, state: &mut State, var_stack: &mut Vec<Var>) -> (Vec<Stmt>, Option<(Var, ValType)>) {
            if let BlockType(Some(type_)) = blocktype {
                let result_var = BlockResult(state.label_count);
                // The block result is available as a variable (for translating branches with values).
                var_stack.push(result_var);

                let init = Stmt::Assign {
                    lhs: result_var,
                    rhs: Const(Val::get_default_value(type_)),
                    type_,
                };

                (vec![init], Some((result_var, type_)))
            } else {
                (Vec::new(), None)
            }
        }

        let label_to_instrs = |label: crate::Label, label_stack: &[_], get_value: &mut dyn FnMut() -> Var| -> Vec<Stmt> {
            let (target, block_result) = *label_stack.iter().rev().nth(label.into_inner()).expect("invalid branch label, not in label stack");

            // println!("{:?}\n{:?} {:?} {:?}", label_stack, label, target, return_info);

            match block_result {
                // Target block needs a result, and is not a loop.
                Some((lhs, type_, false)) => vec![
                    Stmt::Assign{
                        lhs,
                        rhs: VarRef(get_value()),
                        type_,
                    },
                    Stmt::Br {
                        target,
                    }
                ],
                // Target block either has no result, or is a loop.
                Some((_, _, true)) | None => vec![Stmt::Br {
                    target,
                }]
            }
        };


        // Conversion of each WebAssembly instruction to (one or more) Wimpl statements:

        result_instrs.append(&mut match instr {

            highlevel::Instr::Unreachable => vec![Stmt::Unreachable],

            highlevel::Instr::Nop => Vec::new(),

            // Block and loop share so much code, so handle them together.
            highlevel::Instr::Block(blocktype) |
            highlevel::Instr::Loop(blocktype) => {
                let (mut stmts, result_var) = blocktype_to_var_and_init(*blocktype, state, &mut var_stack);
                let is_loop = match instr {
                    highlevel::Instr::Loop(_) => true,
                    highlevel::Instr::Block(_) => false,
                    _ => unreachable!(),
                };

                let label = Label(state.label_count);
                state.label_count += 1;
                state.label_stack.push((label, result_var.map(|(var, ty)| (var, ty, is_loop))));

                let (block_body, ends_with_else) = wimplify_instrs(instrs, context, state)?;
                assert!(!ends_with_else, "block and loop are terminated by end, not else");

                stmts.push(match instr {
                    highlevel::Instr::Block(_) => Stmt::Block {
                        end_label: label,
                        body: Body(block_body),
                    },
                    highlevel::Instr::Loop(_) => Stmt::Loop {
                        begin_label: label,
                        body: Body(block_body),
                    },
                    _ => unreachable!(),
                });
                stmts
            }

            highlevel::Instr::If(blocktype) => {
                let condition = var_stack.pop().expect("if expects a condition which was not found on the stack");

                let (mut stmts, result_var) = blocktype_to_var_and_init(*blocktype, state, &mut var_stack);

                let label = Label(state.label_count);
                state.label_count += 1;
                state.label_stack.push((label, result_var.map(|(var, ty)| (var, ty, false))));

                let (if_body, has_else) = wimplify_instrs(instrs, context, state)?;
                let else_body = if has_else {
                    let (else_body, ends_with_else) = wimplify_instrs(instrs, context, state)?;
                    assert!(!ends_with_else, "else block must end with end instruction");
                    Some(Body(else_body))
                } else {
                    None
                };

                // Wrap `if` inside a `block`, because Wimpl ifs don't have a label, but if a 
                // branch wants to exit the if, it needs to target a label.
                // TODO do not generate the surrounding block if no branch targets it
                // -> requires a precomputed map from branches to targets
                stmts.push(Stmt::Block {
                    end_label: label,
                    body: Body(vec![Stmt::If {
                        condition,
                        if_body: Body(if_body),
                        else_body,
                    }])
                });
                stmts
            },

            highlevel::Instr::Else => {
                // Cannot pop because you still want it to be on the label stack while processing the else body.
                let (_, result_info) = *state.label_stack.last().expect("label stack should include if label");

                // assign of the if statement that we just finished processing
                // the required value returned by if (if any) should be at the top of the var_stack
                if let Some((if_result_var, type_, is_loop_block)) = result_info {
                    assert!(!is_loop_block, "if block result should never be have loop flag set");
                    result_instrs.push(Stmt::Assign {
                        lhs: if_result_var,
                        rhs: VarRef(var_stack.pop().expect("if block is producing a value which is expected on the stack")),
                        type_,
                    })
                }

                // End recursive invocation and return converted body of the current block.
                return Ok((result_instrs, true));
            },

            highlevel::Instr::End => {
                let (_, result_info) = state.label_stack.pop().expect("end of a block expects the matching label to be in the label stack");

                if let Some((block_result_var, type_, _is_loop_block)) = result_info {
                    result_instrs.push(Stmt::Assign{
                        lhs: block_result_var,
                        rhs: VarRef(var_stack.pop().expect("the block is producing a value for which it expect a value on the stack")),
                        type_,
                    });
                };

                // End recursive invocation and return converted body of the current block.
                return Ok((result_instrs, false))
            }

            highlevel::Instr::Br(label) => {
                label_to_instrs(*label, &state.label_stack, &mut || var_stack.pop().expect("br expected a value to return"))
            },

            highlevel::Instr::BrIf(label) => {
                let condition = var_stack.pop().expect("if requires a conditional statement");
                vec![Stmt::If{
                    condition,
                    if_body: Body(label_to_instrs(*label, &state.label_stack, &mut || {
                        *var_stack.last().expect("br_if expected value to return")
                    })),
                    else_body: None,
                }]
            }

            highlevel::Instr::BrTable { table, default } => {
                let index = var_stack.pop().expect("br_table requires an index into the table to be supplied");

                let mut should_pop = false;
                let get_result_val = &mut || {
                    should_pop = true;
                    *var_stack.last().expect("br_table expected value to return")
                };

                let default = Body(label_to_instrs(*default, &state.label_stack, get_result_val));
                let cases = table.iter().copied().map(|label| Body(label_to_instrs(label, &state.label_stack, get_result_val))).collect();

                if should_pop {
                    var_stack.pop().expect("last succeeded, so pop should as well");
                }

                vec![Stmt::Switch {
                    index,
                    cases,
                    default,
                }]
            }

            highlevel::Instr::Return => {
                // This points to the block for the overall function body.
                let target = Label(0);

                if let (target_from_stack, Some((return_var, type_, loop_flag))) = *state.label_stack.first().expect("empty label stack, but expected function ") {
                    assert_eq!(target, target_from_stack, "label stack is invalid, should have been the function label");
                    assert!(!loop_flag, "function should not have loop flag set");

                    let return_val = var_stack.pop().expect("return expects a return value");
                    vec![
                        Stmt::Assign{
                            lhs: return_var,
                            type_,
                            rhs: VarRef(return_val)
                        },
                        Stmt::Br { target }
                    ]
                } else {
                    vec![Stmt::Br { target }]
                }
            }

            highlevel::Instr::Call(func_index) => {
                let n_args = context.module.function(*func_index).type_.params.len();
                let rhs = Call {
                    func: Func::from_idx(*func_index, context.module),
                    args: var_stack.split_off(var_stack.len() - n_args),
                };
                match &ty.results[..] {
                    [] => vec![Stmt::Expr(rhs)],
                    [type_] => {
                        vec![Stmt::Assign {
                            lhs: create_fresh_stack_var(state, &mut var_stack),
                            rhs,
                            type_: *type_,
                        }]
                    }
                    _ => panic!("WebAssembly multi-value extension")
                }
            },

            highlevel::Instr::CallIndirect(func_type, table_index) => {
                assert_eq!(table_index.into_inner(), 0, "WebAssembly MVP must always have a single table");

                let rhs = CallIndirect{
                    type_: func_type.clone(),
                    table_idx: var_stack.pop().expect("call_indirect requires an index"),
                    args: var_stack.split_off(var_stack.len() - func_type.params.len()),
                };
                match &ty.results[..] {
                    [] => vec![Stmt::Expr(rhs)],
                    [type_] => {
                        vec![Stmt::Assign {
                            lhs: create_fresh_stack_var(state, &mut var_stack),
                            rhs,
                            type_: *type_
                        }]
                    }
                    _ => panic!("WebAssembly multi-value extension")
                }
            }

            highlevel::Instr::Drop => Vec::new(),

            highlevel::Instr::Select => {
                let condition = var_stack.pop().expect("select requires a value on the stack for the condition");
                let if_result_val = var_stack.pop().expect("select requires a value on the stack for the then case");
                let else_result_val = var_stack.pop().expect("select requires a value on the stack for the else case");
                let type_ = ty.results[0];
                let lhs = create_fresh_stack_var(state, &mut var_stack);
                vec![
                    Stmt::Assign{
                        lhs,
                        rhs: Const(Val::get_default_value(type_)),
                        type_
                    },
                    Stmt::If {
                        condition,
                        if_body: Body(vec![Stmt::Assign {
                            lhs,
                            rhs: VarRef(if_result_val),
                            type_
                        }]),
                        else_body: Some(Body(vec![Stmt::Assign {
                            lhs,
                            rhs: VarRef(else_result_val),
                            type_
                        }]))
                    }
                ]

            }

            highlevel::Instr::Local(highlevel::LocalOp::Get, local_idx) => {
                vec![Stmt::Assign{
                    rhs: VarRef(local_idx_to_var(context, *local_idx)),
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results[0],
                }]
            }

            highlevel::Instr::Local(highlevel::LocalOp::Set, local_idx) => {
                vec![Stmt::Assign {
                    lhs: local_idx_to_var(context, *local_idx),
                    type_: ty.params[0],
                    rhs: VarRef(var_stack.pop().expect("local.set expects a value on the stack")),
                }]
            }

            highlevel::Instr::Local(highlevel::LocalOp::Tee, local_idx) => {
                vec![Stmt::Assign{
                    lhs: local_idx_to_var(context, *local_idx),
                    type_: ty.params[0],
                    rhs: VarRef(*var_stack.last().expect("local.tee expects a value on the stack")),
                }]
            }

            highlevel::Instr::Global(highlevel::GlobalOp::Get, global_ind) => {
                let global_var = Global(global_ind.into_inner());
                vec![Stmt::Assign{
                    rhs: VarRef(global_var),
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results[0],
                }]

            }

            highlevel::Instr::Global(highlevel::GlobalOp::Set, global_ind) => {
                let global_var = Global(global_ind.into_inner());
                vec![Stmt::Assign{
                    lhs: global_var,
                    rhs: VarRef(var_stack.pop().expect("global.set expects a value on the stack")),
                    type_: *ty.params.get(0).expect("return type of global.set not found"),
                }]
            }

            highlevel::Instr::Load(loadop, memarg) => {
                let rhs = var_stack.pop().expect("Every load consumes a value");
                vec![Stmt::Assign{
                    rhs: Load {
                        op: *loadop,
                        memarg: *memarg,
                        addr: rhs,
                    },
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results[0],
                }]
            }

            highlevel::Instr::Store(op, memarg) => {
                vec![Stmt::Store {
                    op: *op,
                    memarg: *memarg,
                    addr: var_stack.pop().expect("store consumes a value for address from the stack which was not found"),
                    value: var_stack.pop().expect("store consumes a value to store at the address from the stack which was not found"),
                }]
            }

            highlevel::Instr::MemorySize(memory_idx) => {
                assert_eq!(memory_idx.into_inner(), 0, "wasm mvp only has single memory");

                vec![Stmt::Assign{
                    rhs: MemorySize{},
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results[0],
                }]
            }

            highlevel::Instr::MemoryGrow(memory_idx) => {
                assert_eq!(memory_idx.into_inner(), 0, "wasm mvp only has single memory");

                vec![Stmt::Assign {
                    rhs: MemoryGrow {
                        pages: var_stack.pop().expect("memory.grow has to consume a value from stack"),
                    },
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results[0],
                }]
            }

            highlevel::Instr::Const(val) => {
                vec![Stmt::Assign{
                    rhs: Const(*val),
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results[0],
                }]
            }

            highlevel::Instr::Numeric(op) => {
                vec![Stmt::Assign{
                    rhs: Numeric {
                        op: *op,
                        args: var_stack.split_off(var_stack.len() - ty.params.len()),
                    },
                    lhs: create_fresh_stack_var(state, &mut var_stack),
                    type_: ty.results[0],
                }]
            }
        });
    }

    Ok((result_instrs, false))
}

pub fn wimplify_module(module: &highlevel::Module) -> Result<Module, String> {
    let mut wimpl_funcs = Vec::new();
    for (idx, func) in module.functions() {

        //initialize the local variables
        let mut result_instrs = Vec::new();
        for (loc_index, loc) in func.locals() {
            let (loc_name, loc_type) = (&loc.name, loc.type_);
            if let Some(_loc_name) = loc_name {
                todo!("you haven't yet implemented locals having names");
            } else {
                result_instrs.push(Stmt::Assign{
                    lhs: Var::Local(loc_index.into_inner()-func.type_.params.len()),
                    rhs: Expr::Const(Val::get_default_value(loc_type)),
                    type_: loc_type,
                })
            }
        }

        //translate the instructions in the function
        if let Some(code) = func.code() {
            let mut instrs = code.body.as_slice().iter();

            let context = Context {
                module,
                func_ty: &func.type_
            };

            let mut state = State {
                type_checker: TypeChecker::begin_function(func, module),
                label_stack: Vec::new(),
                label_count: 1, // 0 is already used by the function body block.
                stack_var_count: 0,
            };

            let return_var = match func.type_.results[..] {
                [] => None,
                [ty] => Some((Var::Return(0), ty, false)),
                _ => unimplemented!("only WebAssembly MVP is supported, not multi-value extension")
            };
            state.label_stack.push((Label(0), return_var));

            let (mut stmts, _) = wimplify_instrs(&mut instrs, context, &mut state)?;
            result_instrs.append(&mut stmts);
        }

        wimpl_funcs.push(Function{
            type_: func.type_.clone(),
            body: Body(result_instrs),
            name: Func::from_idx(idx, module)
        });
    }

    Ok(Module{
        functions: wimpl_funcs,
        globals: module.globals.clone(),
        tables: module.tables.clone(),
    })
}

pub fn wimplify(path: &str) -> Result<Module, String> {
    wimplify_module(&highlevel::Module::from_file(path).expect("path should point to a valid wasm file"))
}

#[cfg(test)]
mod tests {
    // Convenience imports:
    use super::Module;
    use super::Function;
    use super::Body;
    use super::Func;
    use super::Stmt::{self, *};
    use super::Expr::*;
    use super::Label;
    use super::Var::{self, *};
    //use super::{wimpl, wimpls};
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

        static ref WIMPL_MODULE_SYNTAX_TESTCASES: Vec<(Module, &'static str, &'static str)> = vec![
            (
                Module {
                    functions: Vec::new(),
                    globals: Vec::new(),
                    tables: Vec::new(),
                },
                "module {
}
",
                "empty module"
            ),
            (
                Module {
                    functions: vec![
                        Function {
                            name: Func::Idx(0),
                            type_: FunctionType { params: Vec::new().into_boxed_slice(), results: Vec::new().into_boxed_slice() },
                            body: Body(Vec::new())
                        },
                        Function {
                            name: Func::Idx(1),
                            type_: FunctionType { params: Vec::new().into_boxed_slice(), results: Vec::new().into_boxed_slice() },
                            body: Body(Vec::new())
                        },
                    ],
                    globals: Vec::new(),
                    tables: Vec::new(),
                },
                "module {
  func f0 () -> () @label0: {}
  func f1 () -> () @label0: {}
}
",
                "module with several empty fuctions"
            ),
            (
                Module {
                    functions: vec![
                        Function {
                            name: Func::Idx(0),
                            type_: FunctionType { params: Vec::new().into_boxed_slice(), results: Vec::new().into_boxed_slice() },
                            body: Body(Vec::new())
                        },
                        Function {
                            name: Func::Idx(1),
                            type_: FunctionType { params: vec![ValType::I32].into_boxed_slice(), results:vec![ValType::F64].into_boxed_slice() },
                            body: Body(vec![
                                Assign {
                                    lhs: Stack(0),
                                    rhs: Const(I32(3)),
                                    type_: ValType::I32,
                                },
                                Assign {
                                    lhs: Stack(1),
                                    rhs: Const(I32(4)),
                                    type_: ValType::I32,
                                },
                            ])
                        },
                    ],
                    globals: Vec::new(),
                    tables: Vec::new(),
                },
                "module {
  func f0 () -> () @label0: {}
  func f1 (p0: i32) -> (r0: f64) @label0: {
    s0: i32 = i32.const 3
    s1: i32 = i32.const 4
  }
}
",
                "module with several empty fuctions"
            ),
        ];

        static ref WIMPL_FUNCTION_SYNTAX_TESTCASES: Vec<(Function, &'static str, &'static str)> = vec![
            (
                Function {
                    name: Func::Idx(0),
                    type_: FunctionType { params: Vec::new().into_boxed_slice(), results: Vec::new().into_boxed_slice() },
                    body: Body(Vec::new())
                },
                "func f0 () -> () @label0: {}",
                "empty function"
            ),
            (
                Function {
                    name: Func::Idx(1),
                    type_: FunctionType { params: vec![ValType::I32].into_boxed_slice(), results:vec![ValType::F64].into_boxed_slice() },
                    body: Body(Vec::new())
                },
                "func f1 (p0: i32) -> (r0: f64) @label0: {}",
                "empty function with types"
            ),
            (
                Function {
                    name: Func::Idx(1),
                    type_: FunctionType { params: vec![ValType::I32].into_boxed_slice(), results:vec![ValType::F64].into_boxed_slice() },
                    body: Body(vec![
                        Assign {
                            lhs: Stack(0),
                            rhs: Const(I32(3)),
                            type_: ValType::I32,
                        }
                    ])
                },
                "func f1 (p0: i32) -> (r0: f64) @label0: { s0: i32 = i32.const 3 }",
                "function with i32.const in body"
            ),
            (
                Function {
                    name: Func::Idx(1),
                    type_: FunctionType { params: vec![ValType::I32].into_boxed_slice(), results:vec![ValType::F64].into_boxed_slice() },
                    body: Body(vec![
                        Assign {
                            lhs: Stack(0),
                            rhs: Const(I32(3)),
                            type_: ValType::I32,
                        },
                        Assign {
                            lhs: Stack(1),
                            rhs: Const(I32(4)),
                            type_: ValType::I32,
                        },
                    ])
                },
                "func f1 (p0: i32) -> (r0: f64) @label0: {
  s0: i32 = i32.const 3
  s1: i32 = i32.const 4
}",
                "function with multiple statements"
            ),
        ];

        /// Pairs of Wimpl AST with concrete syntax, and optionally a comment what is
        /// "special" about this testcase. This is used for testing both parsing and
        /// pretty-printing of Wimpl, just in different directions.
        ///
        /// For these examples, the concrete syntax is in the "canonical pretty"
        /// format, i.e., with "standard" whitespace.
        static ref WIMPL_CANONICAL_SYNTAX_TESTCASES: Vec<(Stmt, &'static str, &'static str)> = vec![
            (Unreachable, "unreachable", ""),
            (
                Assign {
                    lhs: Stack(0),
                    type_: ValType::I32,
                    rhs: MemorySize
                },
                "s0: i32 = memory.size",
                ""
            ),
            (
                Assign {
                    lhs: Global(0),
                    type_: ValType::I32,
                    rhs: VarRef(Local(0))
                },
                "g0: i32 = l0",
                "var ref on rhs"
            ),
            (
                Assign {
                    lhs: Stack(0),
                    rhs: Const(I32(1337)),
                    type_: ValType::I32,
                },
                "s0: i32 = i32.const 1337",
                ""
            ),
            (
                Assign {
                    lhs: Stack(1),
                    rhs: Numeric {
                        op: I32Add,
                        args: vec![Stack(2), Stack(3)],
                    },
                    type_: ValType::I32,
                },
                "s1: i32 = i32.add(s2, s3)",
                ""
            ),
            (
                Assign {
                    lhs: Stack(1),
                    rhs: Load {
                        op: I32Load,
                        memarg: Memarg::default(I32Load),
                        addr: Stack(0),
                    },
                    type_: ValType::I32,
                },
                "s1: i32 = i32.load(s0)",
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
                },
                "br @label0",
                ""
            ),
            (
                Expr(Call {
                    func: Func::Idx(7),
                    args: Vec::new(),
                }),
                "call f7 ()",
                "call argument list is always printed, even if empty"
            ),
            (
                Assign {
                    lhs: Stack(1),
                    rhs: CallIndirect {
                        type_: FunctionType::new(&[ValType::I32], &[ValType::I32]),
                        table_idx: Stack(0),
                        args: vec![Stack(2), Stack(3)],
                    },
                    type_: ValType::I32,
                },
                "s1: i32 = call_indirect [i32] -> [i32] (s0) (s2, s3)",
                ""
            ),
            (
                Block {
                    end_label: Label(0),
                    body: Body (vec![]),
                },
                "@label0: block {}",
                "empty block"
            ),
            (
                Block {
                    end_label: Label(1),
                    body: Body(vec![
                        Assign{
                            lhs: Stack(1),
                            type_: ValType::I32,
                            rhs: VarRef(Stack(0)),
                        }]),
                },
                "@label1: block { s1: i32 = s0 }",
                "block with a single instruction, on one line"
            ),
            (
                Loop {
                    begin_label: Label(0),
                    body: Body (vec![
                        Br { target: Label(0) },
                        Unreachable
                    ]),
                },
                r"@label0: loop {
  br @label0
  unreachable
}",
                "loop with multiple instructions, indentation"
            ),
            (
                If {
                        condition: Stack(0),
                        if_body: Body (
                            vec![Br {
                                target: Label(0),
                            }]),
                        else_body: None,
                },
                "if (s0) { br @label0 }",
                "if + br (which is our form of br_if)"
            ),
            (
                Switch {
                        index: Stack(0),
                        cases: vec![
                            Body(vec![Unreachable]),
                            Body(vec![Expr(MemorySize), Br { target: Label(1) }]),
                        ],
                        default: Body(vec![]),
                },
                r"switch (s0) {
  case 0: { unreachable }
  case 1: {
    memory.size
    br @label1
  }
  default: {}
}",
                "switch with multiple cases, some on a single line, others not"
            ),
            (
                Loop {
                    begin_label: Label(1),
                    body: Body(vec![
                        Block {
                            end_label: Label(2),
                            body: Body(vec![
                                Assign{
                                    lhs: Stack(0),
                                    type_: ValType::F64,
                                    rhs: VarRef(Stack(1))
                                },
                                Unreachable,
                            ])
                        },
                        Br { target: Label(1) },
                    ])
                },
    r"@label1: loop {
  @label2: block {
    s0: f64 = s1
    unreachable
  }
  br @label1
}",
            "nested and multi-line loop/block")
        ];

        /// The following examples are NOT in the canonical text format, e.g.,
        /// because they contain too little or too much whitespace.
        /// They are only used for testing parsing, not pretty-printing.
        static ref WIMPL_ALTERNATIVE_SYNTAX_TESTCASES: Vec<(Stmt, &'static str, &'static str)> = vec![
            (
                Assign {
                    lhs: Stack(1),
                    type_: ValType::I32,
                    rhs: MemoryGrow { pages: Stack(0) },
                },
                "s1: i32 = memory.grow ( s0 )",
                "extra space around arguments"),
            (
                Expr(Call {
                    func: Func::Idx(2),
                    args: vec![Stack(2), Stack(3)],
                }),
                "call f2 ( s2, s3 )",
                "extra space around call arguments"
            ),
            (
                Assign{
                    lhs: Stack(1),
                    rhs: CallIndirect {
                        type_: FunctionType::new(&[ValType::I32], &[ValType::I32]),
                        table_idx: Stack(0),
                        args: vec![],
                    },
                    type_: ValType::I32,
                },
                "s1: i32 = call_indirect [  i32  ] ->[i32] (s0) ()",
                "non-standard spacing around function type"
            ),
            (
                Assign{
                    lhs: Stack(1),
                    rhs: Numeric {
                        op: I32Add,
                        args: vec![Stack(2), Stack(3)],
                    },
                    type_: ValType::I32,
                },
                "s1: i32 = i32.add (s2,s3)",
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
                    end_label: Label(0),
                    body: Body(vec![]),
                },
                "@label0:block{}",
                "minimal space in block"
            ),
            (
                Block {
                        end_label: Label(2),
                        body: Body(vec![
                            Assign {
                                lhs: Stack(1),
                                rhs: VarRef(Stack(0)),
                                type_: ValType::I32,
                            },
                            Expr(VarRef(Stack(1)))
                        ]),
                },
                "@label2: block { s1: i32 = s0 s1 }",
                "weird but valid parse: expression statement with only a variable reference"
            )
        ];
    }

    #[test]
    fn pretty_print_stmt() {
        for (i, (wimpl, text, msg)) in WIMPL_CANONICAL_SYNTAX_TESTCASES.iter().enumerate() {
            assert_eq!(&wimpl.to_string(), text, "\ntest #{}\n{}", i, msg);
        }
    }

    #[test]
    fn pretty_print_function() {
        for (i, (wimpl, text, msg)) in WIMPL_FUNCTION_SYNTAX_TESTCASES.iter().enumerate() {
            assert_eq!(&wimpl.to_string(), text, "\ntest #{}\n{}", i, msg);
        }
    }

    #[test]
    fn pretty_print_module() {
        for (i, (wimpl, text, msg)) in WIMPL_MODULE_SYNTAX_TESTCASES.iter().enumerate() {
            assert_eq!(&wimpl.to_string(), text, "\ntest #{}\n{}", i, msg);
        }
    }

    #[test]
    fn parse_var() {
        assert_eq!(Ok(Stack(0)), "s0".parse());
        assert_eq!(Ok(Global(42)), "g42".parse());
        assert_eq!(Ok(BlockResult(1)), "b1".parse());
        assert_eq!(Ok(Param(2)), "p2".parse());
        assert_eq!(Ok(Return(0)), "r0".parse());

        // Negative tests:
        assert!(
            "".parse::<Var>().is_err(),
            "empty is not allowed"
        );
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
    fn parse_func_id() {
        assert_eq!(Ok(Func::Idx(13)), "f13".parse());
        assert_eq!(Ok(Func::Named("bla".to_string())), "bla".parse());

        // Negative tests:
        assert!(
            "".parse::<Var>().is_err(),
            "empty is not allowed"
        );
        assert!(
            "123\n ".parse::<Func>().is_err(),
            "only number for function name is not allowed"
        );
    }

    #[test]
    fn parse_expr() {
        assert_eq!(Ok(MemorySize), "memory.size".parse());
        assert_eq!(Ok(MemoryGrow { pages: Local(0) }), "memory.grow (l0)".parse());
        assert_eq!(Ok(VarRef(Global(1))), "g1".parse());
        assert_eq!(Ok(Numeric {
            op: I32Add,
            args: vec![Stack(0), Local(1)]
        }), "i32.add(s0, l1)".parse());
        // More complex expressions are tested in the statements.
    }

    #[test]
    fn parse_stmt() {
        let parse_testcases = WIMPL_CANONICAL_SYNTAX_TESTCASES
            .iter()
            .chain(WIMPL_ALTERNATIVE_SYNTAX_TESTCASES.iter());
        for (i, (wimpl, text, msg)) in parse_testcases.enumerate() {
            let parsed = Stmt::from_str(text);
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
            let parsed = Stmt::from_str(text).unwrap();
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
            let parsed = Stmt::from_str(text).unwrap();
            let pretty = parsed.to_string();
            let parsed_pretty = Stmt::from_str(&pretty).unwrap();
            assert_eq!(parsed, parsed_pretty, "\ntest #{}\n{}", i, msg);
        }
    }

    #[test]
    fn parse_file() {
        let instrs = Stmt::from_text_file("tests/wimpl/syntax.wimpl");
        assert!(instrs.is_ok());
    }

    #[test]
    fn macros() {
        let _ = wimpl!(g0: f32 = f32.const 1.1);
        let _ = wimpl!(s2: i32 = i32.add (s0, s1));
        let _ = wimpl!(call_indirect [ ] ->[] (s1) ());

        // Tricky cases, because rustc lexes these tokens differently than we need to.
        let _ = wimpl!(s3: i32 = i32.load offset=3 (s0));
        let _ = wimpl!(@label0: block {});

        // Multiple instructions:
        let _ = wimpls! {};
        let _ = wimpls! {
            @label2: loop {
                s5: i32 = i32.const 3
                br @label2
            }
            l0: i64 = g0
        };
    }
}

#[cfg(test)]
fn test(path_wimpl: &str, path_wasm: &str) {
    let wimpl_module = wimplify(path_wasm).unwrap();
    let actual = wimpl_module.functions[0].clone().body;
    let expected = Body(Stmt::from_text_file(path_wimpl).unwrap());
    assert_eq!(actual, expected, "\nACTUAL: {}\nEXPECTED: {}\n", actual, expected);
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
fn select_diff_types() {
    test("tests/wimpl/select_diff_types/select.wimpl", "tests/wimpl/select_diff_types/select.wasm");
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
fn if_ret() {
    test("tests/wimpl/if_ret/if.wimpl", "tests/wimpl/if_ret/if.wasm");
}

#[test]
fn if_else() {
    test("tests/wimpl/if_else/if_else.wimpl", "tests/wimpl/if_else/if_else.wasm");
}

//hand written calculator programs

#[test]
fn calc() {
    wimplify("tests/wimpl-wasm-handwritten/calc/add.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn calc_dce() {
    wimplify("tests/wimpl-wasm-handwritten/calc-dce/add-dce.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn calc_virtual() {
    wimplify("tests/wimpl-wasm-handwritten/calc-virtual/add.wasm").expect("error while translating wasm file to wimpl");

}

//USENIX programs

#[test]
fn module_8c087e0290bb39f1e090() {
    wimplify("tests/wimpl-USENIX/8c087e0290bb39f1e090.module/8c087e0290bb39f1e090.module.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn annots() {
    wimplify("tests/wimpl-USENIX/annots/annots.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn module_bb9bb638551198cd3a42() {
    wimplify("tests/wimpl-USENIX/bb9bb638551198cd3a42.module/bb9bb638551198cd3a42.module.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn compiled_wasm() {
    wimplify("tests/wimpl-USENIX/compiled.wasm/compiled.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn module_dac34eee5ed4216c65b2() {
    wimplify("tests/wimpl-USENIX/dac34eee5ed4216c65b2.module/dac34eee5ed4216c65b2.module.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn imagequant_c970f() {
    wimplify("tests/wimpl-USENIX/imagequant.c970f/imagequant.c970f.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn mozjpeg_enc_93395() {
    wimplify("tests/wimpl-USENIX/mozjpeg_enc.93395/mozjpeg_enc.93395.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn optipng_4e77b() {
    wimplify("tests/wimpl-USENIX/optipng.4e77b/optipng.4e77b.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn rotate_4cdaa() {
    wimplify("tests/wimpl-USENIX/rotate.4cdaa/rotate.4cdaa.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn usenix_bin_acrobat_wasm() {
    wimplify("tests/wimpl-USENIX/USENIX_bin_acrobat.wasm/USENIX_bin_acrobat.wasm.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn webp_dec_fa0ab() {
    wimplify("tests/wimpl-USENIX/webp_dec.fa0ab/webp_dec.fa0ab.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn webp_enc_ea665() {
    wimplify("tests/wimpl-USENIX/webp_enc.ea665/webp_enc.ea665.wasm").expect("error while translating wasm file to wimpl");

}

// filtered wasm binaries

#[test]
fn _07735b34f092d6e63c397dfb583b64ceca84c595d13c6912f8b0d414b0f01da9() {
    wimplify("tests/wimpl-filtered-binaries/07735b34f092d6e63c397dfb583b64ceca84c595d13c6912f8b0d414b0f01da9/07735b34f092d6e63c397dfb583b64ceca84c595d13c6912f8b0d414b0f01da9.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _14ee85873e07b6226d416a1fc3bfc2aeae9c44700eac316a04bb6d98b95b605c() {
    wimplify("tests/wimpl-filtered-binaries/14ee85873e07b6226d416a1fc3bfc2aeae9c44700eac316a04bb6d98b95b605c/14ee85873e07b6226d416a1fc3bfc2aeae9c44700eac316a04bb6d98b95b605c.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _1cbe05896a7233e4a5679d69b4fb4e04b3857b50b1a0fc0d1a34054ff40e39bb() {
    wimplify("tests/wimpl-filtered-binaries/1cbe05896a7233e4a5679d69b4fb4e04b3857b50b1a0fc0d1a34054ff40e39bb/1cbe05896a7233e4a5679d69b4fb4e04b3857b50b1a0fc0d1a34054ff40e39bb.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _1e9df781a23d49aaf1e10abc4db11cde4c31e07d9cee2568d723b27bbeff515b() {
    wimplify("tests/wimpl-filtered-binaries/1e9df781a23d49aaf1e10abc4db11cde4c31e07d9cee2568d723b27bbeff515b/1e9df781a23d49aaf1e10abc4db11cde4c31e07d9cee2568d723b27bbeff515b.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _2449e3cbacf8bc6fd02a09b5b2a0f7ad4555046f7afba480d29f4929d39e4b04() {
    wimplify("tests/wimpl-filtered-binaries/2449e3cbacf8bc6fd02a09b5b2a0f7ad4555046f7afba480d29f4929d39e4b04/2449e3cbacf8bc6fd02a09b5b2a0f7ad4555046f7afba480d29f4929d39e4b04.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _32e1e40f2bc99176f2ede6999af1681b9893fca63db2f87ef2d274cd43f1d3e3() {
    wimplify("tests/wimpl-filtered-binaries/32e1e40f2bc99176f2ede6999af1681b9893fca63db2f87ef2d274cd43f1d3e3/32e1e40f2bc99176f2ede6999af1681b9893fca63db2f87ef2d274cd43f1d3e3.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _381e5189553901c1649b5093758fee36b338ee7fcd211a20b1b6e6d374e53bce() {
    wimplify("tests/wimpl-filtered-binaries/381e5189553901c1649b5093758fee36b338ee7fcd211a20b1b6e6d374e53bce/381e5189553901c1649b5093758fee36b338ee7fcd211a20b1b6e6d374e53bce.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _3deb83bb20eccb638a2fbbe09ba323a472654552f8bcd93611e1d4ba20a67ea4() {
    wimplify("tests/wimpl-filtered-binaries/3deb83bb20eccb638a2fbbe09ba323a472654552f8bcd93611e1d4ba20a67ea4/3deb83bb20eccb638a2fbbe09ba323a472654552f8bcd93611e1d4ba20a67ea4.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _3f8cf6588c2ed1e7f92ef8f3c37cbb0a0294a9b2a0b330ecd087cff985b34689() {
    wimplify("tests/wimpl-filtered-binaries/3f8cf6588c2ed1e7f92ef8f3c37cbb0a0294a9b2a0b330ecd087cff985b34689/3f8cf6588c2ed1e7f92ef8f3c37cbb0a0294a9b2a0b330ecd087cff985b34689.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _419963e6d5166128b11ce6eb7138fe6b5c81694196882cba034f296d613d9d0f() {
    wimplify("tests/wimpl-filtered-binaries/419963e6d5166128b11ce6eb7138fe6b5c81694196882cba034f296d613d9d0f/419963e6d5166128b11ce6eb7138fe6b5c81694196882cba034f296d613d9d0f.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _4666a4a9c39036d84f70ebb3e2cb476cff2549377ced946618b293fc6552aae8() {
    wimplify("tests/wimpl-filtered-binaries/4666a4a9c39036d84f70ebb3e2cb476cff2549377ced946618b293fc6552aae8/4666a4a9c39036d84f70ebb3e2cb476cff2549377ced946618b293fc6552aae8.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _4696f2d4f93b20b80cb53d0ead51ebacefd3f407654878578374133e630a1fff() {
    wimplify("tests/wimpl-filtered-binaries/4696f2d4f93b20b80cb53d0ead51ebacefd3f407654878578374133e630a1fff/4696f2d4f93b20b80cb53d0ead51ebacefd3f407654878578374133e630a1fff.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _4ca2a66a0c64388ded652fc19aab816513782fcd86d57862abc35a0d25861314() {
    wimplify("tests/wimpl-filtered-binaries/4ca2a66a0c64388ded652fc19aab816513782fcd86d57862abc35a0d25861314/4ca2a66a0c64388ded652fc19aab816513782fcd86d57862abc35a0d25861314.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _524b1b048e588dc5a207e342503b454641f32ecafcb4d48d7d526bdeea6f5e98() {
    wimplify("tests/wimpl-filtered-binaries/524b1b048e588dc5a207e342503b454641f32ecafcb4d48d7d526bdeea6f5e98/524b1b048e588dc5a207e342503b454641f32ecafcb4d48d7d526bdeea6f5e98.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _57dee2a170d275e099b953a8fdafde6f5becea8a0a2202de68254ad74dfd6fd5() {
    wimplify("tests/wimpl-filtered-binaries/57dee2a170d275e099b953a8fdafde6f5becea8a0a2202de68254ad74dfd6fd5/57dee2a170d275e099b953a8fdafde6f5becea8a0a2202de68254ad74dfd6fd5.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _58fd82e10ee3f41aef7088281c2747eb4aa07300b4eefbc566080a074f6e9f3c() {
    wimplify("tests/wimpl-filtered-binaries/58fd82e10ee3f41aef7088281c2747eb4aa07300b4eefbc566080a074f6e9f3c/58fd82e10ee3f41aef7088281c2747eb4aa07300b4eefbc566080a074f6e9f3c.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _616140f09fb1466810df2b2bb70a5d4d692581d5c50985d58b66b23537ece6cb() {
    wimplify("tests/wimpl-filtered-binaries/616140f09fb1466810df2b2bb70a5d4d692581d5c50985d58b66b23537ece6cb/616140f09fb1466810df2b2bb70a5d4d692581d5c50985d58b66b23537ece6cb.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _6a0160516ec0012faf38d1a1e138806fc3085e956498e049276341e67eb63648() {
    wimplify("tests/wimpl-filtered-binaries/6a0160516ec0012faf38d1a1e138806fc3085e956498e049276341e67eb63648/6a0160516ec0012faf38d1a1e138806fc3085e956498e049276341e67eb63648.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _6c2ed8ebb8fe662fe7e0675e45dea2dbcdb387ce8367809390f58a1bcb63f3a8() {
    wimplify("tests/wimpl-filtered-binaries/6c2ed8ebb8fe662fe7e0675e45dea2dbcdb387ce8367809390f58a1bcb63f3a8/6c2ed8ebb8fe662fe7e0675e45dea2dbcdb387ce8367809390f58a1bcb63f3a8.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _7165c282edac43a4731b7aaae47e049de200dd1b5cc3c7710a5b57989927b394() {
    wimplify("tests/wimpl-filtered-binaries/7165c282edac43a4731b7aaae47e049de200dd1b5cc3c7710a5b57989927b394/7165c282edac43a4731b7aaae47e049de200dd1b5cc3c7710a5b57989927b394.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _77b3e5c5903371f52b3a829f889349d4ccee4f82fa2791325e5e4aea89efd793() {
    wimplify("tests/wimpl-filtered-binaries/77b3e5c5903371f52b3a829f889349d4ccee4f82fa2791325e5e4aea89efd793/77b3e5c5903371f52b3a829f889349d4ccee4f82fa2791325e5e4aea89efd793.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _787959bde2695ac32a0ee4bb92350f6568139c75f120012770fb3de012919736() {
    wimplify("tests/wimpl-filtered-binaries/787959bde2695ac32a0ee4bb92350f6568139c75f120012770fb3de012919736/787959bde2695ac32a0ee4bb92350f6568139c75f120012770fb3de012919736.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _835d0731f9ae86c0147196aeed12f6967e3886edd4d0b85637bb37a2f02b4875() {
    wimplify("tests/wimpl-filtered-binaries/835d0731f9ae86c0147196aeed12f6967e3886edd4d0b85637bb37a2f02b4875/835d0731f9ae86c0147196aeed12f6967e3886edd4d0b85637bb37a2f02b4875.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _88c0ee6c82e21d686b0ca9ab15c7fa6c551bd49dcfb6493d59a845ccd9cfc88e() {
    wimplify("tests/wimpl-filtered-binaries/88c0ee6c82e21d686b0ca9ab15c7fa6c551bd49dcfb6493d59a845ccd9cfc88e/88c0ee6c82e21d686b0ca9ab15c7fa6c551bd49dcfb6493d59a845ccd9cfc88e.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _8a5f2590830612d57d229c543645e64db599c1b2ea975b78a86c9ac5d6e5d88a() {
    wimplify("tests/wimpl-filtered-binaries/8a5f2590830612d57d229c543645e64db599c1b2ea975b78a86c9ac5d6e5d88a/8a5f2590830612d57d229c543645e64db599c1b2ea975b78a86c9ac5d6e5d88a.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _8ae683329370c6a0d5e5cd533ea34ae6fe17433ad0106524397448539dc0a14f() {
    wimplify("tests/wimpl-filtered-binaries/8ae683329370c6a0d5e5cd533ea34ae6fe17433ad0106524397448539dc0a14f/8ae683329370c6a0d5e5cd533ea34ae6fe17433ad0106524397448539dc0a14f.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _8ebf4e44c47b6b61d313bd2580bd788a1daa029541fe210cccfa13d1bb66cc89() {
    wimplify("tests/wimpl-filtered-binaries/8ebf4e44c47b6b61d313bd2580bd788a1daa029541fe210cccfa13d1bb66cc89/8ebf4e44c47b6b61d313bd2580bd788a1daa029541fe210cccfa13d1bb66cc89.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _9073aea62a25c574c19a69ed7232d6abf666cccb190e485a9860ce8fa244bd5b() {
    wimplify("tests/wimpl-filtered-binaries/9073aea62a25c574c19a69ed7232d6abf666cccb190e485a9860ce8fa244bd5b/9073aea62a25c574c19a69ed7232d6abf666cccb190e485a9860ce8fa244bd5b.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _9110face5f3ebd6d321619a8c5378c64e1ad159b0d4ce7fc31ee7b4702e013d8() {
    wimplify("tests/wimpl-filtered-binaries/9110face5f3ebd6d321619a8c5378c64e1ad159b0d4ce7fc31ee7b4702e013d8/9110face5f3ebd6d321619a8c5378c64e1ad159b0d4ce7fc31ee7b4702e013d8.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _921b6ab9805103b1bdca68f0e705cb80b499a24fce4e74943fd9e0b36ea0910c() {
    wimplify("tests/wimpl-filtered-binaries/921b6ab9805103b1bdca68f0e705cb80b499a24fce4e74943fd9e0b36ea0910c/921b6ab9805103b1bdca68f0e705cb80b499a24fce4e74943fd9e0b36ea0910c.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _92d3be911b9e7a6a5293c6ccedfcb734f5a35adba6ea7b9ced1a810e9716092f() {
    wimplify("tests/wimpl-filtered-binaries/92d3be911b9e7a6a5293c6ccedfcb734f5a35adba6ea7b9ced1a810e9716092f/92d3be911b9e7a6a5293c6ccedfcb734f5a35adba6ea7b9ced1a810e9716092f.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _9813df4ed1e42ea1cd0ec3b43d51a0beb80518980eb269c5bb35062710d4edee() {
    wimplify("tests/wimpl-filtered-binaries/9813df4ed1e42ea1cd0ec3b43d51a0beb80518980eb269c5bb35062710d4edee/9813df4ed1e42ea1cd0ec3b43d51a0beb80518980eb269c5bb35062710d4edee.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13() {
    wimplify("tests/wimpl-filtered-binaries/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _a8f75a78f2ef9f331c1d7e1327d90ea0c3a198e09d792c9bd7e0ca43d362f725() {
    wimplify("tests/wimpl-filtered-binaries/a8f75a78f2ef9f331c1d7e1327d90ea0c3a198e09d792c9bd7e0ca43d362f725/a8f75a78f2ef9f331c1d7e1327d90ea0c3a198e09d792c9bd7e0ca43d362f725.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _b6736dcdf2ff1eae4b54839fc3c25cef63ea4f3900acfed203c0bf692a771d60() {
    wimplify("tests/wimpl-filtered-binaries/b6736dcdf2ff1eae4b54839fc3c25cef63ea4f3900acfed203c0bf692a771d60/b6736dcdf2ff1eae4b54839fc3c25cef63ea4f3900acfed203c0bf692a771d60.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _bc3b3bf954993dc4914c3e924f4587b260680e8249d5f44f6be2eecae94082da() {
    wimplify("tests/wimpl-filtered-binaries/bc3b3bf954993dc4914c3e924f4587b260680e8249d5f44f6be2eecae94082da/bc3b3bf954993dc4914c3e924f4587b260680e8249d5f44f6be2eecae94082da.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _c0d83afa613ef8df9dd24c3b8737c8bdde493524b8ec0f7c5a928b7fe765fa73() {
    wimplify("tests/wimpl-filtered-binaries/c0d83afa613ef8df9dd24c3b8737c8bdde493524b8ec0f7c5a928b7fe765fa73/c0d83afa613ef8df9dd24c3b8737c8bdde493524b8ec0f7c5a928b7fe765fa73.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _c46e332b470498643fadd1d598b0285bd44c1f60204321ad4c01a5a3a5e22338() {
    wimplify("tests/wimpl-filtered-binaries/c46e332b470498643fadd1d598b0285bd44c1f60204321ad4c01a5a3a5e22338/c46e332b470498643fadd1d598b0285bd44c1f60204321ad4c01a5a3a5e22338.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _d25b99a719fef7cc681c79e8a77e17e87e6f7a3c423032f9b962f62c003dc38d() {
    wimplify("tests/wimpl-filtered-binaries/d25b99a719fef7cc681c79e8a77e17e87e6f7a3c423032f9b962f62c003dc38d/d25b99a719fef7cc681c79e8a77e17e87e6f7a3c423032f9b962f62c003dc38d.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _d49d001de11a69755e3b9014bcdc88c1ba77eeafc8b4e8db0f92fe31e8e6aee2() {
    wimplify("tests/wimpl-filtered-binaries/d49d001de11a69755e3b9014bcdc88c1ba77eeafc8b4e8db0f92fe31e8e6aee2/d49d001de11a69755e3b9014bcdc88c1ba77eeafc8b4e8db0f92fe31e8e6aee2.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _d70070b5a582d75eaa0d5896e56ca67a5399ab43c9e830791f1e2e8334404c90() {
    wimplify("tests/wimpl-filtered-binaries/d70070b5a582d75eaa0d5896e56ca67a5399ab43c9e830791f1e2e8334404c90/d70070b5a582d75eaa0d5896e56ca67a5399ab43c9e830791f1e2e8334404c90.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _e6b183e40f2671dc0f87e6d85070077e9bea18e7ad50cd7e7cd31e2a9ade937b() {
    wimplify("tests/wimpl-filtered-binaries/e6b183e40f2671dc0f87e6d85070077e9bea18e7ad50cd7e7cd31e2a9ade937b/e6b183e40f2671dc0f87e6d85070077e9bea18e7ad50cd7e7cd31e2a9ade937b.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _ec2393766f8f2a21803b4a062f1d71c184f6eae0be60f13703e3a43e5fa493b0() {
    wimplify("tests/wimpl-filtered-binaries/ec2393766f8f2a21803b4a062f1d71c184f6eae0be60f13703e3a43e5fa493b0/ec2393766f8f2a21803b4a062f1d71c184f6eae0be60f13703e3a43e5fa493b0.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _f2943fa8ae6133fb532fa9b036ba81342828b2e1e78b3ae47731619edcbca4dd() {
    wimplify("tests/wimpl-filtered-binaries/f2943fa8ae6133fb532fa9b036ba81342828b2e1e78b3ae47731619edcbca4dd/f2943fa8ae6133fb532fa9b036ba81342828b2e1e78b3ae47731619edcbca4dd.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _f4cd145be9df9b4b35e2ba3a95c6a6f0f5f8284a03de77c0e627a78d97fdaea2() {
    wimplify("tests/wimpl-filtered-binaries/f4cd145be9df9b4b35e2ba3a95c6a6f0f5f8284a03de77c0e627a78d97fdaea2/f4cd145be9df9b4b35e2ba3a95c6a6f0f5f8284a03de77c0e627a78d97fdaea2.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _fc762c3b4338c7d7a6bb31d478cfbe5717ebefb0e91d6d27b82a21fc169c7afe() {
    wimplify("tests/wimpl-filtered-binaries/fc762c3b4338c7d7a6bb31d478cfbe5717ebefb0e91d6d27b82a21fc169c7afe/fc762c3b4338c7d7a6bb31d478cfbe5717ebefb0e91d6d27b82a21fc169c7afe.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _fd6372aef6ff7d9ecffcc7f3d8d00963bebf39d68451c5ef36c039616ccbded3() {
    wimplify("tests/wimpl-filtered-binaries/fd6372aef6ff7d9ecffcc7f3d8d00963bebf39d68451c5ef36c039616ccbded3/fd6372aef6ff7d9ecffcc7f3d8d00963bebf39d68451c5ef36c039616ccbded3.wasm").expect("error while translating wasm file to wimpl");
}
