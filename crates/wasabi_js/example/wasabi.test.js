import { expect, test } from 'vitest'

const wasabi = require("../pkg");
const fs = require('fs');

// Binary from https://github.com/danleh/wasabi/blob/fe12347f3557ca1db64b33ce9a83026143fa2e3f/tutorial-pldi2019/task0/2-add/add.wat
test("add.wasm", async () => {
    const { buff, res } = setup('example/add.wasm');
    const { instance } = await WebAssembly.instantiate(buff, res);
    expect(instance.exports.add(1, 2)).toBe(3);
});

// Binary from https://github.com/danleh/wasabi/blob/fe12347f3557ca1db64b33ce9a83026143fa2e3f/tutorial-pldi2019/task0/1-hello/hello.wat
test("hello.wasm", async () => {
    const { buff, res } = setup('example/add.wasm');
    const arr = new Uint8Array(buff);
    const { instance } = await WebAssembly.instantiate(buff, res);
});

// Binary from https://gist.github.com/doehyunbaek/3ffa3140c41b7283bf35f34d4d9ecf64
test("global.wasm", async () => {
    const { buff, res } = setup('example/global.wasm');
    res.console = console
    res.env = {from_js: new WebAssembly.Global({value: "i64", mutable: false}, 0n)}
    const { instance } = await WebAssembly.instantiate(buff, res);
    expect(instance.exports.export_global()).toBe(10n)
});


function setup(path) {
    const buf = fs.readFileSync(path);
    const arr = new Uint8Array(buf);
    const { instrumented, js } = wasabi.instrument_wasm(arr);
    const res = eval(js);
    const buff = new Uint8Array(instrumented);
    return { buff, res };
}
