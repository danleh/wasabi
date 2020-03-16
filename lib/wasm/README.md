# `wasm` library

For parsing and manipulating WebAssembly binaries, used by Wasabi internally.

Core features:
- Supports **WebAssembly version 1 (MVP)** and mutable globals, but other than that **no extensions**.
- Supports **`name` custom section**; other custom sections are passed through unmodified as bytes but have no "semantic" representation.
- Consists of four main parts:
    * **Low-level AST** data structures (in `src/ast/lowlevel.rs`), that have a 1:1 mapping to the WebAssembly binary spec.
    * **WasmBinary** trait for decoding (parsing) a binary into the low-level AST and encoding the low-level AST back into a binary. Most of the trait implementations can be mechanically derived from the low-level AST (with a procedural macro in `binary_derive/`), which cuts down maintenance.
    * **High-level AST** data structures (in `src/ast/highlevel.rs`), that is much more convenient to modify than the low-level AST directly. Also tries to be idiomatic Rust. E.g., types are inlined (and not in a separate section), imported functions/globals do not have to come before non-imported ones, etc.pp. You should always work on the high-level AST if possible.
    * **Conversion** between low-level and high-level AST and back.  
- Unfortunately, not a streaming parser, i.e., the AST "owns" all data (in Rust lingo), which means lots of memory allocation. 
