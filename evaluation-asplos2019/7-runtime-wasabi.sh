#!/bin/sh
cargo install --path .. --force 1>&2
echo "file, num_threads, time_ms"
# try for multiple CPU cores
for num_threads in $(seq 0 8)
do
	# average over multiple runs
	for i in $(seq 1 20)
	do
		for file in wasm/original/*.wasm
		do
			t_start=$(date +%s%N)
			# write results to /dev/null -> faster?
			# wasabi $file /dev/null/out
			(RAYON_NUM_THREADS=$num_threads wasabi $file)
			time_ms=$((($(date +%s%N) - $t_start)/1000000))
			echo "$(basename $file), $num_threads, $time_ms"
		done
	done
done
