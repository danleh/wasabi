const blake3 = require('blake3-wasm');

const hash1 = blake3.hash('foo', 16); 

let key = blake3.hash('foo');
const hash2 = blake3.keyedHash(key, "some string"); 

