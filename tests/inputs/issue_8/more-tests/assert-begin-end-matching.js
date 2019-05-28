function assert_equals(expected, actual, msg) {
  if (expected !== actual) {
    print("assertion FAILED: " + expected + " !== " + actual);
    if (msg !== undefined) print(msg);
    quit(1);
  }
}

let basename = arguments[0];
load(basename + ".wasabi.js");

Wasabi.analysis = {

  open_calls: 0, 
  open_blocks: 0, 

  begin: function(location, type) {
    if (type === "function") Wasabi.analysis.open_calls++;
    Wasabi.analysis.open_blocks++;
  }, 

  end: function(location, type) {
    if (type === "function") Wasabi.analysis.open_calls--;
    Wasabi.analysis.open_blocks--;
  }, 
}

let module = new WebAssembly.Module(readbuffer(basename + ".wasm"));
let instance = new WebAssembly.Instance(module);
instance.exports.main();
assert_equals(0, Wasabi.analysis.open_calls);
assert_equals(0, Wasabi.analysis.open_blocks);

