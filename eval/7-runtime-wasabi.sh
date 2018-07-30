#!/bin/sh
cargo -q install --path .. --force
echo "file, real, user, sys"
for i in $(seq 1 10)
do
	for file in wasm/original/*.wasm
	do
		echo -n "$(basename $file), "
		/usr/bin/time -f "%E, %U, %S" wasabi $file 2>&1
	done
done
