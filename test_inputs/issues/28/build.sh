#!/bin/bash
wat2wasm memory-grow-size.wat
cargo run memory-grow-size.wasm
wasm-objdump -xhds out/memory-grow-size.wasm
cat out/memory-grow-size.wasabi.js