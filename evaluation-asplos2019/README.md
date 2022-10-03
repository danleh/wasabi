# Wasabi ASPLOS 2019 Paper Evaluation

This directory contains the scripts and data for the experiments in our ASPLOS 2019 paper [*Wasabi: A Framework for Dynamically Analyzing
WebAssembly*](https://software-lab.org/publications/asplos2019_Wasabi.pdf).

We evaluate Wasabi on real-world WebAssembly programs (PolyBench suite, PSPDFKit, Unreal Engine 4 demo) in terms of code size and runtime overhead.
We also measure the runtime of the instrumentation itself and test that the instrumented programs produce the same results as the original programs.  

The scripts and intermediate results are numbered in their evaluation order, i.e., start with `0-setup.sh`.
The final results (that are also in the paper) are in `results/`.

We originally ran the experiments on Ubuntu 16.04.
