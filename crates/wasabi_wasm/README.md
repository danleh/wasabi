# Wasabi's WebAssembly library

For parsing and manipulating WebAssembly binaries, used by Wasabi internally, but also potentially useful for other projects that want to analyze or manipulate WebAssembly binaries.

Core features:
- Supports **WebAssembly version 1 (MVP)** and mutable globals, but other than that **no extensions**.
- Supports **`name` custom section**; other custom sections are passed through unmodified as bytes but have no "semantic" representation.
- Fully "owned" AST (in Rust lingo), which makes manipulation simple, but incurs quite some memory allocation on initial parsing.
- The AST is more high-level and easier to work with than the binary format directly.
It is closer to the abstract structure of a module (e.g., inlined types) than the binary format (e.g., types in a separate section, refered to by indices).
- Uses [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools) crates (in particular `wasmparser` and `wasm-encode`) for the low-level parsing and encoding of the binary format.
