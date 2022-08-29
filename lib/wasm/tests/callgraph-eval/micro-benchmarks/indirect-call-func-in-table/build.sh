#!/bin/bash
wat2wasm "main.wat"
wasm-interp --host-print --run-all-exports --trace "main.wasm"
