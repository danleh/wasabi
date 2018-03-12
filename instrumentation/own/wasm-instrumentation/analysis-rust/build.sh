#!/bin/sh
# use native Rust wasm backend
# must use optimizations because of some LLVM bug, see: https://github.com/rust-lang/rust/issues/46367
# use LTO for smaller module sizes, see: https://github.com/rust-lang/rust/pull/48125
rustc --target=wasm32-unknown-unknown -O analysis.rs -C lto
wasm-objdump analysis.wasm -hx