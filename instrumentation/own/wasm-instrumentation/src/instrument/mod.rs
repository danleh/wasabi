// direct modification of binary, very low-level
pub mod direct;

// hook-style instrumentation, analysis happens in callbacks
mod add_hooks;
pub use self::add_hooks::add_hooks;