#!/bin/bash
wat2wasm "main.wat"
wasm-interp --wasi --trace "main.wasm"
wasmtime "main.wasm"