This document shall list the largest breaking changes for Wasabi's wasm library.

# v0.5.0 (2020-05-27)

- Save and return a mapping of sections <-> byte offsets and function bodies ("code") to byte offsets during decoding. Changes the signature of the `WasmBinary::decode` trait method.

# v0.4.0 (2020-03-17)

- AST types are directly under the crate, not prefixed with `ast::` anymore.
- Typed error handling for parsing low-level AST (i.e., no longer based on `io::Result`).
- High-level functions and global types now model that they can be exclusively either imported or not.
- Merged (redundant) InstrType and FunctionType.
- Remove Global/Local prefix from LocalGet/LocalSet/LocalTee/GlobalSet/GlobalGet.
- Use `typename` from stdlib instead of external dependency. Requires up to date Rust version. 
- Handling of custom section order while parsing/serializing.
- Semantic representation of name custom section (debug info) in the low-level AST, and inlined names in the high-level AST.

# v0.3.0 and before

Not a crate of its own yet, but directly part of Wasabi.
Most features only partially working or not at all.
Also no documentation.
