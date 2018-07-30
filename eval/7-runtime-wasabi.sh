#!/bin/sh
cargo install --path .. --force 1>&2
echo "file, time [ms]"
for i in $(seq 1 20)
do
	for file in wasm/original/*.wasm
	do
		echo -n "$(basename $file), "
		t_start=$(date +%s%N)
		# sequential instrumentation
		# (RAYON_NUM_THREADS=1 wasabi $file)
		# parallel
		wasabi $file
		# write results to /dev/null
		# wasabi $file /dev/null/out
		t_ms=$((($(date +%s%N) - $t_start)/1000000))
		echo "$t_ms"
	done
done
