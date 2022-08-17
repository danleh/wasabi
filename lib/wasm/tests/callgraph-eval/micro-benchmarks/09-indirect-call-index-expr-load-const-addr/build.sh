#!/bin/bash
wat2wasm "main.wat"
wasm-interp --host-print "main.wasm" --run-all-exports
