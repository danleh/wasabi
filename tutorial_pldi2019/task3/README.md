# Task 3: Reverse Engineering

In this task, we want to simulate a very simple reverse engineering challenge.
Start serving the `1-static` directory with `python -m SimpleHTTPServer` and open http://localhost:8000/password.html.
You will be prompted for a password.
If you give the wrong one, the website will only show "Incorrect :(".

What is the right password?

## Step 1: Static Analysis

To be fair, no hacker would directly turn to dynamic analysis, if there is an easier way.
By inspecting `password.html`, we see that the check is performed by this line:

```
Module.ccall('check', 'number', ['string'], [ input ]);
```

The `Module` object is defined in `password.js` and with a bit of digging, we see that it loads `password.wasm`.
Now, we can proceed in many ways, e.g.,

- Disassemble `password.wasm` with `wasm2wat`,
- Disassemble `password.wasm` with `wasm-objdump`, or
- The simplest option of all: just run `strings password.wasm`

Mhm. That last line looks curious.
Try it!

## Step 2: Dynamic Analysis

Alright, so for the example in `1-static`, the password was in cleartext in the binary and so dynamic analysis would be a bit overkill.
Let's look at `2-dynamic` now.
Serve it again with `python -m SimpleHTTPServer` and open http://localhost:8000/password.html.
Everything looks to be the same, except that:

- The password from the previous step is not working :(
- `strings` on `password.wasm` doesn't seem to help.

Disassembling `password.wasm` shows some interesting contents in the `data` WebAssembly section.
But we are too lazy to find out how this is decrypted.

Let's make the program do the work for us and observe all memory accesses with Wasabi.
By now you know the drill:

- Instrument with `wasabi password.wasm .`
- Add scripts `analysis.js` and `password.wasabi.js` to the head of `password.html`.
- Implement the `load` and `store` hooks in `analysis.js`. (Just dump their `arguments` via `console.log()`)
- Open http://localhost:8000/password.html and check the JavaScript console for output.

The `store`d values look promising!
Write them to a global array with an analysis like

```
let array = [];
Wasabi.analysis = {
    store: function(location, op, addr, value) {
        array.push(value);
    }
}
```

Convert to ASCII with `String.fromCharCode(...array)`.

:-)
