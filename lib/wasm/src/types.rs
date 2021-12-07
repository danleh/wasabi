use crate::{highlevel::{Instr, LocalOp, Function, Module}, FunctionType};

// TODO return iterator of (instruction, type) instead of owning Vec.
// TODO take iterator of instrs instead of Vec.
// TODO proper error type, not String.
// See https://webassembly.github.io/spec/core/valid/instructions.html for a
// good overview of the typing rules for individual instructions and sequences
// of instructions.
// It also introduces the two types of polymorphism in WebAssembly:
// - value-polymorphic: handled by Wasabi's type stack type.
// - stack-polymorphic: handled in Wasabi with the unreachable_depth, which
// is unfortunately NOT enough for here :( FIXME
// For a good implementation, see the validation algorithm description in the
// spec: https://webassembly.github.io/spec/core/appendix/algorithm.html#algo-valid
// TODO combine all of the above in an iterater adapter of:
//   Iter<Instr>, &Function, &Module -> Iter<(Instr, Type)>, which keeps state s
// as described in the reference algorithm below, namely:
//   value and control stack.
pub fn types(instrs: &[Instr], function: &Function, module: &Module) -> Result<Vec<FunctionType>, String> {
    let mut types = Vec::with_capacity(instrs.len());
    for instr in instrs {
        use Instr::*;
        let ty = match (instr, instr.to_type()) {
            // For all those where we know the type already from the instruction
            // alone.
            // FIXME includes `Unreachable` even though it is stack-polymorphic.
            (_, Some(ty)) => ty,

            (Local(op, idx), _) => {
                let local_ty = function.param_or_local_type(*idx);
                let op_ty = op.to_type(local_ty);
                // type_stack.instr(&op_ty);
                op_ty
            }
            (Global(op, idx), _) => {
                let global_ty = module.global(*idx);
                let op_ty = op.to_type(global_ty.type_.0);
                op_ty
            },

            (Call(idx), _) => module.function(*idx).type_.clone(),

            // Value-polymorphic:
            // TODO use type stack
            (Drop, _) => todo!(),
            (Select, _) => todo!(),

            // All of those are stack-polymorphic:
            (Unreachable, _) => todo!(),
            (Block(_), _) => todo!(),
            (Loop(_), _) => todo!(),
            (If(_), _) => todo!(),
            (Else, _) => todo!(),
            (End, _) => todo!(),
            (Br(_), _) => todo!(),
            (BrIf(_), _) => todo!(),
            (BrTable { table, default }, _) => todo!(),
            (Return, _) => todo!(),

            (instr, ty) => unreachable!("instruction {:?} with non-primitive type {:?} not handled", instr, ty),
        };
        types.push(ty);
    }
    Ok(types)
}