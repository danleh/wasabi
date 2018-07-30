#!/bin/bash

# ensure emcc is on PATH, e.g., by running 
# FIXME: the following path isn't machine-independent
# source ~/emsdk/emsdk_env.sh

dataset_size_list=$(readlink -f 1_polybench_dataset_sizes)

cd programs/polybench-c-4.2.1-beta/
mkdir -p build/
while read srcfile
do
	srcdir=$(dirname $srcfile)
	name=$(basename $srcfile .c)
	
	# different dataset size per program to get similar runtimes
	dataset_size=$(sed -n -e "s/$name;//p" $dataset_size_list)
	echo "$name, $dataset_size"

	# for detailed options, see https://kripken.github.io/emscripten-site/docs/compiling/WebAssembly.html#webassembly
	# and http://web.cse.ohio-state.edu/~pouchet.2/software/polybench/#documentation
	# possible dataset sizes for polybench (with -D<size>_DATASET): SMALL, MEDIUM, LARGE, LARGE (default), EXTRALARGE
	emcc -O3 -I utilities -I $srcdir utilities/polybench.c $srcfile -s ALLOW_MEMORY_GROWTH=1 --emrun -DPOLYBENCH_TIME -D$dataset_size -o build/$name.html
done < utilities/benchmark_list
cd -
