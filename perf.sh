#!/bin/sh
# see https://gist.github.com/KodrAus/97c92c07a90b1fdd6853654357fd557a

# ensure perf is installed and has appropriate permissions to access counters
# ensure https://github.com/Yamakaky/rust-unmangle/blob/master/rust-unmangle and https://github.com/brendangregg/FlameGraph are available on path and work
cargo build --release
export RAYON_NUM_THREADS=1
perf record --call-graph=lbr target/release/wasabi eval/wasm/original/pspdfkit.wasm
perf script | stackcollapse-perf.pl | rust-unmangle | flamegraph.pl > flame.svg

# for heap usage, use heaptrack
heaptrack wasabi eval/wasm/original/pspdfkit.wasm
