mod ast;
// Export AST types directly under crate, without ast prefix.
pub use crate::ast::*;
mod function_type;

mod error;
// Export error types directly under the crate.
pub use crate::error::*;

pub mod types;

mod encode;
mod extensions;
mod parse;

#[cfg(test)]
mod tests;

// See long comment on Windows 10 allocator performance with parallel parsing in `parse.rs`.
#[cfg(target_os = "windows")]
use mimalloc::MiMalloc;
#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/*
 TODO WHEN CONTINUING
 - support multi-table, multi-memory because they are anyway pretty much supported (and make for less special cases)
 - make AST blocks nested, remove end/else opcodes
*/
