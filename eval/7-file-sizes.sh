#!/bin/sh
for file in wasm/original/*.wasm
do
	wc -c $file
done
