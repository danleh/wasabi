//! Functions for iterating over Wimpl statements and expressions.

use super::{Body, Stmt, Expr};

impl Body {
    // A "facade", such that one doesn't have to borrow the closure arguments manually.
    pub fn visit_pre_order<'a>(&'a self, mut f_stmt: impl FnMut(&'a Stmt), mut f_expr: impl FnMut(&'a Expr)) {
        self.visit_pre_order_(&mut f_stmt, &mut f_expr)
    }

    pub fn visit_stmt_pre_order<'a>(&'a self, mut f_stmt: impl FnMut(&'a Stmt)) {
        self.visit_pre_order_(&mut f_stmt, &mut |_| ())
    }

    pub fn visit_expr_pre_order<'a>(&'a self, mut f_expr: impl FnMut(&'a Expr)) {
        self.visit_pre_order_(&mut |_| (), &mut f_expr)
    }

    fn visit_pre_order_<'a>(&'a self, f_stmt: &mut dyn FnMut(&'a Stmt), f_expr: &mut dyn FnMut(&'a Expr)) {
        for stmt in &self.0 {
            stmt.visit_pre_order_(f_stmt, f_expr)
        }
    }
}

impl Stmt {
    pub fn visit_pre_order<'a>(&'a self, mut f_stmt: impl FnMut(&'a Stmt), mut f_expr: impl FnMut(&'a Expr)) {
        self.visit_pre_order_(&mut f_stmt, &mut f_expr)
    }

    pub fn visit_stmt_pre_order<'a>(&'a self, mut f_stmt: impl FnMut(&'a Stmt)) {
        self.visit_pre_order_(&mut f_stmt, &mut |_| ())
    }

    pub fn visit_expr_pre_order<'a>(&'a self, mut f_expr: impl FnMut(&'a Expr)) {
        self.visit_pre_order_(&mut |_| (), &mut f_expr)
    }

    fn visit_pre_order_<'a>(&'a self, f_stmt: &mut dyn FnMut(&'a Stmt), f_expr: &mut dyn FnMut(&'a Expr)) {
        // Pre-order traversal: always visit the current statement first.
        f_stmt(self);

        // Then traverse into the expressions and recursive statements.
        use Stmt::*;
        match self {
            // Non-recursive statements:
            Unreachable => {}
            Expr(expr) => {
                expr.visit_pre_order_(f_expr);
            }
            Assign { lhs: _, type_: _, rhs } => {
                rhs.visit_pre_order_(f_expr);
            }
            Store { op: _, memarg: _, addr, value } => {
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
    pub fn visit_pre_order<'a>(&'a self, mut f: impl FnMut(&'a Expr)) {
        self.visit_pre_order_(&mut f)
    }

    // Because expressions don't contain statements, this only needs a visitor function for `Expr`s.
    fn visit_pre_order_<'a>(&'a self, f: &mut dyn FnMut(&'a Expr)) {
        f(self);

        use Expr::*;
        match self {
            VarRef(_) => {},
            Const(_) => {},
            Load { op: _, memarg: _, addr } => {
                addr.visit_pre_order_(f);
            },
            MemorySize => {},
            MemoryGrow { pages } => {
                pages.visit_pre_order_(f);
            },
            Numeric { op: _, args } => {
                for arg in args {
                    arg.visit_pre_order_(f);
                }
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
}

#[test]
fn example_how_to_share_state_across_visitors() {
    use std::fmt::Write;

    let module = crate::wimpl::Module::from_wasm_file("tests/wimpl/wimplify_expected/block/block_nested.wasm").unwrap();

    // Must use RefCell because one cannot create two mutable references, one for each closure.
    // So instead borrow check at runtime.
    let output = std::cell::RefCell::new(String::new());
    for func in &module.functions {
        func.body.visit_pre_order(
            |x| writeln!(output.borrow_mut(), "stmt: {x}\n").unwrap(), 
            |x| writeln!(output.borrow_mut(), "expr: {x}\n").unwrap()
        );
    }
    println!("{}", output.into_inner())
}

#[test]
fn example_collect_constants() {
    let module = crate::wimpl::Module::from_wasm_file("tests/wimpl/wimplify_expected/block/block_nested.wasm").unwrap();

    println!("{}", module);

    // Because this state is only captured mutably in the expression closure, this doesn't need
    // `RefCell`.
    let mut all_constants = std::collections::BTreeSet::new();
    for func in &module.functions {
        func.body.visit_pre_order(
            |_| (), 
            |expr| if let Expr::Const(val) = expr {
                all_constants.insert(*val);
            }
        );
    }
    println!("{:?}", all_constants)
}
