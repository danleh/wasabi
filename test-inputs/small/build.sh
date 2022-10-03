#!/bin/sh
for wat in *.wat; do wat2wasm "$wat"; done
rm -rf build/
mkdir build
mv *.wasm build/