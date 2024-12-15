//! High-level, simple, idiomatic-Rust WebAssembly AST:
//!  - Types are inlined instead of referenced by type idx (i.e., no manual handling of Type "pool")
//!  - Function + Code sections are merged into one list of functions,
//!    same for tables: Table + Element sections and memories: Memory + Data sections.
//!  - Imports and exports are part of the respective item, not stored externally and referring to
//!    their item by index.
//!  - Similar instructions are grouped together, for easier uniform handling, e.g., T.const
//!    instructions, loads, stores, unary, and binary instructions.
//!  - Debug names (from the name custom section) are attached to their respective items (module,
//!    functions, and locals).

use core::fmt;
use std::hash;
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;

use ordered_float::OrderedFloat;
use serde::Serialize;
use smallvec::SmallVec;

pub use crate::function_type::FunctionType;

use crate::extensions::WasmExtension;
use crate::EncodeError;
use crate::ParseError;
use crate::ParseWarnings;

/* Values and types. */

/// A primitive WebAssembly value, e.g., an integer or floating-point number.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Val {
    I32(i32),
    I64(i64),
    // Wrap floats, such that they can be ordered and compared (unlike IEEE754 floats),
    // to make it possible, e.g., to put instructions in HashSets etc.
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
}

impl Val {
    /// Get the corresponding WebAssembly type for this value.
    pub fn to_type(&self) -> ValType {
        match *self {
            Val::I32(_) => ValType::I32,
            Val::I64(_) => ValType::I64,
            Val::F32(_) => ValType::F32,
            Val::F64(_) => ValType::F64,
        }
    }

    /// Parse a number as the given type `ty` and return as a typed value.
    // Use Result instead of Option for consistency with stdlib FromStr trait.
    #[allow(clippy::result_unit_err)]
    pub fn from_str(str: &str, ty: ValType) -> Result<Self, ()> {
        Ok(match ty {
            ValType::I32 => Val::I32(str.parse().map_err(|_| ())?),
            ValType::I64 => Val::I64(str.parse().map_err(|_| ())?),
            ValType::F32 => Val::F32(str.parse().map_err(|_| ())?),
            ValType::F64 => Val::F64(str.parse().map_err(|_| ())?),
        })
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::I32(v) => v.fmt(f),
            Val::I64(v) => v.fmt(f),
            Val::F32(v) => v.into_inner().fmt(f),
            Val::F64(v) => v.into_inner().fmt(f),
        }
    }
}

/// A WebAssembly value type, e.g., `i32` or `f64`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

#[test]
fn val_type_is_small() {
    assert_eq!(std::mem::size_of::<ValType>(), 1)
}

impl ValType {
    /// Produce a zero value (e.g., 0 or 0.0) for this type.
    pub fn zero(self) -> Val {
        match self {
            ValType::I32 => Val::I32(0),
            ValType::I64 => Val::I64(0),
            ValType::F32 => Val::F32(OrderedFloat(0.0)),
            ValType::F64 => Val::F64(OrderedFloat(0.0)),
        }
    }

    /// Convert to the standard string representation, as in the WebAssembly
    /// specification and text format.
    pub fn to_str(self) -> &'static str {
        match self {
            ValType::I32 => "i32",
            ValType::I64 => "i64",
            ValType::F32 => "f32",
            ValType::F64 => "f64",
        }
    }

    /// Convert to a single character, e.g., as used by Emscripten.
    /// Lowercase is for 32 bit, uppercase is for 64 bit;
    /// `i` for integers, `f` for floats.
    pub fn to_char(self) -> char {
        match self {
            ValType::I32 => 'i',
            ValType::I64 => 'I',
            ValType::F32 => 'f',
            ValType::F64 => 'F',
        }
    }

    /// Reverse of `to_char`.
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'i' => Some(ValType::I32),
            'I' => Some(ValType::I64),
            'f' => Some(ValType::F32),
            'F' => Some(ValType::F64),
            _ => None,
        }
    }
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for ValType {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(match str.trim() {
            "i32" => ValType::I32,
            "i64" => ValType::I64,
            "f32" => ValType::F32,
            "f64" => ValType::F64,
            _ => return Err(()),
        })
    }
}

/// Limits for tables and memories.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Limits {
    pub initial_size: u32,
    pub max_size: Option<u32>,
}

/// Type of global (scalar) variables.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalType(pub ValType, pub Mutability);

impl fmt::Display for GlobalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.1 {
            Mutability::Const => write!(f, "{}", self.0),
            Mutability::Mut => write!(f, "mut {}", self.0),
        }
    }
}

/// Mutability of global (scalar) variables.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Mutability {
    Const,
    Mut,
}

/* Indices and block labels. */

/// A WebAssembly index into one of the possible "index spaces" (e.g., functions, globals, etc.)
///
/// The type parameter T is only used for static distinction between the different index spaces,
/// but has no representation at runtime.
/// Since we don't own T, we don't want a "drop check" and must use fn() -> T, as is
/// recommended in the rustonomicon: https://doc.rust-lang.org/beta/nomicon/phantom-data.html
pub struct Idx<T>(u32, PhantomData<fn() -> T>);

impl<T> Idx<T> {
    pub fn to_u32(self) -> u32 {
        self.0
    }
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

impl<T> From<u32> for Idx<T> {
    #[inline]
    fn from(u: u32) -> Self {
        Idx(u, PhantomData)
    }
}

// TODO replace with TryFrom with custom NonU32IndexError
impl<T> From<usize> for Idx<T> {
    #[inline]
    fn from(u: usize) -> Self {
        Idx(
            u.try_into().expect("wasm32 only allows u32 indices"),
            PhantomData,
        )
    }
}

// Custom `Debug`: print a human-readable version of the index space T, but don't print PhantomData.
// E.g. print `Idx<Function>(3, PhantomData)` as `Function 3`
impl<T> fmt::Debug for Idx<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = std::any::type_name::<T>().split("::").last().unwrap();
        write!(f, "{} {}", type_name, self.0)
    }
}

// Implement some traits manually, because derive adds unnecessary requirements due to the `T` type
// parameter, which we don't want. (T is only a marker and not actually contained in Idx<T>.)
impl<T> Clone for Idx<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Idx<T> {}

impl<T> PartialEq for Idx<T> {
    fn eq(&self, other: &Idx<T>) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Idx<T> {}

impl<T> hash::Hash for Idx<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.0)
    }
}

// Marker trait, showing that `Idx<T>` is compatible with nohash_hasher.
impl<T> nohash_hasher::IsEnabled for Idx<T> {}

impl<T> PartialOrd for Idx<T> {
    fn partial_cmp(&self, other: &Idx<T>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Idx<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Serialize for Idx<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/// Similar to indices, labels are just a typed wrapper around numbers in the binary format.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Label(u32);

impl Label {
    pub fn to_u32(self) -> u32 {
        self.0
    }
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
}

impl From<u32> for Label {
    #[inline]
    fn from(u: u32) -> Self {
        Label(u)
    }
}

// TODO replace with TryFrom with custom NonU32IndexError
impl From<usize> for Label {
    #[inline]
    fn from(u: usize) -> Self {
        Label(u.try_into().expect("wasm32 only allows u32 labels"))
    }
}

impl Serialize for Label {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/* Overall module structure, sections. */

/// A top-level WebAssembly module.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Module {
    // From the name section, if present, e.g., compiler-generated debug info.
    pub name: Option<String>,

    pub functions: Vec<Function>,
    pub globals: Vec<Global>,

    // TODO make these options to ensure there is only a single one of each
    pub tables: Vec<Table>,
    pub memories: Vec<Memory>,

    pub start: Option<Idx<Function>>,

    pub custom_sections: Vec<RawCustomSection>,

    pub metadata: ModuleMetadata,
}

// TODO Crazy idea: make Module (and Function, Global) generic over InstrRepr, which
// could even store the original WebAssembly bytes as a vector and only parse on demand,
// which is more compact, requires less allocations, and is (obviously) faster on module creation.
// Alternatively, the InstrRepr could also be a tree of blocks, or flat, or even a completely
// diferent language, such as Wimpl or Stackl.

impl Module {
    pub fn new() -> Self {
        Self::default()
    }

    // TODO Generify this to work for any R: io::Read.
    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, Offsets, ParseWarnings), ParseError> {
        crate::parse::parse_module(bytes)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<(Self, Offsets, ParseWarnings), ParseError> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    // TODO Some standard version that prints warnings?
    // pub fn from_file_with_offsets_wasmparser(path: impl AsRef<Path>) -> Result<(Self, Offsets), Box<dyn std::error::Error>> {
    //     let bytes = std::fs::read(path)?;
    //     let (module, offsets, warnings) = ast::wasmparser::parser::parse_module_with_offsets(&bytes)?;
    //     for warning in warnings {
    //         println!("warning: {}", warning);
    //         use std::error::Error;
    //         if let Some(source) = warning.source() {
    //             println!("caused by: {}", source);
    //         }
    //     }
    //     Ok((module, offsets))
    // }

    // TODO Generify this to work for any W: io::Write.
    // Unfortunately, wasm-encode only offers its `Encode` trait for `Vec<u8>`,
    // so it is not quite so easy.
    pub fn to_bytes(&self) -> Result<Vec<u8>, EncodeError> {
        crate::encode::encode_module(self)
    }

    pub fn to_file(&self, path: impl AsRef<Path>) -> Result<usize, EncodeError> {
        let bytes = self.to_bytes()?;
        let len = bytes.len();
        std::fs::write(path, bytes)?;
        Ok(len)
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct ModuleMetadata {
    used_extensions: Vec<WasmExtension>,
    // TODO
    // original_section_offsets: SectionOffsets
}

impl ModuleMetadata {
    pub fn add_used_extension(&mut self, extension: WasmExtension) {
        if !self.used_extensions.contains(&extension) {
            self.used_extensions.push(extension);
        }
    }

    pub fn used_extensions(&self) -> impl Iterator<Item = WasmExtension> + '_ {
        self.used_extensions.iter().copied()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ImportOrPresent<T> {
    Import(String, String),
    Present(T),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Function {
    // Type is inlined here compared to low-level/binary/spec representation.
    pub type_: FunctionType,
    pub code: ImportOrPresent<Code>,
    // Functions/globals/memories/tables can be exported multiple times under different names.
    // But the export names must be unique (not ensured in this representation!).
    pub export: Vec<String>,
    // From the name section, if present, e.g., compiler-generated debug info.
    pub name: Option<String>,
    // Invariant: param_names.len() == type_.inputs().len(), i.e., one optional name per type.
    // TODO Since you cannot access param_names mutably without checking the invariant before,
    // it is currently impossible to change the number of function parameters without breaking the
    // invariant.
    // However, so far it was never necessary to change the type signature of an existing function.
    param_names: Vec<Option<String>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Global {
    pub type_: GlobalType,
    pub init: ImportOrPresent<Expr>,
    pub export: Vec<String>,
    // TODO name from name section.
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Table {
    pub limits: Limits,
    // Unlike functions and globals, an imported table can still be initialized with elements.
    pub import: Option<(String, String)>,
    pub elements: Vec<Element>,
    pub export: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Memory {
    pub limits: Limits,
    // Unlike functions and globals, an imported memory can still be initialized with data elements.
    pub import: Option<(String, String)>,
    pub data: Vec<Data>,
    pub export: Vec<String>,
}

// TODO rename: Body, and CodeOrImport -> BodyOrImport
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Code {
    pub locals: Vec<Local>,
    // TODO rename to instrs
    pub body: Expr,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Local {
    pub type_: ValType,
    // From the name section, if present, e.g., compiler-generated debug info.
    pub name: Option<String>,
}

// For function parameters, the name and type are spread over the Function.type_ and
// Function.param_names fields, so we cannot hand out a single reference like for &Local.
// Instead we have two reference types (one mutable, one not) for the (non-explicit) Param that
// encapsulate the two separate references to type_ and name.
// As a common "supertype" to both Param and Local (mutable) references, we also have
// ParamOrLocalRef/ParamOrLocalMut with type and name accessor functions.

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ParamOrLocalRef<'a> {
    Param(ParamRef<'a>),
    Local(&'a Local),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ParamRef<'a> {
    // ValType is a Copy-type and smaller than a pointer, so store as value instead of reference.
    pub type_: ValType,
    pub name: Option<&'a str>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Element {
    pub offset: Expr,
    pub functions: Vec<Idx<Function>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Data {
    pub offset: Expr,
    pub bytes: Vec<u8>,
}

/// Metainformation how low-level sections and function bodies map to byte offsets in the binary.
// TODO Attach either directly to functions/sections or to the module (but rather the former, otherwise it can get easily lost).
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Offsets {
    /// Section offsets point to the beginning of the content of a section, i.e., after the size.
    pub sections: Vec<(SectionId, usize)>,
    /// Code offsets are only present for non-imported function, and also point to after the size
    /// in the code element (similar to section offsets).
    pub functions_code: Vec<(Idx<Function>, usize)>,
}

impl Offsets {
    /// Returns the offsets for all sections with the given id.
    pub fn section_offsets(&self, section: SectionId) -> Vec<usize> {
        self.sections
            .iter()
            .cloned()
            .filter_map(|(sec, offset)| if sec == section { Some(offset) } else { None })
            .collect()
    }

    /// Returns the (original) function index with the  given offset of its code (if any).
    pub fn function_offset_to_idx(&self, code_offset: usize) -> Option<Idx<Function>> {
        self.functions_code
            .iter()
            .cloned()
            .find_map(|(func, offset)| {
                if offset == code_offset {
                    Some(func)
                } else {
                    None
                }
            })
    }

    /// Returns the code offset of the (original) function with the given index (if any).
    pub fn function_idx_to_offset(&self, idx: Idx<Function>) -> Option<usize> {
        self.functions_code
            .iter()
            .cloned()
            .find_map(|(func, offset)| if func == idx { Some(offset) } else { None })
    }
}

/// A not-yet-parsed custom section.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RawCustomSection {
    pub name: String,
    pub content: Vec<u8>,
    /// The section that came _before_ this custom section,
    /// `None` if this was the first section in the binary.
    /// Used during serialization to place the custom section at the right order/position.
    pub previous_section: Option<SectionId>,
}

/// Marker for the different sections in a wasm module,
/// used for ordering (custom) sections during serialization.
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum SectionId {
    // Order is important! Follows the ordering of sections in the binary format
    // (except for custom sections, which can appear anywhere).
    // https://webassembly.github.io/spec/core/binary/modules.html#binary-module
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
    Custom(String),
}

/* Code. */

pub type Expr = Vec<Instr>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Memarg {
    /// The alignment of load/stores is just a hint for the VM that says "the effective address of
    /// this load/store should be aligned to <alignment>".
    /// However, if that hint is wrong and the actual address is not aligned, the load/store still
    /// produces the same behavior, potentially just much slower.
    /// (Since the underlying architecture might issue a trap/signal/exception that must be handled.)
    ///
    /// In the binary, the alignment is stored as the exponent of a power of 2.
    /// That is, the actual alignment value will be 2^alignment_exp.
    ///
    /// The actual alignment must be a power of 2 and smaller or equal to the "natural alignment"
    /// of the load/store instruction.
    /// The default alignment (e.g., if none is given in the text format) is the natural alignment
    /// (not zero!).
    ///
    /// See https://webassembly.github.io/spec/core/syntax/instructions.html#memory-instructions
    /// and https://webassembly.github.io/spec/core/text/instructions.html#memory-instructions.
    pub alignment_exp: u8,

    pub offset: u32,
}

impl Memarg {
    pub fn default(op: impl MemoryOp) -> Self {
        Self {
            offset: 0,
            alignment_exp: op.natural_alignment_exp(),
        }
    }

    pub fn is_default(self, op: impl MemoryOp) -> bool {
        self == Self::default(op)
    }

    pub fn alignment(self) -> u32 {
        2u32.pow(self.alignment_exp as u32)
    }

    /// Formats non-default fields, depends on natural alignment of `op`.
    pub fn fmt(&self, f: &mut fmt::Formatter<'_>, op: impl MemoryOp) -> fmt::Result {
        let is_natural_alignment = self.alignment_exp == op.natural_alignment_exp();
        match (self.offset, is_natural_alignment) {
            (0, true) => Ok(()),
            (0, false) => write!(f, "align={}", self.alignment()),
            (_, true) => write!(f, "offset={}", self.offset),
            (_, false) => write!(f, "offset={} align={}", self.offset, self.alignment()),
        }
    }

    /// Parses Memarg, fills fields with defaults for `op`.
    // Use Result instead of Option for consistency with stdlib FromStr trait.
    #[allow(clippy::result_unit_err)]
    pub fn from_str(s: &str, op: impl MemoryOp) -> Result<Self, ()> {
        let mut result = Memarg::default(op);

        for field in s.split(' ') {
            // FIXME: Allows for the fields to appear multiple times.

            let field = field.trim();
            if field.is_empty() {
                continue;
            }

            // Wasm text format does not allow a space around the equals sign,
            // so we can directly match against offset= as a single "token".
            if let Some(rest) = field.strip_prefix("offset=") {
                result.offset = rest.parse().map_err(|_| ())?;
            } else if let Some(rest) = field.strip_prefix("align=") {
                let align: usize = rest.parse().map_err(|_| ())?;
                // FIXME: Doesn't check that align_exp is in range for u8.
                // TODO: Use usize::log2() once stabilized and TryInto.
                let align_exp = (align as f64).log2() as u8;
                result.alignment_exp = align_exp
            } else {
                // Invalid Memarg field.
                return Err(());
            }
        }

        Ok(result)
    }
}

#[test]
fn instr_size_should_not_be_too_large() {
    assert_eq!(std::mem::size_of::<LocalOp>(), 1);
    assert_eq!(std::mem::size_of::<GlobalOp>(), 1);

    assert_eq!(std::mem::size_of::<LoadOp>(), 1);
    assert_eq!(std::mem::size_of::<StoreOp>(), 1);

    assert_eq!(std::mem::size_of::<UnaryOp>(), 1);
    assert_eq!(std::mem::size_of::<BinaryOp>(), 1);

    assert_eq!(std::mem::size_of::<Idx<Local>>(), 4);
    assert_eq!(std::mem::size_of::<Idx<Global>>(), 4);
    assert_eq!(std::mem::size_of::<Idx<Function>>(), 4);
    assert_eq!(std::mem::size_of::<Label>(), 4);

    assert_eq!(std::mem::size_of::<Memarg>(), 8);

    // These are pretty large, but the only way to get it smaller is to store things out-of-line.
    assert_eq!(std::mem::size_of::<Val>(), 16);
    assert_eq!(std::mem::size_of::<Instr>(), 24);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Instr {
    // TODO: See below on `Block` for a plan on how to get rid of unreachable code.
    Unreachable,
    Nop,

    // TODO: Make highlevel::Instr nesting, i.e., Block(FunctionType, Vec<Instr>)
    // see, e.g., the reference interpreter: https://github.com/WebAssembly/spec/blob/master/interpreter/valid/valid.ml
    // This would get rid of else and end.
    // TODO: One could also remove dead code by construction, by having optional
    // terminator instructions, which move control-flow unconditionally somewhere else:
    // Unreachable, Br, BrTable, Return.
    // We would get:
    // enum TerminatorInstr { Unreachable, Br(Label), BrTable(Vec<Label>, Label), Return }
    // and remove those four from the normal instructions.
    // We would have those terminators in:
    // Block(FunctionType, Body), Loop(FunctionType, Body), If(FunctionType, Body, Option<Body>)
    // with
    // struct Body(Vec<Instr>, Option<TerminatorInstr>)
    Block(FunctionType),
    Loop(FunctionType),
    If(FunctionType),
    Else,
    End,

    Br(Label),
    // TODO: Replace with If(FunctionType, Body([], Some(Br(Label))), None)?
    BrIf(Label),
    BrTable { table: Box<[Label]>, default: Label },

    // TODO: Replace with Br(toplevel)
    Return,
    Call(Idx<Function>),
    // TODO: remove Idx<Table>, always 0 in MVP.
    CallIndirect(FunctionType, Idx<Table>),

    // TODO: Include the type explicitly in the instruction to remove
    // value-polymorphism.
    // However, this would require type checking during lowlevel parsing :(
    Drop,
    // TODO: Replace with `If([ty, ty] -> [ty], ...)
    Select,

    // TODO: Get rid of all locals by using block params and results only + a pick or copy
    // instruction, that copies the nth value on the stack to the top.
    // Benefit: fully in SSA form, decoalesced locals, liveness information explicit.
    Local(LocalOp, Idx<Local>),
    Global(GlobalOp, Idx<Global>),

    Load(LoadOp, Memarg),
    Store(StoreOp, Memarg),

    // TODO: remove Idx<Memory>, always 0 in MVP.
    MemorySize(Idx<Memory>),
    MemoryGrow(Idx<Memory>),

    Const(Val),
    Unary(UnaryOp),
    Binary(BinaryOp),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum LocalOp {
    Get,
    Set,
    Tee,
}

impl LocalOp {
    pub fn to_type(self, local_ty: ValType) -> FunctionType {
        match self {
            LocalOp::Get => FunctionType::new(&[], &[local_ty]),
            LocalOp::Set => FunctionType::new(&[local_ty], &[]),
            LocalOp::Tee => FunctionType::new(&[local_ty], &[local_ty]),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GlobalOp {
    Get,
    Set,
}

impl GlobalOp {
    pub fn to_type(self, global_ty: ValType) -> FunctionType {
        match self {
            GlobalOp::Get => FunctionType::new(&[], &[global_ty]),
            GlobalOp::Set => FunctionType::new(&[global_ty], &[]),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum LoadOp {
    I32Load,
    I64Load,
    F32Load,
    F64Load,

    I32Load8S,
    I32Load8U,
    I32Load16S,
    I32Load16U,

    I64Load8S,
    I64Load8U,
    I64Load16S,
    I64Load16U,
    I64Load32S,
    I64Load32U,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum StoreOp {
    I32Store,
    I64Store,
    F32Store,
    F64Store,

    I32Store8,
    I32Store16,

    I64Store8,
    I64Store16,
    I64Store32,
}

/// Common trait for `LoadOp` and `StoreOp`.
pub trait MemoryOp: Sized + Copy {
    fn to_name(self) -> &'static str;
    fn to_type(self) -> FunctionType;

    // See comments on Memarg type for more information on the alignment hint and natural alignment.
    fn natural_alignment_exp(self) -> u8;
    fn natural_alignment(self) -> u32 {
        2u32.pow(self.natural_alignment_exp() as u32)
    }
}

impl MemoryOp for LoadOp {
    fn to_name(self) -> &'static str {
        use LoadOp::*;
        match self {
            I32Load => "i32.load",
            I64Load => "i64.load",
            F32Load => "f32.load",
            F64Load => "f64.load",

            I32Load8S => "i32.load8_s",
            I32Load8U => "i32.load8_u",
            I32Load16S => "i32.load16_s",
            I32Load16U => "i32.load16_u",

            I64Load8S => "i64.load8_s",
            I64Load8U => "i64.load8_u",
            I64Load16S => "i64.load16_s",
            I64Load16U => "i64.load16_u",
            I64Load32S => "i64.load32_s",
            I64Load32U => "i64.load32_u",
        }
    }

    fn to_type(self) -> FunctionType {
        use LoadOp::*;
        use ValType::*;
        match self {
            I32Load => FunctionType::new(&[I32], &[I32]),
            I64Load => FunctionType::new(&[I32], &[I64]),
            F32Load => FunctionType::new(&[I32], &[F32]),
            F64Load => FunctionType::new(&[I32], &[F64]),

            I32Load8S => FunctionType::new(&[I32], &[I32]),
            I32Load8U => FunctionType::new(&[I32], &[I32]),
            I32Load16S => FunctionType::new(&[I32], &[I32]),
            I32Load16U => FunctionType::new(&[I32], &[I32]),
            I64Load8S => FunctionType::new(&[I32], &[I64]),
            I64Load8U => FunctionType::new(&[I32], &[I64]),
            I64Load16S => FunctionType::new(&[I32], &[I64]),
            I64Load16U => FunctionType::new(&[I32], &[I64]),
            I64Load32S => FunctionType::new(&[I32], &[I64]),
            I64Load32U => FunctionType::new(&[I32], &[I64]),
        }
    }

    fn natural_alignment_exp(self) -> u8 {
        use LoadOp::*;
        match self {
            I32Load => 2,
            I64Load => 3,
            F32Load => 2,
            F64Load => 3,

            I32Load8S => 0,
            I32Load8U => 0,
            I32Load16S => 1,
            I32Load16U => 1,
            I64Load8S => 0,
            I64Load8U => 0,
            I64Load16S => 1,
            I64Load16U => 1,
            I64Load32S => 2,
            I64Load32U => 2,
        }
    }
}

impl MemoryOp for StoreOp {
    fn to_name(self) -> &'static str {
        use StoreOp::*;
        match self {
            I32Store => "i32.store",
            I64Store => "i64.store",
            F32Store => "f32.store",
            F64Store => "f64.store",

            I32Store8 => "i32.store8",
            I32Store16 => "i32.store16",
            I64Store8 => "i64.store8",
            I64Store16 => "i64.store16",
            I64Store32 => "i64.store32",
        }
    }

    fn to_type(self) -> FunctionType {
        use StoreOp::*;
        use ValType::*;
        match self {
            I32Store => FunctionType::new(&[I32, I32], &[]),
            I64Store => FunctionType::new(&[I32, I64], &[]),
            F32Store => FunctionType::new(&[I32, F32], &[]),
            F64Store => FunctionType::new(&[I32, F64], &[]),

            I32Store8 => FunctionType::new(&[I32, I32], &[]),
            I32Store16 => FunctionType::new(&[I32, I32], &[]),
            I64Store8 => FunctionType::new(&[I32, I64], &[]),
            I64Store16 => FunctionType::new(&[I32, I64], &[]),
            I64Store32 => FunctionType::new(&[I32, I64], &[]),
        }
    }

    fn natural_alignment_exp(self) -> u8 {
        use StoreOp::*;
        match self {
            I32Store => 2,
            I64Store => 3,
            F32Store => 2,
            F64Store => 3,

            I32Store8 => 0,
            I32Store16 => 1,
            I64Store8 => 0,
            I64Store16 => 1,
            I64Store32 => 2,
        }
    }
}

impl fmt::Display for LoadOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_name())
    }
}

impl fmt::Display for StoreOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_name())
    }
}

impl FromStr for LoadOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use LoadOp::*;
        Ok(match s {
            "i32.load" => I32Load,
            "i64.load" => I64Load,
            "f32.load" => F32Load,
            "f64.load" => F64Load,
            "i32.load8_s" => I32Load8S,
            "i32.load8_u" => I32Load8U,
            "i32.load16_s" => I32Load16S,
            "i32.load16_u" => I32Load16U,
            "i64.load8_s" => I64Load8S,
            "i64.load8_u" => I64Load8U,
            "i64.load16_s" => I64Load16S,
            "i64.load16_u" => I64Load16U,
            "i64.load32_s" => I64Load32S,
            "i64.load32_u" => I64Load32U,
            _ => return Err(()),
        })
    }
}

impl FromStr for StoreOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use StoreOp::*;
        Ok(match s {
            "i32.store" => I32Store,
            "i64.store" => I64Store,
            "f32.store" => F32Store,
            "f64.store" => F64Store,
            "i32.store8" => I32Store8,
            "i32.store16" => I32Store16,
            "i64.store8" => I64Store8,
            "i64.store16" => I64Store16,
            "i64.store32" => I64Store32,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum UnaryOp {
    I32Eqz,
    I64Eqz,

    I32Clz,
    I32Ctz,
    I32Popcnt,

    I64Clz,
    I64Ctz,
    I64Popcnt,

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,

    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,

    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,

    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,

    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,

    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,

    // sign-extension operators from sign-ext-ops proposal
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum BinaryOp {
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,

    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_name())
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_name())
    }
}

impl UnaryOp {
    pub fn to_name(&self) -> &'static str {
        use UnaryOp::*;
        match self {
            I32Eqz => "i32.eqz",
            I64Eqz => "i64.eqz",
            I32Clz => "i32.clz",
            I32Ctz => "i32.ctz",
            I32Popcnt => "i32.popcnt",
            I64Clz => "i64.clz",
            I64Ctz => "i64.ctz",
            I64Popcnt => "i64.popcnt",
            F32Abs => "f32.abs",
            F32Neg => "f32.neg",
            F32Ceil => "f32.ceil",
            F32Floor => "f32.floor",
            F32Trunc => "f32.trunc",
            F32Nearest => "f32.nearest",
            F32Sqrt => "f32.sqrt",
            F64Abs => "f64.abs",
            F64Neg => "f64.neg",
            F64Ceil => "f64.ceil",
            F64Floor => "f64.floor",
            F64Trunc => "f64.trunc",
            F64Nearest => "f64.nearest",
            F64Sqrt => "f64.sqrt",
            I32WrapI64 => "i32.wrap_i64",
            I32TruncF32S => "i32.trunc_f32_s",
            I32TruncF32U => "i32.trunc_f32_u",
            I32TruncF64S => "i32.trunc_f64_s",
            I32TruncF64U => "i32.trunc_f64_u",
            I64ExtendI32S => "i64.extend_i32_s",
            I64ExtendI32U => "i64.extend_i32_u",
            I64TruncF32S => "i64.trunc_f32_s",
            I64TruncF32U => "i64.trunc_f32_u",
            I64TruncF64S => "i64.trunc_f64_s",
            I64TruncF64U => "i64.trunc_f64_u",
            F32ConvertI32S => "f32.convert_i32_s",
            F32ConvertI32U => "f32.convert_i32_u",
            F32ConvertI64S => "f32.convert_i64_s",
            F32ConvertI64U => "f32.convert_i64_u",
            F32DemoteF64 => "f32.demote_f64",
            F64ConvertI32S => "f64.convert_i32_s",
            F64ConvertI32U => "f64.convert_i32_u",
            F64ConvertI64S => "f64.convert_i64_s",
            F64ConvertI64U => "f64.convert_i64_u",
            F64PromoteF32 => "f64.promote_f32",
            I32ReinterpretF32 => "i32.reinterpret_f32",
            I64ReinterpretF64 => "i64.reinterpret_f64",
            F32ReinterpretI32 => "f32.reinterpret_i32",
            F64ReinterpretI64 => "f64.reinterpret_i64",
            I32Extend8S => "i32.extend8_s",
            I32Extend16S => "i32.extend16_s",
            I64Extend8S => "i64.extend8_s",
            I64Extend16S => "i64.extend16_s",
            I64Extend32S => "i64.extend32_s",
        }
    }

    pub fn to_type(self) -> FunctionType {
        use UnaryOp::*;
        use ValType::*;
        match self {
            I32Eqz => FunctionType::new(&[I32], &[I32]),
            I64Eqz => FunctionType::new(&[I64], &[I32]),

            I32Clz | I32Ctz | I32Popcnt => FunctionType::new(&[I32], &[I32]),
            I64Clz | I64Ctz | I64Popcnt => FunctionType::new(&[I64], &[I64]),

            F32Abs | F32Neg | F32Ceil | F32Floor | F32Trunc | F32Nearest | F32Sqrt => {
                FunctionType::new(&[F32], &[F32])
            }
            F64Abs | F64Neg | F64Ceil | F64Floor | F64Trunc | F64Nearest | F64Sqrt => {
                FunctionType::new(&[F64], &[F64])
            }

            // conversions
            I32WrapI64 => FunctionType::new(&[I64], &[I32]),
            I32TruncF32S | I32TruncF32U => FunctionType::new(&[F32], &[I32]),
            I32TruncF64S | I32TruncF64U => FunctionType::new(&[F64], &[I32]),
            I64ExtendI32S | I64ExtendI32U => FunctionType::new(&[I32], &[I64]),
            I64TruncF32S | I64TruncF32U => FunctionType::new(&[F32], &[I64]),
            I64TruncF64S | I64TruncF64U => FunctionType::new(&[F64], &[I64]),
            F32ConvertI32S | F32ConvertI32U => FunctionType::new(&[I32], &[F32]),
            F32ConvertI64S | F32ConvertI64U => FunctionType::new(&[I64], &[F32]),
            F32DemoteF64 => FunctionType::new(&[F64], &[F32]),
            F64ConvertI32S | F64ConvertI32U => FunctionType::new(&[I32], &[F64]),
            F64ConvertI64S | F64ConvertI64U => FunctionType::new(&[I64], &[F64]),
            F64PromoteF32 => FunctionType::new(&[F32], &[F64]),
            I32ReinterpretF32 => FunctionType::new(&[F32], &[I32]),
            I64ReinterpretF64 => FunctionType::new(&[F64], &[I64]),
            F32ReinterpretI32 => FunctionType::new(&[I32], &[F32]),
            F64ReinterpretI64 => FunctionType::new(&[I64], &[F64]),
            I32Extend8S | I32Extend16S => FunctionType::new(&[I32], &[I32]),
            I64Extend8S | I64Extend16S | I64Extend32S => FunctionType::new(&[I64], &[I64]),
        }
    }
}

impl FromStr for UnaryOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UnaryOp::*;
        Ok(match s {
            "i32.eqz" => I32Eqz,
            "i64.eqz" => I64Eqz,
            "i32.clz" => I32Clz,
            "i32.ctz" => I32Ctz,
            "i32.popcnt" => I32Popcnt,
            "i64.clz" => I64Clz,
            "i64.ctz" => I64Ctz,
            "i64.popcnt" => I64Popcnt,
            "f32.abs" => F32Abs,
            "f32.neg" => F32Neg,
            "f32.ceil" => F32Ceil,
            "f32.floor" => F32Floor,
            "f32.trunc" => F32Trunc,
            "f32.nearest" => F32Nearest,
            "f32.sqrt" => F32Sqrt,
            "f64.abs" => F64Abs,
            "f64.neg" => F64Neg,
            "f64.ceil" => F64Ceil,
            "f64.floor" => F64Floor,
            "f64.trunc" => F64Trunc,
            "f64.nearest" => F64Nearest,
            "f64.sqrt" => F64Sqrt,
            "i32.wrap_i64" => I32WrapI64,
            "i32.trunc_f32_s" => I32TruncF32S,
            "i32.trunc_f32_u" => I32TruncF32U,
            "i32.trunc_f64_s" => I32TruncF64S,
            "i32.trunc_f64_u" => I32TruncF64U,
            "i64.extend_i32_s" => I64ExtendI32S,
            "i64.extend_i32_u" => I64ExtendI32U,
            "i64.trunc_f32_s" => I64TruncF32S,
            "i64.trunc_f32_u" => I64TruncF32U,
            "i64.trunc_f64_s" => I64TruncF64S,
            "i64.trunc_f64_u" => I64TruncF64U,
            "f32.convert_i32_s" => F32ConvertI32S,
            "f32.convert_i32_u" => F32ConvertI32U,
            "f32.convert_i64_s" => F32ConvertI64S,
            "f32.convert_i64_u" => F32ConvertI64U,
            "f32.demote_f64" => F32DemoteF64,
            "f64.convert_i32_s" => F64ConvertI32S,
            "f64.convert_i32_u" => F64ConvertI32U,
            "f64.convert_i64_s" => F64ConvertI64S,
            "f64.convert_i64_u" => F64ConvertI64U,
            "f64.promote_f32" => F64PromoteF32,
            "i32.reinterpret_f32" => I32ReinterpretF32,
            "i64.reinterpret_f64" => I64ReinterpretF64,
            "f32.reinterpret_i32" => F32ReinterpretI32,
            "f64.reinterpret_i64" => F64ReinterpretI64,
            "i32.extend8_s" => I32Extend8S,
            "i32.extend16_s" => I32Extend16S,
            "i64.extend8_s" => I64Extend8S,
            "i64.extend16_s" => I64Extend16S,
            "i64.extend32_s" => I64Extend32S,
            _ => return Err(()),
        })
    }
}

impl BinaryOp {
    pub fn to_name(&self) -> &'static str {
        use BinaryOp::*;
        match self {
            I32Eq => "i32.eq",
            I32Ne => "i32.ne",
            I32LtS => "i32.lt_s",
            I32LtU => "i32.lt_u",
            I32GtS => "i32.gt_s",
            I32GtU => "i32.gt_u",
            I32LeS => "i32.le_s",
            I32LeU => "i32.le_u",
            I32GeS => "i32.ge_s",
            I32GeU => "i32.ge_u",
            I64Eq => "i64.eq",
            I64Ne => "i64.ne",
            I64LtS => "i64.lt_s",
            I64LtU => "i64.lt_u",
            I64GtS => "i64.gt_s",
            I64GtU => "i64.gt_u",
            I64LeS => "i64.le_s",
            I64LeU => "i64.le_u",
            I64GeS => "i64.ge_s",
            I64GeU => "i64.ge_u",
            F32Eq => "f32.eq",
            F32Ne => "f32.ne",
            F32Lt => "f32.lt",
            F32Gt => "f32.gt",
            F32Le => "f32.le",
            F32Ge => "f32.ge",
            F64Eq => "f64.eq",
            F64Ne => "f64.ne",
            F64Lt => "f64.lt",
            F64Gt => "f64.gt",
            F64Le => "f64.le",
            F64Ge => "f64.ge",
            I32Add => "i32.add",
            I32Sub => "i32.sub",
            I32Mul => "i32.mul",
            I32DivS => "i32.div_s",
            I32DivU => "i32.div_u",
            I32RemS => "i32.rem_s",
            I32RemU => "i32.rem_u",
            I32And => "i32.and",
            I32Or => "i32.or",
            I32Xor => "i32.xor",
            I32Shl => "i32.shl",
            I32ShrS => "i32.shr_s",
            I32ShrU => "i32.shr_u",
            I32Rotl => "i32.rotl",
            I32Rotr => "i32.rotr",
            I64Add => "i64.add",
            I64Sub => "i64.sub",
            I64Mul => "i64.mul",
            I64DivS => "i64.div_s",
            I64DivU => "i64.div_u",
            I64RemS => "i64.rem_s",
            I64RemU => "i64.rem_u",
            I64And => "i64.and",
            I64Or => "i64.or",
            I64Xor => "i64.xor",
            I64Shl => "i64.shl",
            I64ShrS => "i64.shr_s",
            I64ShrU => "i64.shr_u",
            I64Rotl => "i64.rotl",
            I64Rotr => "i64.rotr",
            F32Add => "f32.add",
            F32Sub => "f32.sub",
            F32Mul => "f32.mul",
            F32Div => "f32.div",
            F32Min => "f32.min",
            F32Max => "f32.max",
            F32Copysign => "f32.copysign",
            F64Add => "f64.add",
            F64Sub => "f64.sub",
            F64Mul => "f64.mul",
            F64Div => "f64.div",
            F64Min => "f64.min",
            F64Max => "f64.max",
            F64Copysign => "f64.copysign",
        }
    }

    pub fn to_type(self) -> FunctionType {
        use BinaryOp::*;
        use ValType::*;
        match self {
            I32Eq | I32Ne | I32LtS | I32LtU | I32GtS | I32GtU | I32LeS | I32LeU | I32GeS
            | I32GeU => FunctionType::new(&[I32, I32], &[I32]),
            I64Eq | I64Ne | I64LtS | I64LtU | I64GtS | I64GtU | I64LeS | I64LeU | I64GeS
            | I64GeU => FunctionType::new(&[I64, I64], &[I32]),

            F32Eq | F32Ne | F32Lt | F32Gt | F32Le | F32Ge => FunctionType::new(&[F32, F32], &[I32]),
            F64Eq | F64Ne | F64Lt | F64Gt | F64Le | F64Ge => FunctionType::new(&[F64, F64], &[I32]),

            I32Add | I32Sub | I32Mul | I32DivS | I32DivU | I32RemS | I32RemU | I32And | I32Or
            | I32Xor | I32Shl | I32ShrS | I32ShrU | I32Rotl | I32Rotr => {
                FunctionType::new(&[I32, I32], &[I32])
            }
            I64Add | I64Sub | I64Mul | I64DivS | I64DivU | I64RemS | I64RemU | I64And | I64Or
            | I64Xor | I64Shl | I64ShrS | I64ShrU | I64Rotl | I64Rotr => {
                FunctionType::new(&[I64, I64], &[I64])
            }
            F32Add | F32Sub | F32Mul | F32Div | F32Min | F32Max | F32Copysign => {
                FunctionType::new(&[F32, F32], &[F32])
            }
            F64Add | F64Sub | F64Mul | F64Div | F64Min | F64Max | F64Copysign => {
                FunctionType::new(&[F64, F64], &[F64])
            }
        }
    }
}

impl FromStr for BinaryOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BinaryOp::*;
        Ok(match s {
            "i32.eq" => I32Eq,
            "i32.ne" => I32Ne,
            "i32.lt_s" => I32LtS,
            "i32.lt_u" => I32LtU,
            "i32.gt_s" => I32GtS,
            "i32.gt_u" => I32GtU,
            "i32.le_s" => I32LeS,
            "i32.le_u" => I32LeU,
            "i32.ge_s" => I32GeS,
            "i32.ge_u" => I32GeU,
            "i64.eq" => I64Eq,
            "i64.ne" => I64Ne,
            "i64.lt_s" => I64LtS,
            "i64.lt_u" => I64LtU,
            "i64.gt_s" => I64GtS,
            "i64.gt_u" => I64GtU,
            "i64.le_s" => I64LeS,
            "i64.le_u" => I64LeU,
            "i64.ge_s" => I64GeS,
            "i64.ge_u" => I64GeU,
            "f32.eq" => F32Eq,
            "f32.ne" => F32Ne,
            "f32.lt" => F32Lt,
            "f32.gt" => F32Gt,
            "f32.le" => F32Le,
            "f32.ge" => F32Ge,
            "f64.eq" => F64Eq,
            "f64.ne" => F64Ne,
            "f64.lt" => F64Lt,
            "f64.gt" => F64Gt,
            "f64.le" => F64Le,
            "f64.ge" => F64Ge,
            "i32.add" => I32Add,
            "i32.sub" => I32Sub,
            "i32.mul" => I32Mul,
            "i32.div_s" => I32DivS,
            "i32.div_u" => I32DivU,
            "i32.rem_s" => I32RemS,
            "i32.rem_u" => I32RemU,
            "i32.and" => I32And,
            "i32.or" => I32Or,
            "i32.xor" => I32Xor,
            "i32.shl" => I32Shl,
            "i32.shr_s" => I32ShrS,
            "i32.shr_u" => I32ShrU,
            "i32.rotl" => I32Rotl,
            "i32.rotr" => I32Rotr,
            "i64.add" => I64Add,
            "i64.sub" => I64Sub,
            "i64.mul" => I64Mul,
            "i64.div_s" => I64DivS,
            "i64.div_u" => I64DivU,
            "i64.rem_s" => I64RemS,
            "i64.rem_u" => I64RemU,
            "i64.and" => I64And,
            "i64.or" => I64Or,
            "i64.xor" => I64Xor,
            "i64.shl" => I64Shl,
            "i64.shr_s" => I64ShrS,
            "i64.shr_u" => I64ShrU,
            "i64.rotl" => I64Rotl,
            "i64.rotr" => I64Rotr,
            "f32.add" => F32Add,
            "f32.sub" => F32Sub,
            "f32.mul" => F32Mul,
            "f32.div" => F32Div,
            "f32.min" => F32Min,
            "f32.max" => F32Max,
            "f32.copysign" => F32Copysign,
            "f64.add" => F64Add,
            "f64.sub" => F64Sub,
            "f64.mul" => F64Mul,
            "f64.div" => F64Div,
            "f64.min" => F64Min,
            "f64.max" => F64Max,
            "f64.copysign" => F64Copysign,
            _ => return Err(()),
        })
    }
}

impl Instr {
    /// Returns the instruction name as in the WebAssembly specification and
    /// text format.
    /// This is only the mnemonic, without instruction arguments.
    pub fn to_name(&self) -> &'static str {
        use Instr::*;
        match *self {
            Unreachable => "unreachable",
            Nop => "nop",

            Block(_) => "block",
            Loop(_) => "loop",
            If(_) => "if",
            Else => "else",
            End => "end",

            Br(_) => "br",
            BrIf(_) => "br_if",
            BrTable { .. } => "br_table",

            Return => "return",
            Call(_) => "call",
            CallIndirect(_, _) => "call_indirect",

            Drop => "drop",
            Select => "select",

            Local(LocalOp::Get, _) => "local.get",
            Local(LocalOp::Set, _) => "local.set",
            Local(LocalOp::Tee, _) => "local.tee",
            Global(GlobalOp::Get, _) => "global.get",
            Global(GlobalOp::Set, _) => "global.set",

            MemorySize(_) => "memory.size",
            MemoryGrow(_) => "memory.grow",

            Const(Val::I32(_)) => "i32.const",
            Const(Val::I64(_)) => "i64.const",
            Const(Val::F32(_)) => "f32.const",
            Const(Val::F64(_)) => "f64.const",

            Load(op, _) => op.to_name(),
            Store(op, _) => op.to_name(),
            Unary(op) => op.to_name(),
            Binary(op) => op.to_name(),
        }
    }

    /// Returns a type for those instructions, for which it can be determined
    /// by just looking at the instruction, and which does neither need to
    /// additional context information like the function or module, nor type
    /// inference (for stack-polymorphic and value-polymorphic instructions).
    pub fn simple_type(&self) -> Option<FunctionType> {
        use Instr::*;
        use ValType::*;
        match *self {
            Nop => Some(FunctionType::new(&[], &[])),
            Load(ref op, _) => Some(op.to_type()),
            Store(ref op, _) => Some(op.to_type()),
            MemorySize(_) => Some(FunctionType::new(&[], &[I32])),
            MemoryGrow(_) => Some(FunctionType::new(&[I32], &[I32])),
            Const(ref val) => Some(FunctionType::new(&[], &[val.to_type()])),
            Unary(ref op) => Some(op.to_type()),
            Binary(ref op) => Some(op.to_type()),
            CallIndirect(ref func_ty, _) => Some(FunctionType::from_iter(
                func_ty.inputs().iter().copied().chain(std::iter::once(I32)),
                func_ty.results().iter().copied(),
            )),

            // Difficult because of nesting and block types.
            Block(_) | Loop(_) | If(_) | Else | End => None,
            // Depends on the branch target block.
            Br(_) | BrIf(_) | BrTable { .. } => None,
            // Need to inspect the current/called function type.
            Return | Call(_) => None,
            // Need lookup in locals/globals
            Local(_, _) | Global(_, _) => None,
            // Value-polymorphic, need abstract type stack.
            Drop | Select => None,
            // Stack-polymorphic, needs type inference (br* above as well).
            Unreachable => None,
        }
    }
}

impl FromStr for Instr {
    // TODO return more elaborate error type than just `()`.
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        use Instr::*;

        fn parse_idx<T>(str: &str) -> Result<Idx<T>, ()> {
            let int: usize = str.parse().map_err(|_| ())?;
            Ok(int.into())
        }
        fn parse_label(str: &str) -> Result<Label, ()> {
            let u: usize = str.parse().map_err(|_| ())?;
            Ok(u.into())
        }

        let (operator, rest) = str.split_once(char::is_whitespace).ok_or(())?;
        Ok(match operator {
            "unreachable" => Unreachable,
            "nop" => Nop,

            "block" => Block(FunctionType::from_str(rest)?),
            "loop" => Loop(FunctionType::from_str(rest)?),
            "if" => If(FunctionType::from_str(rest)?),

            "else" => Else,
            "end" => End,

            "br" => Br(parse_label(rest)?),
            "br_if" => BrIf(parse_label(rest)?),
            "br_table" => {
                let mut labels = rest
                    .split_whitespace()
                    .map(parse_label)
                    .collect::<Result<Vec<_>, _>>()?;
                // The last label is the default label (which must be present).
                let default = labels.pop().ok_or(())?;
                BrTable {
                    table: labels.into_boxed_slice(),
                    default,
                }
            }

            "return" => Return,
            "call" => {
                let func_idx = parse_idx(rest)?;
                Call(func_idx)
            }
            "call_indirect" => {
                let ty = FunctionType::from_str(rest)?;
                // For the WebAssembly MVP there is only a single table, so the
                // table index was not printed. Instead assume 0.
                let table_idx = Idx::from(0u32);
                CallIndirect(ty, table_idx)
            }

            "drop" => Drop,
            "select" => Select,

            "local.get" => Local(LocalOp::Get, parse_idx(rest)?),
            "local.set" => Local(LocalOp::Set, parse_idx(rest)?),
            "local.tee" => Local(LocalOp::Tee, parse_idx(rest)?),
            "global.get" => Global(GlobalOp::Get, parse_idx(rest)?),
            "global.set" => Global(GlobalOp::Set, parse_idx(rest)?),

            // For the WebAssembly MVP there is only a single memory, so the
            // memory index was not printed. Instead assume 0.
            "memory.size" => MemorySize(Idx::from(0u32)),
            "memory.grow" => MemoryGrow(Idx::from(0u32)),

            "i32.const" => Const(Val::from_str(rest, ValType::I32)?),
            "i64.const" => Const(Val::from_str(rest, ValType::I64)?),
            "f32.const" => Const(Val::from_str(rest, ValType::F32)?),
            "f64.const" => Const(Val::from_str(rest, ValType::F64)?),

            op if LoadOp::from_str(op).is_ok() => {
                let op = LoadOp::from_str(op).unwrap();
                Load(op, Memarg::from_str(rest, op)?)
            }
            op if StoreOp::from_str(op).is_ok() => {
                let op = StoreOp::from_str(op).unwrap();
                Store(op, Memarg::from_str(rest, op)?)
            }

            op if UnaryOp::from_str(op).is_ok() => UnaryOp::from_str(op).map(Unary)?,
            op if BinaryOp::from_str(op).is_ok() => BinaryOp::from_str(op).map(Binary)?,

            _ => return Err(()),
        })
    }
}

// FIXME Does not respect the width formatting modifier.
impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_name())?;

        // add arguments if instructions has any
        use self::Instr::*;
        match self {
            // instructions without arguments
            Unreachable | Nop | Drop | Select | Return | Else | End | MemorySize(_)
            | MemoryGrow(_) | Unary(_) | Binary(_) => Ok(()),

            Block(ty) | Loop(ty) | If(ty) => write!(f, " {ty}"),

            Br(label) => write!(f, " {}", label.to_u32()),
            BrIf(label) => write!(f, " {}", label.to_u32()),
            BrTable { table, default } => {
                for label in table.iter() {
                    write!(f, " {}", label.to_u32())?;
                }
                write!(f, " {}", default.to_u32())
            }

            Call(func_idx) => write!(f, " {}", func_idx.to_u32()),
            // We don't print the table index, because we also don't for memory.size and memory.grow,
            // and because in the MVP the table index is going to be 0 anyway.
            CallIndirect(func_ty, _table_idx) => write!(f, " {func_ty}"),

            Local(_, local_idx) => write!(f, " {}", local_idx.to_u32()),
            Global(_, global_idx) => write!(f, " {}", global_idx.to_u32()),

            Load(op, memarg) => {
                if !memarg.is_default(*op) {
                    f.write_str(" ")?;
                }
                memarg.fmt(f, *op)
            }
            Store(op, memarg) => {
                if !memarg.is_default(*op) {
                    f.write_str(" ")?;
                }
                memarg.fmt(f, *op)
            }

            Const(val) => write!(f, " {val}"),
        }
    }
}

/* Impls/functions for typical use cases on WASM modules. */

impl Module {
    // Convenient iterators over functions, globals, tables, and memories that include the (typed,
    // high-level) index as well.
    // TODO Add _mut variants for globals, tables, and memories, if needed.

    pub fn functions(&self) -> impl Iterator<Item = (Idx<Function>, &Function)> {
        self.functions
            .iter()
            .enumerate()
            .map(|(i, f)| (i.into(), f))
    }

    pub fn functions_mut(&mut self) -> impl Iterator<Item = (Idx<Function>, &mut Function)> {
        self.functions
            .iter_mut()
            .enumerate()
            .map(|(i, f)| (i.into(), f))
    }

    pub fn globals(&self) -> impl Iterator<Item = (Idx<Global>, &Global)> {
        self.globals.iter().enumerate().map(|(i, g)| (i.into(), g))
    }

    pub fn tables(&self) -> impl Iterator<Item = (Idx<Table>, &Table)> {
        self.tables.iter().enumerate().map(|(i, t)| (i.into(), t))
    }

    pub fn memories(&self) -> impl Iterator<Item = (Idx<Memory>, &Memory)> {
        self.memories.iter().enumerate().map(|(i, m)| (i.into(), m))
    }

    // Convenient accessors of functions for the typed, high-level index.
    // TODO Add the same for globals, tables, and memories, if needed.

    pub fn function(&self, idx: Idx<Function>) -> &Function {
        &self.functions[idx.to_usize()]
    }

    pub fn function_mut(&mut self, idx: Idx<Function>) -> &mut Function {
        &mut self.functions[idx.to_usize()]
    }

    pub fn global(&self, idx: Idx<Global>) -> &Global {
        &self.globals[idx.to_usize()]
    }

    pub fn global_mut(&mut self, idx: Idx<Global>) -> &mut Global {
        &mut self.globals[idx.to_usize()]
    }

    pub fn add_function(
        &mut self,
        type_: FunctionType,
        locals: Vec<ValType>,
        body: Vec<Instr>,
    ) -> Idx<Function> {
        self.functions.push(Function::new(
            type_,
            Code {
                locals: locals.into_iter().map(Local::new).collect(),
                body,
            },
            Vec::new(),
        ));
        (self.functions.len() - 1).into()
    }

    pub fn add_function_import(
        &mut self,
        type_: FunctionType,
        module: String,
        name: String,
    ) -> Idx<Function> {
        self.functions
            .push(Function::new_imported(type_, module, name, Vec::new()));
        (self.functions.len() - 1).into()
    }

    pub fn add_global(
        &mut self,
        type_: ValType,
        mut_: Mutability,
        init: Vec<Instr>,
    ) -> Idx<Global> {
        self.globals.push(Global {
            type_: GlobalType(type_, mut_),
            init: ImportOrPresent::Present(init),
            export: Vec::new(),
        });
        (self.globals.len() - 1).into()
    }
}

impl Function {
    pub fn new(type_: FunctionType, code: Code, export: Vec<String>) -> Self {
        Function {
            type_,
            code: ImportOrPresent::Present(code),
            export,
            name: None,
            param_names: Vec::new(),
        }
    }

    pub fn new_imported(
        type_: FunctionType,
        import_module: String,
        import_name: String,
        export: Vec<String>,
    ) -> Self {
        Function {
            type_,
            code: ImportOrPresent::Import(import_module, import_name),
            export,
            name: None,
            param_names: Vec::new(),
        }
    }

    pub fn import(&self) -> Option<(&str, &str)> {
        if let ImportOrPresent::Import(module, name) = &self.code {
            Some((module.as_str(), name.as_str()))
        } else {
            None
        }
    }

    pub fn code(&self) -> Option<&Code> {
        if let ImportOrPresent::Present(t) = &self.code {
            Some(t)
        } else {
            None
        }
    }

    pub fn code_mut(&mut self) -> Option<&mut Code> {
        if let ImportOrPresent::Present(t) = &mut self.code {
            Some(t)
        } else {
            None
        }
    }

    pub fn into_code(self) -> Option<Code> {
        if let ImportOrPresent::Present(t) = self.code {
            Some(t)
        } else {
            None
        }
    }

    pub fn instrs(&self) -> &[Instr] {
        self.code().map(|code| code.body.as_slice()).unwrap_or(&[])
    }

    pub fn instrs_mut(&mut self) -> Option<&mut Vec<Instr>> {
        self.code_mut().map(|code| &mut code.body)
    }

    pub fn instr_count(&self) -> usize {
        self.code().map(|code| code.body.len()).unwrap_or(0)
    }

    pub fn modify_instrs(&mut self, f: impl Fn(Instr) -> Vec<Instr>) {
        if let Some(body) = self.instrs_mut() {
            let new_body = Vec::with_capacity(body.len());
            let old_body = ::std::mem::replace(body, new_body);
            for instr in old_body.into_iter() {
                body.append(&mut f(instr));
            }
        }
    }

    /// add a new local with type ty and return its index
    pub fn add_fresh_local(&mut self, ty: ValType) -> Idx<Local> {
        let param_count = self.param_count();
        let locals = &mut self
            .code_mut()
            .expect("cannot add local to imported function")
            .locals;
        let new_idx = param_count + locals.len();
        locals.push(Local::new(ty));
        new_idx.into()
    }

    pub fn add_fresh_locals(&mut self, tys: &[ValType]) -> SmallVec<[Idx<Local>; 8]> {
        tys.iter().map(|ty| self.add_fresh_local(*ty)).collect()
    }

    // Functions for the number of parameters and non-parameter locals.

    pub fn param_count(&self) -> usize {
        self.type_.inputs().len()
    }

    pub fn local_count(&self) -> usize {
        self.code().map(|code| code.locals.len()).unwrap_or(0)
    }

    // Accessors and iterators for parameters and locals uniformly.

    pub fn param_or_local(&self, idx: Idx<Local>) -> ParamOrLocalRef {
        self.param_or_locals()
            .nth(idx.to_usize())
            .expect("invalid local index")
            .1
    }

    // FIXME no longer possible, because function type is immutable!
    // pub fn param_or_local_mut(&mut self, idx: Idx<Local>) -> ParamOrLocalMut {
    //     self.param_or_locals_mut()
    //         .nth(idx.into_inner())
    //         .expect("invalid local index")
    //         .1
    // }

    pub fn param_or_locals(&self) -> impl Iterator<Item = (Idx<Local>, ParamOrLocalRef)> {
        let params = self.params().map(|(i, p)| (i, ParamOrLocalRef::Param(p)));
        let locals = self.locals().map(|(i, l)| (i, ParamOrLocalRef::Local(l)));
        params.chain(locals)
    }

    // FIXME no longer possible, because function type is immutable!
    // pub fn param_or_locals_mut(&mut self) -> impl Iterator<Item=(Idx<Local>, ParamOrLocalMut)> {
    //     // Unfortunately, we cannot borrow self mutably twice, so we cannot adapt the code from
    //     // param_or_locals() here. (We would have to call self.params_mut() and self.locals_mut()).
    //     // Dirty hack: copy code from params_mut()/locals_mut()/code_mut().
    //     // TODO If there is a smarter way of re-using params_mut() and locals_mut(), I'd be happy to.

    //     self.assert_param_name_len_valid();

    //     let params = self.type_.inputs().iter_mut()
    //         .zip(self.param_names.iter_mut())
    //         .map(|(type_, name)| ParamMut { type_, name })
    //         .map(ParamOrLocalMut::Param);

    //     let code = if let ImportOrPresent::Present(t) = &mut self.code {
    //         Some(t)
    //     } else {
    //         None
    //     };

    //     let locals = code.into_iter()
    //         .flat_map(|code| code.locals.iter_mut())
    //         .map(ParamOrLocalMut::Local);

    //     params.chain(locals)
    //         .enumerate()
    //         .map(|(idx, element)| (idx.into(), element))
    // }

    /// Returns the parameters (type and debug name, if any) together with their index.
    pub fn params(&self) -> impl Iterator<Item = (Idx<Local>, ParamRef)> {
        self.type_.inputs().iter().enumerate().map(|(i, &type_)| {
            (
                i.into(),
                ParamRef {
                    type_,
                    name: self.param_names.get(i).and_then(|name| name.as_deref()),
                },
            )
        })
    }

    // FIXME no longer possible, because function type is immutable!
    // pub fn params_mut(&mut self) -> impl Iterator<Item=(Idx<Local>, ParamMut)> {
    //     self.assert_param_name_len_valid();

    //     self.type_.inputs().iter_mut()
    //         .zip(self.param_names.iter_mut())
    //         .enumerate()
    //         .map(|(idx, (type_, name))|
    //             (idx.into(), ParamMut { type_, name })
    //         )
    // }

    /// Returns the non-parameter locals together with their index.
    /// Returns an empty iterator for imported functions (which don't have non-param locals).
    ///
    /// Since function parameters and locals share the same index space, the returned indices will
    /// start at N, if N is the number of function parameters.
    pub fn locals(&self) -> impl Iterator<Item = (Idx<Local>, &Local)> {
        // The index of the non-parameter locals starts after the parameters.
        // This value needs to be moved (not borrowed) into the innermost closure, hence the two
        // nested move annotations on the closures below.
        let param_count = self.param_count();
        self.code().into_iter().flat_map(move |code| {
            code.locals
                .iter()
                .enumerate()
                .map(move |(idx, local)| ((param_count + idx).into(), local))
        })
    }

    pub fn locals_mut(&mut self) -> impl Iterator<Item = (Idx<Local>, &mut Local)> {
        // Only changed mutability compared with self.locals(), see comments there.
        let param_count = self.param_count();
        self.code_mut().into_iter().flat_map(move |code| {
            code.locals
                .iter_mut()
                .enumerate()
                .map(move |(idx, local)| ((param_count + idx).into(), local))
        })
    }

    // Accessors for a specific parameter/local type/name.

    /// Return the type of the function parameter or non-parameter local with index idx.
    pub fn param_or_local_type(&self, idx: Idx<Local>) -> ValType {
        let idx = idx.to_usize();
        let param_count = self.type_.inputs().len();

        if idx < param_count {
            self.type_.inputs()[idx]
        } else {
            self.code()
                .expect("imported function cannot have locals")
                .locals[idx - param_count]
                .type_
        }
    }

    /// Return the (optional) debug name of the function parameter or non-parameter local with
    /// index idx.
    pub fn param_or_local_name(&self, idx: Idx<Local>) -> Option<&str> {
        let idx = idx.to_usize();
        let param_count = self.type_.inputs().len();

        if idx < param_count {
            self.param_names.get(idx).and_then(|name| name.as_deref())
        } else {
            self.code()
                .expect("imported function cannot have locals")
                .locals[idx - param_count]
                .name
                .as_deref()
        }
    }

    /// Return a mutable reference to the (optional) debug name of the function parameter or
    /// non-parameter local with index idx.
    pub fn param_or_local_name_mut(&mut self, idx: Idx<Local>) -> &mut Option<String> {
        let idx = idx.to_usize();
        let param_count = self.type_.inputs().len();

        if idx < param_count {
            // Ensure the param_names vec is long enough.
            self.param_names.resize(param_count, None);
            &mut self.param_names[idx]
        } else {
            &mut self
                .code_mut()
                .expect("imported function cannot have locals")
                .locals[idx - param_count]
                .name
        }
    }
}

impl Local {
    pub fn new(type_: ValType) -> Self {
        Local { type_, name: None }
    }
}

impl Code {
    pub fn new() -> Self {
        Code {
            locals: Vec::new(),
            body: Vec::new(),
        }
    }
}

impl Default for Code {
    fn default() -> Self {
        Self::new()
    }
}

// See description on enum type above.
impl<'a> ParamOrLocalRef<'a> {
    pub fn type_(self) -> ValType {
        match self {
            ParamOrLocalRef::Param(param) => param.type_,
            ParamOrLocalRef::Local(local) => local.type_,
        }
    }
    pub fn name(self) -> Option<&'a str> {
        match self {
            ParamOrLocalRef::Param(param) => param.name,
            ParamOrLocalRef::Local(local) => local.name.as_deref(),
        }
    }
}

impl Global {
    pub fn new(type_: GlobalType, init: Expr) -> Self {
        Global {
            type_,
            init: ImportOrPresent::Present(init),
            export: Vec::new(),
        }
    }

    pub fn new_imported(type_: GlobalType, import_module: String, import_name: String) -> Self {
        Global {
            type_,
            init: ImportOrPresent::Import(import_module, import_name),
            export: Vec::new(),
        }
    }

    pub fn import(&self) -> Option<(&str, &str)> {
        if let ImportOrPresent::Import(module, name) = &self.init {
            Some((module.as_str(), name.as_str()))
        } else {
            None
        }
    }

    pub fn init(&self) -> Option<&Expr> {
        if let ImportOrPresent::Present(t) = &self.init {
            Some(t)
        } else {
            None
        }
    }
}

impl Table {
    pub fn new(limits: Limits) -> Table {
        Table {
            limits,
            import: None,
            elements: Vec::new(),
            export: Vec::new(),
        }
    }

    pub fn new_imported(limits: Limits, import_module: String, import_name: String) -> Table {
        Table {
            limits,
            import: Some((import_module, import_name)),
            elements: Vec::new(),
            export: Vec::new(),
        }
    }

    pub fn import(&self) -> Option<(&str, &str)> {
        self.import
            .as_ref()
            .map(|(module, name)| (module.as_str(), name.as_str()))
    }
}

impl Memory {
    pub fn new(limits: Limits) -> Memory {
        Memory {
            limits,
            import: None,
            data: Vec::new(),
            export: Vec::new(),
        }
    }

    pub fn new_imported(limits: Limits, import_module: String, import_name: String) -> Memory {
        Memory {
            limits,
            import: Some((import_module, import_name)),
            data: Vec::new(),
            export: Vec::new(),
        }
    }

    pub fn import(&self) -> Option<(&str, &str)> {
        self.import
            .as_ref()
            .map(|(module, name)| (module.as_str(), name.as_str()))
    }
}
