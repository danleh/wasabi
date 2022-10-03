#!/bin/bash

firefox_bin="$(readlink -f browsers/firefox-63-nightly/firefox)"
firefox_args="-no-remote -profile $(readlink -f browsers/firefox-profile)"

chrome_bin="$(readlink -f browsers/chromium-69-nightly/chrome)"
chrome_args="--user-data-dir=$(readlink -f browsers/chromium-profile)"

emrun_output="$(readlink -f results/runtime-analysis.csv)"

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

echo -n "firefox, $analysis, $hooks, \"$comment\", pspdfkit, " >> $emrun_output
cd programs-analysis/pspdfkit/
# save some time: don't do 20 runs (inside the benchmark => runsScaleFactor), because we average over multiple ("our") runs anyway
# ?runsScaleFactor=0.5
emrun --no_emrun_detect --log_stdout "$emrun_output" --browser "$firefox_bin" --browser_args "$firefox_args" index.html
cd - > /dev/null

sleep 2s
