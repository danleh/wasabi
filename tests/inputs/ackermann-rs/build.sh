#!/bin/sh
# optimize -O to reduce size
rustc --target=wasm32-unknown-unknown -O ackermann.rs
wasm2wat ackermann.wasm -o ackermann.wat
rm -rf build/
mkdir build
mv *.wasm *.wat build/
