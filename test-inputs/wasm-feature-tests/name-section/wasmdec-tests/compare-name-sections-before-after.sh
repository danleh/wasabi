#!/bin/bash
for file in *.wasm
do
    cargo run -- --hooks nop "$file"
    wasm-objdump -xhs -j name "$file" > "$file.objdump.txt"
    wasm-objdump -xhs -j name "out/$file" > "out/$file.objdump.txt"
done
