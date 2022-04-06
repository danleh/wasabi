//! Pretty-printing of Wimpl.

use std::{fmt, convert::TryInto};

use crate::wimpl::*;

const PRETTY_PRINT_INDENT: &str = "  ";
const PRETTY_PRINT_NEWLINE_INDENT: &str = "\n  ";

/// Helper function that indents each line of the `Display` output of `x`.
fn indent_once(x: &dyn fmt::Display) -> String {
    // This allocates a new String just for indentation, which seems wasteful.
    // But since we pretty-print Wimpl fairly seldom, and even if, it's not performance criticial,
    // and there are also other places below where we allocate, let's not optimize this for now.
    // If it ever really becomes a problem, use the `pad-adapter` crate.
    format!("{}{}", PRETTY_PRINT_INDENT, x).replace('\n', PRETTY_PRINT_NEWLINE_INDENT)
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
        if let Some((last, rest)) = self.export.split_last() {
            write!(f, "export ")?;
            for name in rest {
                write!(f, "{:?}, ", name)?;
            }
            writeln!(f, "{:?}", last)?;
        }
        write!(f, "func {}", self.name)?;
        write!(f, " (")?;
        for (i, ty) in self.type_.inputs().iter().enumerate() {
            write!(f, "{}: {}", Var::Param(i.try_into().expect("more than 2^32 parameters")), ty)?;
            if i < self.type_.inputs().len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ") -> (")?;
        for (i, ty) in self.type_.results().iter().enumerate() {
            write!(f, "{}: {}", Var::Return(i.try_into().expect("more than 2^32 return values")), ty)?;
            if i < self.type_.results().len() - 1 {
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

        // HACK If the inner part (inside the curly braces) is only a single line, e.g., a single br
        // instruction and doesn't contain any nested structures (no curly braces or parentheses),
        // then print the whole body as a single line as well.
        // TODO A better way would be using some advanced pretty printer algorithm, e.g., using
        // Wadler's document-based approach.
        if inner.lines().count() == 1 && !inner.contains(['(', '{']) {
            write!(f, "{{ {} }}", inner)
        } else {
            write!(f, "{{\n{}\n}}", indent_once(&inner))
        }
    }
}

impl fmt::Display for FunctionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionId::Name(s) => write!(f, "{}", s),
            FunctionId::Idx(i) => write!(f, "f{}", i),
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
        // The `InstrId` is only for our internal metadata, so don't display that.
        use StmtKind::*;
        match &self.kind {

            Unreachable => f.write_str("unreachable")?,

            Expr(expr) => write!(f, "{}", expr)?,

            Assign { lhs, rhs , type_} => {
                write!(f, "{}: {} = {}", lhs, type_, rhs)?;
            },

            // i32.store offset=3 align=4 (s0) (s1)
            // The first argument is addr, second is value.
            Store {
                op,
                addr,
                value,
            } => {
                write!(f, "{} ({}) ({})", op, addr, value)?;
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
        // The `InstrId` is only for our internal metadata, so don't display that.
        use ExprKind::*;
        match &self.kind {

            VarRef(var) => write!(f, "{}", var),

            // i32.const 3
            Const(val) => write!(f, "{}.const {}", val.to_type(), val),

            // i32.load offset=3 align=4 (s0)
            Load {
                op,
                addr
            } => {
                write!(f, "{}({})", op, addr)
            },

            MemorySize => write!(f, "memory.size"),
            MemoryGrow { pages } => write!(f, "memory.grow({})", pages),

            // f32.neg(s0)
            Unary(op, arg) => write!(f, "{}({})", op, arg),
            Binary(op, left, right) => write!(f, "{}({}, {})", op, left, right),

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
