#!/bin/sh

rm -rf build/
mkdir build/

# Requires WebAssembly on WASI Rust target, install with:
rustup target add wasm32-wasi

# Version at the time of building the .wasm file:
rustc --version > build/rust-version

# Optimize with -O to reduce size.
rustc --target=wasm32-wasi -O ackermann.rs -o build/ackermann.wasm

# Generate text format version for human inspection.
wasm2wat --generate-names build/ackermann.wasm -o build/ackermann.wasm.wat
