use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use wasm::{
    wimpl::{self, analyze::param_exprs, Expr, Function, Module, Stmt, Var},
    ValType,
};

use TypeConstraint::*;
use ValType::*;

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("{:?}", args);

    let wasm_path = &args[1];
    let module = wimpl::Module::from_wasm_file(wasm_path).unwrap();

    for function in &module.functions {
        println!("{}:", function.name);
        let var_constraints = collect_var_constraints(&module, function);
        for (var, constraints) in var_constraints.0 {
            let type_ = match var {
                Var::Local(_) => continue,
                Var::Global(i) => module.globals[i as usize].type_.0,
                Var::Stack(_) => continue,
                Var::Param(i) => function.type_.inputs()[i as usize],
                Var::BlockResult(_) => continue,
                Var::Return(_) => continue,
            };
            println!("  {}: {} (WebAssembly type), constraints:", var, type_);
            for constraint in constraints {
                println!("   {}", constraint);
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConstraintMap(BTreeMap<Var, BTreeSet<TypeConstraint>>);

impl ConstraintMap {
    pub fn add(&mut self, var: Var, constraint: TypeConstraint) {
        if constraint != Any {
            self.0.entry(var).or_default().insert(constraint);
        }
    }
}

pub fn collect_var_constraints(module: &Module, function: &Function) -> ConstraintMap {
    let mut constraints = ConstraintMap::default();

    fn collect_var_constraints(
        expr: &Expr,
        constraint: TypeConstraint,
        constraints: &mut ConstraintMap,
        module: &Module,
    ) {
        use Expr::*;
        match expr {
            VarRef(var) => {
                constraints.add(*var, constraint);
            }
            Const(_) => {}
            Load { op, addr } => {
                // TODO get value type from op.
                collect_var_constraints(addr, ptr(constraint), constraints, module);
            }
            MemorySize => {}
            MemoryGrow { pages } => {
                // TODO
                // assert_eq!(constraint.join(&Base(I32)), Base(I32), "invalid constraint {} for memory.grow", constraint);
                // Pass on i32 as the constraint since its the more specific and the assert just
                // checked that its compatible with the incoming constraint.
                collect_var_constraints(pages, Base(I32), constraints, module);
            }
            Unary(op, _) => {
                // TODO e.g. eqz
            }
            Binary(op, _, _) => {
                // TODO add for offsets
            }
            Call { func, args } => {
                // TODO add constraints for the called function's parameters
                let func_ty = module
                    .function(func.clone())
                    .expect("function not found")
                    .type_;
                for (arg, ty) in args.iter().zip(func_ty.inputs().iter().copied()) {
                    collect_var_constraints(arg, Base(ty), constraints, module);
                }
            }
            CallIndirect {
                type_,
                table_idx,
                args,
            } => {
                collect_var_constraints(table_idx, FuncPtr, constraints, module);
                for (arg, ty) in args.iter().zip(type_.inputs().iter().copied()) {
                    collect_var_constraints(arg, Base(ty), constraints, module);
                }
            }
        }
    }
    // Closure over constraints, such that we don't have to pass that argument always around.
    let mut collect_var_constraints =
        |expr, constraint| collect_var_constraints(expr, constraint, &mut constraints, module);

    function.body.visit_stmt_pre_order(|stmt| {
        use Stmt::*;
        match stmt {
            Unreachable => {}
            Expr(expr) => collect_var_constraints(expr, Any),
            Assign { lhs, type_, rhs } => {
                // TODO should we add another constraint for lhs?
                collect_var_constraints(rhs, Base(*type_));
            }
            Store { op, addr, value } => {
                collect_var_constraints(addr, ptr(Any));
                // TODO get value type from store op.
                collect_var_constraints(value, Any);
            }
            Br { target: _ } => {}
            Block {
                body: _,
                end_label: _,
            } => {}
            Loop {
                begin_label: _,
                body: _,
            } => {}

            If {
                condition,
                if_body: _,
                else_body: _,
            } => {
                // collect_var_constraints(condition, Bool);
                collect_var_constraints(condition, Base(I32));
            }
            Switch {
                index,
                cases: _,
                default: _,
            } => {
                collect_var_constraints(index, Base(I32));
            }
        }

        // Go into all recursively nested statements as well (e.g., in block or loop).
        true
    });

    constraints
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
// WARNING The derived PartialOrd and Ord are not semantically for the lattice structure, but only
// for this datastructure representation, printing etc.
#[derive(PartialOrd, Ord)]
pub enum TypeConstraint {
    Any,
    Base(ValType),
    // TODO I don't think bool actually helps a lot, because it doesn't require 0 or 1, any i32 works...
    // Bool,
    FuncPtr,
    Ptr(Box<TypeConstraint>),
}

pub fn ptr(t: TypeConstraint) -> TypeConstraint {
    Ptr(Box::new(t))
}

impl fmt::Display for TypeConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Any => f.write_str("any"),
            Base(ty) => ty.fmt(f),
            // Bool => f.write_str("bool"),
            FuncPtr => f.write_str("func_ptr"),
            Ptr(t) => write!(f, "ptr({})", t),
        }
    }
}

// impl PartialOrd for TypeConstraint {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         use std::cmp::Ordering::*;
//         match (self, other) {
//             _ if self == other => Some(Equal),

//             // Any can be compared to everything and is always the largest of the two.
//             (_, Any) => Some(Less),
//             (Any, _) => Some(Greater),

//             // Non-i32 types cannot be compared to anything but themselves and Any.
//             (Base(I64 | F32 | F64), _) | (_, Base(I64 | F32 | F64)) => None,

//             // Bool is a subtype of i32, but incomparable to anything else.
//             (Bool, Base(I32)) => Some(Less),
//             (Base(I32), Bool) => Some(Greater),
//             (Bool, _) | (_, Bool) => None,

//             // Func (technically: indices into the table) is a subtype of i32.
//             (FuncPtr, Base(I32)) => Some(Less),
//             (Base(I32), FuncPtr) => Some(Greater),
//             (FuncPtr, _) | (_, FuncPtr) => None,

//             // Pointer is a subtype of i32.
//             (Ptr(_), Base(I32)) => Some(Less),
//             (Base(I32), Ptr(_)) => Some(Greater),
//             // Recursively compare the pointee types.
//             // TODO is this right? is ptr(i64) not comparable with ptr(i32)?
//             (Ptr(a), Ptr(b)) => a.partial_cmp(b),

//             (t1, t2) => unreachable!("{} and {} should be covered above", t1, t2),
//         }
//     }
// }

impl TypeConstraint {
    // Return the more specific of the two types or None (if the types are incompatible)
    pub fn meets(&self, other: &TypeConstraint) -> Option<TypeConstraint> {
        match (self, other) {
            _ if self == other => Some(self.clone()),

            // Everything is more specific than t.
            (t, Any) | (Any, t) => Some(t.clone()),

            // Non-i32 types cannot be compared to anything but themselves and Any.
            (Base(I64 | F32 | F64), _) | (_, Base(I64 | F32 | F64)) => None,

            // Func (technically: indices into the table) is a subtype of i32.
            (FuncPtr, Base(I32)) | (Base(I32), FuncPtr) => Some(FuncPtr),
            (FuncPtr, _) | (_, FuncPtr) => None,

            // Pointer is a subtype of i32.
            (t @ Ptr(_), Base(I32)) | (Base(I32), t @ Ptr(_)) => Some(t.clone()),
            // Recursively compare the pointee types.
            // TODO is this right? is ptr(i64) not comparable with ptr(i32)?
            (Ptr(a), Ptr(b)) => a.meets(b).map(ptr),

            (t1, t2) => unreachable!("{} and {} should be covered above", t1, t2),
        }
    }

    /// The least upper bound of the two type constraints, i.e., the least general type that still covers both.
    pub fn join(&self, other: &TypeConstraint) -> TypeConstraint {
        //     use ValType::*;
        //     use TypeConstraint::*;
        //     use std::cmp::Ordering::*;
        //     match (self, other) {
        //         _ if self == other => self.clone(),
        //         (Any, _) | (_, Any) => Any,

        //         // Unequal base types are not compatible.

        //     }
        todo!()
    }
}
