#!/bin/bash
wat2wasm "main.wat"
# wasm-interp unfortunately doesn't run the start function by default :(
# wasm-interp --trace "main.wasm"
