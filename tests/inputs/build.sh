#!/bin/sh
for dir in */
do
	echo $dir
	cd $dir
	./build.sh
	cd - > /dev/null
done
