#!/bin/bash
egrep -v '^#|^$' 3_hooks_to_instrument_list | while read line
do
	hooks=$(echo $line | cut -d';' -f 1)
	echo "--hooks argument: $hooks"
	out_dir=wasm/instrumented/$(echo $line | cut -d';' -f 2)/
	echo "output directory: $out_dir"
	mkdir -p $out_dir
	# parallelize instrumentation, see https://unix.stackexchange.com/questions/103920/parallelize-a-bash-for-loop
	(
	for file in wasm/original/*.wasm
	do
		cargo run --release -- --hooks=$hooks $file $out_dir &
	done
	wait
	)
done
