#!/bin/sh
wat2wasm module.wat -o module.wasm
wasm2wat module.wasm -o module.wasm.wat