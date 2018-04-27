use binary::WasmBinary;
use serde::{Serialize, Serializer};
use std::fmt::{self, Write};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

mod higherlevel;
pub mod highlevel;
pub mod lowlevel;
pub mod convert;

/* AST nodes common to high- and low-level representations. */

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum Val {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Val {
    fn to_type(&self) -> ValType {
        match *self {
            Val::I32(_) => ValType::I32,
            Val::I64(_) => ValType::I64,
            Val::F32(_) => ValType::F32,
            Val::F64(_) => ValType::F64,
        }
    }
}

/* Types */

#[derive(WasmBinary, Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ValType {
    #[tag = 0x7f] I32,
    #[tag = 0x7e] I64,
    #[tag = 0x7d] F32,
    #[tag = 0x7c] F64,
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::with_capacity(3);
        write!(s, "{:?}", self)?;
        write!(f, "{}", s.to_lowercase())
    }
}

#[derive(WasmBinary, Debug, PartialEq, Eq, Clone, Hash, Serialize, new)]
#[tag = 0x60]
pub struct FunctionType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockType(pub Option<ValType>);

#[derive(WasmBinary, Debug, Clone)]
pub struct TableType(pub ElemType, pub Limits);

#[derive(WasmBinary, Debug, Copy, Clone)]
pub enum ElemType {
    // only value in WASM version 1
    #[tag = 0x70]
    Anyfunc,
}

#[derive(WasmBinary, Debug, Clone)]
pub struct MemoryType(pub Limits);

#[derive(Debug, Copy, Clone)]
pub struct Limits {
    pub initial_size: u32,
    pub max_size: Option<u32>,
}

#[derive(WasmBinary, Debug, Copy, Clone)]
pub struct GlobalType(pub ValType, pub Mutability);

#[derive(WasmBinary, Debug, Copy, Clone)]
pub enum Mutability {
    #[tag = 0x00] Const,
    #[tag = 0x01] Mut,
}


/* Indices */

#[derive(WasmBinary)]
pub struct Idx<T>(pub usize, PhantomData<T>);

impl<T> From<usize> for Idx<T> {
    #[inline]
    fn from(u: usize) -> Self { Idx(u, PhantomData) }
}

// custom Debug: print index type T, don't print PhantomData
impl<T> fmt::Debug for Idx<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = unsafe { ::std::intrinsics::type_name::<T>() };
        let type_name = type_name.split("::").last().unwrap();
        f.write_str(type_name)?;
        f.write_char(' ')?;
        self.0.fmt(f)
    }
}

// implement some traits manually, since derive(Copy/Eq) add requirements like T: Clone/PartialEq,
// which we do not want (T is only a marker and not actually contained).
impl<T> Clone for Idx<T> {
    #[inline]
    fn clone(&self) -> Self { self.0.into() }
}

impl<T> Copy for Idx<T> {}

impl<T> PartialEq for Idx<T> {
    fn eq(&self, other: &Idx<T>) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Idx<T> {}

impl<T> Hash for Idx<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> Serialize for Idx<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

// Unit structs as markers for indices that do not have their own "content" type
// I.e., Local is just ValType, Label is not represented at all.

#[derive(Debug)]
pub struct Local;

#[derive(Debug)]
pub struct Label;


/* Code */

#[derive(WasmBinary, Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Memarg {
    pub alignment: u32,
    pub offset: u32,
}