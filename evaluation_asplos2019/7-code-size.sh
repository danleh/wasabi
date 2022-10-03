#!/bin/sh
echo "path, program, hooks, bytes"
for file in $(find wasm/ -name *.wasm | sort)
do
	program=$(basename $file .wasm)
	hooks=$(basename $(dirname $file))
	echo -n "$file, $program, $hooks, "
	wc -c $file | cut -d' ' -f1
done
