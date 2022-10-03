#!/bin/sh
cargo install --path .. --force
echo "program, hook count"
for file in wasm/original/*.wasm
do
	program=$(basename $file .wasm)
	hook_count=$(wasabi $file | grep -o -E '[0-9]+')
	echo "$program, $hook_count"
done
