#!/bin/bash -e

# Usage: ./buildBrowserTest.sh myProgram.c myAnalysis.js
#
# Afterwards, load myProgram.html in browser.

name=${1%.*}
analysis=${2%.*}

emcc ${name}.c -s WASM=1 -s EXPORTED_FUNCTIONS='["_main", "_malloc", "_markSource", "_sink"]' -o ${name}.html

rm -rf out/

wasabi ${name}.wasm

mv ${name}.wasm ${name}.orig.wasm

cp out/* .

sed -i "/<script async type=\"text\/javascript\" src=\"${name}.js\"><\/script>/a <script src=\"${name}.wasabi.js\"></script>" ${name}.html

sed -i "/<script src=\"${name}.wasabi.js\"><\/script>/a <script src=\"${analysis}.js\"></script>" ${name}.html

