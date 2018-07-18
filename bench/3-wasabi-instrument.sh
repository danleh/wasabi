#!/bin/bash
egrep -v '^#|^$' 3_hooks_to_instrument_list | while read line
do
	hooks=$(echo $line | cut -d';' -f 1)
	echo "--hooks argument: $hooks"
	out_dir=wasm/instrumented/$(echo $line | cut -d';' -f 2)/
	echo "output directory: $out_dir"
	mkdir -p $out_dir
	for file in wasm/original/*.wasm
	do
		echo "  $file"
		cargo run --release -q -- --hooks=$hooks $file $out_dir
	done
done
