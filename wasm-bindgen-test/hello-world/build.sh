#!/bin/sh
cargo build --target wasm32-unknown-unknown
# see https://github.com/rustwasm/wasm-bindgen
# --no-modules: do not produce ES6 modules (for consumption by Webpack), but rather modify the window global object
wasm-bindgen --no-modules target/wasm32-unknown-unknown/debug/hello_world.wasm --out-dir . 