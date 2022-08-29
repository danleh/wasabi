#!/bin/bash
wat2wasm "main.wat"
wasm-interp --trace --run-all-exports "main.wasm"
