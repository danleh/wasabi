use wasm::{Val, ValType, ValType::I32, ValType::I64};
use wasm::highlevel::{
    Instr, Instr::Const, Instr::Numeric, NumericOp::I32WrapI64, NumericOp::I64ShrS,
};

/*
 * Helper functions for turning i64's into two i32's so that we can pass them to JavaScript
 */

pub fn convert_i64_type(ty: &ValType) -> &[ValType] {
    match ty {
        &I64 => &[I32, I32],
        ty => std::slice::from_ref(ty),
    }
}

/// instr is assumed to have no side-effects or influences on the stack (other than pushing one value)
/// so that we can execute it safely twice (once for lower and higher bit half).
/// ty is necessary because for some instructions, the type cannot be determined but needs external information, e.g., for LocalGet
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
