#!/bin/bash

firefox_bin="$(readlink -f browsers/firefox-63-nightly/firefox)"
firefox_args="-no-remote -profile $(readlink -f browsers/firefox-profile)"

chrome_bin="$(readlink -f browsers/chromium-69-nightly/chrome)"
chrome_args="--user-data-dir=$(readlink -f browsers/chromium-profile)"

emrun_output="$(readlink -f 6_results)"

analysis="$1"

# ensure emcc is on PATH, e.g., by running 
source ~/Documents/SOLA/WebAssembly/tools/emsdk/emsdk_env.sh

# break out of loop on Ctrl+C
trap exit SIGINT SIGTERM

for file in programs-analysis/polybench-c-4.2.1-beta/*.html
do
	name=$(basename $file .html)
	echo -n "firefox;$analysis;$name;" >> $emrun_output
	emrun --log_stdout "$emrun_output" --browser "$firefox_bin" --browser_args "$firefox_args" --kill_exit "$file"
done
