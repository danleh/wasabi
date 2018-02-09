#!/bin/bash
cargo build --release
# TODO run on every wasm file in input for 20 times
time for i in {1..50}; do ../target/release/wasm-instrumentation bananabread/bb.wasm silent; done
