use rustc_hash::FxHashMap;

use crate::wimpl::{Expr, Var, Body};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct VarExprMap(FxHashMap<Var, Option<Expr>>);

impl VarExprMap {
    pub fn from_body(body: &Body) -> Self {
        let mut self_ = Self::default();

        // Recursive "visitor" over statements/bodies.
        // TODO overapproximates because variable map is created before the constraint construction,
        // so more variables will be Top/None (namely those assigned after a call in a block).
        fn collect_var_expr(body: &Body, var_expr: &mut VarExprMap) {
            for stmt in &body.0 {
                use crate::wimpl::Stmt::*; 
                match stmt {
                    Unreachable => {},
                    Expr(_) => {},
                    Assign { lhs, type_: _, rhs } => var_expr.add(lhs, rhs),
                    Store { .. } => {},
                    Br { .. } => {},
                    
                    // Recursive cases:
                    // TODO Extract out in recursive visitor pattern.
                    Block { body, end_label: _ } => collect_var_expr(body, var_expr),
                    Loop { begin_label: _, body } => collect_var_expr(body, var_expr),
                    If { condition:_ , if_body, else_body } => {
                        collect_var_expr(if_body, var_expr);
                        if let Some(else_body) = else_body {
                            collect_var_expr(else_body, var_expr)
                        }
                    },
                    Switch { index: _, cases, default } => {
                        for case in cases {
                            collect_var_expr(case, var_expr);
                        }
                        collect_var_expr(default, var_expr)
                    },
                }
            }
        }
        collect_var_expr(body, &mut self_);

        self_
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
