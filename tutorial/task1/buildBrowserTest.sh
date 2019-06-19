#!/bin/bash -e

# Usage: ./buildBrowserTest.sh myProgram.wat myAnalysis.js
#
# Afterwards, load myProgram.html in browser.

name=${1%.*}
analysis=${2%.*}

wat2wasm ${name}.wat

wasabi ${name}.wasm

mv out/${name}.wasm ${name}_instr.wasm
mv out/${name}.wasabi.js ${name}.wasabi.js

cp template.html ${name}.html

sed -i -e "s/FILENAME_TEST/${name}/g" ${name}.html
sed -i -e "s/FILENAME_INSTR/${name}_instr/g" ${name}.html
sed -i -e "s/FILENAME_ANALYSIS/${analysis}/g" ${name}.html

rm -rf ${name}_out
mkdir ${name}_out
mv ${name}.wasm ${name}_out
mv ${name}.wasabi.js ${name}_out
mv ${name}_instr.wasm ${name}_out
mv ${name}.html ${name}_out

