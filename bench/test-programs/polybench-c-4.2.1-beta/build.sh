#!/bin/bash
# source ~/Documents/SOLA/WebAssembly/tools/emsdk/emsdk_env.sh --build=Release
emcc -O3 -I utilities -I linear-algebra/kernels/atax utilities/polybench.c linear-algebra/kernels/atax/atax.c -s WASM=1 -s ALLOW_MEMORY_GROWTH=1 --emrun -DPOLYBENCH_TIME -DEXTRALARGE_DATASET -o atax.html
cargo run --release atax.wasm
cp atax.html atax.js out/
sed -i '/<script async type="text\/javascript" src="atax.js"><\/script>/a <script src="atax.wasabi.js"></script>' out/atax.html
#emrun --browser ~/Downloads/nightly\ browsers\ as\ of\ 2018-07-11\ 13-30\ UTC+0/chrome-linux/chrome out/atax.html 
#emrun --no_browser --port=8080 out/atax.html &
#~/Downloads/nightly\ browsers\ as\ of\ 2018-07-11\ 13-30\ UTC+0/firefox-63.0a1.de.linux-x86_64/firefox/firefox -no-remote -new-instance -profile ./ffprofile/ http://localhost:8080/atax.html
