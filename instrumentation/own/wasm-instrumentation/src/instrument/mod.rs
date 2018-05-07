// direct modification of binary, very low-level
pub mod direct;

// hook-style instrumentation, analysis happens in callbacks
mod add_hooks;
pub use self::add_hooks::add_hooks;

// used in add_hooks
mod convert_i64;
mod static_info;
mod block_stack;
mod type_stack;
mod hook_map;