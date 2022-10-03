//! Parsing of the Wimpl text format.

use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1},
    character::complete::{alphanumeric1, multispace1, not_line_ending, u32},
    combinator::{all_consuming, map, map_res, opt, value},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    AsChar, Finish, IResult,
};

use crate::*;

/// User-facing error for parsing the Wimpl text format.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParseError {
    /// Line number in the full input where the error occurred (1-indexed).
    line: usize,

    /// Byte offset inside the line where the error occurred (0-indexed).
    column: usize,

    /// Erroneous input line text.
    input_line: String,

    nom_error_kind: Option<nom::error::ErrorKind>,
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
                    nom_error_kind: Some(error.code)
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

impl FromStr for FunctionId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(idx) = s.strip_prefix('f') {
            let idx = idx.parse().map_err(|_| ())?;
            FunctionId::Idx(idx)
        } else {
            // Functions must start with an alphabetic character, to avoid confusion with constants.
            match s.chars().next() {
                Some(c) if c.is_alphabetic() => FunctionId::Name(s.to_string().into()),
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


/// Type abbreviation for nom's own parser result type.
pub type NomResult<'input, O> = IResult<&'input str, O>;

/// Adapt a nom parser such that it returns a standard Rust `Result`.
fn adapt_nom_parser<'input, O>(
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
fn func(input: &str) -> NomResult<FunctionId> {
    map_res(alphanumeric1, FunctionId::from_str)(input)
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

fn arg_single(input: &str) -> NomResult<Expr> {
    delimited(
        pair(tag("("), ws), expr, pair(ws, tag(")"))
    )(input)
}
fn arg_list(input: &str) -> NomResult<Vec<Expr>> {
    delimited(
        pair(tag("("), ws),
        separated_list0(tuple((ws, tag(","), ws)), expr),
        pair(ws, tag(")")),
    )(input)
}

// Memarg parsing depends on result of previous LoadOp/StoreOp parsing.
// This is easier to write in direct than in point-free style, so we do.
fn load(input: &str) -> NomResult<ExprKind> {
    let (input, op) = op::<LoadOp>(input)?;
    let (input, addr) = arg_single(input)?;
    Ok((
        input,
        ExprKind::Load {
            op,
            addr: Box::new(addr),
        },
    ))
}

fn store(input: &str) -> NomResult<StmtKind> {
    let (input, op) = op::<StoreOp>(input)?;
    let (input, addr) = arg_single(input)?;
    let (input, ()) = ws(input)?;
    let (input, value) = arg_single(input)?;
    Ok((
        input,
        StmtKind::Store {
            op,
            addr,
            value,
        },
    ))
}

fn expr(input: &str) -> NomResult<Expr> {
    use ExprKind::*;

    let var_ref = map(var, VarRef);

    // This is a bit tricky to parse, because `Val::from_str` requires a tyle as input,
    // so parse the type first, the number only approximately, and then pass both later.
    let const_ = map_res(
        tuple((
            val_type,
            tag(".const"),
            ws,
            // HACK Accept any potential integer or float literal character,
            // the rest of the parsing is done by Val::from_str.
            // TODO Check which characters are actually valid for int/float literals in Wasm.
            take_while1(|c: char| c.is_alphanum() || c == '.'),
        )),
        |(ty, _, (), number)| Val::from_str(number, ty).map(Const),
    );

    let memory_size = map(tag("memory.size"), |_| MemorySize);
    let memory_grow = map(
        tuple((tag("memory.grow"), ws, arg_single)),
        |(_, (), pages)| MemoryGrow { pages: Box::new(pages) },
    );

    let unary = map_res(
        tuple((op::<UnaryOp>, ws, arg_list)),
        |(op, (), mut args)| -> Result<_, _> {
            if args.len() != 1 { return Err("wrong number of arguments for unary operator") }
            let arg = args.pop().unwrap();
            Ok(Unary(op, Box::new(arg)))
        },
    );
    let binary = map_res(
        tuple((op::<BinaryOp>, ws, arg_list)),
        |(op, (), mut args)| -> Result<_, _> {
            if args.len() != 2 { return Err("wrong number of arguments for binary operator") }
            let right = args.pop().unwrap();
            let left = args.pop().unwrap();
            Ok(Binary(op, Box::new(left), Box::new(right)))
        },
    );

    let call = map(
        tuple((tag("call"), ws, func, ws, arg_list)),
        |(_, (), func, (), args)| Call { func, args }
    );

    // HACK For call_indirect, we know nothing besides the argument list is following
    // the function type, so consume up to the opening parenthesis.
    // FIXME However, this will fail to recognize comments after the function type!
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
            CallIndirect { type_, table_idx: Box::new(table_idx), args }
    );

    map(
        // Parse `ExprKind`...
        alt((
            var_ref, 
            const_, 
            load, 
            memory_size, 
            memory_grow, 
            unary, 
            binary, 
            call, 
            call_indirect
        )),
        // ... and add fresh `InstrId`.
        Expr::new
    )(input)
}

/// Parse multiple statements, with possibly preceding and trailing whitespace.
fn stmts_ws(input: &str) -> NomResult<Vec<Stmt>> {
    preceded(ws, many0(terminated(stmt, ws)))(input)
}

fn body(input: &str) -> NomResult<Body> {
    map(delimited(tag("{"), stmts_ws, tag("}")), Body)(input)
}

/// Parse a single statement, without surrounding whitespace.
fn stmt(input: &str) -> NomResult<Stmt> {
    use StmtKind::*;

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

    map(
        // Parse `StmtKind`...
        alt((
            unreachable,
            assign,
            store,
            br,
            block,
            loop_,
            if_,
            switch,

            // HACK Put this case below assign, otherwise the parser parses a LHS variable, e.g.,
            // `l0 = ...` as `Stmt::Expr(Expr::VarRef(...))` and then fails on the rest from `=` onward.
            expr_stmt,
        )),
        // ... and add fresh `InstrId`.
        Stmt::new
    )(input)
}

// Adapt the nom parsers above such that they can be used with Rust `parse()` / `from_str`.

impl Stmt {
    pub fn from_str_multiple(input: &str) -> Result<Vec<Self>, ParseError> {
        adapt_nom_parser(stmts_ws, input)
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
        adapt_nom_parser(expr, input)
    }
}

impl FromStr for Stmt {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        adapt_nom_parser(stmt, input)
    }
}
