#!/bin/sh
wat2wasm module.wat -o module.wasm
wasm2wat module.wasm -o module.wasm.wat

rustc --target=wasm32-unknown-unknown ackermann.rs -O
wasm2wat ackermann.wasm -o ackermann.wasm.wat
