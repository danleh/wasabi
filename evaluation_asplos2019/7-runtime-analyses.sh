#!/bin/sh
while read hooks
do
	echo "hooks: $hooks"

	./5-prepare-programs-for-analysis.sh none $hooks

	# average over multiple runs
	for i in $(seq 1 2)
	do
		# ./6-run-polybench.sh none $hooks
		# ./6-run-ue4.sh none $hooks
		./6-run-pspdfkit.sh none $hooks
	done
done < 6_hooks_to_run_list
