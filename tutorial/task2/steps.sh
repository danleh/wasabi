# Test if vanilla works
unzip bb.zip -d bb
pushd bb
python -m SimpleHTTPServer
popd

# Instrument
pushd bb
cp bb.wasm bb.wasm.bak
wasabi --hooks=call bb.wasm .
# TODO Add wasabi.js to bb.html
popd

cp analysis-template.js bb/analysis.js
# TODO Write analysis

# Visualize call graph
dot -Ksfdp -Tpdf call-graph.dot > call-graph.pdf
