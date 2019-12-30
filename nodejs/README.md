# Nodejs Wrapper for Wasabi

## Compiling from Source

* Dependencies and tools
  * Make sure you've installed all dependencies for building [wasabi](https://github.com/danleh/wasabi) first.
  * Install neon-cli
    ```
    npm install -g neon-cli
    ```
  * Build
    ```
    neon build or npm build
    ```
* Usage

```
const fs = require("fs");
const Wasabi = require("wasabi-nodejs");
const { HIGH_LEVEL_HOOKS } = Wasabi;

// Read .wasm file
let binary_file = fs.readFileSync("/path/to/wasm/file");
binary_file = Buffer.from(binary_file);

const WasabiObj = new Wasabi(
  // `binary_file`
  binary_file,
  // `enabled_hooks`: List of hooks, not supply for all
  [HIGH_LEVEL_HOOKS.Begin, HIGH_LEVEL_HOOKS.If]
);

// Get fields
// console.log(WasabiObj.get("binary_file"));
// console.log(WasabiObj.get("enabled_hooks"));

// Set fields
// console.log(WasabiObj.set("binary_file", "another_binary_file"));
// console.log(WasabiObj.set("enabled_hooks", [HIGH_LEVEL_HOOKS.BrIf]));

let result = WasabiObj.exec();

// JS hooks
let js = result.js;

// Write instrumented .wasm file
let wasm = new Uint8Array(result.wasm);
console.log(result.wasm);
fs.writeFileSync("hello.wasm", new Buffer(wasm), function(err) {
  if (err) {
    console.log(err);
  } else {
    console.log(chunk.length);
  }
});
```
