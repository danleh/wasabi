#!/bin/sh
for wat in *.wat; do wat2wasm "$wat"; done
for wasm in *.wasm; do wasm-validate "$wasm"; done