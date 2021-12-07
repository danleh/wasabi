use crate::{
    highlevel::{Function, Instr, LocalOp, Module},
    FunctionType, ValType,
};

/// Standard type-checking for WebAssembly instruction sequences, as described
/// with a reference algorithm in the specification:
/// https://webassembly.github.io/spec/core/appendix/algorithm.html#algo-valid

// TODO track height

#[derive(Debug)]
pub struct TypeChecker {
    /// Option<ValType> as a replacement for Unknown types, so `None` == Unknown.
    value_stack: Vec<Option<ValType>>,
    control_stack: Vec<ControlFrame>,
}

// TODO error type

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            value_stack: Vec::new(),
            control_stack: Vec::new(),
        }
    }

    /*
     * Value stack operations:
     */

    pub fn push_val(&mut self, type_: Option<ValType>) {
        self.value_stack.push(type_)
    }

    #[must_use]
    pub fn pop_val(&mut self) -> Result<Option<ValType>, String> {
        let frame = self.top_ctrl()?;
        let height = frame.height;
        let unreachable = frame.unreachable;
        // TODO(Daniel): I don't really understand why this is necessary: understand and document.
        if self.value_stack.len() == height  {
            if unreachable {
                return Ok(None);
            } else {
                return Err("value stack underflow".into());
            }
        }
        self.value_stack
            .pop()
            .ok_or("expected a value, but value stack was empty".into())
    }

    #[must_use]
    pub fn pop_val_expected(
        &mut self,
        expected: Option<ValType>,
    ) -> Result<Option<ValType>, String> {
        let actual = self.pop_val()?;
        match (actual, expected) {
            (Some(actual), Some(expected)) if actual == expected => Ok(Some(actual)),
            (Some(actual), Some(expected)) => Err(format!(
                "expected type {:?}, but got {:?}",
                expected, actual
            )),
            _ => Ok(actual),
        }
    }

    pub fn push_vals(&mut self, types: &[ValType]) {
        for ty in types {
            self.value_stack.push(Some(*ty));
        }
    }

    #[must_use]
    pub fn pop_vals_expected(&mut self, expected: &[ValType]) -> Result<Vec<Option<ValType>>, String> {
        expected
            .iter()
            // The expected types must be checked in reverse order...
            .rev()
            .map(|expected| self.pop_val_expected(Some(*expected)))
            // ...but returned in the correct order, so reverse again.
            .rev()
            // Goes from `Iter<Result<...>>` to `Result<Vec<...>>`.
            .collect()
    }

    /*
     * Control stack operations:
     */

    pub fn push_ctrl(
        &mut self,
        instr: &Instr,
        inputs: Vec<ValType>,
        results: Vec<ValType>,
    ) {
        self.control_stack.push(ControlFrame {
            block_instr: match instr {
                Instr::Block(_) => BlockInstr::Block,
                Instr::If(_) => BlockInstr::If,
                Instr::Loop(_) => BlockInstr::Loop,
                _ => panic!("non-control instruction {:?}", instr),
            },
            inputs: inputs.clone(),
            results,
            height: self.value_stack.len(),
            unreachable: false,
        });
        self.push_vals(&inputs);
    }

    #[must_use]
    pub fn top_ctrl(&mut self) -> Result<&mut ControlFrame, String> {
        self.control_stack.last_mut().ok_or("empty control stack".to_string())
    }

    #[must_use]
    pub fn pop_ctrl(&mut self) -> Result<ControlFrame, String> {
        let frame = self.top_ctrl()?;
        let results = frame.results.clone();
        let height = frame.height;
        self.pop_vals_expected(&results)?;
        if self.value_stack.len() != height {
            return Err("value stack underflow".into());
        }
        Ok(self.control_stack.pop().expect("already checked with top_ctrl()"))
    }

    pub fn label_types(&self, frame: &ControlFrame) -> Vec<ValType> {
        if frame.block_instr == BlockInstr::Loop {
            frame.inputs.clone()
        } else {
            frame.results.clone()
        }
    }

    #[must_use]
    pub fn unreachable(&mut self) -> Result<(), String> {
        let new_stack_height = self.top_ctrl()?.height;
        // Drop all values in the current block from the stack.
        self.value_stack.resize_with(new_stack_height, || panic!("stack underflow"));
        self.top_ctrl()?.unreachable = true;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ControlFrame {
    block_instr: BlockInstr,
    
    // Input and result types of the block.
    inputs: Vec<ValType>,
    results: Vec<ValType>,

    // Height of the value stack at the start of the block, used to check that
    // operands do not underflow the current block.
    height: usize,
    
    // Flag recording whether the remainder of the block is unreachable, used to
    // handle stack-polymorphic typing after branches.
    unreachable: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum BlockInstr {
    Block,
    If,
    Loop,
}

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
pub fn types(
    instrs: &[Instr],
    function: &Function,
    module: &Module,
) -> Result<Vec<FunctionType>, String> {
    let mut types = Vec::with_capacity(instrs.len());
    for instr in instrs {
        use Instr::*;
        let ty = match (instr, instr.to_type()) {
            (Unreachable, _) => todo!(), // FIXME

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
            }

            (Call(idx), _) => module.function(*idx).type_.clone(),

            // Value-polymorphic:
            // TODO use type stack
            (Drop, _) => todo!(),
            (Select, _) => todo!(),

            // All of those are stack-polymorphic:
            (Block(_), _) => todo!(),
            (Loop(_), _) => todo!(),
            (If(_), _) => todo!(),
            (Else, _) => todo!(),
            (End, _) => todo!(),
            (Br(_), _) => todo!(),
            (BrIf(_), _) => todo!(),
            (BrTable { table, default }, _) => todo!(),
            (Return, _) => todo!(),

            (instr, None) => unreachable!(
                "instruction {:?} with non-primitive type not handled",
                instr
            ),
        };
        types.push(ty);
    }
    Ok(types)
}
