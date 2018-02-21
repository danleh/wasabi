#!/bin/sh
# use native Rust wasm backend
# must use optimizations because of some LLVM bug, see: https://github.com/rust-lang/rust/issues/46367
rustc --target=wasm32-unknown-unknown -O hello.rs -C lto -o hello.wasm
# TODO should no longer be necessary once Rust switches to LLD, see: https://github.com/rust-lang-nursery/rust-wasm/issues/2 and https://github.com/rust-lang/rust/pull/48125
wasm-gc hello.wasm -o hello.gc.wasm
# see https://github.com/rust-lang-nursery/rust-wasm/blob/master/src/hello-world.md
wasm-opt -Os hello.gc.wasm -o hello.gc.opt.wasm
wasm2wat hello.gc.opt.wasm -o hello.gc.opt.wat
