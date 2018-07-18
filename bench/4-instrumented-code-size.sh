#!/bin/sh
(
echo "bytes \t overhead \t hooks"
none_size=$(du --apparent-size wasm/instrumented/none | cut -f 1)
du --apparent-size wasm/instrumented/* | sort | while read line
do
	size=$(echo $line | cut -d' ' -f 1)
	hooks=$(basename $(echo $line | cut -d' ' -f 2))
	overhead=$(echo "scale=5;(($size/$none_size)-1)*100" | bc)
	echo "$size \t $overhead%     \t $hooks"
done
) > 4_results
