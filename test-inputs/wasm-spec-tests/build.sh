#!/bin/sh

rm -rf build/
mkdir build/

# Clone and build the Ocaml spec interpreter.
# Requires ocamlbuild/opam.
rm -rf spec/
git clone git@github.com:WebAssembly/spec.git
cd spec/interpreter/
make
# Version at the time of building the .wasm file.
git log -1 > ../../build/spec-repository-version
./wasm -e '' -v > ../../build/spec-interpreter-version
cd ../../

# Convert .wast test files to .wasm binaries.
for file in spec/test/core/*.wast
do
	name=$(basename $file .wast)
	spec/interpreter/wasm $file -o build/$name.wasm || echo "error building $file"
done

for file in build/*.wasm
do
	if ! wasm-validate "$file"; then
		echo "invalid binary $file, deleting..."
		rm "$file"
	else
		wasm2wat --generate-names "$file" -o "$file.wat"
	fi
done
