#!/bin/sh
echo "file, time [ms]"
for i in $(seq 1 10)
do
	for file in wasm/original/*.wasm
	do
		echo -n "$(basename $file), "
		t_start=$(date +%s%N)
		cargo run -q --release -- $file
		t_ms=$((($(date +%s%N) - $t_start)/1000000))
		echo "$t_ms"
	done
done
