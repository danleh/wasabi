use ast::{Val, ValType, ValType::*};
use ast::highlevel::{Instr, Instr::*, NumericOp::*};
use std::slice::from_ref;

pub fn convert_i64_type(ty: &ValType) -> &[ValType] {
    match ty {
        &I64 => &[I32, I32],
        ty => from_ref(ty),
    }
}

// instr is assumed to have no side-effects or influences on the stack (other than pushing one value)
// so that we can execute it safely twice (once for lower and higher bit half).
// ty is necessary when the type cannot be determined only from the instr, e.g., for GetLocal
pub fn convert_i64_instr(instr: Instr, ty: ValType) -> Vec<Instr> {
    match ty {
        I64 => vec![
            instr.clone(),
            Numeric(I32WrapI64), // low bits
            instr,
            Const(Val::I64(32)), // shift high bits to the right
            Numeric(I64ShrS),
            Numeric(I32WrapI64), // high bits
        ],
        _ => vec![instr],
    }
}
