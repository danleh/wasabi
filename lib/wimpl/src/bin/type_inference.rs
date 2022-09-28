use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use rustc_hash::FxHashMap;

use wasm::{ValType, FunctionType, MemoryOp, BinaryOp, UnaryOp, LoadOp, StoreOp};
use wimpl::{self, analyze::{param_exprs, collect_call_indirect_args, collect_call_indirect_idx_expr, print_map_count}, ExprKind, Function, Module, StmtKind, Var, FunctionId, Expr, traverse::VisitOptionBodyExt};

use TypeConstraint::*;
use ValType::*;

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("{:?}", args);

    let wasm_path = &args[1];
    let module = wimpl::Module::from_wasm_file(wasm_path).unwrap();

    println!("total function: {}", module.functions.len());
    
    println!("most frequent call_indirect expressions:");
    print_map_count(&collect_call_indirect_idx_expr(&module));

    let mut call_indirect_args: BTreeMap<FunctionType, BTreeMap<Vec<String>, usize>> = BTreeMap::default();

    // let mut var_constraints = BTreeSet::default();

    for function in &module.functions {
        // println!("{}:", function.name);

        for (func_ty, args) in collect_call_indirect_args(function) {
            let args_count_map = call_indirect_args.entry(func_ty).or_default();
            for (args, count) in args {
                *args_count_map.entry(args).or_default() += count;
            }
        }

        // var_constraints.extend(collect_var_constraints(&module, function).0);

        // let var_constraints = collect_var_constraints(&module, function);
        // for (var, constraint) in var_constraints.0 {
        //     let type_ = match var {
        //         Var::Local(_) => continue,
        //         Var::Global(i) => module.globals[i as usize].type_.0,
        //         Var::Stack(_) => continue,
        //         Var::Param(i) => function.type_.inputs()[i as usize],
        //         Var::BlockResult(_) => continue,
        //         Var::Return(_) => continue,
        //     };
        //     println!("  {}:", var);
        //     println!("    {} (WebAssembly type)", type_);
        //     for constraint in constraint {
        //         println!("    {}", constraint);
        //     }
        // }
    }

    for (func_ty, args) in call_indirect_args {
        println!("{}: ({} potential targets by type)", func_ty, module.functions.iter().filter(|f| f.type_ == func_ty).count());
        let mut args = args.into_iter().collect::<Vec<_>>();
        args.sort_by_key(|(args, count)| (std::cmp::Reverse(*count), args.clone()));
        for (args, count) in args {
            println!("  {:6} × {}", count, args.join(", "));
        }
    }

    // for (func, var, constraint) in var_constraints {
    //     println!("{}.{} <: {}", func, var, constraint);
    // }
}

#[derive(Debug, Clone, Default)]
// pub struct ConstraintMap(BTreeMap<Var, BTreeSet<TypeConstraint>>);
// pub struct ConstraintMap(BTreeMap<Var, TypeConstraint>);
pub struct ConstraintMap(BTreeSet<(FunctionId, Var, TypeConstraint)>);

impl ConstraintMap {
    pub fn add(&mut self, function: &Function, var: Var, constraint: TypeConstraint) {
        if constraint == Any {
            return;
        }
        self.0.insert((function.name.clone(), var, constraint));
        // self.0.entry(var).or_default().insert(constraint);
        // let in_map = self.0.entry(var).or_default();
        // *in_map = in_map.meets(&constraint).expect(&format!("incompatible constraints for var {}!? {} and {}", var, in_map, constraint));
    }
}

pub fn collect_var_constraints(module: &Module, function: &Function) -> ConstraintMap {
    let mut constraints = ConstraintMap::default();

    // TODO this should also RETURN a constraint for the current expression RESULT.
    fn collect_var_constraints(
        expr: &Expr,
        constraint: TypeConstraint,
        constraints: &mut ConstraintMap,
        module: &Module,
        function: &Function,
    ) {
        use ExprKind::*;
        use UnaryOp::*;
        use BinaryOp::*;
        match &expr.kind {
            VarRef(var) => {
                constraints.add(function, *var, constraint);
            }
            Const(_) => {}
            Load { op, addr } => {
                let value_ty = op.to_type().results()[0];
                // let constraint = constraint.meets(&Base(value_ty)).expect(&format!("illegal incoming constraint for load, incoming: {}, value type: {}", constraint, value_ty));
                collect_var_constraints(addr, ptr(constraint), constraints, module, function);
                collect_var_constraints(addr, ptr(Base(value_ty)), constraints, module, function);
            }
            MemorySize => {}
            MemoryGrow { pages } => {
                // TODO
                // assert_eq!(constraint.join(&Base(I32)), Base(I32), "invalid constraint {} for memory.grow", constraint);
                // Pass on i32 as the constraint since its the more specific and the assert just
                // checked that its compatible with the incoming constraint.
                collect_var_constraints(pages, Base(I32), constraints, module, function);
            }
            // Unary(op, arg) if op.to_type().inputs()[0] != I32 => {
            //     // TODO e.g. eqz
            //     collect_var_constraints(arg, constraint, constraints, module);
            // }
            Unary(_, _) => {
                // TODO nothing
            }
            Binary(I32Add, left, right) => {
                // FIXME is this even correct?
                collect_var_constraints(left, constraint.clone(), constraints, module, function);
                collect_var_constraints(right, constraint, constraints, module, function);
            }
            Binary(op, left, right) => {
                // TODO handle more other pointer operations
            }
            Call { func, args } => {
                let func_ty = module
                    .function(func.clone())
                    .expect("function not found")
                    .type_;
                // Add constraints for the called function's parameters.
                for (i, (arg, ty)) in args.iter().zip(func_ty.inputs().iter().copied()).enumerate() {
                    collect_var_constraints(arg, Var(func.clone(), wimpl::Var::Param(i as u32)), constraints, module, function);
                }
            }
            CallIndirect {
                type_,
                table_idx,
                args,
            } => {
                collect_var_constraints(table_idx, FuncPtr(*type_), constraints, module, function);
                for (arg, ty) in args.iter().zip(type_.inputs().iter().copied()) {
                    collect_var_constraints(arg, Base(ty), constraints, module, function);
                }
            }
        }
    }
    // Closure over constraints, such that we don't have to pass that argument always around.
    let mut collect_var_constraints =
        |expr, constraint| collect_var_constraints(expr, constraint, &mut constraints, module, function);

    function.body.visit_stmt_pre_order(|stmt| {
        use StmtKind::*;
        match &stmt.kind {
            Unreachable => {}
            Expr(expr) => collect_var_constraints(expr, Any),
            Assign { lhs, type_, rhs } => {
                // TODO should we add another constraint for lhs?
                collect_var_constraints(rhs, Var(function.name(), *lhs));
            }
            Store { op, addr, value } => {
                let value_ty = op.to_type().inputs()[1];
                collect_var_constraints(addr, ptr(Base(value_ty)));
                // TODO get value type from store op.
                collect_var_constraints(value, Base(value_ty));
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
    FuncPtr(FunctionType),
    Var(FunctionId, Var),
    // TODO a single Ptr "capability" (cf. Polymorphic Type Inference for Machine Code) requires
    // that ptrs are always unified, i.e. non-directional type inference!
    // TODO We would need Load/Store capabilities, where Store(T) -> (implies) Load(T).
    Ptr(Box<TypeConstraint>),
    // TODO I don't think bool actually helps a lot, because it doesn't require 0 or 1, any i32 works...
    // Bool,
    // NonPtr, // an i32 that is certainly NOT a pointer
}

impl Default for TypeConstraint {
    fn default() -> Self {
        Any
    }
}

pub fn ptr(t: TypeConstraint) -> TypeConstraint {
    Ptr(Box::new(t))
}

impl fmt::Display for TypeConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Any => f.write_str("any"),
            Base(ty) => ty.fmt(f),
            Var(func, var) => write!(f, "{}.{}", func, var),
            // Bool => f.write_str("bool"),
            FuncPtr(func_ty) => write!(f, "func_ptr({})", func_ty),
            Ptr(ty) => write!(f, "ptr({})", ty),
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
            (Var(..), _) | (_, Var(..)) => todo!(),

            // Everything is more specific than t.
            (t, Any) | (Any, t) => Some(t.clone()),

            // Non-i32 types cannot be compared to anything but themselves and Any.
            (Base(I64 | F32 | F64), _) | (_, Base(I64 | F32 | F64)) => None,

            // Func (technically: indices into the table) is a subtype of i32.
            (func_ptr @ FuncPtr(_), Base(I32)) | (Base(I32), func_ptr @ FuncPtr(_)) => Some(func_ptr.clone()),
            (FuncPtr(_), _) | (_, FuncPtr(_)) => None,

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
