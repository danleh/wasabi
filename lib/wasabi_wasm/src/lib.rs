mod ast;
// Export AST types directly under crate, without ast prefix.
pub use crate::ast::*;
mod function_type;

mod error;
// Export error types directly under the crate.
pub use crate::error::*;

pub mod types;

mod extensions;
mod parse;
mod encode;

#[cfg(test)]
mod tests;

// See long comment on allocator performance with parallel parsing on Windows 10 in `parse.rs`.
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/*
 TODO WHEN CONTINUING
 - support multi-value, multi-table, multi-memory because they are anyway pretty much supported (and make for less special cases)
 - make AST blocks nested, remove end/else opcodes
 - remove blocktype, replace with function type (this should make our AST multi-value capable)
 - rename wasm crate to wasabi-wasm
*/
