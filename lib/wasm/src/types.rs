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
pub struct InferredValType(Option<ValType>);

#[derive(Debug, thiserror::Error, Copy, Clone, Eq, PartialEq)]
#[error("unconstrained/unknown value type")]
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
    fn try_from(maybe_unknown: InferredValType) -> Result<Self, Self::Error> {
        maybe_unknown.0.ok_or(UnconstrainedTypeError)
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
    /// Because reachable code never has unconstrained variables, this is a
    /// stack of `ValType`, not `InferredValType`.
    Reachable(Vec<ValType>)
}

impl From<Vec<ValType>> for StackType {
    #[inline(always)]
    fn from(tys: Vec<ValType>) -> Self {
        Self::Reachable(tys)
    }
}

#[derive(Debug, thiserror::Error, Copy, Clone, Eq, PartialEq)]
#[error("unreachable code")]
pub struct UnreachableError;

impl TryFrom<StackType> for Vec<ValType> {
    type Error = UnreachableError;

    fn try_from(maybe_unreachable: StackType) -> Result<Self, Self::Error> {
        match maybe_unreachable {
            StackType::Unreachable => Err(UnreachableError),
            StackType::Reachable(tys) => Ok(tys)
        }
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


/// Similar to a `FunctionType`, but distinguishes between instructions in
/// unreachable code, where a fully-constrained type cannot be given due to
/// value- and stack-polymorphism, and "normal" instructions in reachable code,
/// which can always be typed with simple `ValType`s.
/// 
/// A more fully-featured type checker might want to assign types also to the
/// instructions that are in unreachable code, but those types have two
/// problems:
/// 1. They cannot be expressed with the surface WebAssembly type syntax, 
/// because of unconstrained types, i.e., they would be over `InferredValType`.
/// 2. They cannot be determined in a streaming, O(1) memory and O(1) runtime
/// fashion, because it is only clear which values a stack-polymorphic 
/// instruction produces at the _end_ of blocks.
/// For these reasons, we restrict ourselves here to producing only types
/// for reachable instructions. Note that dead code is still type _checked_ (!)
/// by our `TypeChecker`, it just doesn't _assign_ types to each instruction.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InferredInstructionType {
    Unreachable,
    Reachable(FunctionType)
}

// impl From<FunctionType> for InferredInstructionType {
//     #[inline(always)]
//     fn from(func_ty: FunctionType) -> Self {
//         InferredInstructionType::Reachable(func_ty)
//     }
// }

impl TryFrom<InferredInstructionType> for FunctionType {
    type Error = UnreachableError;

    fn try_from(maybe_unreachable: InferredInstructionType) -> Result<Self, Self::Error> {
        match maybe_unreachable {
            InferredInstructionType::Unreachable => Err(UnreachableError),
            InferredInstructionType::Reachable(func_ty) => Ok(func_ty),
        }
    }
}


// TODO move comment up
// Implements the relaxed type-checking for dead code from
// https://github.com/WebAssembly/relaxed-dead-code-validation/blob/main/proposals/relaxed-dead-code-validation/Push-Pop.md
// In particular, this will admit some instruction sequences that were formerly
// ill-typed, but only for statically-known dead code.

/// Holds the state during type checking.
///
/// Strongly inspired by the official type-checking algorithm, as described
/// in the specification: https://webassembly.github.io/spec/core/appendix/algorithm.html#algo-valid
/// 
/// There are some differences to the validation algorithm as it is in the spec:
/// 1. We not only _check_, but also _assign_ types for each (reachable) 
/// instruction.
/// 2. We have nested "sub-stacks" for each block, instead of one "flat"
/// value stack + one control stack.
/// 3. We compute the label type for each block (i.e., the values that a branch
/// to a certain block "transports") once and store them, instead of determining
/// them anew (loop vs. others) each time they are queried.
#[derive(Debug)]
pub struct TypeChecker<'module> {
    /// For looking up the type of globals and functions.
    module: &'module Module,
    /// For looking up the type of parameters and locals.
    function: &'module Function,
    
    /// Nested sub-stacks for each block.
    /// Merges the value and control stacks in the specification algorithm.
    block_stack: Vec<BlockFrame>,
}

#[derive(Debug)]
struct BlockFrame {
    value_stack: Vec<InferredValType>,

    /// This is equivalent to having an "ellipsis" pseudo-type at the bottom
    /// of the `value_stack`.
    unreachable: bool,

    /// Needed to type-check that the correct results are on the current
    /// sub-stack before leaving a block.
    results: Vec<ValType>,

    /// Needed to type-check branches targeting this block: Which values does
    /// the branch "transport" from the current stack to this one (the target)?
    /// For loops, these are the inputs to the loop block, for every other block
    /// (if, else, block) it's the output types.
    label_inputs: Vec<ValType>,

    // TODO Once we switch `hl::Instr` to use nested blocks, it is no longer
    // necessary to check that an `else` instruction closes an `if` block.
    // Since we assume we only work on valid Wasm modules anyway, and once that
    // change is made, we don't need to check against the block op anyway,
    // let's not capture it here, unlike in the spec validation algorithm.
    // op: Instr
}

impl<'module> TypeChecker<'module> {
    /*
    High-level API of the type checker.
    */

    /// Begin type checking a function.
    /// This ensures that the final end instruction and branches/returns out of
    /// the function are correctly typed.
    pub fn begin_function(function: &'module Function, module: &'module Module) -> Self {
        let mut self_ = TypeChecker {
            module,
            function,
            block_stack: Vec::new(),
        };
        self_.push_func_block(function.type_.results.to_vec());
        self_
    }

    /// Check and infer the type for the next instruction.
    pub fn check_next_instr(&mut self, instr: &'_ Instr) -> Result<InferredInstructionType, TypeError> {
        type_check_instr(self, instr, self.function, self.module)
    }

    /// Returns the type stack of the current block (without the surrounding
    /// parent stacks, since they are not accessible from inside anyway).
    pub fn current_block_type_stack(&self) -> Result<StackType, TypeError> {
        let frame = self.top_block()?;
        Ok(if frame.unreachable {
            StackType::Unreachable
        } else {
            let tys = frame.value_stack.iter().copied()
                .map(ValType::try_from)
                .collect::<Result<Vec<ValType>, _>>()
                .expect("unknown types should only appear in unreachable code");
            StackType::Reachable(tys)
        })
    }

    /*
     * Value stack operations:
     */

    fn pop_val(&mut self) -> Result<InferredValType, TypeError> {
        let frame = self.top_block()?;
        if frame.unreachable {
            // Once we are in unreachable code, the prior stack-polymorphic
            // instructions can produce necessary value "out of thin air".
            if frame.value_stack.is_empty() {
                return Ok(InferredValType::unknown())
            }
            // However, if there are already concrete types pushed onto the
            // stack, produce those first (i.e., proceed with the following,
            // "regular" logic).
        }

        frame.value_stack.pop().ok_or("expected a value, but value stack was empty".into())
    }

    fn pop_val_expected(&mut self, expected: ValType) -> Result<ValType, TypeError> {
        let actual = self.pop_val()?;
        expected.join(actual)
            .ok_or(format!("expected type {}, but got {}", expected, actual).into())
    }

    fn pop_vals_expected(
        &mut self,
        expected: &[ValType],
    ) -> Result<Vec<ValType>, TypeError> {
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

    fn push_val(&mut self, type_: impl Into<InferredValType>) -> Result<(), TypeError> {
        self.top_block()?.value_stack.push(type_.into());
        Ok(())
    }

    fn push_vals(&mut self, types: &[ValType]) -> Result<(), TypeError> {
        for &ty in types {
            self.push_val(ty)?;
        }
        Ok(())
    }

    /*
     * Control stack operations:
     */
    pub fn push_func_block(&mut self, results: Vec<ValType>) {
        self.block_stack.push(BlockFrame {
            value_stack: Vec::new(),
            unreachable: false,
            results: results.clone(),
            label_inputs: results,
        })
    }

    pub fn push_block(&mut self, instr: &Instr, inputs: Vec<ValType>, results: Vec<ValType>) {
        let label_types = match instr {
            Instr::Block(_) | Instr::If(_) | Instr::Else => results.clone(),
            Instr::Loop(_) => inputs.clone(),
            _ => unreachable!("push_ctrl() should not be called with non-control instruction {:?}", instr),
        };
        self.block_stack.push(BlockFrame {
            value_stack: inputs.iter().cloned().map(InferredValType::from).collect(),
            unreachable: false,
            results,
            label_inputs: label_types
        });
    }

    fn top_block(&mut self) -> Result<&mut BlockFrame, TypeError> {
        // TODO Once `hl::Instr` is nested, there can no longer be more ends
        // then block begins, so this error won't be possible any more and we
        // could use `expect()` here, and remove the `Result` from this function
        // and many others in the type checker.
        self.block_stack.last_mut().ok_or("empty block stack".into())
    }

    pub fn get_block(&self, label: Label) -> Result<&BlockFrame, TypeError> {
        self.block_stack
            .iter()
            .rev()
            .nth(label.0 as usize)
            .ok_or(format!("invalid branch target label {}", label.0).into())
    }

    pub fn pop_block(&mut self) -> Result<BlockFrame, TypeError> {
        // Check that the current (inner) block has the correct result types on
        // the top of its stack. This needs to happen before calling 
        // `block_stack.pop()`, otherwise `pop_vals_expected()` will pop WRONGLY
        // from the parent (outer) stack.
        let frame = self.top_block()?;
        let results = frame.results.as_slice();
        self.pop_vals_expected(results)?;
        
        Ok(self
            .block_stack
            .pop()
            .expect("already checked with top_block()"))
    }

    /// Switches to "unreachable mode" for this block, i.e., allows 
    /// stack-polymorphic instructions to produce an arbitrary number of values.
    /// 
    /// Like the spec algorithm, we make the unreachable instructions consume
    /// all remaining values on the stack, even though the typing rules would
    /// actually permit it to not do that. This results in an instruction
    /// type for unreachable that is not always "stack-minimal", i.e., it might
    /// be the unreachable first clears of values that in then produces out of
    /// thin air again because of its stack-polymorphic result.
    /// 
    /// Also, we don't return the cleared values from the stack here, because
    /// we return the "non-consuming" type for unreachable instructions, i.e.,
    /// we simplify the assigned instruction type to never consume anything.
    #[must_use]
    pub fn unreachable(&mut self) -> Result<(), TypeError> {
        let frame = self.top_block()?;
        frame.value_stack.clear();
        frame.unreachable = true;
        Ok(())
    }
}

pub fn type_check_module(module: &Module) -> Result<(), TypeError> {
    for (func_idx, function) in module.functions() {
        if let Some(code) = function.code() {
            let mut type_checker = TypeChecker::begin_function(function, module);
            for (instr_idx, instr) in code.body.iter().enumerate() {
                let _instr_type_ignored = type_checker
                    .check_next_instr(instr)
                    // Add type error location information.
                    .map_err(|mut e| {
                        e.function = Some(func_idx);
                        e.instruction = Some(instr_idx.into());
                        e
                    })?;
            }
        }
    }
    Ok(())
}

fn type_check_instr(state: &mut TypeChecker, instr: &Instr, function: &Function, module: &Module) -> Result<InferredInstructionType, TypeError> {
    // If we are already in dead code, do not return a concrete instruction 
    // type, because it could contain unconstrained types and would not be able
    // to produce a type in O(1) runtime/memory in all cases.
    let was_unreachable = state.top_block()?.unreachable;
    let inferred_type = |func_ty| {
        if was_unreachable {
            InferredInstructionType::Unreachable
        } else {
            InferredInstructionType::Reachable(func_ty)
        }
    };
    
    // In the simple cases, we know the type from the instruction alone.
    if let Some(ty) = instr.simple_type() {
        state.pop_vals_expected(&ty.params)?;
        state.push_vals(&ty.results);
        return Ok(inferred_type(ty))
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
            inferred_type(op_ty)
        }
        Global(op, idx) => {
            let global_ty = module.global(*idx);
            let op_ty = op.to_type(global_ty.type_.0);
            state.pop_vals_expected(&op_ty.params)?;
            state.push_vals(&op_ty.results);
            inferred_type(op_ty)
        }
        Call(idx) => {
            let function_ty = module.function(*idx).type_.clone();
            state.pop_vals_expected(&function_ty.params)?;
            state.push_vals(&function_ty.results);
            inferred_type(function_ty)
        }

        // Value-polymorphic:
        Drop => {
            let ty = state.pop_val()?;
            match (ValType::try_from(ty), was_unreachable) {
                (_, true) => InferredInstructionType::Unreachable,
                (Ok(ty), false) => InferredInstructionType::Reachable(FunctionType::new(&[ty], &[])),
                (Err(UnconstrainedTypeError), false) => unreachable!("unconstrained stack type should never appear in reachable code"),
            }
        }
        Select => {
            state.pop_val_expected(ValType::I32)?;
            let ty1 = state.pop_val()?;
            let ty2 = state.pop_val()?;
            let ty = ty1.join(ty2)
                .ok_or(TypeError::from(format!("incompatible types {} and {} for select arguments", ty1, ty2)))?;
            state.push_val(ty);
            match (ValType::try_from(ty), was_unreachable) {
                (_, true) => InferredInstructionType::Unreachable,
                (Ok(ty), false) => InferredInstructionType::Reachable(FunctionType::new(&[ValType::I32, ty, ty], &[ty])),
                (Err(UnconstrainedTypeError), false) => unreachable!("unconstrained stack type should never appear in reachable code"),
            }
        }

        // Blocks, i.e., block/loop/if/else.
        // HACK Attach the input type (always empty in the MVP) to the begin
        // instruction and the return type (one or none) to the matching end.
        // In the reference interpreter and the formalisations, blocks are
        // nested in the AST already, so this hack is not necessary.
        // See https://github.com/WebAssembly/spec/blob/master/interpreter/valid/valid.ml
        // and https://github.com/WasmCert/WasmCert-Isabelle/blob/master/WebAssembly/Wasm_Checker_Types.thy
        Block(block_ty) | Loop(block_ty) => {
            state.push_block(instr, Vec::new(), block_ty.0.into_iter().collect());
            // TODO Once we support the multi-value extension, this won't be
            // an empty input always.
            inferred_type(FunctionType::new(&[], &[]))
        }
        If(block_ty) => {
            state.pop_val_expected(ValType::I32)?;
            state.push_block(instr, Vec::new(), block_ty.0.into_iter().collect());
            inferred_type(FunctionType::new(&[ValType::I32], &[]))
        }
        End => {
            let frame = state.pop_block()?;
            state.push_vals(&frame.results);
            inferred_type(FunctionType::new(&[], &frame.results))
        }
        Else => {
            let frame = state.pop_block()?;
            // TODO We don't do this check here, because once `hl::Instr` is 
            // nested, this invariant will be true by construction of the AST.
            // if frame.block_instr != BlockInstr::If {
            //     Err("else instruction not matching if")?
            // }
            // Because in the MVP blocks have no inputs, assume empty as the 
            // input type here. However, once we handle the multi-value 
            // extension, we need to get the if's input type from either the
            // type checker (i.e., a `frame.inputs` field), or if we have a 
            // nested `hl::Instr` AST from the AST node directly.
            state.push_block(instr, Vec::new(), frame.results.clone());
            inferred_type(FunctionType::new(&[], &frame.results))
        }

        // Branches: br_if is the only branch that is not followed by dead code.
        BrIf(label) => {
            // Condition.
            state.pop_val_expected(ValType::I32)?;
            
            let label_inputs = state.get_block(*label)?.label_inputs.as_slice();
            state.pop_vals_expected(label_inputs)?;
            state.push_vals(label_inputs)?;

            // The result type is the condition + all types from the target label.
            let mut input_tys = vec![ValType::I32];
            input_tys.extend_from_slice(label_inputs);
            inferred_type(FunctionType::new(&input_tys, &[]))
        }

        // All of these branches are followed by dead code, which makes their
        // types stack-polymorphic and switches the type checker to unreachable.
        Br(label) => {
            let label_inputs = state.get_block(*label)?.label_inputs.as_slice();
            state.unreachable()?;
            inferred_type(FunctionType::new(label_inputs, &[]))
        }
        BrTable { table, default } => {
            // Branch index.
            state.pop_val_expected(ValType::I32)?;
            
            // Pop and immediately push every branch target label types again,
            // this also ensures they are all the same.
            for label in table {
                let label_inputs = state.get_block(*label)?.label_inputs.as_slice();
                state.pop_vals_expected(label_inputs)?;
                state.push_vals(label_inputs)?;
            }

            // Check the default label types.
            let label_inputs = state.get_block(*default)?.label_inputs.as_slice();
            state.pop_vals_expected(label_inputs)?;

            state.unreachable()?;

            // The result type is the condition + the types from all target labels.
            let mut input_tys = vec![ValType::I32];
            input_tys.extend_from_slice(label_inputs);
            inferred_type(FunctionType::new(&input_tys, &[]))
        }
        Return => {
            let tys = &function.type_.results[..];
            state.pop_vals_expected(tys)?;
            state.unreachable()?;
            inferred_type(FunctionType::new(&tys, &[]))
        }

        // The prototypical stack-polymorphic instruction.
        // Here you can see well our simplification: we return a concrete type
        // for unreachable, that does not consume anything from the stack,
        // but this only works because subsequent dead code won't get a concrete
        // instruction type anyway.
        // This only concerns the assigned instruction type, the type checker
        // will still validate that every dead instruction gets its required
        // inputs from the stack-polymorphic unreachabel instruction. We just
        // cannot tell those types HERE, only later, once they are popped (and
        // the "ellipsis" is "expanded" in concrete types).
        Unreachable => {
            state.unreachable()?;
            inferred_type(FunctionType::new(&[], &[]))
        }

        instr => unreachable!(
            "instruction {:?} should either be handled by `simple_type()` or match above",
            instr
        ),
    })
}

// /// Extension trait for adding types to instruction sequences.
// pub trait WithTypes: Sized {
//     /// Checks and infers types for instructions from `function`.
//     fn with_types<'a>(self, function: &'a Function, module: &'a Module) -> WithTypesIter<'a, Self>;
// }

// impl<Iter, I> WithTypes for Iter
// where
//     Iter: Iterator<Item = I>,
//     I: Borrow<Instr>
// {
//     fn with_types<'a>(self, function: &'a Function, module: &'a Module) -> WithTypesIter<'a, Self> {
//         let mut type_checker = TypeChecker::new();
//         type_checker.push_ctrl_func(function.type_.results.to_vec());
//         WithTypesIter(self, TypeChecker::new(), function, module)
//     }
// }

// pub struct WithTypesIter<'a, I>(I, TypeChecker, &'a Function, &'a Module);

// impl<'a, Iter, I> Iterator for WithTypesIter<'a, Iter>
// where
//     Iter: Iterator<Item = I>,
//     I: Borrow<Instr>
// {
//     type Item = (I, Result<InstructionType, TypeError>);

//     fn next(&mut self) -> Option<Self::Item> {
//         // TODO add location information to error
//         let instr = self.0.next()?;
//         let ty = self.1.instr(instr.borrow(), self.2, self.3);
//         Some((instr, ty))
//     }
// }

#[cfg(test)]
mod tests {
    
}