#!/bin/sh

rm -rf build/
mkdir build

wat2wasm unknown-type.wat -o build/unknown-type.wasm
