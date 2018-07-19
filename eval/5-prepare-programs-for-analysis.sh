#!/bin/bash

# copy set-up programs before modifying them for the analysis
mkdir -p programs-analysis/polybench-c-4.2.1-beta/
cp -r programs/EpicZenGarden/ programs-analysis/
cp -r programs/pspdfkit-webassembly-benchmark-master/ programs-analysis/
cp programs/polybench-c-4.2.1-beta/build/* programs-analysis/polybench-c-4.2.1-beta/
