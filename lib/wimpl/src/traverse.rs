//! Functions for iterating over Wimpl statements and expressions.

use std::ops::Add;

use crate::{Body, Stmt, Expr};

impl Body {
    // A "facade", such that one doesn't have to borrow the closure arguments manually.
    pub fn visit_pre_order<'a>(&'a self, mut f_stmt: impl FnMut(&'a Stmt) -> bool, mut f_expr: impl FnMut(&'a Expr) -> bool) {
        self.visit_pre_order_(&mut f_stmt, &mut f_expr)
    }

    pub fn visit_stmt_pre_order<'a>(&'a self, mut f_stmt: impl FnMut(&'a Stmt) -> bool) {
        self.visit_pre_order_(&mut f_stmt, &mut |_| true)
    }

    pub fn visit_expr_pre_order<'a>(&'a self, mut f_expr: impl FnMut(&'a Expr) -> bool) {
        self.visit_pre_order_(&mut |_| true, &mut f_expr)
    }

    fn visit_pre_order_<'a>(&'a self, f_stmt: &mut dyn FnMut(&'a Stmt) -> bool, f_expr: &mut dyn FnMut(&'a Expr) -> bool) {
        for stmt in &self.0 {
            stmt.visit_pre_order_(f_stmt, f_expr)
        }
    }
}

/// Convenience extension trait to allow visiting a `Option<Body>`, which is common for imported
/// functions.
pub trait VisitOptionBodyExt {
    fn visit_pre_order<'a>(&'a self, f_stmt: impl FnMut(&'a Stmt) -> bool, f_expr: impl FnMut(&'a Expr) -> bool);
    fn visit_stmt_pre_order<'a>(&'a self, f_stmt: impl FnMut(&'a Stmt) -> bool);
    fn visit_expr_pre_order<'a>(&'a self, f_expr: impl FnMut(&'a Expr) -> bool);
    fn visit_pre_order_<'a>(&'a self, f_stmt: &mut dyn FnMut(&'a Stmt) -> bool, f_expr: &mut dyn FnMut(&'a Expr) -> bool);
}

impl VisitOptionBodyExt for Option<Body> {
    fn visit_pre_order<'a>(&'a self, f_stmt: impl FnMut(&'a Stmt) -> bool, f_expr: impl FnMut(&'a Expr) -> bool) {
        if let Some(body) = self {
            body.visit_pre_order(f_stmt, f_expr)
        }
    }

    fn visit_stmt_pre_order<'a>(&'a self, f_stmt: impl FnMut(&'a Stmt) -> bool) {
        if let Some(body) = self {
            body.visit_stmt_pre_order(f_stmt)
        }
    }

    fn visit_expr_pre_order<'a>(&'a self, f_expr: impl FnMut(&'a Expr) -> bool) {
        if let Some(body) = self {
            body.visit_expr_pre_order(f_expr)
        }
    }

    fn visit_pre_order_<'a>(&'a self, f_stmt: &mut dyn FnMut(&'a Stmt) -> bool, f_expr: &mut dyn FnMut(&'a Expr) -> bool) {
        if let Some(body) = self {
            body.visit_pre_order_(f_stmt, f_expr)
        }
    }
}


impl Stmt {
    pub fn visit_pre_order<'a>(&'a self, mut f_stmt: impl FnMut(&'a Stmt) -> bool, mut f_expr: impl FnMut(&'a Expr) -> bool) {
        self.visit_pre_order_(&mut f_stmt, &mut f_expr)
    }

    pub fn visit_stmt_pre_order<'a>(&'a self, mut f_stmt: impl FnMut(&'a Stmt) -> bool) {
        self.visit_pre_order_(&mut f_stmt, &mut |_| true)
    }

    pub fn visit_expr_pre_order<'a>(&'a self, mut f_expr: impl FnMut(&'a Expr) -> bool) {
        self.visit_pre_order_(&mut |_| true, &mut f_expr)
    }

    fn visit_pre_order_<'a>(&'a self, f_stmt: &mut dyn FnMut(&'a Stmt) -> bool, f_expr: &mut dyn FnMut(&'a Expr) -> bool) {
        // Pre-order traversal: always visit the current statement first.
        if !f_stmt(self) {
            // Abort traversal if visitor returns false.
            return;
        }

        // Then traverse into the expressions and recursive statements.
        use super::StmtKind::*;
        match &self.kind {
            // Non-recursive statements:
            Unreachable => {}
            Expr(expr) => {
                expr.visit_pre_order_(f_expr);
            }
            Assign { lhs: _, type_: _, rhs } => {
                rhs.visit_pre_order_(f_expr);
            }
            Store { op: _, addr, value } => {
                addr.visit_pre_order_(f_expr);
                value.visit_pre_order_(f_expr);
            }
            Br { target: _ } => {}

            // Recursive through `Body`:
            Block { body, end_label: _ } => {
                body.visit_pre_order_(f_stmt, f_expr);
            }
            Loop { begin_label: _, body } => {
                body.visit_pre_order_(f_stmt, f_expr);
            }
            If { condition, if_body, else_body } => {
                condition.visit_pre_order_(f_expr);
                if_body.visit_pre_order_(f_stmt, f_expr);
                if let Some(else_body) = else_body {
                    else_body.visit_pre_order_(f_stmt, f_expr);
                }
            }
            Switch { index, cases, default } => {
                index.visit_pre_order_(f_expr);
                for body in cases {
                    body.visit_pre_order_(f_stmt, f_expr);
                }
                default.visit_pre_order_(f_stmt, f_expr);
            }
        }
    }
}

impl Expr {
    pub fn visit_pre_order<'a>(&'a self, mut f: impl FnMut(&'a Expr) -> bool) {
        self.visit_pre_order_(&mut f)
    }

    // Because expressions don't contain statements, this only needs a visitor function for `Expr`s.
    fn visit_pre_order_<'a>(&'a self, f: &mut dyn FnMut(&'a Expr) -> bool) {
        if !f(self) {
            return;
        }

        use super::ExprKind::*;
        match &self.kind {
            VarRef(_) => {},
            Const(_) => {},
            Load { op: _, addr } => {
                addr.visit_pre_order_(f);
            },
            MemorySize => {},
            MemoryGrow { pages } => {
                pages.visit_pre_order_(f);
            },
            Unary(_, arg) => {
                arg.visit_pre_order_(f);
            },
            Binary(_, left, right) => {
                left.visit_pre_order_(f);
                right.visit_pre_order_(f);
            },
            Call { func: _, args } => {
                for arg in args {
                    arg.visit_pre_order_(f);
                }
            },
            CallIndirect { type_: _, table_idx, args } => {
                table_idx.visit_pre_order_(f);
                for arg in args {
                    arg.visit_pre_order_(f);
                }
            },
        }
    }

    // // TODO to take a generic transformer closure and return type T
    // fn convert<'a, T: From<&'a Expr> + Merge>(&self) -> T {
    //     if !f(self) {
    //         return;
    //     }

    //     use Expr::*;
    //     match self {
    //         VarRef(_) => {},
    //         Const(_) => {},
    //         Load { op: _, addr } => {
    //             addr.visit_pre_order_(f);
    //         },
    //         MemorySize => {},
    //         MemoryGrow { pages } => {
    //             pages.visit_pre_order_(f);
    //         },
    //         Unary(_, arg) => {
    //             arg.visit_pre_order_(f);
    //         },
    //         Binary(_, left, right) => {
    //             left.visit_pre_order_(f);
    //             right.visit_pre_order_(f);
    //         },
    //         Call { func: _, args } => {
    //             for arg in args {
    //                 arg.visit_pre_order_(f);
    //             }
    //         },
    //         CallIndirect { type_: _, table_idx, args } => {
    //             table_idx.visit_pre_order_(f);
    //             for arg in args {
    //                 arg.visit_pre_order_(f);
    //             }
    //         },
    //     }
    // }
}

#[test]
fn example_how_to_share_state_across_visitors() {
    use std::fmt::Write;

    let module = crate::Module::from_wasm_file("tests/wimplify_expected/block/block_nested.wasm").unwrap();

    // Must use RefCell because one cannot create two mutable references, one for each closure.
    // So instead borrow check at runtime by wrapping `output` in a `RefCell`.
    let output = std::cell::RefCell::new(String::new());
    for func in &module.functions {
        func.body.visit_pre_order(
            |x| {
                writeln!(output.borrow_mut(), "stmt:\n{x}\n").unwrap();
                true
            }, 
            |x| {
                writeln!(output.borrow_mut(), "expr:\n{x}\n").unwrap();
                true
            }
        );
    }
    println!("{}", output.into_inner())
}

#[test]
fn example_collect_constants() {
    let module = crate::Module::from_wasm_file("tests/wimplify_expected/block/block_nested.wasm").unwrap();

    println!("{}", module);

    // Because this state is only captured mutably in a single expression closure, this example
    // doesn't need `RefCell`.
    let mut all_constants = std::collections::BTreeSet::new();
    for func in &module.functions {
        func.body.visit_expr_pre_order(
            |expr| {
                if let super::ExprKind::Const(val) = &expr.kind {
                    all_constants.insert(*val);
                }
                true
            }
        );
    }
    println!("all constants in the module: {:?}", all_constants)
}
