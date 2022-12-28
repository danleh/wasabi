#!/bin/sh

rm -rf build/
mkdir build

for wat in src/*.wat
do
    wat2wasm "$wat" -o "build/$(basename $wat).wasm"
done
