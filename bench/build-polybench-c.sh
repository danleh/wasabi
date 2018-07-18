#!/bin/bash
# ensure emcc is on PATH, e.g., by running 
source ~/Documents/SOLA/WebAssembly/tools/emsdk/emsdk_env.sh
cd programs/polybench-c-4.2.1-beta/
mkdir -p build/
while read srcfile
do
	srcdir=$(dirname $srcfile)
	name=$(basename $srcfile .c)
	# for detailed options, see https://kripken.github.io/emscripten-site/docs/compiling/WebAssembly.html#webassembly
	emcc -O3 -I utilities -I $srcdir utilities/polybench.c $srcfile -s ALLOW_MEMORY_GROWTH=1 --emrun -DPOLYBENCH_TIME -o build/$name.html
done < utilities/benchmark_list
cd -