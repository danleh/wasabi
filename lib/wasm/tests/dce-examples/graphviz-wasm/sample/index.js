const fs = require("fs");
global.Wasabi = require('./node_modules/graphviz-wasm/graphviz/graphviz.wasabi.js');

let analysis = require('./../../analysis.js');
const graphviz = require('graphviz-wasm');

async function main () {
    await graphviz.loadWASM (); // First of all you need to load the WASM instance and wait for it

    const dot = `
    strict graph {
        a -- b
        a -- b
        b -- a [color=blue]
    }
    `;
    const svg = graphviz.layout ( dot );

    require('./../../collect-data.js')
}

main()