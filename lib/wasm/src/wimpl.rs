use std::{
    collections::VecDeque,
    fmt::{self, Write},
    io::{self, ErrorKind},
    iter::FromIterator,
    path::Path,
    str::FromStr, slice::Iter,
};

use crate::{highlevel::{MemoryOp, Global, Table}, types::{InferredInstructionType, TypeChecker}, Val, ValType, Idx};
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
    pub fn function(&self, idx: Idx<highlevel::Function>) -> &Function {
        &self.functions[idx.into_inner()]
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Function {
    /// Either the name of a function (from debug info originally), or a 
    /// numerical index.
    pub name: Func,
    pub type_: FunctionType,
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
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
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

// FIXME: add test for this
impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "module {{").expect("");
        for func in &self.functions {
            writeln!(f, "{}", indent_once(func))?;
        }
        writeln!(f, "}}")
    }
}

// FIXME: add test for this
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
    // TODO move to impl Body FromStr?
    pub fn from_str_multiple(input: &str) -> Result<Vec<Self>, ParseError> {
        parser::adapt_nom_parser(parser::stmts_ws, input)
    }

    /// Convenience function to parse Wimpl from a filename.
    // FIXME wimpl file should be a module, no?
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

// Export macros.
// pub(crate) use wimpl;
// pub(crate) use wimpls;

#[derive(Clone, Default)]
pub struct State {
    pub label_count: usize,
    pub stack_var_count: usize,
    pub var_stack: Vec<Var>,
    // TODO: change usize to Label
    // if you are in a block or an if => false 
    // if you are in a loop => true 
    pub label_stack: Vec<(usize, Option<(Var, ValType, bool)>)>,  //TODO: label stack needs to flag if block or loop 
    pub else_taken: bool, 
    pub num_params: usize, 
}

impl State {
    fn new (param_len: usize) -> Self {
        State {
            label_count: 1,
            stack_var_count: 0,
            var_stack: Vec::new(),
            label_stack: Vec::new(),
            else_taken: false,
            num_params: param_len,
        }
    }
}



fn wimplify_instrs(
    instrs: &mut VecDeque<&highlevel::Instr>,
    tys: &mut VecDeque<InferredInstructionType>,
    state: &mut State,
) -> Result<Vec<Stmt>, String> {
    
    use Expr::*; 
    use Var::*; 
     
    let mut result_instrs = Vec::new();

    while instrs.len() > 0 {

        let instr = instrs.pop_front().expect("instruction list not expected to be empty");
        let ty = tys.pop_front().expect("type list not expected to be empty");
        
        let ty = match ty {
            InferredInstructionType::Unreachable => {
                match instr {
                    highlevel::Instr::End => return Ok(result_instrs),
                    highlevel::Instr::Else => {
                        state.else_taken = true; 
                        return Ok(result_instrs)
                    } 
                    _ => continue  //return Ok(Vec::new())
                }                
            },
            InferredInstructionType::Reachable(ty) => ty,
        };
        
        println!("{}, {}, {:?}", instr, ty, state.var_stack);

        let n_inputs = ty.params.len();
        let n_results = ty.results.len();
    
        let lhs = if n_results == 0 {
            None
        } else if n_results == 1 {
            match instr {
                // tee lhs is the argument itself, ie, rhs which is handled in the match arm for it below
                highlevel::Instr::Local(highlevel::LocalOp::Tee,_) => None, 
                // end does not produce a value, it indicates that you have to save var_stack top into the return variable 
                highlevel::Instr::End => None,
                // brif does not produce a value, the returned value is stored in the label's return variable 
                highlevel::Instr::BrIf(_) => None,  
                _ => Some(Stack(state.stack_var_count)) 
            }
        } else {
            panic!("cannot return more than one value in wasm1.0")
        };

        let lhs_ty = lhs.map(|_|ty.results[0]);
        
        let resolve_local_idx_to_var = |local_idx| {
            if local_idx < state.num_params {
                Param(local_idx)
            } else {
                Local(local_idx - state.num_params)
            }
        };

        let label_to_instrs = |label_stack: &[_], label: crate::Label, get_value: &mut dyn FnMut() -> Var| -> Vec<Stmt> {
            let (target, return_info) = *label_stack.iter().rev().nth(label.into_inner()).expect("label stack should never be empty"); 
            let target = Label(target); 
            
            match return_info {
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

        let result_instr: Vec<Stmt> = match instr {

            highlevel::Instr::Unreachable => vec![Stmt::Unreachable],

            highlevel::Instr::Nop => Vec::new(),

            highlevel::Instr::Block(blocktype) |
            highlevel::Instr::Loop(blocktype) => {
                
                let btype = blocktype.0; 

                //first, if the block returns a value, create a variable that will store the return and keep track of the variable 
                //if it doesn't return anything, create an empty vector  
                //block variable has same usize as the labelnumber!
                let mut result_var = None; 
                let mut res_instr_vec = if let Some(btype) = btype {
                    result_var = Some(BlockResult(state.label_count)); 
                    state.stack_var_count += 1; 
                    vec![Stmt::Assign { 
                        lhs: result_var.expect("result variable expected since block is producing a value"), 
                        rhs: Const(Val::get_default_value(btype)) ,  
                        type_: btype,  
                    }]
                } else {
                    Vec::new()
                }; 
                
                //save current stack state and prepare to go into a new block 
                let curr_label_count = state.label_count;
                state.label_count += 1;
                
                //create new block state 
                // TODO split into three parts: one part mutable (label_count, stack_var_count), one
                // part not mutable ("global constants": num_params), and "local state" var_stack, 
                // label_stack, else_taken).
                let mut block_state = State {
                    label_count: state.label_count, // TODO: mut ref and shared so not cloned 
                    stack_var_count: state.stack_var_count,
                    var_stack: Vec::new(),  // TODO: -> initialize before while - local var of the function 
                    label_stack: state.label_stack.clone(), //parameter to the function used to initalize a local version of it - not a mutable reference 
                    else_taken: false, // TODO: part of the return value of wimplify_instrs
                    num_params: state.num_params, // TODO: pulled out of state, not mutable -> functtion parameter thats not mutable 
                }; 
                if let Some(result_var) = result_var {
                    let loop_flag = match instr {
                        highlevel::Instr::Loop(_) => true, 
                        highlevel::Instr::Block(_) => false, 
                        _ => unreachable!("should only be translating block or loop")                    
                    };
                    let block_result = Some((result_var, btype.expect("block type is expected since the block has a result variable"), loop_flag));
                    block_state.label_stack.push((curr_label_count, block_result))                
                } else {
                    block_state.label_stack.push((curr_label_count, None)); 
                }

                //call wimplify on remaining instructions with new block state 
                let block_body = wimplify_instrs(instrs, tys, &mut block_state).expect("a non-empty instruction list in wasm cannot produce an empty list of instructions for wimpl");
                
                //update block state
                state.label_count = block_state.label_count;
                state.stack_var_count = block_state.stack_var_count; 
                if btype.is_some() {
                    state.var_stack.push(result_var.expect("block is producing a result but no associated result variable found"));     
                }

                res_instr_vec.push(match instr {
                    highlevel::Instr::Block(_) => Stmt::Block{
                        end_label: Label(curr_label_count),
                        body: Body(block_body),
                    },
                    highlevel::Instr::Loop(_) => Stmt::Loop{
                        begin_label: Label(curr_label_count),
                        body: Body(block_body),
                    },
                    _ => panic!("should not execute any instruction that is not block or loop at this point")
                }); 

                res_instr_vec 
            }
            
            highlevel::Instr::If(blocktype) => {
                
                let blocktype = blocktype.0; 

                let mut result_var = None; 
                let mut res_vec = if let Some(btype) = blocktype {
                    result_var = Some(BlockResult(state.label_count)); 
                    state.stack_var_count += 1;
                    vec![Stmt::Assign { 
                        lhs: result_var.expect("if produces a result but associated result variable not found"), 
                        rhs: Const(Val::get_default_value(btype)) , 
                        type_: btype,  
                    }]
                } else {
                    Vec::new()
                }; 
                
                let curr_label_count = state.label_count;
                state.label_count += 1;
                
                let mut if_state = State {
                    label_count: state.label_count,
                    stack_var_count: state.stack_var_count,
                    var_stack: Vec::new(),
                    label_stack: state.label_stack.clone(),
                    else_taken: false,
                    num_params: state.num_params, 
                }; 
                // TODO: maybe this is better?
                // state.clone();
                // if_state.else_take = false;
                // if_state.var_stack = Vec::new(); 

                if let Some(result_var) = result_var {
                    if_state.label_stack.push((curr_label_count, Some((result_var, blocktype.expect("block type expected since there is an associated result variable for the block"), false))))                
                } else {
                    if_state.label_stack.push((curr_label_count, None)); 
                }
                
                let if_body = wimplify_instrs(instrs, tys, &mut if_state).expect("if_body in wasm should not return an empty list of instructions for wimpl");            

                let else_body = if if_state.else_taken {
                    if_state.else_taken = false; 
                    Some(Body(wimplify_instrs(instrs, tys, &mut if_state).expect("else_body in wasm should not return an empty list of wimpl instructions")))       
                } else {
                    None
                }; 

                state.label_count = if_state.label_count;
                state.stack_var_count = if_state.stack_var_count; 
                if let Some(_btype) = blocktype {
                    state.var_stack.push(result_var.expect("block produces a result but associated result variable not found")); 
                }

                res_vec.push(Stmt::Block {
                    end_label: Label(curr_label_count),
                    body: Body(vec![Stmt::If{
                        condition: state.var_stack.pop().expect("if expects a condition which was not found on the stack"), 
                        if_body: Body(if_body),
                        else_body,
                    }])
                }); 

                res_vec            
            },

            highlevel::Instr::Else => {
                
                state.else_taken = true; 
                
                // cannot pop because you still want it to be on the label stack while processing the else body 
                //let (_, return_info) = *state.label_stack.last().expect("label stack should never be empty");
                let (_, return_info) = state.label_stack[state.label_stack.len()-1]; 
                

                // assign of the if statement that we just finished processing 
                // we use state.var_stack.pop() and not args.pop() here because, else will never produce a value 
                // or consume it, but here we have to create the assign of the if-block if needed   
                // hence, args will be [] and the required value should be at the top of the var_stack
                if let Some((lhs, type_, loop_flag)) = return_info {
                    assert!(!loop_flag, "if block result should never be have loop_flag set");
                    result_instrs.push(Stmt::Assign{
                        lhs, 
                        rhs: VarRef(state.var_stack.pop().expect("if block is producing a value which is expected on the stack")),  
                        type_, 
                    })
                }

                return Ok(result_instrs); 
            },

            highlevel::Instr::End => {
                 
                //println!("label stack: {:?}", state.label_stack);
                let (_, return_info) = state.label_stack.pop().expect("end of a block expects the matching label to be in the label stack"); 
                //why not args here: else type does not produce a value, we rely on the label stack for that information 
                //println!("return info: {:?}", return_info);  
                if let Some((ret_var, type_, _loop_block)) = return_info {
                    result_instrs.push(Stmt::Assign{
                        lhs: ret_var,
                        rhs: VarRef(state.var_stack.pop().expect("the block is producing a value for which it expect a value on the stack")), 
                        type_,
                    });
                }; 
                //println!("result instrs: {:?}", result_instrs); 
                return Ok(result_instrs)
            }

            highlevel::Instr::Br(label) => {
                let var_stack = &mut state.var_stack;
                label_to_instrs(&state.label_stack, *label, &mut || var_stack.pop().expect("br expected a value to return"))
            },

            highlevel::Instr::BrIf(label) => {
                let condition = state.var_stack.pop().expect("if requires a conditional statement"); 
                vec![Stmt::If{
                    condition, 
                    if_body: Body(label_to_instrs(&state.label_stack, *label, &mut || {
                        // TODO fixup var_stack
                        *state.var_stack.last().expect("br_if expected value to return")
                    })), 
                    else_body: None,
                }]
            }

            highlevel::Instr::BrTable { table, default } => { 
                // Index used for br_table.
                let idx = state.var_stack.pop().expect("br_table requires an index into the table to be supplied"); 
                
                let mut should_pop = false;
                let get_result_val = &mut || {
                    should_pop = true;
                    *state.var_stack.last().expect("br_table expected value to return")
                };
 
                let default = Body(label_to_instrs(&state.label_stack, *default, get_result_val));
                let cases = table.iter().copied().map(|label| Body(label_to_instrs(&state.label_stack, label, get_result_val))).collect(); 

                if should_pop {
                    state.var_stack.pop().expect("last succeeded, so pop should as well");
                }
                
                vec![Stmt::Switch {
                    index: idx,
                    cases,
                    default,
                }]
            }

            highlevel::Instr::Return => {
                let target = Label(0);
                if let (_, Some((return_var, type_, loop_flag))) = state.label_stack.pop().expect("empty label stack, but expected function ") {
                    let return_val = state.var_stack.pop().expect("return expects a return value");
                    vec![
                        Stmt::Assign{ 
                            lhs: return_var, 
                            type_,
                            rhs: VarRef(return_val)
                        }, 
                        Stmt::Br{ target }
                    ]
                } else {
                    vec![Stmt::Br{ target }]
                }
            }

            highlevel::Instr::Call(idx) => {
                let call_rhs = Call {
                    func: Func::Idx(idx.into_inner()), 
                    args: state.var_stack.split_off(state.var_stack.len()-n_inputs),
                }; 
                if let Some(lhs) = lhs {
                    vec![Stmt::Assign{
                        lhs,
                        rhs: call_rhs,
                        type_: lhs_ty.expect("lhs has to have a type"), 
                    }]
                } else {
                    vec![Stmt::Expr(call_rhs)]
                }            
            }, 

            highlevel::Instr::CallIndirect(fn_type, index) => {
                assert_eq!(index.into_inner(), 0, "wasm mvp must always have a single table"); 

                // in call_indirect,
                // the last variable on the stack is the index value
                // the rest (till you collect all the needed parameters are arguments
                let callind_rhs = CallIndirect{
                    type_: fn_type.clone(), 
                    table_idx: state.var_stack.pop().expect("call_indirect requires an index"),
                    args: state.var_stack.split_off(state.var_stack.len()-fn_type.params.len()),
                }; 

                if let Some(lhs) = lhs {
                    vec![Stmt::Assign{
                        lhs,
                        rhs: callind_rhs,
                        type_: lhs_ty.expect("lhs has to have a type"), 
                    }]
                } else {
                    vec![Stmt::Expr(callind_rhs)]
                }
            }

            highlevel::Instr::Drop => Vec::new(), 

            highlevel::Instr::Select => { 
                let arg1 = state.var_stack.pop().expect("select requires a value on the stack for the condition"); //cond  
                let arg2 = state.var_stack.pop().expect("select requires a value on the stack for the then case"); //if
                let arg3 = state.var_stack.pop().expect("select requires a value on the stack for the else case"); //else
                let type_ = ty.results[0]; 
                
                if let Some(lhs) = lhs {
                    vec![
                
                        Stmt::Assign{ 
                            lhs, 
                            rhs: Const(Val::get_default_value(type_)),  
                            type_ 
                        },
                
                        Stmt::If {
                            condition: arg1,
                            if_body: Body(vec![Stmt::Assign{
                                lhs, 
                                rhs: VarRef(arg2), 
                                type_ 
                            }]),
                            else_body: Some(Body(vec![Stmt::Assign{ 
                                lhs, 
                                rhs: VarRef(arg3), 
                                type_ 
                            }]))
                        }
                    ]     
                } else {
                    panic!("select has to produce a value")
                }
            }

            highlevel::Instr::Local(highlevel::LocalOp::Get, local_idx) => {
                let rhs = resolve_local_idx_to_var(local_idx.into_inner()); 
                if let Some(lhs) = lhs {
                    vec![Stmt::Assign{
                        lhs, 
                        rhs: VarRef(rhs), 
                        type_: lhs_ty.expect("variable that local.get is saving a value into has to have a type"), 
                    }]
                } else {
                    panic!("local.get requires a local variable to save a value into");
                }
            }

            highlevel::Instr::Local(highlevel::LocalOp::Set, local_idx) => {
                let lhs = resolve_local_idx_to_var(local_idx.into_inner()); 
                vec![Stmt::Assign {
                    lhs, 
                    rhs: VarRef(state.var_stack.pop().expect("local.set expects a value on the stack")),
                    type_: *ty.params.get(0).expect("return type of local.set not found"), 
                }]            
            }

            highlevel::Instr::Local(highlevel::LocalOp::Tee, local_idx) => {
                let lhs = resolve_local_idx_to_var(local_idx.into_inner()); 
                let rhs = state.var_stack.pop().expect("local.tee expects a value on the stack");
                state.var_stack.push(rhs); 
                vec![Stmt::Assign{
                    lhs,
                    rhs: VarRef(rhs),
                    type_: *ty.params.get(0).expect("return type of local.set not found"), 
                }]
            }

            highlevel::Instr::Global(highlevel::GlobalOp::Get, global_ind) => {
                let global_var = Global(global_ind.into_inner());
                if let Some(lhs) = lhs {
                    vec![Stmt::Assign{
                        lhs, 
                        rhs: VarRef(global_var),
                        type_: lhs_ty.expect("return type of global.get not found"), 
                    }]
                } else {
                    panic!("global.get requires a local variable to save a value into");
                }
            }

            highlevel::Instr::Global(highlevel::GlobalOp::Set, global_ind) => {
                let global_var = Global(global_ind.into_inner());
                vec![Stmt::Assign{
                    lhs: global_var,
                    rhs: VarRef(state.var_stack.pop().expect("global.set expects a value on the stack")),
                    type_: *ty.params.get(0).expect("return type of global.set not found"), 
                }]
            }

            highlevel::Instr::Load(loadop, memarg) => {
                let lhs = lhs.expect("Every load produces a value");
                let rhs = state.var_stack.pop().expect("Every load consumes a value");
                vec![Stmt::Assign{
                    lhs,
                    rhs: Load {
                        op: *loadop,
                        memarg: *memarg,
                        addr: rhs,
                    },
                    type_: lhs_ty.expect("return type of load not found"), 
                }]
            }

            highlevel::Instr::Store(storeop, memarg) => {
                vec![Stmt::Store {
                    op: *storeop,
                    memarg: *memarg,
                    addr: state.var_stack.pop().expect("store consumes a value for address from the stack which was not found"),
                    value: state.var_stack.pop().expect("store consumes a value to store at the address from the stack which was not found"),
                }]
            }

            highlevel::Instr::MemorySize(_) => {
                if let Some(lhs) = lhs {
                    vec![Stmt::Assign{ lhs, rhs: MemorySize{}, type_: lhs_ty.expect("lhs requires a type") }]
                } else {
                    panic!("memory size has to produce a value"); 
                }
            }

            highlevel::Instr::MemoryGrow(ind) => {
                assert_eq!(ind.into_inner(), 0, "wasm mvp only has single memory");
                if let Some(lhs) = lhs {
                    vec![Stmt::Assign{
                        lhs, 
                        rhs: MemoryGrow {
                            pages: state.var_stack.pop().expect("memory_grow has to consume a value from stack"),
                        },
                        type_: lhs_ty.expect("lhs has to have a type"), 
                    }]
                } else {
                    panic!("memory grow has to produce a value"); 
                }
            }

            highlevel::Instr::Const(val) => {
                if let Some(lhs) = lhs {
                    vec![Stmt::Assign{ lhs, rhs: Const(*val), type_: lhs_ty.expect("lhs has to have a type")}]
                } else {
                    panic!("const has to produce a value "); 
                }
            }

            highlevel::Instr::Numeric(op) => {
                if let Some(lhs) = lhs {
                    vec![Stmt::Assign{
                        lhs, 
                        rhs: Numeric {
                            op: *op,
                            args: state.var_stack.split_off(state.var_stack.len()-n_inputs),
                        },
                        type_: lhs_ty.expect("lhs requires a type"), 
                    }]
                } else {
                    panic!("numeric op has to produce a value "); 
                }
            }
        };
    
        for ins in result_instr {
            result_instrs.push(ins);
        }

        //if !instrs.is_empty() {
        if let Some(lhs) = lhs { 
            state.var_stack.push(lhs); 
            if let Stack(_) = lhs {
                state.stack_var_count += 1;
            }
        }
        //let res = wimplify_instrs(instrs, tys, state).expect("non-empty instruction list in wasm cannot produce an empty list of wimpl instructions");
        //for inst in res {
        //    result_instrs.push(inst); 
        //}
       // } 
    }
    Ok(result_instrs)
}

pub fn wimplify_module (module: &highlevel::Module) -> Result<Module, String> {
    let mut wimpl_funcs = Vec::new(); 
    for (ind, func) in module.functions() {
        
        //initialize the local variables 
        let mut result_instrs = Vec::new(); 
        for (loc_ind, loc) in func.locals() {
            let (loc_name, loc_type) = (&loc.name, loc.type_); 
            if let Some(_loc_name) = loc_name {
                todo!("you haven't yet implemented locals having names");     
            } else {
                result_instrs.push(Stmt::Assign{
                    lhs: Var::Local(loc_ind.into_inner()-func.type_.params.len()),
                    rhs: Expr::Const(Val::get_default_value(loc_type)),
                    type_: loc_type,
                })
            }
        }

        if let Some(code) = func.code() {
            let instrs = code.body.as_slice();
                    // FIXME: generate type on-demand inside wimplify, not here beforehand.
            let mut tys = VecDeque::new();
            let mut type_checker = TypeChecker::begin_function(func, module);
            for instr in instrs {
                let ty = type_checker.check_next_instr(instr).map_err(|e| e.to_string())?;
                tys.push_back(ty);
            }

            let mut instrs = VecDeque::from_iter(instrs); //TODO: pass in iterator instead of vecdeque
            let mut ty = VecDeque::from_iter(tys);

            let mut state = State::new(func.type_.params.len()); 
            if func.type_.results.len() == 0 { state.label_stack.push((0, None)); } 
            else { 
                state.label_stack.push((0, Some((Var::Return(0), func.type_.results[0], false)))); 
            }

            for inst in wimplify_instrs(&mut instrs, &mut ty, &mut state).expect("non-empty instruction list for wasm cannot produce an empty list of wimpl instructions") {
                result_instrs.push(inst); 
            }
        } 
        
        let name = if let Some(name) = &func.name {
            Func::Named(name.clone())
        } else {
            Func::Idx(ind.into_inner())
        }; 

        wimpl_funcs.push(Function{
            type_: func.type_.clone(),
            body: Body(result_instrs), 
            name,
        }); 
    }

    Ok(Module{
        functions: wimpl_funcs,
        globals: module.globals.clone(),
        tables: module.tables.clone(),
    })
}

pub fn wimplify (path: &str) -> Result<Module, String> {
    wimplify_module(&highlevel::Module::from_file(path).expect("path should point to a valid wasm file"))
}
 
#[cfg(test)]
mod tests {
    // Convenience imports:
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
        todo!()
    }

    #[test]
    fn pretty_print_module() {
        todo!()
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
        // FIXME update test input
        let instrs = Stmt::from_text_file("tests/wimpl/syntax.wimpl");
        assert!(instrs.is_ok());
    }
    
    #[test]
    fn macros() {
        let _ = wimpl!(g0: f32 = f32.const 1.1);
        let _ = wimpl!(s2: i32 = i32.add (s0, s1));
        let _ = wimpl!(call_indirect [ ] ->[] (s1) ());

        // Tricky, because rustc lexes these tokens differently than we need to.
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
fn test (path_wimpl: &str, path_wasm: &str) {
    // FIXME:! we cannot just comment out tests that don't run!

    let wimpl_module = wimplify(path_wasm).expect(""); 
    println!("ACTUAL");
    println!("{}", wimpl_module);

    let expected = Stmt::from_text_file(path_wimpl).unwrap();
    println!("EXPECTED");
    for instr in &expected {
        println!("{}", instr);
    }

     
    assert_eq!(wimpl_module.functions[0].body.0, expected);
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

// TODO: fix wimpl test 
#[test]
fn br_nested() {
    test("tests/wimpl/br_nested/br.wimpl", "tests/wimpl/br_nested/br.wasm");
}

//TODO: fix wimpl test 
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
    let wimpl_module = wimplify("tests/wimpl-wasm-handwritten/calc/add.wasm").expect(""); 
    println!("{}", wimpl_module);
}

#[test]
fn calc_dce() {  
    let wimpl_module = wimplify("tests/wimpl-wasm-handwritten/calc-dce/add-dce.wasm").expect(""); 
    println!("{}", wimpl_module);
}

#[test]
fn calc_virtual() {  
    let wimpl_module = wimplify("tests/wimpl-wasm-handwritten/calc-virtual/add.wasm").expect(""); 
    println!("{}", wimpl_module);
}

//USENIX programs 

#[test]
fn module_8c087e0290bb39f1e090() { //TODO: stack overflow 
    let wimpl_module = wimplify("tests/wimpl-USENIX/8c087e0290bb39f1e090.module/8c087e0290bb39f1e090.module.wasm").expect(""); 
    println!("{}", wimpl_module);
}

//br-if line 52241
//br-if ends ends loop 52039
// br to a loop restarts the loop -> does not consume the result -> no assign [br] 
// br to a block ends the block -> consumes the result -> pop from var stack if result is needed [assign, br]
#[test]
fn annots() { 
    let wimpl_module = wimplify("tests/wimpl-USENIX/annots/annots.wasm").expect(""); 
    println!("{}", wimpl_module);
}

#[test]
fn module_bb9bb638551198cd3a42() { 
    let wimpl_module = wimplify("tests/wimpl-USENIX/bb9bb638551198cd3a42.module/bb9bb638551198cd3a42.module.wasm").expect(""); 
    println!("{}", wimpl_module);
}

#[test]
fn compiled_wasm() {  //TODO: same error hmm 
    let wimpl_module = wimplify("tests/wimpl-USENIX/compiled.wasm/compiled.wasm").expect(""); 
    println!("{}", wimpl_module);
}

#[test]
fn module_dac34eee5ed4216c65b2() {   
    let wimpl_module = wimplify("tests/wimpl-USENIX/dac34eee5ed4216c65b2.module/dac34eee5ed4216c65b2.module.wasm").expect(""); 
    println!("{}", wimpl_module);
}

// TODO: br_if line 2344 
// br_if produces a value 
// why does br_if produce a value? when the label it is jumping to expects a value as a result 
#[test]
fn imagequant_c970f() {  
    let wimpl_module = wimplify("tests/wimpl-USENIX/imagequant.c970f/imagequant.c970f.wasm").expect(""); 
    println!("{}", wimpl_module);
}


//TODO: same error as above 
#[test]
fn mozjpeg_enc_93395() {  
    let wimpl_module = wimplify("tests/wimpl-USENIX/mozjpeg_enc.93395/mozjpeg_enc.93395.wasm").expect(""); 
    println!("{}", wimpl_module);
}

//TODO: bug
#[test]
fn optipng_4e77b() {  
    let wimpl_module = wimplify("tests/wimpl-USENIX/optipng.4e77b/optipng.4e77b.wasm").expect(""); 
    println!("{}", wimpl_module);
}

//TODO: test after that return empty bug is fixed 
#[test]
fn rotate_4cdaa() {  
    let wimpl_module = wimplify("tests/wimpl-USENIX/rotate.4cdaa/rotate.4cdaa.wasm").expect(""); 
    println!("{}", wimpl_module);
}


#[test]
fn USENIX_bin_acrobat_wasm() {  
    let wimpl_module = wimplify("tests/wimpl-USENIX/USENIX_bin_acrobat.wasm/USENIX_bin_acrobat.wasm.wasm").expect(""); 
    println!("{}", wimpl_module);
}

//TODO: same bug 
#[test]
fn webp_dec_fa0ab() {  
    let wimpl_module = wimplify("tests/wimpl-USENIX/webp_dec.fa0ab/webp_dec.fa0ab.wasm").expect(""); 
    println!("{}", wimpl_module);
}

//TODO: 
#[test]
fn webp_enc_ea665() {  
    let wimpl_module = wimplify("tests/wimpl-USENIX/webp_enc.ea665/webp_enc.ea665.wasm").expect(""); 
    println!("{}", wimpl_module);
}

