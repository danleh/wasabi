use std::{cmp::Reverse, fmt::{self, Display}, iter::FromIterator, cell::RefCell, collections::HashMap, ops::{Add, BitAnd, BitOr, RangeInclusive}};

use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{wimpl::{Body, Expr, Module, Stmt, Var}, highlevel::{StoreOp, LoadOp}};

use super::FunctionId;

// TODO Analysis for identification of heap allocation function ("malloc")
// Often required for "allocation site abstraction" in pointer analysis
// But not apparent in WebAssembly
// Idea: constraints: (i) function must return i32 (ii) must have an edge to function with memory.grow instruction

// TODO Inference of function pointer types for locals/parameters
// E.g., call_indirect (i32.load(p0)) (...) would induce the constraint p0: ptr<func>
// and call_indirect(i32.load offset=4(i32.load(p0))) would induce p0: ptr<struct { offset 4: ptr<func> }>

// TODO Refine types by collecting constraints on all parameter and local usages
// E.g., lifting func (i32, i32) to func (ptr<func>, ptr<struct>) ...
// and use that to make call_indirect constraints more precise (because there are higher types)

/// A map from a variable to the assigned expression (or over-approximating Top if there were
/// multiple assignments).
// FIXME Currently disregards kill sets (e.g., when the expression depends on a variable that
// is then later re-assigned) and loops (e.g., when a variable is lexically only a single time
// assigned, but at program execution multiple times in multiple iterations).
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

// HACKY Abstract away part of an expression by simple string replacement.
pub fn abstract_expr(expr: &Expr) -> String {
    let expr = expr.to_string();

    lazy_static::lazy_static! {
        static ref MEMARG: Regex = Regex::new(r"\s+offset=\d+\s*").unwrap();
        static ref ALIGN: Regex = Regex::new(r"\s+align=\d+\s*").unwrap();

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

    // let expr = MEMARG.replace_all(&expr, "");
    let expr = ALIGN.replace_all(&expr, "");
    let expr = CONST.replace_all(&expr, "const <const>");
    // let expr = FUNC.replace_all(&expr, "call <func>");

    expr.to_string()
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
            if let Store { op: StoreOp::I32Store, addr, value } = expr {
                *addrs.borrow_mut().entry(abstract_expr(addr)).or_default() += 1;
                *values.entry(abstract_expr(value)).or_default() += 1;
            }
        },
        |expr| {
            if let Load { op: LoadOp::I32Load, addr } = expr {
                *addrs.borrow_mut().entry(abstract_expr(addr)).or_default() += 1;
            }
        });
    }
    (addrs.into_inner(), values)
}

pub fn collect_memory_functions(module: &Module) -> Vec<(FunctionId, bool, bool)> {
    let mut result = Vec::new();
    for func in &module.functions {
        let mut has_memory_size = false;
        let mut has_memory_grow = false;
        use crate::wimpl::Expr::*;
        func.body.visit_expr_pre_order(|expr| match expr {
            MemorySize => has_memory_size = true,
            MemoryGrow { pages: _ } => has_memory_grow = true,
            _ => {}
        });
        if has_memory_size || has_memory_grow {
            result.push((func.name(), has_memory_size, has_memory_grow));
        }
    }
    result.sort();
    result
}

pub fn collect_function_direct_call_count(module: &Module) -> FxHashMap<FunctionId, usize> {
    let mut result: FxHashMap<FunctionId, usize> = FxHashMap::default();
    for func in &module.functions {
        use crate::wimpl::Expr::*;
        func.body.visit_expr_pre_order(|expr| 
            if let Call { func, args: _ } = expr { 
                *result.entry(func.clone()).or_default() += 1 
            }
        );
    }
    result
}

/// Utility function for sorting a "count map" by count in descending order.
pub fn sort_map_count<T: Ord + Clone, Hasher>(map: &HashMap<T, usize, Hasher>) -> Vec<(T, usize)> {
    let mut vec = Vec::from_iter(map.iter().map(|(t, count)| (t.clone(), *count)));
    vec.sort_by_key(|(expr, count)| Reverse((*count, expr.clone())));
    vec
}

/// Utility function to print out a "count map", with percentages.
pub fn print_map_count<T: Ord + Clone + Display, Hasher>(map: &HashMap<T, usize, Hasher>) {
    let total: f64 = map.iter().map(|(_, count)| *count as f64).sum();
    for (t, count) in sort_map_count(map).into_iter().take(20) {
        let percent = count as f64 / total * 100.0;
        println!("{:8} ({:5.2}%)  {}", count, percent, t);
    }
}

/// An over-approximation of a WebAssembly i32 value as an interval.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct I32Range {
    min: u32,
    max_inclusive: u32
}

impl Default for I32Range {
    fn default() -> Self {
        Self { min: 0, max_inclusive: u32::MAX }
    }
}

impl I32Range {
    pub fn new(min: u32, max_inclusive: u32) -> Self {
        Self { min, max_inclusive }
    }

    pub fn exact(val: u32) -> I32Range {
        Self { min: val, max_inclusive: val }
    }

    pub fn iter(self) -> impl Iterator<Item = u32> {
        self.min..=self.max_inclusive
    }
}

impl Add for I32Range {
    type Output = I32Range;
    fn add(self, rhs: Self) -> Self::Output {
        let min_min = self.min.checked_add(rhs.min);
        let min_max = self.min.checked_add(rhs.max_inclusive);
        let max_min = self.max_inclusive.checked_add(rhs.min);
        let max_max = self.max_inclusive.checked_add(rhs.max_inclusive);
        match (min_min, min_max, max_min, max_max) {
            (Some(min_min), Some(min_max), Some(max_min), Some(max_max)) => {
                I32Range::new(min_min.min(max_min).min(min_max), max_max.max(min_max).max(max_min))
            }
            _overflow => I32Range::default(),
        }
    }
}

impl BitAnd for I32Range {
    type Output = I32Range;
    fn bitand(self, rhs: Self) -> Self::Output {
        let min_min = self.min & rhs.min;
        let min_max = self.min & rhs.max_inclusive;
        let max_min = self.max_inclusive & rhs.min;
        let max_max = self.max_inclusive & rhs.max_inclusive;
        I32Range::new(min_min.min(max_min).min(min_max), max_max.max(min_max).max(max_min))
    }
}

impl BitOr for I32Range {
    type Output = I32Range;
    fn bitor(self, rhs: Self) -> Self::Output {
        let min_min = self.min | rhs.min;
        let min_max = self.min | rhs.max_inclusive;
        let max_min = self.max_inclusive | rhs.min;
        let max_max = self.max_inclusive | rhs.max_inclusive;
        I32Range::new(min_min.min(max_min).min(min_max), max_max.max(min_max).max(max_min))
    }
}

#[test]
fn test_add_without_overflow() {
    assert_eq!(I32Range::new(3, 20).add(I32Range::exact(5)), I32Range::new(8, 25));
    assert_eq!(I32Range::new(3, 20).add(I32Range::new(0, 2)), I32Range::new(3, 22));
}

/// Panics if called with a non-i32 expression.
pub fn approx_i32_eval(expr: &Expr) -> I32Range {
    use Expr::*;
    use crate::Val;
    use crate::ValType;
    use crate::highlevel::NumericOp::*;
    match expr {
        Const(Val::I32(val)) => I32Range::exact(*val as u32),
        Const(_) => panic!("should only be called with an i32 expression"),

        // Over-approximate cases:
        VarRef(_) => I32Range::default(),
        Load { op: _, addr: _ } => I32Range::default(),
        // TODO In the future we could use the module's memory limits to restrict min = min size and
        // max = max size (if any). 
        // Not for now, however because memory.* does not appear in call_indirect expressions.
        MemorySize => I32Range::default(),
        MemoryGrow { pages: _ } => I32Range::default(),
        Call { func: _, args: _ } => I32Range::default(),
        CallIndirect { type_: _, table_idx: _, args: _ } => I32Range::default(),

        // Recursive "evaluation".
        Numeric { op, args: _  } if op.to_type().results()[0] != ValType::I32 => panic!("should only be called with an i32 expression"),
        // "Boolean" i32 results.
        // TODO Proper evaluation with ranged integer arithmetic.
        Numeric { op: I32Eqz, args: _ } => I32Range::new(0, 1),
        Numeric { op: I32Eq, args: _ } => I32Range::new(0, 1),
        Numeric { op: I32Ne, args: _ } => I32Range::new(0, 1),
        Numeric { op: I32LtS, args: _ } => I32Range::new(0, 1),
        Numeric { op: I32LtU, args: _ } => I32Range::new(0, 1),
        Numeric { op: I32GtS, args: _ } => I32Range::new(0, 1),
        Numeric { op: I32GtU, args: _ } => I32Range::new(0, 1),
        Numeric { op: I32LeS, args: _ } => I32Range::new(0, 1),
        Numeric { op: I32LeU, args: _ } => I32Range::new(0, 1),
        Numeric { op: I32GeS, args: _ } => I32Range::new(0, 1),
        Numeric { op: I32GeU, args: _ } => I32Range::new(0, 1),
        Numeric { op, args } => match &args[..] {
            // TODO Implement I32Clz etc.
            [_unary] => I32Range::default(),
            [first, second] => match op {
                I32Add => approx_i32_eval(first).add(approx_i32_eval(second)),
                I32And => approx_i32_eval(first).bitand(approx_i32_eval(second)),
                I32Or => approx_i32_eval(first).bitor(approx_i32_eval(second)),

                // TODO Implement more NumericOps.
                // I32Sub => todo!(),
                // I32Mul => todo!(),
                // I32DivS => todo!(),
                // I32DivU => todo!(),
                // I32RemS => todo!(),
                // I32RemU => todo!(),
                // I32Xor => todo!(),
                // I32Shl => todo!(),
                // I32ShrS => todo!(),
                // I32ShrU => todo!(),
                // I32Rotl => todo!(),
                // I32Rotr => todo!(),
                _ => I32Range::default(),
            },
            _ => unreachable!("Wasm has only unary and binary numeric operations")
        },
    }
}