#!/bin/bash

firefox="$(readlink -f browsers/firefox-63-nightly/firefox) -no-remote -profile $(readlink -f browsers/firefox-profile)"
chrome="$(readlink -f browsers/chromium-69-nightly/chrome) --user-data-dir=$(readlink -f browsers/chromium-profile)"

echo "$firefox"
echo "$chrome"

emrun_output="$(readlink -f emrun_output)"

# ensure emcc is on PATH, e.g., by running 
source ~/Documents/SOLA/WebAssembly/tools/emsdk/emsdk_env.sh

emrun --no_browser --port 6337 --log_stdout $emrun_output --serve_after_exit --serve_root programs-analysis/polybench-c-4.2.1-beta . &
emrun_pid=$!

for name in programs-analysis/polybench-c-4.2.1-beta/*.html
do
	name=$(basename $name .html)
	echo -n "$name;" >> $emrun_output
	$firefox http://localhost:6337/$name.html
done

kill $emrun_pid