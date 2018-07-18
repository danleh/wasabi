#!/bin/sh
mkdir -p wasm/original/
cp programs/polybench-c-4.2.1-beta/build/*.wasm wasm/original/
cp programs/pspdfkit-webassembly-benchmark-master/public/vendor/pspdfkit/pspdfkit-lib/pspdfkit.wasm wasm/original/
gzip -c -d programs/EpicZenGarden/2017-03-16-ZenGarden/UE4Game-HTML5-Shipping.wasm.gz > wasm/original/UE4Game-HTML5-Shipping.wasm
