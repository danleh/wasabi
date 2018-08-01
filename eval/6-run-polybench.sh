#!/bin/bash

firefox_bin="$(readlink -f browsers/firefox-63-nightly/firefox)"
firefox_args="-no-remote -profile $(readlink -f browsers/firefox-profile)"

chrome_bin="$(readlink -f browsers/chromium-69-nightly/chrome)"
chrome_args="--user-data-dir=$(readlink -f browsers/chromium-profile)"

emrun_output="$(readlink -f results/runtime-analysis.csv)"

timeout="300s"

analysis=$1
hooks=$2
comment="$3"

if [ -z $analysis ] ||  [ -z $hooks ]
then
	echo "usage: $0 <analysis> <hooks> [<comment>]"
	echo "(all arguments are just passed as strings into the results.csv)"
	exit
fi

# ensure emcc is on PATH, e.g., by running 
# FIXME: the following path isn't machine-independent
source ~/Documents/SOLA/WebAssembly/tools/emsdk/emsdk_env.sh

# break out of loop on Ctrl+C
trap exit SIGINT SIGTERM

for file in programs-analysis/polybench-c-4.2.1-beta/*.html
do
	name=$(basename $file .html)
	echo -n "firefox, $analysis, $hooks, \"$comment\", $name, " >> $emrun_output
	timeout $timeout emrun --log_stdout "$emrun_output" --browser "$firefox_bin" --browser_args "$firefox_args" --kill_exit "$file"
	if [ $? -eq 124 ]
	then
		echo "timeout $timeout" >> $emrun_output
	fi

	# emrun --browser "$firefox_bin" --browser_args "$firefox_args" --serve_after_exit --serve_after_close "$file"
	# break
done
