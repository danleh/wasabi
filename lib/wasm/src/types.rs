use std::{path::PathBuf, fmt, borrow::Borrow, convert::TryFrom};

use crate::{
    highlevel::{Function, Instr, LocalOp, Module},
    FunctionType, Idx, Label, ValType,
};

// TODO module comment: type checking for WebAssembly instruction sequences, functions, whole modules.
// follows bla, but infers types.

// TODO data gathering:
// for our dataset: how many functions without memory loads/stores
// how many functions with call_indirect?

/// Strongly inspired by the official type-checking algorithm, as described
/// in the specification:
/// https://webassembly.github.io/spec/core/appendix/algorithm.html#algo-valid
/// Except that we don't care about stack height validation, because we assume
/// that only valid modules can be analyzed; and that we produce types for each
/// instruction, instead of only checking if they are valid (see `InstructionType`).

// See https://webassembly.github.io/spec/core/valid/instructions.html for a
// good overview of the typing rules for individual instructions and sequences
// of instructions.
// It also introduces the two types of polymorphism in WebAssembly:
// - value-polymorphic: handled by Wasabi's type stack type.
// - stack-polymorphic: handled in Wasabi with the unreachable_depth, which
// is unfortunately NOT enough for here :( FIXME
// For a good implementation, see the validation algorithm description in the
// spec: https://webassembly.github.io/spec/core/appendix/algorithm.html#algo-valid

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeError {
    pub message: String,

    // Optional location information.
    pub instruction: Option<Idx<Instr>>,
    pub function: Option<Idx<Function>>,
}

impl TypeError {
    pub fn new(message: impl AsRef<str>) -> Self {
        Self {
            message: message.as_ref().to_string(),
            instruction: None,
            function: None,
        }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("type error")?;
        if let Some(function) = self.function {
            write!(f, ", function #{}", function.into_inner())?;
        }
        if let Some(instruction) = self.instruction {
            write!(f, ", instruction #{}", instruction.into_inner())?;
        }
        write!(f, ": {}", self.message)
    }
}

impl<T: AsRef<str>> From<T> for TypeError {
    #[inline(always)]
    fn from(str: T) -> Self {
        Self::new(str.as_ref())
    }
}

impl std::error::Error for TypeError {}


/// Inferred types for the operand stack and individual instructions can be
/// "unconstrained" or "unknown" in the sense that a stack slot can be
/// value-polymorphic, i.e., the concrete type is not known statically. 
/// We use `None` for such cases here.
/// 
/// Unconstrained types come up due to a combination of stack-polymorphic and 
/// value-polymorphic instructions. Consider for example:
/// ```wasm
/// unreachable  ;; stack polymorphic type as per spec: [t*] -> [t*]
/// drop         ;; value polymorphic type as per spec: [t] -> []
/// ```
/// Unreachable can produce an arbitrary number of values of different types,
/// whereas drop takes any value as argument. Our type checker thus infers:
/// ```wasm
/// unreachable  ;; inferred type: [] -> [?]
/// drop         ;; inferred type: [?] -> []
/// Where `?` signifies such an unconstrained type.
/// 
/// Inferred types form a lattice, where `forall t: ValType. t < ?` with `<`
/// as the subtyping relation. That is, any regular value type can be used in
/// place of `?`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct InferredValType(pub Option<ValType>);

#[derive(Debug, thiserror::Error, Copy, Clone, Eq, PartialEq)]
#[error("unconstrained or unknown type")]
pub struct UnconstrainedTypeError;

impl InferredValType {
    #[inline(always)]
    pub fn unknown() -> Self {
        InferredValType(None)
    }

    #[inline(always)]
    pub fn known(ty: ValType) -> Self {
        InferredValType(Some(ty))
    }

    /// Computes the join of the two types, i.e., the "least upper bound" type.
    /// Returns `None` if there is no valid join of the two types, e.g., because
    /// the two types are incompatible, i.e., different concrete value types.
    /// 
    /// Examples:
    /// ```text
    /// join(i32, i32) = Some(i32)
    /// join(i32, ?) = Some(i32)
    /// join(?, ?) = Some(?)
    /// join(i32, f32) = None
    /// ```
    #[must_use]
    pub fn join(self, other: impl Into<InferredValType>) -> Option<InferredValType> {
        match (self.0, other.into().0) {
            // Both unconstrained, join is also unconstrained.
            (None, None) => Some(InferredValType::unknown()),
            // Both equal, return either one is valid join.
            (Some(ty1), Some(ty2)) if ty1 == ty2 => Some(InferredValType::from(ty1)),
            // Join with unconstrained type returns the concrete type.
            (None, Some(ty)) | (Some(ty), None) => Some(InferredValType::from(ty)),
            // Incompatible concrete value types, no join.
            (Some(_), Some(_)) => None,
        }
    }
}

impl ValType {
    /// Like join on two `InferredValType`, but will never result in an unknown
    /// type.
    pub fn join(self, other: InferredValType) -> Option<ValType> {
        match other.0 {
            None => Some(self),
            Some(other) if other == self => Some(self),
            Some(_) => None
        }
    }
}

impl From<ValType> for InferredValType {
    #[inline(always)]
    fn from(ty: ValType) -> Self {
        Self::known(ty)
    }
}

impl TryFrom<InferredValType> for ValType {
    type Error = UnconstrainedTypeError;

    #[inline(always)]
    fn try_from(with_unknown: InferredValType) -> Result<Self, Self::Error> {
        with_unknown.0.ok_or(UnconstrainedTypeError)
    }
}

impl fmt::Display for InferredValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(ty) => ty.fmt(f),
            None => f.write_str("?"),
        }
    }
}


/// Inferred type for the operand stack during type checking.
/// 
/// WebAssembly has unusual typing rules for checking (statically known) dead
/// code, e.g., after branches or the unreachable instruction.
/// It is special because those instructions are stack-polymorphic, i.e., they
/// cannot just produce an unconstrained (value-polymorphic) single type,
/// but even an arbitrary _number of_ types.
/// Those stack-polymorphic instructions can effectively produce arbitrary
/// values "out of thin air". Only the subsequent instructions then determine
/// how many of those values are created (to satisfy those instructions' inputs).
/// Since stack polymorphism only appears after branches or unreachable, the
/// subsequent instructions are always known to be dead code, so this doesn't
/// produce a problem at runtime.
/// 
/// To account for stack polymorphism, this type distinguishes between
/// unreachable code (which is stack polymorphic) and regular code (which is not).
/// 
/// See the reference interpreter for another possible implementation
/// https://github.com/WebAssembly/spec/blob/master/interpreter/valid/valid.ml
/// or Conrad Watt's paper, page 9, right column
/// https://www.cl.cam.ac.uk/~caw77/papers/mechanising-and-verifying-the-webassembly-specification.pdf
/// for details.
/// 
/// As for a discussion of why WebAssembly is typed this way, see
/// https://github.com/WebAssembly/design/issues/1379#issuecomment-694173065
/// and all other comments in this thread.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StackType {
    Unreachable,
    Reachable(Vec<InferredValType>)
}

impl StackType {
    pub fn unreachable() -> Self {
        StackType::Unreachable
    }

    pub fn reachable<T: AsRef<[U]>, U: Into<InferredValType>>(tys: T) -> Self {
        StackType::Reachable(tys.as_ref().iter().map(|&ty| ty.into()).collect())
    }
}

impl<T: AsRef<[ValType]>> From<T> for StackType {
    #[inline(always)]
    fn from(tys: T) -> Self {
        Self::reachable(tys)
    }
}

impl TryFrom<&StackType> for Vec<ValType> {
    type Error = UnconstrainedTypeError;

    fn try_from(with_unknown: &StackType) -> Result<Self, Self::Error> {
        match with_unknown {
            StackType::Unreachable => Err(UnconstrainedTypeError),
            StackType::Reachable(with_unknown) => 
                with_unknown.iter().copied().map(ValType::try_from).collect()
        }
    }
}

impl TryFrom<StackType> for Vec<ValType> {
    type Error = UnconstrainedTypeError;

    #[inline(always)]
    fn try_from(with_unknown: StackType) -> Result<Self, Self::Error> {
        TryFrom::try_from(&with_unknown)
    }
}

impl fmt::Display for StackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackType::Unreachable => f.write_str("unreachable"),
            StackType::Reachable(tys) => {
                f.write_str("[")?;
                if let Some((last, tys)) = tys.split_last() {
                    for ty in tys {
                        write!(f, "{}, ", ty)?;
                    }
                    write!(f, "{}", last)?;
                }
                f.write_str("]")
            }
        }
    }
}


/// Similar to a `FunctionType`, lifted to include both value polymorphism and
/// stack polymorphism.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InstructionType {
    pub inputs: Vec<InferredValType>,
    pub results: StackType,
}

impl InstructionType {
    pub fn new(inputs: Vec<InferredValType>, results: Vec<InferredValType>) -> Self {
        InstructionType {
            inputs,
            results,
        }
    }

    pub fn lift(function_type: &FunctionType) -> Self {
        fn lift(tys: &[ValType]) -> Vec<InferredValType> {
            tys.iter().copied().map(InferredValType::from).collect()
        }
        InstructionType {
            inputs: lift(&function_type.params),
            results: lift(&function_type.results).into(),
        }
    }
}

impl<T: Borrow<FunctionType>> From<T> for InstructionType {
    #[inline(always)]
    fn from(function_type: T) -> Self {
        InstructionType::lift(function_type.borrow())
    }
}

impl TryFrom<&InstructionType> for FunctionType {
    type Error = UnconstrainedTypeError;

    fn try_from(with_unknown: &InstructionType) -> Result<Self, Self::Error> {
        fn try_from(with_unknown: &Vec<InferredValType>) -> Result<Vec<ValType>, UnconstrainedTypeError> {
            with_unknown.iter().copied().map(ValType::try_from).collect()
        }
        Ok(FunctionType::new(
            &try_from(&with_unknown.inputs)?,
            &try_from(&with_unknown.results)?,
        ))
    }
}

impl TryFrom<InstructionType> for FunctionType {
    type Error = UnconstrainedTypeError;

    fn try_from(with_unknown: InstructionType) -> Result<Self, Self::Error> {
        TryFrom::try_from(&with_unknown)
    }
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // HACK Reuse fmt::Display implementation, unreachable case never happens.
        write!(f, "{} -> {}", StackType::Reachable(self.inputs), StackType::Reachable(self.results))
    }
}


/// Holds the state during type checking.
///
/// Strongly inspired by the official type-checking algorithm, as described
/// in the specification: https://webassembly.github.io/spec/core/appendix/algorithm.html#algo-valid
/// 
/// There are two differences in our implementation from the specification:
/// - We don't do stack height validation in branches.
// TODO This is fine for now, as we assume only valid WebAssembly modules
// are analyzed/transformed by our tools. Eventually, we should implement this.
/// - We not only _check_, but also _infer_ types for each instruction.
#[derive(Debug)]
pub struct TypeChecker {
    // TODO include function and module in here, will add lifetime to TypeChecker
    
    value_stack: StackType,
    control_stack: Vec<ControlFrame>,
}

impl TypeChecker {
    /*
    High-level API of the type checker.
    */

    /// Begin type checking a function.
    /// This ensures that the final end instruction and branches/returns out of
    /// the function are correctly typed.
    pub fn begin_function(type_: &FunctionType) -> Self {
        let mut self_ = TypeChecker {
            value_stack: StackType::Reachable(Vec::new()),
            control_stack: Vec::new(),
        };
        self_.push_ctrl_func(type_.results.to_vec());
        self_
    }

    /// Check and infer the type for another instruction.
    // TODO move function and module into self.
    pub fn check_next_instr(&mut self, instr: &Instr, function: &Function, module: &Module) -> Result<InstructionType, TypeError> {
        type_check_instr(self, instr, function, module)
    }

    pub fn current_stack_type(&self) -> &StackType {
        &self.value_stack
    }

    /*
     * Value stack operations:
     */

    #[must_use]
    pub fn pop_val(&mut self) -> Result<InferredValType, TypeError> {
        // TODO(Daniel): I don't really understand why this is necessary: 
        // understand and document stack height validation.
        // let frame = self.top_ctrl()?;
        // let height = frame.height;
        // if self.value_stack.len() == height {
            // if unreachable {
            //     return Ok(InferredValType::unknown());
            // }
        //     else {
        //         return Err(Error::new("value stack underflow"));
        //     }
        // }

        match &mut self.value_stack {
            StackType::Unreachable => Ok(InferredValType::unknown()),
            StackType::Reachable(value_stack) => 
                value_stack
                    .pop()
                    .ok_or("expected a value, but value stack was empty".into())
        }
    }

    #[must_use]
    pub fn pop_val_expected(&mut self, expected: ValType) -> Result<ValType, TypeError> {
        let actual = self.pop_val()?;
        expected.join(actual)
            .ok_or(format!("expected type {}, but got {}", expected, actual).into())
    }

    #[must_use]
    pub fn pop_vals_expected(
        &mut self,
        expected: &[ValType],
    ) -> Result<Vec<InferredValType>, TypeError> {
        let actual: Result<Vec<_>, _> = expected
            .iter()
            // The expected types must be checked in reverse order...
            .rev()
            .map(|&expected| self.pop_val_expected(expected))
            .collect();
        // ...and the result must be reversed again for it to be correct.
        let mut actual = actual?;
        actual.reverse();
        Ok(actual)
    }

    pub fn push_val(&mut self, type_: impl Into<InferredValType>) {
        self.value_stack.push(type_.into())
    }

    pub fn push_vals(&mut self, types: &[ValType]) {
        for &ty in types {
            self.value_stack.push(ty.into());
        }
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
        });
        self.push_vals(&inputs);
    }

    #[must_use]
    pub fn top_ctrl(&mut self) -> Result<&mut ControlFrame, TypeError> {
        self.control_stack
            .last_mut()
            .ok_or("empty control stack".into())
    }

    #[must_use]
    pub fn get_ctrl(&self, label: Label) -> Result<&ControlFrame, TypeError> {
        self.control_stack
            .iter()
            .rev()
            .nth(label.0 as usize)
            .ok_or(format!("invalid control stack label {}", label.0).into())
    }

    #[must_use]
    pub fn pop_ctrl(&mut self) -> Result<ControlFrame, TypeError> {
        let frame = self.top_ctrl()?;
        let results = frame.results.clone();
        let height = frame.height;
        self.pop_vals_expected(&results)?;
        // FIXME
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

    /// Switches to "unreachable mode" for this block, i.e., allows 
    /// stack-polymorphic instructions to produce an arbitrary number of values.
    #[must_use]
    pub fn unreachable(&mut self) -> Result<(), TypeError> {
        let new_stack_height = self.top_ctrl()?.height;
        // Drop all values in the current block from the stack.
        self.value_stack
            // FIXME stack underflow to TypeError
            .resize_with(new_stack_height, || panic!("stack underflow"));
        self.top_ctrl()?.unreachable = true;
        Ok(())
    }
}

#[derive(Debug)]
struct ControlFrame {
    block_instr: BlockInstr,

    // Input and result types of the block.
    inputs: Vec<ValType>,
    results: Vec<ValType>,

    // Height of the value stack at the start of the block, used to check that
    // operands do not underflow the current block.
    height: usize,
}

#[derive(Debug, Eq, PartialEq)]
enum BlockInstr {
    Function,
    Block,
    If,
    Else,
    Loop,
}

// TODO add location information
pub fn type_check_module(module: &Module) -> Result<(), TypeError> {
    for (func_idx, func) in module.functions() {
        if let Some(code) = func.code() {
            // println!("  func ${}  {}", func_idx.into_inner(), func.type_);
            types(&code.body, func, &module).map_err(|mut err| {
                err.function = Some(func_idx);
                err
            })?;
        }
    }
    Ok(())
}

fn type_check_instr(state: &mut TypeChecker, instr: &Instr, function: &Function, module: &Module) -> Result<InstructionType, TypeError> {
    // In the simple cases, we know the type from the instruction alone.
    if let Some(ty) = instr.simple_type() {
        // TODO add TypeChecker::signature() helper
        state.pop_vals_expected(&ty.params)?;
        state.push_vals(&ty.results);
        return Ok(ty.into())
    }

    use Instr::*;
    Ok(match instr {
        // Cases which are still monomorphic, but where we need additional 
        // information from the context (function, module) for typing.
        Local(op, idx) => {
            let local_ty = function.param_or_local_type(*idx);
            let op_ty = op.to_type(local_ty);
            state.pop_vals_expected(&op_ty.params)?;
            state.push_vals(&op_ty.results);
            op_ty.into()
        }
        Global(op, idx) => {
            let global_ty = module.global(*idx);
            let op_ty = op.to_type(global_ty.type_.0);
            state.pop_vals_expected(&op_ty.params)?;
            state.push_vals(&op_ty.results);
            op_ty.into()
        }
        Call(idx) => {
            let function_ty = module.function(*idx).type_.clone();
            state.pop_vals_expected(&function_ty.params)?;
            state.push_vals(&function_ty.results);
            function_ty.into()
        }

        // Value-polymorphic:
        Drop => {
            let ty = state.pop_val()?;
            InstructionType::new(vec![ty], Vec::new())
        }
        Select => {
            state.pop_val_expected(ValType::I32)?;
            let ty1 = state.pop_val()?;
            let ty2 = state.pop_val()?;
            let ty = ty1.join(ty2)
                .ok_or(format!("incompatible types {} and {} for select arguments", ty1, ty2).into())?;
            state.push_val(ty);
            InstructionType::new(
                vec![ValType::I32.into(), ty, ty],
                vec![ty]
            )
        }

        // Blocks, i.e., block/loop/if/else.
        // HACK Attach the input type (always empty in the MVP) to the begin
        // instruction and the return type (one or none) to the matching end.
        // In the reference interpreter and the formalisations, blocks are
        // nested in the AST already, so this hack is not necessary.
        // See https://github.com/WebAssembly/spec/blob/master/interpreter/valid/valid.ml
        // and https://github.com/WasmCert/WasmCert-Isabelle/blob/master/WebAssembly/Wasm_Checker_Types.thy
        Block(block_ty) | Loop(block_ty) => {
            state.push_ctrl(instr, Vec::new(), block_ty.0.into_iter().collect());
            InstructionType::new(Vec::new(), Vec::new())
        }
        If(block_ty) => {
            state.pop_val_expected(ValType::I32)?;
            state.push_ctrl(instr, Vec::new(), block_ty.0.into_iter().collect());
            InstructionType::new(
                vec![ValType::I32.into()],
                Vec::new()
            )
        }
        End => {
            let frame = state.pop_ctrl()?;
            state.push_vals(&frame.results);
            InstructionType::new(
                Vec::new(),
                StackType::from(frame.results).0
            )
        }
        Else => {
            let frame = state.pop_ctrl()?;
            if frame.block_instr != BlockInstr::If {
                Err("else instruction not matching if")?
            }
            state.push_ctrl(instr, frame.inputs, frame.results.clone());
            InstructionType::new(
                Vec::new(),
                StackType::from(frame.results).0,
            )
        }

        // Branches: br_if is the only branch that is not followed by dead code.
        BrIf(label) => {
            state.pop_val_expected(ValType::I32)?;
            let ty = state.label_types(state.get_ctrl(*label)?);
            let ty =
                state.pop_vals_expected(&ty)?;
            state.push_vals(&state.label_types(state.get_ctrl(*label)?));
            InstructionType {
                inputs: ty.clone(),
                results: ty,
            }
        }
        // All of these branches are followed by dead code, which makes their
        // types stack-polymorphic:
        Br(label) => {
            let ty =
                state.pop_vals_expected(&state.label_types(state.get_ctrl(*label)?))?;
            state.unreachable()?;
            InstructionType {
                inputs: ty,
                results: Vec::new(),
            }
        }
        BrTable { table, default } => {
            state.pop_val_expected(ValType::I32)?;
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
        Return => {
            todo!("currently not stack polymorphic "); 
            let ty = &function.type_.results;
            state.pop_vals_expected(ty)?;
            state.unreachable()?;
            InstructionType {
                inputs: ty.iter().map(|ty| Some(*ty)).collect(),
                results: Vec::new(),
            }
            
        }

        // The prototypical stack-polymorphic type.
        Unreachable => {
            state.unreachable()?;
            todo!("should produce 'ellipsis' type")
            // ty.into()
        }

        instr => unreachable!(
            "instruction {:?} should either be handled by `simple_type()` or match above",
            instr
        ),
    })
}

/// Extension trait for adding types to instruction sequences.
pub trait WithTypes: Sized {
    /// Checks and infers types for instructions from `function`.
    fn with_types<'a>(self, function: &'a Function, module: &'a Module) -> WithTypesIter<'a, Self>;
}

impl<Iter, I> WithTypes for Iter
where
    Iter: Iterator<Item = I>,
    I: Borrow<Instr>
{
    fn with_types<'a>(self, function: &'a Function, module: &'a Module) -> WithTypesIter<'a, Self> {
        let mut type_checker = TypeChecker::new();
        type_checker.push_ctrl_func(function.type_.results.to_vec());
        WithTypesIter(self, TypeChecker::new(), function, module)
    }
}

pub struct WithTypesIter<'a, I>(I, TypeChecker, &'a Function, &'a Module);

impl<'a, Iter, I> Iterator for WithTypesIter<'a, Iter>
where
    Iter: Iterator<Item = I>,
    I: Borrow<Instr>
{
    type Item = (I, Result<InstructionType, TypeError>);

    fn next(&mut self) -> Option<Self::Item> {
        // TODO add location information to error
        let instr = self.0.next()?;
        let ty = self.1.instr(instr.borrow(), self.2, self.3);
        Some((instr, ty))
    }
}
