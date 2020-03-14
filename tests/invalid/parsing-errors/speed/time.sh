#!/bin/bash
for run in {1..3}
do
    time wasabi UE4Game-HTML5-Shipping.wasm
    time ../../../../target/release/wasabi UE4Game-HTML5-Shipping.wasm
done
