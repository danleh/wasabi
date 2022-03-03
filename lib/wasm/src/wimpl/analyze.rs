use std::fmt;

use rustc_hash::FxHashMap;

use crate::wimpl::{Stmt, Expr, Var, Body};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct VarExprMap(FxHashMap<Var, Option<Expr>>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VarExprMapResult<'a> {
    Precise(&'a Expr),
    Top,
    Uninitialized(Var)
}

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

    pub fn get(&self, var: &Var) -> VarExprMapResult {
        match self.0.get(var) {
            // Expression itself refers to a variable, resolve that recursively:
            Some(Some(Expr::VarRef(var))) => self.get(var),
            // Non-recursive case: non-var expression.
            Some(Some(other_expr)) => VarExprMapResult::Precise(other_expr),
            // Overapproximated (e.g., because the variable was assigned twice):
            Some(None) => VarExprMapResult::Top,
            // Uninitialized variable, e.g., parameter:
            None => VarExprMapResult::Uninitialized(*var),
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