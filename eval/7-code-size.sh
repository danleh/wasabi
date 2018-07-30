#!/bin/sh
echo "path, filename, hooks, bytes"
for file in $(find wasm/ -name *.wasm | sort)
do
	name=$(basename $file .wasm)
	hooks=$(basename $(dirname $file))
	echo -n "$file, $name, $hooks, "
	wc -c $file | cut -d' ' -f1
done
