
//! Code for type checking and type inference of WebAssembly instructions.
//! 
//! # Introduction to WebAssembly type checking
//! 
//! _Type checking_ and _inferring types_ for instructions should not be confused with each other.
//! In type checking, we make sure the overall module/function/sequence of instructions is valid. 
//! In type inference, we assign a concrete type for each individual instruction. Successful type
//! inference thus implies that the program is type correct, but requires more work on top.
//! The WebAssembly [validation algorithm in the specification](https://webassembly.github.io/spec/core/appendix/algorithm.html#algo-valid)
//! does only type checking, not inference. This code does both.
//! 
//! Having a type for each instruction, i.e., type inference, is useful also for further downstream 
//! analysis, e.g., for folding expressions as in the `.wat` text format (strictly speaking one only
//! needs arity for that), when doing static instrumentation (as in 
//! [Wasabi](http://wasabi.software-lab.org)), or for other transformations on WebAssembly.
//! 
//! WebAssembly type checking and type inference can be though of in increasing levels of 
//! difficulty.
//! - In the simplest case, the _instruction itself_ already gives you a precise type, e.g., 
//! `i32.add` takes two `i32` value types as input and returns a single `i32`, the instruction type
//! would be `[i32, i32] -> [i32]`.
//! - Next, some instructions require additional _static context_ to infer the type. E.g., the 
//! return type of `local.get $l0` depends on the type of local `$0`, and the arguments and return 
//! value of `call f` depends on the called function `f`'s type.
//! - Some instructions are _value polymorphic_. That is, they can have several different types,
//! depending on preceding instructions. E.g., the `drop` in `i32.const 0; drop` takes an `i32` as
//! input, wheras the `drop` in `i64.const 0; drop` takes an `i64`. Type checking value polymorphic
//! instructions requires modeling an abstract _type stack_, which models the implicit evaluation
//! stack at the type level.
//! - Then there are _blocks_ and _branches_, where the branch instruction's type depends on the
//! targeted block. Blocks are nested and have their own "local" type stack, i.e., instructions 
//! inside a block cannot access values on the parent stack of their surrounding block.
//! This requires modeling the type stack as a nested "stack of stacks".
//! - Finally, there are instructions that are _stack polymorphic_. These can not only have one of
//! several value types, but even a different _number_ of inputs and outputs. Stack-polymorphism
//! only appears in instructions that leave the program in an _unreachable_ state, i.e., it is
//! statically known that following instructions inside the current block will never be executed.
//! For example any code after an unconditional branch `br` (which leaves the current block) or 
//! after an `unreachable` instruction (which terminates the program with an error) is unreachable.
//! 
//! # Unreachable and stack-polymorphism
//! 
//! Unfortunately, in particular the last case (unreachable code / stack-polymorphic instructions)
//! makes WebAssembly type checking and even more so type inference unusually hard.
//! Consider for example:
//! ```wasm
//! unreachable  ;; stack-polymorphic type as per spec: [t*] -> [t'*]
//! i32.add      ;; "simple" type: [i32, i32] -> [i32]
//! ```
//! This program is well-typed (valid), because you can choose some sequence of types for the output
//! `t'*` of the unreachable instruction that makes the type stack hold the right inputs for the
//! next `i32.add` instruction. Effectively, stack-polymorphic instructions can produce an arbitrary
//! number of values "out of thin air". Only the subsequent instructions then determine how many of
//! those values are created (to satisfy those instructions' inputs). Since stack polymorphism only 
//! appears after branches or `unreachable`, the subsequent instructions are always known to be dead 
//! code, so this doesn't produce a problem, because those values don't actually have to be produced
//! at runtime, only during type checking.
//! 
//! It does however complicate type checking and inference, when the instructions following a 
//! stack-polymorphic instruction _do not_ constrain the type to a concrete value type. 
//! Consider the altered example:
//! ```wasm
//! unreachable  ;; stack-polymorphic type as per spec: [t*] -> [t'*]
//! drop         ;; value-polymorphic type as per spec: [t] -> []
//! ```
//! Even though the number of outputs for the `unreachable` can be determined (one), the concrete
//! type in place of the type variable `t` cannot, because `drop` can drop any type.
//! If we wanted to assign a type for the stack between those two instructions, we could choose any 
//! concrete type, the stack top is effectively unconstrained.
//! If we wanted to infer a concrete type for each instruction, we could thus choose:
//! ```wasm
//! unreachable  ;; inferred type: [] -> [?]
//! drop         ;; inferred type: [?] -> []
//! ```
//! Where `?` signifies such an unconstrained type. Unfortunately, `?` is not a valid value type
//! as per the WebAssembly core language. That is, the "most general" or _principal type_ for this
//! instruction sequence is not expressible in the WebAssembly type language, but instead requires
//! this additional element `?`. This is why our type checker/inference produces `InferredValType`s.
//! 
//! Finally, the last issue from unreachable/stack-polymorphic instructions in WebAssembly is that
//! while _type checking_ can be performed in a _streaming_ fashion over incoming instructions,
//! _assigning_ types to those incoming instructions _cannot_ be done streaming!
//! This is because, as you can see from the `unreachable; i32.add` example, the concrete type for
//! `unreachable` is only known once the subsequent code is seen.
//! 
//! # The problem
//! 
//! Ideally, a type inference algorithm for WebAssembly would have all of the following three
//! characteristics:
//! - Streaming processing, i.e., it can always immediately produce an answer whenever we get the
//! next instruction as input.
//! - Fully concrete types in the surface language, i.e., it can assign/infer concrete instruction 
//! types that only contain "WebAssembly value types", not "unknown" (`?`) or other escape hatches.
//! - Assign a type for _all_ instructions in the program, not just a subset.
//! 
//! With the current design of WebAssembly, there is no way to get all three at the same time.
//! You can either decide to have a non-streaming algorithm and assign concrete types for all 
//! instructions, 
//! or you can have a streaming algorithm that produces types in a "richer" type language (and which
//! is thus harder to make use of for downstream analyses), 
//! or you can have a streaming algorithm that does not assign types to all instructions (e.g., the 
//! spec validation algorithm, which doesn't assign types at all).
//! 
//! There were long discussions as to why WebAssembly is typed this way, see
//! [this comment](https://github.com/WebAssembly/design/issues/1379#issuecomment-694173065)
//! and all other comments in that issue thread.
//! The reasons boil down to:
//! - They wanted to do _some_ type checking for unreachable code (and not turn the type checker
//! completely off), as to avoid security bugs in compilers generating native code from such
//! "unchecked" input.
//! - They wanted to make sure any individually type-correct instruction sequence can be combined
//! with another one, if their "input and output types" matched. This makes writing compilers
//! easier that can glue together any individually to WebAssmebly translated code.
//! 
//! While I can sympathize with the first argument, I don't think the second argument justified the
//! high cost in complexity, especially because all of this complexity is only there for code that
//! is anyway provably dead.
//! I personally think this is a big wart in WebAssembly's design and it was a bad idea to allow
//! for unreachable code and stack-polymorphism in the type system in the first place.
//! It makes type checking (and even more so type inference) pretty hard to understand and error 
//! prone to implement.
//! It would have been better to fully disallow unreachable code, which would be "maximally 
//! strict" for WebAssembly as an interchange format, and would have allowed extensions (e.g., to 
//! the current state of affairs, if so desired) later on.
//! 
//! # Our algorithm
//! 
//! Now that we are stuck with this design and some (few) real-world binaries actually do contain
//! unreachable code, we need to decide for a type assignment algorithm.
//! 
//! One option would have been to implement "relaxed type checking for dead code", as described in
//! [this proposal](https://github.com/WebAssembly/relaxed-dead-code-validation/blob/main/proposals/relaxed-dead-code-validation/Push-Pop.md).
//! This effectively goes to the other extreme, by turning off type checking in unreachable code
//! completely. We did not want that, because our algorithm would then permit WebAssembly programs 
//! that are invalid as per the current specification (1.1).
//! 
//! Instead, we chose the following compromise:
//! - Our algorithm is streaming.
//! - We _type check_ all instructions, including unreachable code, as in the spec validation 
//! algorithm.
//! - We assign concrete, simple types (i.e., using only WebAssembly core value types, not `?`) for
//! all instructions that are reachable.
//! - For all unreachable instructions, we do _not_ assign an instruction type.
//! 
// TODO Inspect V8, SpiderMonkey, JSC, Wasmtime, Wasmer, WABT, Binaryen, spec interpreter for how 
// they do type checking.
// TODO Measure how many real-world binaries use unreachable code.
// TODO Prove that a streaming, fully concrete, all instruction algorithm cannot exist; by showing
// that you can insert always more instructions after an unreachable before its type is fixed.
// TODO Implement a non-streaming algorithm that assigns concrete types to all instructions;
// by either doing O(n^2) runtime and scanning for the next end, or using O(n) memory and buffering 
// types until an end is seen.
// TODO Measure in pratice how many instructions there are after an unreachable instruction, i.e.,
// how high the "latency"/memory overhead of such an algorithm would be.
// TODO Find bugs in WebAssembly type checking implementations by fuzzing, reading source code,
// constructing examples.
// TODO Extend the WebAssembly language with instructions dup/pick, subtyping to show difficulties
// in different type checking algorithms.
// TODO Lessons learned:
// - Make interchange formats as strict as possible in the beginning.
// - Declarative type rules can hide a lot of the implementation choices and difficulties.
//! 
//! In terms of type checking implementations, see:
//! - The reference interpreter, which assigns types, but in a richer type language:
//! https://github.com/WebAssembly/spec/blob/master/interpreter/valid/valid.ml
//! - Conrad Watt's paper, page 9, right column:
//! https://www.cl.cam.ac.uk/~caw77/papers/mechanising-and-verifying-the-webassembly-specification.pdf
//! - The instruction index in the specification: https://webassembly.github.io/spec/core/valid/instructions.html 
//! for a good overview of the typing rules for individual and sequences of instructions.
//! - WABT type checker: https://github.com/WebAssembly/wabt/blob/main/src/type-checker.cc
//! which follows the validation algorithm closely.

use std::{fmt, convert::TryFrom};

use crate::{
    Function, Instr, Module, Global, ImportOrPresent, Code, FunctionType,
    Idx, Label, ValType,
};

/// Value type inferred by the type checker.
/// 
/// Inferred types for the operand stack and individual instructions can be "unconstrained" or 
/// "unknown" in the sense that the stack slot can be value-polymorphic, i.e., the concrete type is
/// not known statically. We use `None` to represent such unconstrained value types here.
/// 
/// Inferred types form a lattice, where `forall t: ValType. t <= ?` with `<` as the subtyping 
/// relation and `?` as the symbol for unconstrained types. 
/// That is, any regular value type can be used in place of `?`.
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

    /// Computes the join of two inferred types, i.e., the "least upper bound" type.
    /// Returns `None` if there is no valid join of the two types, e.g., because the two types are
    /// incompatible, e.g., an i32 and an i64.
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
            // Both are concrete and equal, return either one as the valid join.
            (Some(ty1), Some(ty2)) if ty1 == ty2 => Some(InferredValType::from(ty1)),
            // Join with an unconstrained type returns the concrete type.
            (None, Some(ty)) | (Some(ty), None) => Some(InferredValType::from(ty)),
            // Incompatible concrete value types don't have a join.
            (Some(_), Some(_)) => None,
        }
    }
}

impl ValType {
    /// Like join on two `InferredValType`, but will never result in an unknown type.
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


/// Type of the operand stack during type checking.
/// 
/// In reachable code, this is just a stack of `ValType`s. 
/// However, in unreachable code, the combination of value- and stack-polymorphism (see above) 
/// can result in unconstrained types that cannot be expressed with WebAssembly's simple `ValType`s.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StackType {
    /// Because reachable code never has unconstrained types, this is a simple stack of `ValType`.
    Reachable(Vec<ValType>),

    /// Unreachable code can have unconstrained type variables.
    Unreachable(Vec<InferredValType>),
}

impl From<Vec<ValType>> for StackType {
    #[inline(always)]
    fn from(tys: Vec<ValType>) -> Self {
        Self::Reachable(tys)
    }
}

impl fmt::Display for StackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackType::Unreachable(maybe_unknown_tys) => {
                f.write_str("[")?;
                if let Some((last, tys)) = maybe_unknown_tys.split_last() {
                    for ty in tys {
                        write!(f, "{}, ", ty)?;
                    }
                    write!(f, "{}", last)?;
                }
                f.write_str("] (unreachable)")
            }
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

#[derive(Debug, thiserror::Error, Copy, Clone, Eq, PartialEq)]
#[error("unreachable code does not have a stack that can be typed with only concrete value types")]
pub struct UnreachableError;

/// Convert to a "regular", fully-constrained stack type. This is only possible for reachable code.
impl TryFrom<StackType> for Vec<ValType> {
    type Error = UnreachableError;

    fn try_from(maybe_unreachable: StackType) -> Result<Self, Self::Error> {
        match maybe_unreachable {
            StackType::Unreachable(_) => Err(UnreachableError),
            StackType::Reachable(tys) => Ok(tys)
        }
    }
}


/// Type inferred for an instruction.
/// 
/// Similar to a `FunctionType`, but distinguishes between instructions in unreachable code, where
/// a fully-constrained type cannot be given due to value- and stack-polymorphism, and "normal" 
/// instructions in reachable code, which can always be typed with simple `ValType`s.
/// 
/// A more fully-featured type checker might want to assign types also to the instructions that are
/// in unreachable code, but those instruction types have two problems:
/// 1. They cannot be expressed with the surface WebAssembly type syntax (i.e., `ValType`s), 
/// because of unconstrained types, i.e., they would be over `InferredValType`.
/// 2. They cannot be determined in a streaming fashion with O(1) memory and O(1) runtime, because 
/// it is only clear which values a stack-polymorphic instruction produces at the _end_ of blocks.
/// 
/// For these two reasons, we restrict ourselves here to assign only types
/// for reachable instructions. 
/// 
/// Note that we still type _check_ dead code exactly as stringently as the spec validation 
/// algorithm, we just doesn't _assign_ types to those instructions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InferredInstructionType {
    Unreachable,
    Reachable(FunctionType)
}

impl fmt::Display for InferredInstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InferredInstructionType::Unreachable => f.write_str("unreachable"),
            InferredInstructionType::Reachable(ty) => ty.fmt(f)
        }
    }
}

impl TryFrom<InferredInstructionType> for FunctionType {
    type Error = UnreachableError;

    fn try_from(maybe_unreachable: InferredInstructionType) -> Result<Self, Self::Error> {
        match maybe_unreachable {
            InferredInstructionType::Unreachable => Err(UnreachableError),
            InferredInstructionType::Reachable(func_ty) => Ok(func_ty),
        }
    }
}



/// Error during type checking of a module, function, or instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeError(pub Box<TypeErrorRepr>);

// Put the error fields behind a pointer to make `Result<_, TypeError>` only the size of a pointer,
// not humongously large (>100 bytes because of the many fields!).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeErrorRepr {
    pub message: String,

    // Optional type error location information.
    pub instruction: Option<Instr>,
    pub instruction_idx: Option<Idx<Instr>>,

    pub function_idx: Option<Idx<Function>>,
    pub function_name: Option<String>,

    pub global_idx: Option<Idx<Global>>,
    pub global_name: Option<String>,
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("type error")?;
        if let Some(function_idx) = self.0.function_idx {
            write!(f, ", function #{}", function_idx.to_usize())?;
            if let Some(function_name) = &self.0.function_name {
                write!(f, " ({})", function_name)?;
            }
        }
        if let Some(global_idx) = self.0.global_idx {
            write!(f, ", global init expression #{}", global_idx.to_usize())?;
            if let Some(global_name) = &self.0.global_name {
                write!(f, " ({})", global_name)?;
            }
        }
        if let Some(instruction_idx) = self.0.instruction_idx {
            write!(f, ", instruction #{}", instruction_idx.to_usize())?;
            if let Some(instruction) = &self.0.instruction {
                write!(f, " ({}) ", instruction)?;
            }
        }
        write!(f, ": {}", self.0.message)
    }
}

impl<T: AsRef<str>> From<T> for TypeError {
    fn from(message: T) -> Self {
        Self(Box::new(TypeErrorRepr {
            message: message.as_ref().to_string(),
            instruction: None,
            instruction_idx: None,
            function_idx: None,
            function_name: None,
            global_idx: None,
            global_name: None,
        }))
    }
}

impl std::error::Error for TypeError {}


/// Holds the state during type checking and type inference.
/// 
/// See the module comment for general notes on WebAssembly type checking.
/// 
/// This implementation is strongly inspired by the official type-checking algorithm, as described
/// in the specification (https://webassembly.github.io/spec/core/appendix/algorithm.html#algo-valid)
/// but differs in a couple of ways:
/// - We produce types for each instruction, instead of only checking if they are valid.
/// - Minor details, see below, e.g., merged value and control stack, nested stack instead of 
/// numerical stack height, precomputed label types.
#[derive(Debug)]
pub struct TypeChecker<'module> {
    /// For looking up the type of globals and functions.
    module: &'module Module,
    /// For looking up the type of the current function's parameters and locals.
    function: &'module Function,
    
    /// Nested sub-stacks for each block.
    /// Merges the value and control stacks in the specification algorithm.
    /// 
    /// In my opinion this makes the implementation a bit clearer, less error prone, and closer to
    /// the abstract machine execution semantics. Instead of manually managing and checking against
    /// a numerical "stack height" (as in the spec algorithm), stack underflow (that is, popping a 
    /// value from an empty stack, or attempting to pop from the parent stack of a surrounding 
    /// block) is automatically detected because `pop()` on an individual block's value stack 
    /// returns an `Option` that cannot be ignored.
    /// The downside is that it requires more memory (multiple `Vec`s with capacity and length 
    /// fields instead of a single stack) and more allocations (as each sub-stack has to grow 
    /// individually and is not reused).
    /// I think that's a fair tradeoff for this non-performance critical and education codebase.
    block_stack: Vec<BlockFrame>,
}

#[derive(Debug)]
struct BlockFrame {
    value_stack: Vec<InferredValType>,

    /// This is equivalent to having an "ellipsis" pseudo-type at the bottom of the `value_stack`.
    /// (This would be an alternative implementation strategy, but it would require adding another
    /// kind of value type = val_type | unknown | ellipsis, so this keeps the type ceremony down.)
    unreachable: bool,

    /// Needed for type-checking that the correct results are on the current sub-stack before 
    /// leaving a block.
    expected_results: Vec<ValType>,

    /// Needed for type-checking branches targeting this block: frame Which values does the branch 
    /// "transport" from the current stack to the one represented by this frame (the target)?
    /// For loops, these are the inputs to the loop block, for every other block (if, else, block),
    /// it's the output types.
    /// 
    /// In the spec algorithm, this is implemented as a function that switches when queried between
    /// input and result types, depending on the block type. But since the block type is known when
    /// the frame is created, we can switch there already and store the fixed type here.
    label_inputs: Vec<ValType>,

    // TODO Once we switch `hl::Instr` to use nested blocks, it is no longer
    // necessary to check that an `else` instruction closes an `if` block.
    // Since we assume we only work on valid Wasm modules anyway, and once that
    // change is made, we don't need to check against the block op anyway,
    // let's not capture it here, unlike in the spec validation algorithm.
    // block_op: Instr
}

impl<'module> TypeChecker<'module> {
    // Convenience functions for type checking without having to instantiate a type checker.
    // 
    // They do not return the types for individual instructions, only a result
    // whether overall type checking was successful.
    // TODO Make those functions not part of the TypeChecker impl, but free functions instead.
    // TODO rename e.g., from `check_module` to `type_check_module`.

    /// Type checks all functions and globals in a `module`.
    pub fn check_module(module: &Module) -> Result<(), TypeError> {
        for (func_idx, function) in module.functions() {
            Self::check_function(function, module)
                // Add type error location information.
                .map_err(|mut e| {
                    e.0.function_idx = Some(func_idx);
                    e
                })?;
        }

        for (global_idx, global) in module.globals() {
            Self::check_global_init(global, module)
                // Add type error location information.
                .map_err(|mut e| {
                    e.0.global_idx = Some(global_idx);
                    e
                })?;
        }

        Ok(())
    }

    /// Type checks all instructions in a `function`.
    pub fn check_function(function: &Function, module: &Module) -> Result<(), TypeError> {
        if let Some(code) = function.code() {
            let mut type_checker = TypeChecker::begin_function(function, module);
            for (instr_idx, instr) in code.body.iter().enumerate() {
                let _instr_type_ignored = type_checker
                    .check_next_instr(instr)
                    // Add type error location information.
                    .map_err(|mut e| {
                        e.0.instruction_idx = Some(instr_idx.into());
                        e.0.instruction = Some(instr.clone());

                        e.0.function_name = function.name.clone();

                        e
                    })?;
            }
        }
        Ok(())
    }

    /// Type checks the initialization expression of a `global`.
    pub fn check_global_init(global: &Global, module: &Module) -> Result<(), TypeError> {
        if let ImportOrPresent::Present(init) = &global.init {
            let pseudo_function_for_init = Function::new(
                FunctionType::new(&[], &[]), 
                Code {
                    locals: Vec::new(),
                    body: init.clone()
                },
                Vec::new()
            );
            let mut type_checker = TypeChecker::begin_function(&pseudo_function_for_init, module);
            for (instr_idx, instr) in init.iter().enumerate() {
                let _instr_type_ignored = type_checker
                    .check_next_instr(instr)
                    // Add type error location information.
                    .map_err(|mut e| {
                        e.0.instruction_idx = Some(instr_idx.into());
                        e.0.instruction = Some(instr.clone());
                        e
                    })?;
            }
        }
        Ok(())
    }


    // High-level API of the type checker.

    /// Begin type checking a function.
    /// This ensures that the final end instruction and branches/returns out of the function are
    /// correctly typed.
    pub fn begin_function(function: &'module Function, module: &'module Module) -> Self {
        let mut self_ = TypeChecker {
            module,
            function,
            block_stack: Vec::new(),
        };
        self_.push_func_block(function.type_.results().to_vec());
        self_
    }

    /// Check and infer the type for the next instruction.
    /// The spec validation algorithm does only checking, not assigning a type.
    pub fn check_next_instr(&mut self, instr: &'_ Instr) -> Result<InferredInstructionType, TypeError> {
        // Pulled out of the impl only for formatting reasons: put very long function after the
        // interface definitions here.
        check_instr(self, instr, self.function, self.module)
    }

    /// Returns the type stack in the current block (without the surrounding parent stacks, since
    /// they are not accessible from inside the current block anyway).
    pub fn current_block_type_stack(&self) -> Result<StackType, TypeError> {
        let frame = self.top_block()?;
        Ok(if frame.unreachable {
            StackType::Unreachable(frame.value_stack.clone())
        } else {
            let tys = frame.value_stack.iter()
                .copied()
                .map(ValType::try_from)
                .collect::<Result<Vec<ValType>, _>>()
                .expect("unconstrained/unknown types should only appear in unreachable code");
            StackType::Reachable(tys)
        })
    }


    // Low-level API of the type checker.

    // First, value stack operations, i.e., about pushing, popping, and checking individual value
    // types on the stack.

    fn pop_val(&mut self) -> Result<InferredValType, TypeError> {
        let frame = self.top_block_mut()?;
        if frame.unreachable {
            // Once we are in unreachable code, the prior stack-polymorphic instructions can produce
            // necessary values "out of thin air", matching any possible type.
            if frame.value_stack.is_empty() {
                return Ok(InferredValType::unknown())
            }
            // However, if there are already concrete types pushed onto the stack, produce those 
            // first (i.e., proceed with the following, "regular" logic).
        }

        frame.value_stack.pop().ok_or_else(|| "expected a value, but value stack was empty".into())
    }

    fn pop_val_expected(&mut self, expected: ValType) -> Result<(), TypeError> {
        let actual = self.pop_val()?;
        expected.join(actual).ok_or_else(|| TypeError::from(format!("expected type {}, but got {}", expected, actual)))?;
        Ok(())
    }

    fn pop_vals_expected(&mut self, expected: &[ValType]) -> Result<(), TypeError> {
        // The expected types must be checked in reverse order.
        for &expected in expected.iter().rev() {
            self.pop_val_expected(expected)?;
        }
        Ok(())
    }

    fn push_val(&mut self, type_: impl Into<InferredValType>) -> Result<(), TypeError> {
        self.top_block_mut()?.value_stack.push(type_.into());
        Ok(())
    }

    fn push_vals(&mut self, types: &[ValType]) -> Result<(), TypeError> {
        for &ty in types {
            self.push_val(ty)?;
        }
        Ok(())
    }

    // Control stack operations, i.e., when starting, leaving, or jumping to a block.

    fn push_func_block(&mut self, results: Vec<ValType>) {
        self.block_stack.push(BlockFrame {
            value_stack: Vec::new(),
            unreachable: false,
            expected_results: results.clone(),
            label_inputs: results,
        })
    }

    fn push_block(&mut self, instr: &Instr, inputs: Vec<ValType>, results: Vec<ValType>) {
        let label_types = match instr {
            Instr::Block(_) | Instr::If(_) | Instr::Else => results.clone(),
            Instr::Loop(_) => inputs.clone(),
            _ => unreachable!("push_block() should never be called with non-block instruction {:?}", instr),
        };
        self.block_stack.push(BlockFrame {
            value_stack: inputs.iter().cloned().map(InferredValType::from).collect(),
            unreachable: false,
            expected_results: results,
            label_inputs: label_types
        });
    }

    fn top_block(&self) -> Result<&BlockFrame, TypeError> {
        // TODO Once `hl::Instr` is nested, there can no longer be more ends
        // then block begins, so this error won't be possible any more and we
        // could use `expect()` here, and remove the `Result` from this function
        // and many others in the type checker.
        self.block_stack.last().ok_or_else(|| "empty block stack".into())
    }

    fn top_block_mut(&mut self) -> Result<&mut BlockFrame, TypeError> {
        // TODO Once `hl::Instr` is nested, there can no longer be more ends
        // then block begins, so this error won't be possible any more and we
        // could use `expect()` here, and remove the `Result` from this function
        // and many others in the type checker.
        self.block_stack.last_mut().ok_or_else(|| "empty block stack".into())
    }

    fn get_block(&self, label: Label) -> Result<&BlockFrame, TypeError> {
        self.block_stack
            .iter()
            .rev()
            .nth(label.to_usize())
            .ok_or_else(|| format!("invalid branch target label {}", label.to_u32()).into())
    }

    fn pop_block(&mut self) -> Result<BlockFrame, TypeError> {
        // Check that the current (inner) block has the correct result types on
        // the top of its stack. This needs to happen before calling 
        // `block_stack.pop()`, otherwise `pop_vals_expected()` will pop WRONGLY
        // from the parent (outer) stack.
        let frame = self.top_block()?;
        let results = frame.expected_results.clone();
        self.pop_vals_expected(&results)?;
        
        Ok(self
            .block_stack
            .pop()
            .expect("already checked with top_block()"))
    }

    /// Switches to "unreachable mode" for the current block, i.e., allows stack-polymorphic 
    /// instructions to produce an arbitrary number of values.
    /// 
    /// Like the spec algorithm, we let unreachable instructions consume all remaining values on 
    /// the stack, even though the declarative typing rules do not require this behavior.
    /// (It's just the easiest way to avoid that remaining values on the stack can make dead code
    /// type incorrect. Instead, it clears everything and then recovers what is required again.)
    /// This results in a type for the unreachable instruction that is not "stack-minimal", i.e.,
    /// unreachable might first clear values from the stack that it then produces immediately again 
    /// for the subsequent instructions. Because we do not assign types for instructions in dead 
    /// code anyway, this "stack non-minimalism" isn't observable.
    /// Even though internally we clear the current value stack, the returned type is still 
    /// "non-consuming"
    fn unreachable(&mut self) -> Result<Vec<InferredValType>, TypeError> {
        let frame = self.top_block_mut()?;
        frame.unreachable = true;
        let cleared_values = std::mem::take(&mut frame.value_stack);
        Ok(cleared_values)
    }
}

#[inline(always)]
fn check_instr(state: &mut TypeChecker, instr: &Instr, function: &Function, module: &Module) -> Result<InferredInstructionType, TypeError> {
    // If we are already in dead code, do not return a concrete instruction type, because it could
    // contain unconstrained types (which we don't want), and we also would not be able to produce a
    // type in O(1) runtime/memory (which we also don't want).
    let was_unreachable = state.top_block()?.unreachable;
    let to_inferred_type = |func_ty| {
        if was_unreachable {
            InferredInstructionType::Unreachable
        } else {
            InferredInstructionType::Reachable(func_ty)
        }
    };
    
    // In the simple cases, we know the type from the instruction alone.
    if let Some(ty) = instr.simple_type() {
        state.pop_vals_expected(ty.inputs())?;
        state.push_vals(ty.results())?;
        return Ok(to_inferred_type(ty))
    }

    // The other cases are a bit more complex:
    use Instr::*;
    Ok(match instr {
        // Instructions which are still monomorphic, but where we need additional information from
        // the context (current function, module, etc.) for typing.
        Local(op, idx) => {
            let local_ty = function.param_or_local_type(*idx);
            let op_ty = op.to_type(local_ty);
            state.pop_vals_expected(op_ty.inputs())?;
            state.push_vals(op_ty.results())?;
            to_inferred_type(op_ty)
        }
        Global(op, idx) => {
            let global_ty = module.global(*idx);
            let op_ty = op.to_type(global_ty.type_.0);
            state.pop_vals_expected(op_ty.inputs())?;
            state.push_vals(op_ty.results())?;
            to_inferred_type(op_ty)
        }
        Call(idx) => {
            let function_ty = module.function(*idx).type_;
            state.pop_vals_expected(function_ty.inputs())?;
            state.push_vals(function_ty.results())?;
            to_inferred_type(function_ty)
        }

        // Value-polymorphic instructions:
        Drop => {
            let ty = state.pop_val()?;
            match (ValType::try_from(ty), was_unreachable) {
                (_, true) => InferredInstructionType::Unreachable,
                (Ok(ty), false) => InferredInstructionType::Reachable(FunctionType::new(&[ty], &[])),
                (Err(UnconstrainedTypeError), false) => unreachable!("unconstrained value type should never appear in reachable code"),
            }
        }
        Select => {
            state.pop_val_expected(ValType::I32)?;
            let ty1 = state.pop_val()?;
            let ty2 = state.pop_val()?;
            let ty = ty1.join(ty2)
                .ok_or_else(|| TypeError::from(format!("incompatible types {} and {} for select arguments", ty1, ty2)))?;
            state.push_val(ty)?;
            match (ValType::try_from(ty), was_unreachable) {
                (_, true) => InferredInstructionType::Unreachable,
                (Ok(ty), false) => InferredInstructionType::Reachable(FunctionType::new(&[ValType::I32, ty, ty], &[ty])),
                (Err(UnconstrainedTypeError), false) => unreachable!("unconstrained value type should never appear in reachable code"),
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
            // TODO Once we support the multi-value extension, this won't always
            // be an empty input type.
            to_inferred_type(FunctionType::new(&[], &[]))
        }
        If(block_ty) => {
            state.pop_val_expected(ValType::I32)?;
            state.push_block(instr, Vec::new(), block_ty.0.into_iter().collect());
            to_inferred_type(FunctionType::new(&[ValType::I32], &[]))
        }
        End => {
            let frame = state.pop_block()?;
            // If the block stack is emtpy after the end, then this end was for the whole function, 
            // so there is no parent stack to push the results to and `push_vals` would panic.
            let is_function_end = state.block_stack.is_empty();
            if !is_function_end {
                state.push_vals(&frame.expected_results)?;
            }
            to_inferred_type(FunctionType::new(&[], &frame.expected_results))
        }
        Else => {
            let if_frame = state.pop_block()?;
            
            // TODO We don't do this check here, because it would require to a block_instr field
            // in `BlockFrame`. Once `hl::Instr` is nested, this invariant will be true by 
            // construction of the AST anyway and this code and comment can be removed.
            // if frame.block_instr != BlockInstr::If {
            //     Err("else instruction not matching if")?
            // }

            // Because in the MVP blocks have no inputs, assume empty as the input type here. 
            // However, once we handle the multi-value extension, we need to get the if's input type
            // from either the type checker (i.e., a `frame.inputs` field), or if we have a nested
            // `hl::Instr` AST from the AST node directly.
            state.push_block(instr, Vec::new(), if_frame.expected_results.clone());
            to_inferred_type(FunctionType::new(&[], &if_frame.expected_results))
        }

        // Branches: br_if is the only branch that is not followed by dead code.
        BrIf(label) => {
            // Condition.
            state.pop_val_expected(ValType::I32)?;
            
            let label_input_tys = state.get_block(*label)?.label_inputs.clone();
            state.pop_vals_expected(&label_input_tys)?;
            state.push_vals(&label_input_tys)?;

            // The input type is the condition, followed by the type from the
            // target label.
            let mut input_tys = vec![ValType::I32];
            input_tys.extend_from_slice(&label_input_tys);
            to_inferred_type(FunctionType::new(&input_tys, &label_input_tys))
        }

        // All of these branches are followed by dead code, which makes their
        // types stack-polymorphic and switches the type checker to unreachable.
        Br(label) => {
            let label_inputs = state.get_block(*label)?.label_inputs.clone();
            state.unreachable()?;
            to_inferred_type(FunctionType::new(&label_inputs, &[]))
        }
        BrTable { table, default } => {
            // Branch index.
            state.pop_val_expected(ValType::I32)?;
            
            // Pop and immediately push every branch target label types again.
            // This ensures all branch targets have the same type.
            for label in table {
                let label_inputs = state.get_block(*label)?.label_inputs.clone();
                state.pop_vals_expected(&label_inputs)?;
                state.push_vals(&label_inputs)?;
            }

            // Check the default label types.
            let label_inputs = state.get_block(*default)?.label_inputs.clone();
            state.pop_vals_expected(&label_inputs)?;

            state.unreachable()?;

            // The input type is the branch index, followed by the type from the
            // targets (we use the last one, but they are all the same anyways).
            let mut input_tys = vec![ValType::I32];
            input_tys.extend_from_slice(&label_inputs);
            to_inferred_type(FunctionType::new(&input_tys, &[]))
        }
        Return => {
            let tys = function.type_.results();
            state.pop_vals_expected(tys)?;
            state.unreachable()?;
            to_inferred_type(FunctionType::new(tys, &[]))
        }

        // The prototypical stack-polymorphic instruction.
        // Here you can see our type simplification: We return a concrete type
        // for unreachable that does not consume anything from the stack,
        // but this only works because subsequent dead code won't get a concrete
        // instruction type anyway.
        // This only concerns the assigned instruction type, the type checker
        // will still validate that every dead instruction gets its required
        // inputs from the stack-polymorphic unreachabel instruction. We just
        // cannot tell those types HERE, only later, once they are popped (and
        // the "ellipsis" is "expanded" in concrete types).
        Unreachable => {
            state.unreachable()?;
            to_inferred_type(FunctionType::new(&[], &[]))
        }

        instr => unreachable!(
            "instruction {:?} should have been either be handled by `simple_type()` or the match above",
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
    use test_utilities::wasm_files;

    use crate::{Function, Module, Code, Instr, Instr::*, UnaryOp::*, BinaryOp::*, LocalOp, FunctionType, ValType, Val, ValType::*, BlockType, Idx, Label};

    use super::TypeChecker;

    // Utility test functions.

    fn init_function_module_type_checker() -> TypeChecker<'static> {
        // Add some (different) types for parameter, local, return types.
        // Use `Box::leak` to be able to return the type checker from this function (dirty but ok 
        // for testing).
        let function = Box::leak(Box::new(Function::new(FunctionType::new(&[I64], &[F64]), Code {
                locals: vec![crate::Local::new(F32)],
                body: Vec::new(),
            }, Vec::new())
        ));
        let module = Box::leak(Box::new(Module::default()));
        TypeChecker::begin_function(function, module)
    }

    fn assert_reachable_type(type_checker: &mut TypeChecker, instr: Instr, inputs: &[ValType], results: &[ValType]) {
        use crate::types::InferredInstructionType::*;
        match type_checker.check_next_instr(&instr) {
            Err(e) => panic!("type checking failed for instruction {}:\n{}", instr, e),
            Ok(Unreachable) => panic!("type checking produced unreachable type"),
            Ok(Reachable(ty)) => assert_eq!(ty, FunctionType::new(inputs, results), "wrong type for instruction {}", instr),
        }
    }

    fn assert_unreachable_type(type_checker: &mut TypeChecker, instr: Instr) {
        use crate::types::InferredInstructionType::*;
        match type_checker.check_next_instr(&instr) {
            Err(e) => panic!("type checking failed for instruction {}:\n{}", instr, e),
            Ok(Reachable(ty)) => panic!("expected unreachable type but got {}", ty),
            Ok(Unreachable) => {},
        }
    }
    

    // Actual tests:

    #[test]
    pub fn spec_tests_should_typecheck() {
        for path in wasm_files("../../tests/inputs/spec/").unwrap() {
            println!("\t{}", path.display());
            assert!(TypeChecker::check_module(&Module::from_file(path).unwrap().0).is_ok());
        }
    }

    #[test]
    pub fn simple_type_is_known_from_instruction() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Const(Val::I32(0)), &[], &[I32]);
        assert_reachable_type(&mut type_checker, Const(Val::I32(0)), &[], &[I32]);
        assert_reachable_type(&mut type_checker, Binary(I32Add), &[I32, I32], &[I32]);
    }

    #[test]
    pub fn function_parameter_type() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Local(LocalOp::Get, Idx::from(0u32)), &[], &[I64]);
    }

    #[test]
    pub fn function_local_type() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Const(Val::F32(0.0.into())), &[], &[F32]);
        assert_reachable_type(&mut type_checker, Local(LocalOp::Set, Idx::from(1u32)), &[F32], &[]);
    }

    #[test]
    pub fn value_polymorphic_drop_works() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Const(Val::F32(0.0.into())), &[], &[F32]);
        assert_reachable_type(&mut type_checker, Drop, &[F32], &[]);
    }

    #[test]
    pub fn can_type_check_but_not_infer_type_in_dead_code() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Unreachable, &[], &[]);
        assert_unreachable_type(&mut type_checker, Const(Val::I64(1)));
        assert_unreachable_type(&mut type_checker, Unary(I64Eqz));
        assert!(type_checker.check_next_instr(&Unary(F32Abs)).is_err());
    }

    #[test]
    pub fn unconstrained_polymorphic_type_in_dead_code() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Unreachable, &[], &[]);
        assert_unreachable_type(&mut type_checker, Drop);
    }

    #[test]
    pub fn first_unreachable_is_live_second_dead_code() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Unreachable, &[], &[]);
        assert_unreachable_type(&mut type_checker, Unreachable);
    }

    #[test]
    pub fn return_with_dead_code_following() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Const(Val::F64(0.0.into())), &[], &[F64]);
        assert_reachable_type(&mut type_checker, Return, &[F64], &[]);
        assert_unreachable_type(&mut type_checker, Binary(I32Add));
    }

    #[test]
    pub fn block_result_type_attached_to_end() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Block(BlockType(Some(I64))), &[], &[]);
        assert_reachable_type(&mut type_checker, Const(Val::I64(0)), &[], &[I64]);
        assert_reachable_type(&mut type_checker, End, &[], &[I64]);
    }

    #[test]
    pub fn unconditional_branch_leaves_end_unreachable() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Block(BlockType(None)), &[], &[]);
        assert_reachable_type(&mut type_checker, Br(Label::from(0u32)), &[], &[]);
        assert_unreachable_type(&mut type_checker, End);
    }

    #[test]
    pub fn branch_with_result_to_nested_block() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Block(BlockType(Some(I64))), &[], &[]);
        assert_reachable_type(&mut type_checker, Block(BlockType(None)), &[], &[]);
        assert_reachable_type(&mut type_checker, Const(Val::I64(0)), &[], &[I64]);
        assert_reachable_type(&mut type_checker, Br(Label::from(1u32)), &[I64], &[]);
        assert_unreachable_type(&mut type_checker, End);
        assert_reachable_type(&mut type_checker, Const(Val::I64(0)), &[], &[I64]);
        assert_reachable_type(&mut type_checker, End, &[], &[I64]);
    }

    #[test]
    pub fn conditional_branch_to_loop_begin_has_no_input_besides_condition() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Loop(BlockType(Some(I64))), &[], &[]);
        assert_reachable_type(&mut type_checker, Const(Val::I32(0)), &[], &[I32]);
        assert_reachable_type(&mut type_checker, BrIf(Label::from(0u32)), &[I32], &[]);
        assert_reachable_type(&mut type_checker, Const(Val::I64(0)), &[], &[I64]);
        assert_reachable_type(&mut type_checker, End, &[], &[I64]);
    }

    #[test]
    pub fn br_table_nested_blocks() {
        let mut type_checker = init_function_module_type_checker();
        assert_reachable_type(&mut type_checker, Block(BlockType(Some(I64))), &[], &[]);
        assert_reachable_type(&mut type_checker, Block(BlockType(Some(I64))), &[], &[]);
        assert_reachable_type(&mut type_checker, Const(Val::I64(42)), &[], &[I64]);
        assert_reachable_type(&mut type_checker, Const(Val::I32(3)), &[], &[I32]);
        assert_reachable_type(&mut type_checker, BrTable { 
            table: vec![Label::from(1u32), Label::from(0u32), Label::from(0u32), Label::from(1u32)], 
            default: Label::from(0u32)
        }, &[I32, I64], &[]);
        assert_unreachable_type(&mut type_checker, Binary(I64Add));
        assert_unreachable_type(&mut type_checker, End);
        assert_reachable_type(&mut type_checker, Const(Val::I64(0)), &[], &[I64]);
        assert_reachable_type(&mut type_checker, End, &[], &[I64]);
    }

}