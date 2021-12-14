use std::{path::PathBuf, fmt};

use crate::{
    highlevel::{Function, Instr, LocalOp, Module},
    FunctionType, Idx, Label, ValType,
};

/// Strongly inspired by the official type-checking algorithm, as described
/// in the specification:
/// https://webassembly.github.io/spec/core/appendix/algorithm.html#algo-valid
/// Except that we don't care about stack height validation, because we assume
/// that only valid modules can be analyzed; and that we produce types for each
/// instruction, instead of only checking if they are valid (see `InstructionType`).

#[derive(Debug)]
pub struct TypeChecker {
    /// Option<ValType> as a replacement for Unknown types, so `None` == Unknown.
    value_stack: Vec<Option<ValType>>,
    pub /* FIXME */ control_stack: Vec<ControlFrame>,
}

#[derive(Debug)]
pub struct Error {
    pub msg: String,
    pub file: Option<PathBuf>,
    pub function: Option<Idx<Function>>,
    pub instruction: Option<(usize, Instr)>,
}

impl Error {
    pub fn new(msg: impl Into<String>) -> Self {
        Error {
            msg: msg.into(),
            file: None,
            function: None,
            instruction: None,
        }
    }
}

// TODO data gathering:
// for our dataset: how many functions without memory loads/stores
// how many functions with call_indirect?

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            value_stack: Vec::new(),
            control_stack: Vec::new(),
        }
    }

    pub fn val_stack_str(&self) -> String {
        format!("[{}]", self.value_stack.iter().map(val_type_unknown).collect::<Vec<_>>().join(", "))
    }

    /*
     * Value stack operations:
     */

    pub fn push_val(&mut self, type_: Option<ValType>) {
        self.value_stack.push(type_)
    }

    #[must_use]
    pub fn pop_val(&mut self) -> Result<Option<ValType>, Error> {
        let frame = self.top_ctrl()?;
        let height = frame.height;
        let unreachable = frame.unreachable;
        // TODO(Daniel): I don't really understand why this is necessary: understand and document.
        // if self.value_stack.len() == height {
            if unreachable {
                return Ok(None);
            }
        //     else {
        //         return Err(Error::new("value stack underflow"));
        //     }
        // }
        self.value_stack
            .pop()
            .ok_or(Error::new("expected a value, but value stack was empty"))
    }

    #[must_use]
    pub fn pop_val_expected(
        &mut self,
        expected: Option<ValType>,
    ) -> Result<Option<ValType>, Error> {
        let actual = self.pop_val()?;
        // println!("actual: {:?}, expected: {:?}", actual, expected);
        match (actual, expected) {
            (Some(actual), Some(expected)) if actual == expected => Ok(Some(actual)),
            (Some(actual), Some(expected)) => Err(Error::new(format!(
                "expected type {:?}, but got {:?}",
                expected, actual
            ))),
            _ => Ok(actual),
        }
    }

    pub fn push_vals(&mut self, types: &[ValType]) {
        for ty in types {
            self.value_stack.push(Some(*ty));
        }
    }

    #[must_use]
    pub fn pop_vals_expected(
        &mut self,
        expected: &[ValType],
    ) -> Result<Vec<Option<ValType>>, Error> {
        // println!("expected: {:?}", expected);
        // println!("actual: {:?}", self.value_stack);
        let actual: Result<Vec<_>, _> = expected
            .iter()
            // The expected types must be checked in reverse order...
            .rev()
            .map(|expected| self.pop_val_expected(Some(*expected)))
            .collect();
        let mut actual = actual?;
        actual.reverse();
        Ok(actual)
    }

    /*
     * Control stack operations:
     */
    pub fn push_ctrl_func(&mut self, results: Vec<ValType>) {
        self.control_stack.push(ControlFrame {
            block_instr: BlockInstr::Function,
            inputs: Vec::new(),
            results,
            height: 0,
            unreachable: false,
        })
    }

    pub fn push_ctrl(&mut self, instr: &Instr, inputs: Vec<ValType>, results: Vec<ValType>) {
        self.control_stack.push(ControlFrame {
            block_instr: match instr {
                Instr::Block(_) => BlockInstr::Block,
                Instr::If(_) => BlockInstr::If,
                Instr::Else => BlockInstr::Else,
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
    pub fn top_ctrl(&mut self) -> Result<&mut ControlFrame, Error> {
        self.control_stack
            .last_mut()
            .ok_or(Error::new("empty control stack"))
    }

    #[must_use]
    pub fn get_ctrl(&self, label: Label) -> Result<&ControlFrame, Error> {
        self.control_stack
            .iter()
            .rev()
            .nth(label.0 as usize)
            .ok_or(Error::new(format!(
                "invalid control stack label {}",
                label.0
            )))
    }

    #[must_use]
    pub fn pop_ctrl(&mut self) -> Result<ControlFrame, Error> {
        let frame = self.top_ctrl()?;
        let results = frame.results.clone();
        let height = frame.height;
        self.pop_vals_expected(&results)?;
        // if self.value_stack.len() != height {
        //     return Err(Error::new("value stack underflow"));
        // }
        Ok(self
            .control_stack
            .pop()
            .expect("already checked with top_ctrl()"))
    }

    pub fn label_types(&self, frame: &ControlFrame) -> Vec<ValType> {
        if frame.block_instr == BlockInstr::Loop {
            frame.inputs.clone()
        } else {
            frame.results.clone()
        }
    }

    #[must_use]
    pub fn unreachable(&mut self) -> Result<(), Error> {
        let new_stack_height = self.top_ctrl()?.height;
        // Drop all values in the current block from the stack.
        self.value_stack
            .resize_with(new_stack_height, || panic!("stack underflow"));
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
    Function,
    Block,
    If,
    Else,
    Loop,
}

#[derive(Debug)]
pub struct InstructionType {
    inputs: Vec<Option<ValType>>,
    results: Vec<Option<ValType>>,
}

impl InstructionType {
    pub fn new() -> Self {
        InstructionType {
            inputs: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn lift(func_ty: &FunctionType) -> Self {
        InstructionType {
            inputs: func_ty.params.iter().map(|ty| Some(*ty)).collect(),
            results: func_ty.results.iter().map(|ty| Some(*ty)).collect(),
        }
    }
}

fn val_type_unknown(ty: &Option<ValType>) -> &'static str {
    match ty {
        Some(ValType::I32) => "i32",
        Some(ValType::I64) => "i64",
        Some(ValType::F32) => "f32",
        Some(ValType::F64) => "f64",
        None => "?",
    }
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] -> [{}]", 
            self.inputs.iter().map(val_type_unknown).collect::<Vec<_>>().join(", "),
            self.results.iter().map(val_type_unknown).collect::<Vec<_>>().join(", "),
        )
    }
}

impl From<FunctionType> for InstructionType {
    fn from(func_ty: FunctionType) -> Self {
        InstructionType::lift(&func_ty)
    }
}

#[must_use]
fn merge_unknown_types(
    ty1: Option<ValType>,
    ty2: Option<ValType>,
) -> Result<Option<ValType>, Error> {
    match (ty1, ty2) {
        (None, None) => Ok(None),
        (None, Some(ty)) | (Some(ty), None) => Ok(Some(ty)),
        (Some(ty1), Some(ty2)) => {
            if ty1 == ty2 {
                Ok(Some(ty1))
            } else {
                Err(Error::new(format!(
                    "cannot merge incompatible types {} and {}",
                    ty1, ty2
                )))
            }
        }
    }
}

pub fn check_module(module: &Module) -> Result<(), Error> {
    for (func_idx, func) in module.functions() {
        if let Some(code) = func.code() {
            println!("  func ${}  {}", func_idx.into_inner(), func.type_);
            types(&code.body, func, &module).map_err(|mut err| {
                err.function = Some(func_idx);
                err
            })?;
        }
    }
    Ok(())
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
) -> Result<Vec<InstructionType>, Error> {
    let mut types = Vec::with_capacity(instrs.len());
    let mut state = TypeChecker::new();
    state.push_ctrl_func(function.type_.results.iter().cloned().collect());
    for (instr_idx, instr) in instrs.iter().enumerate() {
        use Instr::*;
        let ty = || -> Result<InstructionType, Error> {
            Ok(match (instr, instr.to_type()) {
                (Unreachable, Some(ty)) => {
                    state.unreachable()?;
                    ty.into()
                }

                // For all those where we know the type already from the instruction
                // alone.
                // FIXME remove unreachable from Instruction::to_type() function
                // TODO rename Instruction::to_type() to `simple_type` or so.
                // TODO then move Unreachable case down
                (_, Some(ty)) => {
                    state.pop_vals_expected(&ty.params)?;
                    state.push_vals(&ty.results);
                    ty.into()
                }

                /* Cases which are still monomorphic, but where we need context
                 * information for the type.
                 */
                (Local(op, idx), _) => {
                    let local_ty = function.param_or_local_type(*idx);
                    let op_ty = op.to_type(local_ty);
                    state.pop_vals_expected(&op_ty.params)?;
                    state.push_vals(&op_ty.results);
                    op_ty.into()
                }
                (Global(op, idx), _) => {
                    let global_ty = module.global(*idx);
                    let op_ty = op.to_type(global_ty.type_.0);
                    state.pop_vals_expected(&op_ty.params)?;
                    state.push_vals(&op_ty.results);
                    op_ty.into()
                }
                (Call(idx), _) => {
                    let op_ty = module.function(*idx).type_.clone();
                    state.pop_vals_expected(&op_ty.params)?;
                    state.push_vals(&op_ty.results);
                    op_ty.into()
                }

                // Value-polymorphic:
                (Drop, _) => {
                    let ty = state.pop_val()?;
                    InstructionType {
                        inputs: vec![ty],
                        results: Vec::new(),
                    }
                }
                (Select, _) => {
                    state.pop_val_expected(Some(ValType::I32))?;
                    let ty1 = state.pop_val()?;
                    let ty2 = state.pop_val()?;
                    let ty = merge_unknown_types(ty1, ty2)?;
                    state.push_val(ty);
                    InstructionType {
                        inputs: vec![Some(ValType::I32), ty, ty],
                        results: Vec::new(),
                    }
                }

                // All of those are stack-polymorphic:
                (Block(block_ty), _) | (Loop(block_ty), _) => {
                    state.push_ctrl(instr, Vec::new(), block_ty.0.into_iter().collect());
                    InstructionType::new()
                }
                (If(block_ty), _) => {
                    state.pop_val_expected(Some(ValType::I32))?;
                    state.push_ctrl(instr, Vec::new(), block_ty.0.into_iter().collect());
                    InstructionType {
                        inputs: vec![Some(ValType::I32)],
                        results: Vec::new(),
                    }
                }
                (End, _) => {
                    let frame = state.pop_ctrl()?;
                    state.push_vals(&frame.results);
                    InstructionType {
                        inputs: Vec::new(),
                        results: frame.results.into_iter().map(Some).collect(),
                    }
                }
                (Else, _) => {
                    let frame = state.pop_ctrl()?;
                    if frame.block_instr != BlockInstr::If {
                        return Err(Error::new("else instruction not matching if"));
                    }
                    state.push_ctrl(instr, frame.inputs, frame.results.clone());
                    InstructionType {
                        inputs: Vec::new(),
                        results: frame.results.into_iter().map(Some).collect(),
                    }
                }
                (Br(label), _) => {
                    let ty =
                        state.pop_vals_expected(&state.label_types(state.get_ctrl(*label)?))?;
                    state.unreachable()?;
                    InstructionType {
                        inputs: ty,
                        results: Vec::new(),
                    }
                }
                (BrIf(label), _) => {
                    state.pop_val_expected(Some(ValType::I32))?;
                    let ty = state.label_types(state.get_ctrl(*label)?);
                    let ty =
                        state.pop_vals_expected(&ty)?;
                    state.push_vals(&state.label_types(state.get_ctrl(*label)?));
                    InstructionType {
                        inputs: ty.clone(),
                        results: ty,
                    }
                }
                (BrTable { table, default }, _) => {
                    state.pop_val_expected(Some(ValType::I32))?;
                    for label in table {
                        state.pop_vals_expected(&state.label_types(state.get_ctrl(*label)?))?;
                        state.push_vals(&state.label_types(state.get_ctrl(*label)?));
                    }
                    let ty =
                        state.pop_vals_expected(&state.label_types(state.get_ctrl(*default)?))?;
                    state.unreachable()?;
                    InstructionType {
                        inputs: ty,
                        results: Vec::new(),
                    }
                }
                (Return, _) => {
                    let ty = &function.type_.results;
                    state.pop_vals_expected(ty)?;
                    state.unreachable()?;
                    InstructionType {
                        inputs: ty.iter().map(|ty| Some(*ty)).collect(),
                        results: Vec::new(),
                    }
                }

                (instr, None) => unreachable!(
                    "instruction {:?} with non-primitive type not handled",
                    instr
                ),
            })
        }()
        .map_err(|mut err| {
            err.instruction = Some((instr_idx, instr.clone()));
            err
        })?;
        println!("    {:<5} {:40} {:30} {}", 
            instr_idx,
            format!("{:?}", instr), 
            format!("{}", ty), 
            state.val_stack_str());
        types.push(ty);
    }
    Ok(types)
}
