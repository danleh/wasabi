#!/bin/sh
# for all hooks
for hooks in wasm/instrumented/*
do
	hooks=$(basename $hooks)
	echo "hooks: $hooks"

	./5-prepare-programs-for-analysis.sh none $hooks

	# average over multiple runs
	for i in $(seq 1 20)
	do
		./6-run-polybench.sh none $hooks
		# TODO PSPDF, UE4
	done
done
