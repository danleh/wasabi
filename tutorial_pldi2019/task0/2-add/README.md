# Task 0.2: Addition in WebAssembly

So far, we have only called a JavaScript function from WebAssembly.
Now, we will call a WebAssembly function from JavaScript and also pass some arguments.

## Step 1: Write a Function for Integer Addition in WebAssembly

Read `add.wat`, copy it, and assemble it to a binary as before with:

```
wat2wasm add.wat
```

## Step 2: How to Call WebAssembly from JavaScript

Understand and copy `add.html`. 
Note some changes compared with the previous task on how we interact with WebAssembly:

1. There is no need to re-`fetch` and `instantiate` the binary on every button click, so just do that once in the beginning.
2. Since the add function is an exported function and not automatically called when instantiating the module, we need to access it via `instance.exports.add`.
3. Since the wasm file doesn't call JavaScript (it just returns a value), there is no need to pass an imports object to `instantiate`.

## Step 3: Run it in the Browser

As before, start a webserver in the directory with:

```
python -m SimpleHTTPServer
```

Open http://localhost:8000/add.html and try some integer additions.

## Bonus: How is the add function different from JavaScript's `+`?

Hint: What about integer overflow? What do you expect in JavaScript? What do we get here? How is that useful?
