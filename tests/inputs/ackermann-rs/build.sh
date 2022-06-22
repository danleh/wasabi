#!/bin/sh

# requires WASI rust target, install with:
# rustup target add wasm32-wasi

rm -rf build/
mkdir build/

# optimize -O to reduce size
rustc --target=wasm32-wasi -O ackermann.rs -o build/ackermann.wasm
wasm2wat --generate-names build/ackermann.wasm -o build/ackermann.wasm.wat
