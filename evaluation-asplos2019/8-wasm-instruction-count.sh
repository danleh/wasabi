#!/bin/sh
for file in wasm/original/*.wasm
do
	instr_count=$(wasm-opcodecnt $file -s "%" | cut -d% -f2 | egrep "[0-9]+" | awk '{ SUM += $1} END { print SUM }')
	echo "$file $instr_count"
done