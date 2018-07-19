#!/bin/bash

# copy programs before modifying them for the analyses
if [ ! -d programs-analysis/ ]
then
	echo "programs-analysis/ directory not found, copying from programs/ and inserting Wasabi scripts in html"
	mkdir -p programs-analysis/polybench-c-4.2.1-beta/
	cp -r programs/EpicZenGarden/ programs-analysis/
	cp -r programs/pspdfkit-webassembly-benchmark-master/ programs-analysis/
	cp programs/polybench-c-4.2.1-beta/build/* programs-analysis/polybench-c-4.2.1-beta/

	# insert *.wasabi.js and analysis.js scripts into html harnesses
	sed -i "4i\    <script src=\"UE4Game-HTML5-Shipping.wasabi.js\"></script><script src=\"analysis.js\"></script>" programs-analysis/EpicZenGarden/2017-03-16-ZenGarden/EpicZenGarden.html
	sed -i "4i\    <script src=\"pspdfkit.wasabi.js\"></script><script src=\"analysis.js\"></script>" programs-analysis/pspdfkit-webassembly-benchmark-master/public/index.html
	for file in programs-analysis/polybench-c-4.2.1-beta/*.html
	do
		name=$(basename $file .html)
		sed -i "4i\    <script src=\"$name.wasabi.js\"></script><script src=\"analysis.js\"></script>" $file
	done
fi

analysis=$1
hooks=$2

if [ -z $analysis ] || [ -z $hooks ]
then
	echo "usage:"
	echo "  $0 <analysis> <hooks>"
	echo "where:"
	echo "  <analysis> file from ../analyses/*, without .js extension"
	echo "  <hooks>    subdirectory from ./wasm/instrumented/*"
	exit
fi

# copy instrumented *.wasm file, generated *.wasabi.js file, and analysis into harness directories
gzip --stdout --fast wasm/instrumented/$hooks/UE4Game-HTML5-Shipping.wasm > programs-analysis/EpicZenGarden/2017-03-16-ZenGarden/UE4Game-HTML5-Shipping.wasm.gz
cp wasm/instrumented/$hooks/UE4Game-HTML5-Shipping.wasabi.js programs-analysis/EpicZenGarden/2017-03-16-ZenGarden/

cp wasm/instrumented/$hooks/pspdfkit.wasabi.js programs-analysis/pspdfkit-webassembly-benchmark-master/public/
cp wasm/instrumented/$hooks/pspdfkit.wasm programs-analysis/pspdfkit-webassembly-benchmark-master/public/vendor/pspdfkit/pspdfkit-lib/

find wasm/instrumented/$hooks/ -type f ! -iname '*UE4*' -and ! -iname '*pspdf*' -exec cp -t programs-analysis/polybench-c-4.2.1-beta/ {} +

cp ../analyses/$analysis.js programs-analysis/EpicZenGarden/2017-03-16-ZenGarden/analysis.js
cp ../analyses/$analysis.js programs-analysis/pspdfkit-webassembly-benchmark-master/public/analysis.js
cp ../analyses/$analysis.js programs-analysis/polybench-c-4.2.1-beta/analysis.js
