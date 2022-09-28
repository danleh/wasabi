use std::str::FromStr;
use std::{fmt, hash};
use std::convert::TryInto;
use std::marker::PhantomData;

use binary_derive::WasmBinary;
use ordered_float::OrderedFloat;
use serde::{Serialize, Serializer};

use crate::WasmBinary;
use crate::highlevel::{MemoryOp, FunctionType};

/* AST nodes common to high- and low-level representations. */

/// WebAssembly primitive values.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Val {
    I32(i32),
    I64(i64),
    // Wrap floats, such that they can be ordered and compared (unlike IEEE754 floats),
    // to make it possible, e.g., to put instructions in HashSets etc.
    // TODO replace those with bitpatterns of the floats, similar to wasmparser::Ieee32 and 64
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
}

impl Val {
	// FIXME move to ValType and instead make ValType::default_value()
    pub fn get_default_value(type_: ValType) -> Self {
        match type_ {
            ValType::I32 => Val::I32(0),
            ValType::I64 => Val::I64(0),
            ValType::F32 => Val::F32(OrderedFloat(0.0)),
            ValType::F64 => Val::F64(OrderedFloat(0.0)),
        }
    }

    /// Convert a value to the corresponding type.
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
            Val::I32(v) => write!(f, "{}", v),
            Val::I64(v) => write!(f, "{}", v),
            Val::F32(v) => write!(f, "{}", v.into_inner()),
            Val::F64(v) => write!(f, "{}", v.into_inner()),
        }
    }
}


/* Types */

#[derive(WasmBinary, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ValType {
    #[tag = 0x7f] I32,
    #[tag = 0x7e] I64,
    #[tag = 0x7d] F32,
    #[tag = 0x7c] F64,
}

#[test]
fn val_type_is_small() {
    assert_eq!(std::mem::size_of::<ValType>(), 1)
}

impl ValType {
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

    /// Convert to a single character, as used, e.g., by Emscripten.
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
            _ => return Err(())
        })
    }
}

/// In the WebAssembly MVP, blocks can return either nothing or a single value.
// TODO replace all occurrences with FunctionType once we support non-MVP binaries, then remove.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct BlockType(pub Option<ValType>);

#[test]
fn block_type_is_small() {
    assert_eq!(std::mem::size_of::<BlockType>(), 1)
}

impl FromStr for BlockType {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        // Re-use implementation for parsing `FunctionType`s.
        let func_ty = FunctionType::from_str(str)?;
        match (func_ty.inputs(), func_ty.results()) {
            ([], []) => Ok(BlockType(None)),
            ([], [ty]) => Ok(BlockType(Some(*ty))),
            // `BlockType` is a subset of all `FunctionType`s.
            _ => Err(())
        }
    }
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ty) => write!(f, "[] -> [{}]", ty),
            None => write!(f, "[] -> []"),
        }
    }
}

#[derive(WasmBinary, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TableType(pub ElemType, pub Limits);

// TODO remove once low-level parser is replaced by wasmparser.
// TODO or rename Anyref to Funcref
#[derive(WasmBinary, Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ElemType {
    // only value in WASM version 1
    #[tag = 0x70]
    Anyfunc,
}

// TODO replace with just limits
#[derive(WasmBinary, Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct MemoryType(pub Limits);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Limits {
    pub initial_size: u32,
    pub max_size: Option<u32>,
}

#[derive(WasmBinary, Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalType(pub ValType, pub Mutability);

impl fmt::Display for GlobalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.1 {
            Mutability::Const => write!(f, "{}", self.0),
            Mutability::Mut => write!(f, "mut {}", self.0),
        }
    }
}

#[derive(WasmBinary, Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Mutability {
    #[tag = 0x00] Const,
    #[tag = 0x01] Mut,
}


/* Indices */

/// A WebAssembly index into one of the possible "index spaces" (e.g., functions, globals, etc.)
/// 
/// The type parameter T is only used for static distinction between the different index spaces, 
/// but has no representation at runtime.
/// Since we don't own T, we don't want a "drop check" and must use fn() -> T, as is
/// recommended in the rustonomicon: https://doc.rust-lang.org/beta/nomicon/phantom-data.html
pub struct Idx<T>(u32, PhantomData<fn() -> T>);

impl<T> Idx<T> {
    pub fn to_u32(self) -> u32 { self.0 }
    pub fn to_usize(self) -> usize { self.0 as usize }
}

// TODO replace with TryFrom with custom NonU32IndexError
impl<T> From<usize> for Idx<T> {
    #[inline]
    fn from(u: usize) -> Self {
        Idx(u.try_into().expect("wasm32 only allows u32 indices"), PhantomData)
    }
}

impl<T> From<u32> for Idx<T> {
    #[inline]
    fn from(u: u32) -> Self {
        Idx(u, PhantomData)
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
    fn clone(&self) -> Self { self.to_usize().into() }
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
        self.0.hash(state)
    }
}

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
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

// Similar to indices, labels are just a typed wrapper around numbers in the binary format.
// TODO make consistent with Idx: make field private, use into_inner().
#[derive(WasmBinary, Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Label(u32);

impl Label {
    pub fn to_u32(self) -> u32 { self.0 }
    pub fn to_usize(self) -> usize { self.0 as usize }
}

// Unfortunately, another impl for u32 would make .into() calls ambiguous with integer literals,
// so only provide the conversion from usize.
impl From<usize> for Label {
    #[inline]
    fn from(u: usize) -> Self {
        Label(u.try_into().expect("wasm32 only allows u32 labels"))
    }
}

impl Serialize for Label {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/* Code */

#[derive(WasmBinary, Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

    // NOTE offset field must come after alignment, because that's how it is
    // layed out in the binary format. Field order is important!
    // TODO Note can be removed once lowlevel parser is removed
    pub offset: u32,
}

impl Memarg {
    pub fn default(op: impl MemoryOp) -> Self {
        Self {
            offset: 0,
            alignment_exp: op.natural_alignment_exp()
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
        match (self.offset, self.alignment_exp == op.natural_alignment_exp()) {
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
            // FIXME Allows for the fields to appear multiple times.

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
                // FIXME Doesn't check that align_exp is in range for u8.
                // TODO Use usize::log2() once stabilized and TryInto.
                let align_exp = (align as f64).log2() as u8;
                result.alignment_exp = align_exp
            } else {
                // Invalid Memarg field.
                return Err(())
            }
        }

        Ok(result)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RawCustomSection {
    pub name: String,
    pub content: Vec<u8>,
    /// The last non-custom section _before_ this custom section. 
    /// Used during serialization to place the custom section at the right order/position.
    /// If there are multiple custom sections after each other, this will be `None`, 
    /// but the custom sections' relative order will be correct, and the first one
    /// will have this set.
    // TODO fix this comment, not correct anymore: will point to any previous section
    pub after: Option<SectionId>,
}
// Order is important! Follows the ordering of sections in the binary format
// (except for custom sections, which can appear anywhere).
// https://webassembly.github.io/spec/core/binary/modules.html#binary-module
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum SectionId {
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

// TODO Remove once the old low-level parser code is gone.
impl SectionId {
    pub fn from_section(section: &crate::lowlevel::Section) -> Self {
        use crate::lowlevel::Section::*;
        use crate::lowlevel::CustomSection;
        match section {
            Type(_) => SectionId::Type,
            Import(_) => SectionId::Import,
            Function(_) => SectionId::Function,
            Table(_) => SectionId::Table,
            Memory(_) => SectionId::Memory,
            Global(_) => SectionId::Global,
            Export(_) => SectionId::Export,
            Start(_) => SectionId::Start,
            Element(_) => SectionId::Element,
            Code(_) => SectionId::Code,
            Data(_) => SectionId::Data,
            Custom(CustomSection::Name(_)) => SectionId::Custom("name".to_string()),
            Custom(CustomSection::Raw(RawCustomSection { name, .. })) => SectionId::Custom(name.clone()),
        }
    }
}