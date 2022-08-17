#!/bin/bash
# Compile without exceptions to make the binary a bit easier to understand/smaller.
# Compile with debug info for better function names.
# But optimize for a realistic binary.
emcc -O2 -g -fno-exceptions main.cpp -o main.wasm
wasm2wat --generate-names -f main.wasm -o main.wasm.wat

wasmtime main.wasm
# Should print B
wasmtime main.wasm asd
# Should print A
