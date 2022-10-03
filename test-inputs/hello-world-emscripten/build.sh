#!/bin/bash

# requires Emscripten and Python 3

rm -rf build/
mkdir build

emcc hello.c -s WASM=1 -o build/hello.html
wasm2wat --generate-names build/hello.wasm -o build/hello.wasm.wat

# python3 -m http.server 8000
