const fs = require("fs");
global.Wasabi = require('./node_modules/blake3-wasm/dist/wasm/nodejs/blake3_js_bg.wasabi.js');

let analysis = require('./../../analysis.js');

const blake3 = require('blake3-wasm');

const hash1 = blake3.hash('foo', 16); 

let key = blake3.hash('foo');
const hash2 = blake3.keyedHash(key, "some string"); 

require('./../../collect-data.js')