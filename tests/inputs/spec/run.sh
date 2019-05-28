#!/bin/sh
for file in build/*.wasm
do
	echo "wasabi $file"
	RAYON_NUM_THREADS=1 cargo run -q -- $file
done

for file in out/*.wasm
do
	echo "wasm-validate $file"
	wasm-validate $file
done
