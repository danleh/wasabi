#!/bin/sh

rm -rf build/
mkdir build

wat2wasm alignment.wat -o build/alignment.wasm
wasm-objdump -d build/alignment.wasm > build/alignment.wasm-objdump.txt
