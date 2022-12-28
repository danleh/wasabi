#!/bin/bash

rm -rf build/
mkdir build/

# Requires Emscripten.
# Version at the time of building the .wasm file:
emcc --version > build/emcc-version

# Optimize for size.
# Also generates a .wasm binary, included in the website.
emcc -Os hello.c -o build/hello.html

wasm2wat --generate-names build/hello.wasm -o build/hello.wasm.wat

# If you want to serve the website locally to open it in a browser:
# python3 -m http.server 8000
