# Task 0.3: Compiling C to WebAssembly

Writing WebAssembly in the text format by hand is quite cumbersome (and not very realistic).
Similarly, our JavaScript wrapper is very crude.
Let's fix both points by using `emscripten` to compile C code to WebAssembly and generate the necessary JavaScript and HTML around it.

Write the standard `hello.c`:

```C
#include <stdio.h>

int main() {
    printf("Hello, world!\n");
    return 0;
}
```

Compile with emscripten to WebAssembly, including an HTML harness.
(Two notes: Make sure your path is correctly set-up for emscripten. Typically this is done by running `source path/to/your/emsdk/emsdk_env.sh`. Also, if you are running an old version of emscripten, you might need to add `-s WASM=1` to the command below.)

```
$ emcc hello.c -o hello.html
$ ls
hello.c  hello.html  hello.js  hello.wasm
```

Note the three generated files: An HTML website, a JavaScript file with wrappers, and the actual WebAssembly module.

Finally, lets see how the generated website looks, by (as always) starting a web server 

```
python -m SimpleHTTPServer
```

and opening http://localhost:8000/hello.html in a browser.

*Bonus: Check the JavaScript and wasm files for some nitty gritty details of Emscripten.*

*Bonus: Note how large the WebAssembly file is. How would you reduce the size of a binary in a regular C compiler? See how that works here.*