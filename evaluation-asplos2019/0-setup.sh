#!/bin/sh

# requirements: wget, tar, unzip, gzip, python 2.7, npm, node js, git

# download and install emscripten, see https://kripken.github.io/emscripten-site/docs/getting_started/downloads.html
git clone https://github.com/juj/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
# to have emcc available, run: 
source ./emsdk_env.sh
cd ..

# download and extract nightly browsers (as of 2018-07-17)
mkdir -p browsers
cd browsers
wget https://download-origin.cdn.mozilla.net/pub/firefox/nightly/2018/07/2018-07-16-22-14-18-mozilla-central/firefox-63.0a1.en-US.linux-x86_64.tar.bz2 -O firefox-63-nightly.tar.bz2
tar -xvf firefox-63-nightly.tar.bz2
mv firefox firefox-63-nightly
mkdir -p firefox-profile
# see https://github.com/scheib/chromium-latest-linux/blob/master/update.sh
wget https://www.googleapis.com/download/storage/v1/b/chromium-browser-snapshots/o/Linux_x64%2F575583%2Fchrome-linux.zip?alt=media -O chromium-69-nightly.zip
unzip chromium-69-nightly.zip
mv chrome-linux chromium-69-nightly
cd ..

# download and extract benchmark programs
mkdir -p programs
cd programs
# PolyBench/C
wget https://downloads.sourceforge.net/project/polybench/polybench-c-4.2.1-beta.tar.gz
tar -xvf polybench-c-4.2.1-beta.tar.gz
# Unreal Epic Zen Garden demo, see https://bugzilla.mozilla.org/show_bug.cgi?id=1347950
wget https://s3.amazonaws.com/mozilla-games/ZenGarden/2017-03-16-ZenGarden.zip
unzip 2017-03-16-ZenGarden.zip 
# PSPDFKit WebAssembly Benchmark
# on how to get a license, see https://github.com/PSPDFKit-labs/pspdfkit-webassembly-benchmark
# this license is valid 60 days from 2018-07-17 on
wget https://github.com/PSPDFKit-labs/pspdfkit-webassembly-benchmark/archive/master.zip -O pspdfkit-webassembly-benchmark-master.zip
unzip pspdfkit-webassembly-benchmark-master.zip
cd ..
