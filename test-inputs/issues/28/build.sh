#!/bin/bash

rm -rf build/
mkdir build/

wat2wasm memory-grow-size.wat -o build/memory-grow-size.wasm

# Instrument with Wasabi and inspect results.
cargo run --manifest-path ../../../crates/wasabi/Cargo.toml build/memory-grow-size.wasm
wasm-objdump -xhds out/memory-grow-size.wasm
cat out/memory-grow-size.wasabi.js
