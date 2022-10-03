#!/bin/sh

# clone and build spec interpreter
git clone https://github.com/WebAssembly/spec.git
cd spec/interpreter/
make
cd ../../

# convert wast test files to binary wasm files
mkdir -p build/
for file in spec/test/core/*.wast
do
	name=$(basename $file .wast)
	# echo "$file"
	# echo "$name"
	spec/interpreter/wasm $file -o build/$name.wasm
done
