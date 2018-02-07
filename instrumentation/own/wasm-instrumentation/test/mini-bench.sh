#!/bin/bash
cargo build --release
time for i in {1..50}; do ../target/release/wasm-instrumentation bananabread/bb.wasm silent; done
