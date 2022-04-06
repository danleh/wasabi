use std::{
    fmt::{self, Write},
    io::{self, ErrorKind},
    path::Path, sync::atomic::{AtomicUsize, self}, collections::HashMap, cmp::Ordering, hash::{Hash, Hasher},
};

use arc_interner::ArcIntern;

use crate::{highlevel::{MemoryOp, Global, Table}, types::{InferredInstructionType, TypeChecker}, Val, ValType, Idx, BlockType};
use crate::{
    highlevel::{self, LoadOp, UnaryOp, BinaryOp, StoreOp, FunctionType},
    Memarg,
};

// TODO(Michelle): fix compile errors in wimpl_opt, add tests, only then include in module hierarchy.
// pub mod optimize;
// pub mod analyze;
// pub mod callgraph;
pub mod wimplify;
// pub mod traverse;

mod pretty_print;
mod parse;

#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
    pub globals: Vec<Global>,
    pub tables: Vec<Table>,

    // From the name section, if present, e.g., compiler-generated debug info.
    // pub name: Option<String>,
    // pub memories: Vec<Memory>,
    // pub start: Option<Idx<Function>>,
    // pub custom_sections: Vec<RawCustomSection>,

    /// Metadata associated with a particular wimpl `Stmt` or `Expr`, identified by its `InstrId`.
    /// Stored out-of-line in order to make the individual AST node not too large.
    // TODO add more information, introduce a `Metadata` with `wasm_src_location` as a field.
    // TODO Make `wasm_src_location` an `Option` because not everything originates from WebAssembly?
    pub metadata: HashMap<InstrId, WasmSrcLocation>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WasmSrcLocation(Idx<highlevel::Function>, Idx<highlevel::Instr>);

impl Module {
    pub fn from_wasm_file(path: impl AsRef<Path>) -> Result<Module, String> {
        let wasm_module = highlevel::Module::from_file(path).map_err(|err| err.to_string())?;
        wimplify::wimplify(&wasm_module)
    }

    // FIXME perf: This linear search can easily give us accidentially quadratic behavior!
    // TODO Memoize lookup with HashMap in lazy_static or non-pub field on Module.
    pub fn function(&self, name: FunctionId) -> Option<&Function> {
        let mut functions = self.functions.iter().filter(|f| f.name == name);
        let function = functions.next();
        assert!(functions.next().is_none(), "non-unique function name {}", name);
        function
    }

    pub fn function_by_idx(&self, idx: Idx<highlevel::Function>) -> &Function {
        &self.functions[idx.to_usize()]
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Function {
    /// Either the name of a function (e.g., from debug info), or a numerical index.
    pub name: FunctionId,
    pub type_: FunctionType,
    // TODO what about imported functions? I think we should make body an Option.
    pub body: Body,

    /// Export name(s) of this function.
    pub export: Vec<String>,
    
    //pub param_names: Vec<Option<String>>,
}

impl Function {
    pub fn name(&self) -> FunctionId {
        self.name.clone()
    }

    pub fn params(&self) -> impl Iterator<Item = (Var, ValType)> {
        self.type_
            .inputs()
            .iter()
            .enumerate()
            .map(|(idx, ty)| (Var::Param(idx as u32), *ty))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub enum FunctionId {
    /// If the function had a name attached to it (e.g., from debug information in the `name` 
    /// custom section or from the export/import name).
    /// The string is stored in a string interner, i.e., deduplicated and such that equality can
    /// be a quick pointer equality.
    Name(ArcIntern<String>),
    /// Otherwise, just refer to the function via its index, which is the same as in the original
    /// WebAssembly module.
    Idx(u32),
}

impl FunctionId {
    pub fn from_idx(idx: Idx<highlevel::Function>, module: &highlevel::Module) -> Self {
        // Try different ways of getting a name for a WebAssembly function.
        // First try if the debug name is present, because it's the most "original" or "close to the source".
        let function = module.function(idx);
        let debug_name = function.name.clone();
        let first_export_name = function.export.first().cloned();
        let import_field_name = match &function.code {
            highlevel::ImportOrPresent::Import(module_name, field_name) => Some(format!("{}.{}", module_name, field_name)),
            highlevel::ImportOrPresent::Present(_) => None,
        };
        let name = debug_name.or(first_export_name).or(import_field_name);
        match name {
            Some(name) => FunctionId::Name(ArcIntern::from(name)),
            None => FunctionId::Idx(idx.to_u32()),
        }
    }

    pub fn from_name(name: String) -> Self {
        FunctionId::Name(ArcIntern::from(name))
    }
}

/// A sequence of instructions, typically as the body of a function or block.
#[derive(Debug, Eq, PartialEq, Clone, Default, Hash, Ord, PartialOrd)]
pub struct Body(pub Vec<Stmt>);

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
pub enum Var {
    // These two correspond to the WebAssembly constructs of the same name.
    // Note that the index of locals in WebAssembly does not have to match the local numbering here,
    // because the "index space" of locals contains also the function parameters.
    Local(u32),
    Global(u32),

    /// Originally an (implicit) stack slot in the WebAssembly operand stack.
    Stack(u32),
    /// Originally a parameter to the current function (which would have been accessed via
    /// `local.get` and was in the same index space as locals).
    Param(u32),
    /// Originally the result value of a block with non-empty block type.
    BlockResult(u32),
    /// Originally the return value of the function.
    /// In the Wasm MVP, there is always only a single one.
    Return(u32),
}

/// An absolute block label, NOT to be confused with the relative branch labels of WebAssembly!
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd, Default)]
pub struct Label(u32);

/// Global, unique identifier for a Wimpl `Stmt` or `Expr` (which was formerly a WebAssembly 
/// instruction, hence the name).
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
pub struct InstrId(usize);

static INSTR_ID_COUNT: AtomicUsize = AtomicUsize::new(0);

impl InstrId {
    pub fn fresh() -> Self {
        Self(INSTR_ID_COUNT.fetch_add(1, atomic::Ordering::SeqCst))
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Stmt {
    id: InstrId,
    kind: StmtKind
}

impl Stmt {
    pub fn new(kind: StmtKind) -> Self {
        Self { id: InstrId::fresh(), kind }
    }

    /// Convenience constructor for creating a `Stmt::Expr(...)`.
    pub fn expr(expr_kind: ExprKind) -> Self {
        Stmt::new(StmtKind::Expr(Expr::new(expr_kind)))
    }
}

// Need to manually implement `PartialEq`, `PartialOrd`, `Ord`, and `Hash` to ignore the `id` field 
// in comparisons.

impl PartialEq for Stmt {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl PartialOrd for Stmt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Stmt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind.cmp(&other.kind)
    }
}

impl Hash for Stmt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}

impl From<StmtKind> for Stmt {
    fn from(kind: StmtKind) -> Self {
        Self::new(kind)
    }
}

// Convenience, to create an expression statement (wrapping the `wimpl::Expr` in a 
// `wimpl::Stmt::Expr` and adding unique `id`s to both in one swoop).
impl From<ExprKind> for Stmt {
    fn from(expr_kind: ExprKind) -> Self {
        Stmt::expr(expr_kind)
    }
}

/// Wimpl instructions make the following major changes over high-level Wasm:
/// - Remove the evaluation/operand stack completely, every instruction takes
/// explicit arguments and optionally produces a (in the Wasm MVP) single LHS.
/// - Stratify, i.e., express instructions that add no expressiveness as
/// combinations of simple instructions, e.g., br_if and select.
/// - Resolve relative, numerical branch targets to explicitly labeled blocks.
/// - Represent stack variables, locals, globals, and function parameters with
/// a single `Variable` construct. As a side-effect of this replaces all
/// local.* and global.* instructions with a single `Assign` instruction.
// TODO Optimize this representation, in particular remove redundant assignments
// between stack variables and locals/globals.
#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub enum StmtKind {

    // Simplify nop: Not necessary for analysis.

    Unreachable,

    /// Expression statement, expression is executed for side-effects only.
    Expr(Expr),

    // TODO Maybe add a variable declaration stmt before blocks, select, locals, etc.
    // DO NOT include initialization expression in it, if you would want that just append an assign.
    // THEN remove ValType from Assign and wimplify label_stack / var_stack.
    // VarDecl(Var, ValType), // let l0: i32

    // TODO Add a drop statement, that assigns the RHS to nothing
    // Drop(Expr) // _ = i32.const 3

    /// This unifies all local.set, global.set, local.tee, local.get, and global.get,
    /// and data-flow before branches ("br with value" in Wasm).
    Assign {
        lhs: Var,
        type_ : ValType,
        rhs: Expr,
    },

    Store {
        op: StoreOp,
        addr: Expr,
        value: Expr,
    },

    // Simplify drop: This is just a dead variable, no instruction needed.
    // Simplify select: Encode as `if (arg0) { s0 = arg1 } else { s0 = arg2 }`.

    /// `br` is the only branching construct in Wimpl.
    /// `br_if` is represented as `if (cond) { .. , br @label }`.
    /// `br_table` as `switch (idx) { case 0: { br @label } ... }`.
    /// `return` is represented as `br @label_body`.
    /// Where for all cases the carried value is assigned explicitly beforehand.
    Br {
        target: Label,
    },

    Block {
        body: Body,
        end_label: Label,
    },

    Loop {
        begin_label: Label,
        body: Body,
    },

    // If the if was target of a branch in the original WebAssembly, wrap
    // this in a Wimpl block to have a branch target label available.
    // TODO technically, we could simplify/represent If by Switch as well...
    // Downside: br_if (which are frequent) would be expanded into a quite complex switch:
    // br_if (cond) (value) @label
    // -> if (cond) { br (value) @label }
    // -> if (cond) { b0 = value; br @label }
    // -> switch (cond) { case 0: {} default: { b0 = value; br @label } }
    If {
        condition: Expr,
        if_body: Body,
        else_body: Option<Body>,
    },

    /// Similar to C switch statement, but doesn't fallthrough from one case to the next.
    Switch {
        index: Expr,
        cases: Vec<Body>,
        default: Body,
    },

}

#[derive(Debug, Eq, Clone)]
pub struct Expr {
    id: InstrId,
    kind: ExprKind,
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Self { id: InstrId::fresh(), kind }
    }

    /// Convenience constructor for creating a `Box<Expr>`.
    pub fn boxed(kind: ExprKind) -> Box<Self> {
        Box::new(Self::new(kind))
    }
}

// Need to manually implement `PartialEq`, `PartialOrd`, `Ord`, and `Hash` to ignore the `id` field 
// in comparisons.

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind.cmp(&other.kind)
    }
}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}

impl From<ExprKind> for Expr {
    fn from(kind: ExprKind) -> Self {
        Self::new(kind)
    }
}

// Conveniece, typically for creating recursive expressions.
impl From<ExprKind> for Box<Expr> {
    fn from(kind: ExprKind) -> Self {
        Expr::boxed(kind)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub enum ExprKind {

    VarRef(Var),

    Const(Val),

    Load {
        op: LoadOp,
        addr: Box<Expr>,
    },

    MemorySize,
    MemoryGrow { pages: Box<Expr> },

    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),

    Call {
        func: FunctionId,
        args: Vec<Expr>,
    },

    CallIndirect {
        type_: FunctionType,
        table_idx: Box<Expr>,
        args: Vec<Expr>,
    },

}


/// Convenience macro to write Wimpl statements in Rust.
#[macro_export]
macro_rules! wimpls {
    ($($tokens:tt)*) => {
        {
            let input_str = std::stringify!($($tokens)*)
                // std::stringify somehow re-wraps the input tokens to 80 columns.
                // Replace those inserted newlines, also to make the following easier.
                .replace("\n", " ")
                // HACK Because the input `tokens` are tokenized by rustc's
                // lexer, it inserts whitespace sometimes where Wimpl/Wasm
                // syntax doesn't accept it. Fix those cases here.
                .replace("offset = ", "offset=")
                .replace("align = ", "align=")
                .replace("@ label", "@label");
            match Stmt::from_str_multiple(&input_str) {
                Ok(instrs) => instrs,
                Err(err) => panic!("Invalid Wimpl instriction(s).\n{}\n(Note: whitespace might be different from your input.)", err)
            }
        }
    }
}

/// Macro for a single Wimpl statement, see `wimpls!`.
#[macro_export]
macro_rules! wimpl {
    ($($tokens:tt)+) => {
        {
            let mut instrs = wimpls!($($tokens)+);
            match (instrs.pop(), instrs.is_empty()) {
                (Some(instr), true) => instr,
                _ => panic!("The wimpl! macro accepts only a single instruction, use wimpls! instead.")
            }
        }
    }
}
