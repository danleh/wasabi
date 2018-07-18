#!/bin/sh

# requirements: wget, tar, unzip, gzip, python 2.7, npm, node js, git

# download and install emscripten, see https://kripken.github.io/emscripten-site/docs/getting_started/downloads.html
#git clone https://github.com/juj/emsdk.git
#cd emsdk
#./emsdk install latest
#./emsdk activate latest
# to have emcc available, run: 
#source ./emsdk_env.sh
#cd ..

# download and extract nightly browsers
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
cd pspdfkit-webassembly-benchmark-master
# needs download key, register for trail to get
npm install --save https://customers.pspdfkit.com/npm/TRIAL-oL6A8_VBUi1QGq68jzqPZ7B6OgMoqr7oAOQ8pXKOtZJUY4r-77rdz_fp3kTBbfUX9aOtbNz-PZF7nadrRWcItA/latest.tar.gz
mkdir -p public/vendor
cp -R node_modules/pspdfkit/dist public/vendor/pspdfkit
npm install
# needs license key, register for trail to get
echo "kffP4ACBJVXTetVoyMiq0PAYaDCJKUDLwScSXoktG0mKLgSE9vtTFfvwvrSgEwdrZFkp4k4_4oJjSnMP7L9KAuD3yZj50OUQR9zFs9exY3gj3O56ft9XZ2R-_QAs1wqeZdp95zJ3V6bjo_DwqNmM9t8o6zLM1-6E45pXqRQnVEpcOumVMxacTn15_FwGlikMEnLHQux0oobMuO0n3yx7zhS6OHUZsRnaEZzfJe6xnw5fd9Nb5FJhP5yf96EnNt33NNyogHnkYON2cOSZ_FYGmu17c6W31xk7qSUGPeMlCdvOEhg-BRUEtLjolUjZMscxWEP0WJXnNSQEBUyfkyrcyCPT_hreJBmGrpMqPekT5NSo-7-sY7aWuBOxZQYAhXkKL72R20irjU_6Ginni_B_JnKbeFT4Dytn7jfAAFqwHWYxz-d1hcOABqdcl6J5Tatq" > public/license-key
# to run: npm start
cd ..
