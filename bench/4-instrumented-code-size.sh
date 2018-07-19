#!/bin/sh
echo "bytes \t wasm overhead \t hooks"
original_size=$(du --apparent-size wasm/original | cut -f 1)
echo "$original_size \t 0% \t <original>"
du -S --exclude=*.wasabi.js --apparent-size wasm/instrumented/* | grep -v 'grouped$' | while read line
do
	size=$(echo $line | cut -d' ' -f 1)
	hooks=$(basename $(echo $line | cut -d' ' -f 2))
	overhead=$(echo "scale=5;(($size/$original_size)-1)*100" | bc)
	echo "$size \t $overhead% \t $hooks"
done | sort -V
