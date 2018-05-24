#!/bin/bash
emcc hello.c -s WASM=1 -o hello.html
wasm2wat hello.wasm -o hello.wat
rm -rf build/
mkdir build
mv *.html *.js *.wasm *.wat build/
