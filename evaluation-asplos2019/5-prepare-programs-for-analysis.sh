#!/bin/bash

# copy programs before modifying them for the analyses
if [ ! -d programs-analysis/ ]
then
	echo "programs-analysis/ directory not found, copying from programs/ and inserting Wasabi scripts in html"
	mkdir -p programs-analysis/polybench-c-4.2.1-beta/ programs-analysis/EpicZenGarden/ programs-analysis/pspdfkit/
	cp -r programs/polybench-c-4.2.1-beta/build/. programs-analysis/polybench-c-4.2.1-beta/
	cp -r programs/EpicZenGarden/. programs-analysis/EpicZenGarden/
	cp -r programs/pspdfkit-webassembly-benchmark-master/build/. programs-analysis/pspdfkit/

	# insert *.wasabi.js and analysis.js scripts into html harnesses
	
	# UE4:
	# also delete IndexedDB so that everything is always compiled fresh (HACKY but works for now)
	# also insert fetch call that posts result render time to emrun in benchmark mode
	sed -i "4i\    <script src=\"UE4Game-HTML5-Shipping.wasabi.js\"></script><script src=\"analysis.js\"></script><script>indexedDB.deleteDatabase(\"UE4_assetDatabase_ZenGarden\");</script>" programs-analysis/EpicZenGarden/2017-03-16-ZenGarden/EpicZenGarden.html
	sed -i "618i\        fetch('stdio.html', { method: 'POST', body: '^out^0^'+totalRenderTime }).then(() => fetch('stdio.html', { method: 'POST', body: '^exit^0' })).then(() => window.close())" programs-analysis/EpicZenGarden/emtimer.js
	
	# PSPDFKit: HACK see below, dont need to insert wasabi.js and analysis.js script tags here
	mv programs-analysis/pspdfkit/vendor/pspdfkit/pspdfkit.js programs-analysis/pspdfkit/vendor/pspdfkit/pspdfkit.js.orig

	# PolyBench
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
	echo "usage: $0 <analysis> <hooks>"
	echo "where:"
	echo "  <analysis> file from ../analyses/*, without .js extension"
	echo "  <hooks>    subdirectory from ./wasm/instrumented/*"
	exit
fi

# copy instrumented *.wasm file, generated *.wasabi.js file, and analysis into harness directories

# UE4
gzip --stdout --fast wasm/instrumented/$hooks/UE4Game-HTML5-Shipping.wasm > programs-analysis/EpicZenGarden/2017-03-16-ZenGarden/UE4Game-HTML5-Shipping.wasm.gz
cp wasm/instrumented/$hooks/UE4Game-HTML5-Shipping.wasabi.js programs-analysis/EpicZenGarden/2017-03-16-ZenGarden/
cp ../analyses/$analysis.js programs-analysis/EpicZenGarden/2017-03-16-ZenGarden/analysis.js

# PSPDFKit
cp wasm/instrumented/$hooks/pspdfkit.wasm programs-analysis/pspdfkit/vendor/pspdfkit/pspdfkit-lib/
# HACK
cat  wasm/instrumented/$hooks/pspdfkit.wasabi.js ../analyses/$analysis.js | ./pspdfkit-splice.py programs-analysis/pspdfkit/vendor/pspdfkit/pspdfkit.js.orig > programs-analysis/pspdfkit/vendor/pspdfkit/pspdfkit.js

# PolyBench
find wasm/instrumented/$hooks/ -type f ! -iname '*UE4*' -and ! -iname '*pspdf*' -exec cp -t programs-analysis/polybench-c-4.2.1-beta/ {} +
cp ../analyses/$analysis.js programs-analysis/polybench-c-4.2.1-beta/analysis.js
