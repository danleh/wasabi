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
const Wasabi = require("wasabi-nodejs");
const { HIGH_LEVEL_HOOKS } = Wasabi;

const WasabiObj = new Wasabi(
  // `input_file`
  "/path/to/wasm/file",
  // `output_dir`
  "/path/to/output/folder",
  // `enabled_hooks`: List of hooks, not supply for all
  [HIGH_LEVEL_HOOKS.Begin, HIGH_LEVEL_HOOKS.If]
);

// Get fields
console.log(WasabiObj.get("input_file"));
console.log(WasabiObj.get("output_dir"));
console.log(WasabiObj.get("enabled_hooks"));

// Set fields
console.log(WasabiObj.set("input_file", "another/path/to/input"));
console.log(WasabiObj.set("output_dir", "another/path/to/output"));
console.log(WasabiObj.set("enabled_hooks", [HIGH_LEVEL_HOOKS.BrIf]));

// Instrument Wasm and generate JavaScript
WasabiObj.exec();
```
