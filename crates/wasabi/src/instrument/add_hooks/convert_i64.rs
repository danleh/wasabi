use wasabi_wasm::BinaryOp::I64ShrS;
use wasabi_wasm::Instr;
use wasabi_wasm::Instr::Binary;
use wasabi_wasm::Instr::Const;
use wasabi_wasm::Instr::Unary;
use wasabi_wasm::UnaryOp::I32WrapI64;
use wasabi_wasm::Val;
use wasabi_wasm::ValType;
use wasabi_wasm::ValType::I32;
use wasabi_wasm::ValType::I64;

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
pub fn convert_i64_instr(append_to: &mut Vec<Instr>, instr: Instr, ty: ValType) {
    match ty {
        I64 => append_to.extend_from_slice(&[
            instr.clone(),
            Unary(I32WrapI64), // low bits
            instr,
            Const(Val::I64(32)), // shift high bits to the right
            Binary(I64ShrS),
            Unary(I32WrapI64), // high bits
        ]),
        _ => append_to.push(instr),
    }
}
