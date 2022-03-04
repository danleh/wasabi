use std::{cmp::Reverse, fmt::{self, Display}, iter::FromIterator, cell::RefCell, collections::HashMap};

use regex::Regex;
use rustc_hash::FxHashMap;

use crate::{wimpl::{Body, Expr, Module, Stmt, Var}, highlevel::{StoreOp, LoadOp}};

// TODO Analysis for identification of heap allocation function ("malloc")
// Often required for "allocation site abstraction" in pointer analysis
// But not apparent in WebAssembly
// Idea: constraints: (i) function must return i32 (ii) must have an edge to function with memory.grow instruction

/// A map from a variable to a 
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct VarExprMap<'module>(FxHashMap<Var, Option<&'module Expr>>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VarExprMapResult<'module> {
    Precise(&'module Expr),
    Top,
    Uninitialized(Var),
}

impl<'module> VarExprMap<'module> {
    pub fn from_body(body: &'module Body) -> Self {
        let mut map = Self::default();

        body.visit_stmt_pre_order(|stmt| {
            if let Stmt::Assign { lhs, type_: _, rhs } = stmt {
                map.add(*lhs, rhs)
            }
        });

        map
    }

    pub fn add(&mut self, var: Var, expr: &'module Expr) {
        self.0
            .entry(var)
            // Overapproximate if there was already a prior assignment to that variable.
            .and_modify(|old_expr| *old_expr = None)
            .or_insert_with(|| Some(expr));
    }

    pub fn get(&self, var: Var) -> VarExprMapResult {
        match self.0.get(&var) {
            // Expression itself refers to a variable, resolve that recursively:
            Some(Some(Expr::VarRef(var))) => self.get(*var),
            // Non-recursive case: non-var expression.
            Some(Some(other_expr)) => VarExprMapResult::Precise(other_expr),
            // Overapproximated (e.g., because the variable was assigned twice):
            Some(None) => VarExprMapResult::Top,
            // Uninitialized variable, e.g., parameter:
            None => VarExprMapResult::Uninitialized(var),
        }
    }
}

impl<'module> fmt::Display for VarExprMap<'module> {
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
pub fn collect_call_indirect_idx_expr(module: &Module) -> FxHashMap<String, usize> {
    let mut result: FxHashMap<String, usize> = FxHashMap::default();
    for func in &module.functions {
        use Expr::*;
        func.body.visit_expr_pre_order(|expr| {
            if let CallIndirect { type_: _, table_idx, args: _ } = expr {
                let table_idx = abstract_expr(table_idx);
                *result.entry(table_idx).or_default() += 1;
            }
        });
    }
    result
}

/// Returns a slightly abstracted form of the call_indirect expressions, sorted descending by count.
pub fn collect_i32_load_store_arg_expr(module: &Module) -> (
    /* addrs */ FxHashMap<String, usize>, 
    /* store values */ FxHashMap<String, usize>,
 ) {
    let addrs: RefCell<FxHashMap<String, usize>> = RefCell::new(FxHashMap::default());
    let mut values: FxHashMap<String, usize> = FxHashMap::default();
    for func in &module.functions {
        use crate::wimpl::Expr::*;
        use crate::wimpl::Stmt::*;
        // TODO / FIXME Can we make the assumption that call_indirect idx are always loaded/stored
        // via full i32s?
        func.body.visit_pre_order(|expr| {
            if let Store { op: StoreOp::I32Store, memarg: _, addr, value } = expr {
                *addrs.borrow_mut().entry(abstract_expr(addr)).or_default() += 1;
                *values.entry(abstract_expr(value)).or_default() += 1;
            }
        },
        |expr| {
            if let Load { op: LoadOp::I32Load, memarg: _, addr } = expr {
                *addrs.borrow_mut().entry(abstract_expr(addr)).or_default() += 1;
            }
        });
    }
    (addrs.into_inner(), values)
}

pub fn sort_map_count<T: Ord + Clone, Hasher>(map: &HashMap<T, usize, Hasher>) -> Vec<(T, usize)> {
    let mut vec = Vec::from_iter(map.iter().map(|(t, count)| (t.clone(), *count)));
    vec.sort_by_key(|(expr, count)| Reverse((*count, expr.clone())));
    vec
}

pub fn print_map_count<T: Ord + Clone + Display, Hasher>(map: &HashMap<T, usize, Hasher>) {
    let total: f64 = map.iter().map(|(_, count)| *count as f64).sum();
    for (t, count) in sort_map_count(map).into_iter().take(20) {
        let percent = count as f64 / total * 100.0;
        println!("{:8} ({:5.2}%)  {}", count, percent, t);
    }
}

// HACK Remove some stuff that is irrelevant for our analysis
pub fn abstract_expr(expr: &Expr) -> String {
    let expr = expr.to_string();

    lazy_static::lazy_static! {
        static ref MEMARG: Regex = Regex::new(r"\s*offset=\d+\s*").unwrap();
        static ref ALIGN: Regex = Regex::new(r"\s*align=\d+\s*").unwrap();

        static ref PARAM: Regex = Regex::new(r"p\d+").unwrap();
        static ref STACK: Regex = Regex::new(r"s\d+").unwrap();
        static ref LOCAL: Regex = Regex::new(r"l\d+").unwrap();
        static ref GLOBAL: Regex = Regex::new(r"g\d+").unwrap();
        static ref RETURN: Regex = Regex::new(r"r\d+").unwrap();
        static ref BLOCK: Regex = Regex::new(r"b\d+").unwrap();

        static ref CONST: Regex = Regex::new(r"const -?\d+").unwrap();
        static ref FUNC: Regex = Regex::new(r"call \w+").unwrap();
    }
    const UNIFY_VARS: bool = false;
    let expr = PARAM.replace_all(&expr, if UNIFY_VARS { "<var>" } else { "<param>" });
    let expr = STACK.replace_all(&expr, if UNIFY_VARS { "<var>" } else { "<stack>" });
    let expr = LOCAL.replace_all(&expr, if UNIFY_VARS { "<var>" } else { "<local>" });
    let expr = GLOBAL.replace_all(&expr, if UNIFY_VARS { "<var>" } else { "<global>" });
    let expr = RETURN.replace_all(&expr, if UNIFY_VARS { "<var>" } else { "<return>" });
    let expr = BLOCK.replace_all(&expr, if UNIFY_VARS { "<var>" } else { "<block>" });

    let expr = MEMARG.replace_all(&expr, "");
    let expr = ALIGN.replace_all(&expr, "");
    let expr = CONST.replace_all(&expr, "const <const>");
    let expr = FUNC.replace_all(&expr, "call <func>");

    expr.to_string()
}
