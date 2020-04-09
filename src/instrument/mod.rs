// Direct modification of binary, very low-level. More an example for how to use the wasm library.
pub mod direct;

// Hook-style instrumentation, analysis happens in callbacks, i.e., added function imports.
pub mod add_hooks;
pub use self::add_hooks::add_hooks;
