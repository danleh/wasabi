#!/bin/bash
# uses bash process substitution
meld <(hexdump -C $1.wasm) <(hexdump -C $1.encoded.wasm)

