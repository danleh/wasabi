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

echo -n "firefox, $analysis, $hooks, \"$comment\", UE4Game-HTML5-Shipping, " >> $emrun_output
cd programs-analysis/EpicZenGarden/
# optional: also disable vsync, audio, and rendering (more "pure-CPU" then, but less real workload), add &novsync&noaudio&fakegl
emrun --no_emrun_detect --log_stdout "$emrun_output" --browser "$firefox_bin" --browser_args "$firefox_args" --serve_root . "2017-03-16-ZenGarden/EpicZenGarden.html?playback"
cd - > /dev/null

sleep 2s
