#!/bin/bash

source ~/Documents/SOLA/WebAssembly/tools/emsdk/emsdk_env.sh

firefox_bin="$(readlink -f ../../browsers/firefox-63-nightly/firefox)"
firefox_args="-no-remote -profile $(readlink -f ../../browsers/firefox-profile)"
echo "$firefox_bin"
echo "$firefox_args"

cd programs/polybench-c-4.2.1-beta/

# # build with reference output option
# mkdir -p build_dump/
# while read srcfile
# do
# 	srcdir=$(dirname $srcfile)
# 	name=$(basename $srcfile .c)
# 	emcc -O3 -I utilities -I $srcdir utilities/polybench.c $srcfile -s ALLOW_MEMORY_GROWTH=1 --emrun -DPOLYBENCH_DUMP_ARRAYS -DSMALL_DATASET -o build_dump/$name.html
# done < utilities/benchmark_list

cd build_dump

# generate reference outputs
# for file in *.html
# do
# 	emrun --log_stderr "$file.orig.out" --browser "$firefox_bin" --browser_args "$firefox_args" --kill_exit "$file"
# done

# # instrument
# for file in *.wasm
# do
# 	cargo run --release -- $file
# 	mv $file $file.orig
# done
# mv out/* ./
# rmdir out/
# for file in *.html
# do
# 	name=$(basename $file .html)
# 	sed -i "4i\    <script src=\"$name.wasabi.js\"></script>" $file
# done

# # generate instrumented outputs
# for file in *.html
# do
# 	emrun --log_stderr "$file.all.out" --browser "$firefox_bin" --browser_args "$firefox_args" --kill_exit "$file"
# done

mkdir -p diff/
for file in *.html
do
	diff $file.orig.out $file.all.out > diff/$file.out
done
