use std::{fmt, iter::FromIterator, cmp::Reverse};

use regex::Regex;
use rustc_hash::FxHashMap;

use crate::wimpl::{Stmt, Expr, Var, Body, Module};

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

/// Returns a slightly abstracted form of the call_indirect expressions, sorted descending by count.
pub fn collect_call_indirect_idx_expr(module: &Module) -> Vec<(String, usize)> {
    let mut result: FxHashMap<String, usize> = FxHashMap::default();
    for func in &module.functions {
        use Expr::*;
        func.body.visit_expr_pre_order(|expr| 
            if let CallIndirect { type_: _, table_idx, args: _ } = expr {
                let table_idx = table_idx.to_string();

                // HACK Remove some stuff that is irrelevant for our analysis
                lazy_static::lazy_static! {
                    static ref MEMARG: Regex = Regex::new(r"\s+offset=\d+\s+").unwrap();
                    static ref PARAM: Regex = Regex::new(r"p\d+").unwrap();
                    static ref STACK: Regex = Regex::new(r"s\d+").unwrap();
                    static ref LOCAL: Regex = Regex::new(r"l\d+").unwrap();
                    static ref CONST: Regex = Regex::new(r"const \d+").unwrap();
                }
                let table_idx = MEMARG.replace_all(&table_idx, "");
                let table_idx = PARAM.replace_all(&table_idx, "<param>");
                let table_idx = STACK.replace_all(&table_idx, "<stack>");
                let table_idx = LOCAL.replace_all(&table_idx, "<local>");
                let table_idx = CONST.replace_all(&table_idx, "const <const>");

                *result.entry(table_idx.to_string()).or_default() += 1;
            }
        );
    }
    let mut result = Vec::from_iter(result);
    result.sort_by_key(|(expr, count)| Reverse((*count, expr.clone())));
    result
}
