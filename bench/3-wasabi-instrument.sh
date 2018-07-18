#!/bin/bash
hooks=''
# use "none" as directory name if instrumenting for no hook
out_dir=wasm/instrumented/${hooks:-none}/
mkdir -p $out_dir
for file in wasm/original/*.wasm
do
	cargo run --release -- --hooks=$hooks $file $out_dir
done
