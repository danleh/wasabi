# Task 0.3: Compiling C to WebAssembly

Writing WebAssembly in the text format by hand is quite cumbersome (and not very realistic).
Similarly, our JavaScript wrapper is very crude.
Let's fix both points by using `emscripten` to compile C code to WebAssembly and generate the necessary JavaScript and HTML around it.

## Step 1: Hello World in C

Write the standard `hello.c`:

```C
#include <stdio.h>

int main() {
    printf("Hello, world!\n");
    return 0;
}
```

## Step 2: Use Emscripten as a C Compiler

Compile with emscripten to WebAssembly, including an HTML harness.

(Two common problems: 1. Make sure your path is correctly set-up for emscripten. Typically this is done by running `source path/to/your/emsdk/emsdk_env.sh`. 2. Make sure you specify an html file as the output, otherwise emscripten won't generate a website, but only plain JavaScript and WebAssembly.)

```
$ emcc hello.c -o hello.html
$ ls
hello.c  hello.html  hello.js  hello.wasm
```

Note the three generated files: An HTML website, a JavaScript file with wrappers, and the actual WebAssembly module.

## Step 3: Running C Code in the Browser

Finally, lets see how the generated website looks, by (as always) starting a web server 

```
python -m SimpleHTTPServer
```

and opening http://localhost:8000/hello.html in a browser.

## Bonus

Check the generated JavaScript and WebAssembly files for some internal details of Emscripten. In particular, see with `wasm-objdump` that the WebAssembly file contains a lot generated code. This corresponds, e.g., to formatting in printf.

Since the WebAssembly file is so large: How would you reduce the size of a binary in a regular C compiler? Does that work here?
