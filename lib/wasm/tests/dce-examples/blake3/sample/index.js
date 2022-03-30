const fs = require("fs");
global.Wasabi = require('./node_modules/blake3-wasm/dist/wasm/nodejs/blake3_js_bg.wasabi.js');

let analysis = require('./../../analysis.js');

const blake3 = require('blake3-wasm');

blake3.hash('foo'); // => Buffer

require('./../../collect-data.js')