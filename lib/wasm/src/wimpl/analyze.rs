use std::fmt;

use rustc_hash::FxHashMap;

use crate::wimpl::{Stmt, Expr, Var, Body};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct VarExprMap(FxHashMap<Var, Option<Expr>>);

impl VarExprMap {
    pub fn from_body(body: &Body) -> Self {
        let mut map = Self::default();

        body.visit_stmt_pre_order(|stmt| 
            if let Stmt::Assign { lhs, type_: _, rhs } = stmt {
                map.add(lhs, rhs)
            }
        );

        map
    }

    pub fn add(&mut self, var: &Var, expr: &Expr) {
        self.0.entry(*var)
            // Overapproximate if there was already a prior assignment to that variable.
            .and_modify(|old_expr| *old_expr = None)
            .or_insert_with(|| Some(expr.clone()));
    }

    /// Returns `Ok(None)` if variable expression was over approximated.
    /// Returns `Err` if variable was never assigned before.
    pub fn get(&self, var: &Var) -> Result<Option<&Expr>, String> {
        match self.0.get(var) {
            // Recursive case: expression itself refers to a variable, resolve that recursively:
            Some(Some(Expr::VarRef(var))) => self.get(var),
            // Non-recursive case: non-var expression.
            Some(Some(other_expr)) => Ok(Some(other_expr)),
            // Overapproximated (because assigned twice):
            Some(None) => Ok(None),
            // Uninitialized variable, e.g., parameter:
            None => Err(format!("uninitialized variable `{}`\nvariable map: {:?}", var, self.0)),
        }
    }
}

impl fmt::Display for VarExprMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{{")?;
        for (var, maybe_expr) in &self.0 {
            match maybe_expr {
                Some(expr) => writeln!(f, "    {var}: {expr},")?,
                None => writeln!(f, "    {var}: Top,")?,
            }
        }
        write!(f, "}}")
    }
}