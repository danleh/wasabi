# Task 1: Your First Wasabi Analysis

The goal of this task is to learn how to build and run a Wasabi analysis. Go to the `tutorial/task1` directory.

## Step 1: Inspect

Have a look at the following files:

* `hello_world.wat`: A minimal WebAssembly module that defines a function "myfunction", which prints the answer to number 42.

* `log_all.js`: A simple Wasabi analysis that implements all available hooks and logs every invocation of every hook.

## Step 2: Build

Build a web site that loads `hello_world.wat` and analyses its execution with `log_all.js`:

`./buildBrowserTest.sh hello_world.wat log_all.js`

The script will 
* print `generated 7 low-level hooks`, and
* create two directories `out` and `hello_world_out`


## Step 3: Understand

Have a look at the files in `hello_world_out`:

* `hello_world.wasm` is the WebAssembly binary version of our minimal WebAssembly module

* `hello_world_instr.wasm` is the instrumented version of the above file. Wasabi adds various instrumentation instructions to monitor each of the original instructions.

* `hello_world.wasabi.js` contains the JavaScript-part of the Wasabi analysis engine. It translate between low-level WebAssembly instructions and the high-level analysis hooks.

* `hello_world.html` is a minimal HTML harness that loads the instrumented WebAssembly module via `WebAssembly.instantiate`. Loading the module will execute `myfunction` and trigger the corresponding analysis hooks in `log_all.js`.

## Step 4: Run

Now, let's load the web site and see our analysis in action!

Start a web server:
`python -m "SimpleHTTPServer"`

Visit [hello_world.html](http://localhost:8000/hello_world_out/hello_world.html) and open the web console (Ctrl+Shift+K in Firefox, Ctrl+Shift+I in Chrome).

You should see `Initializating Wasabi analysis...`, followed by logs of several hooks: ``start``, ``begin function``, ``const, value = 42``, etc. Compare the logged hook invocations to the instructions in the WebAssembly module.

## Step 5: Adapt

Go back to the Wasabi analysis in ``log_all.js`` and adapt it. For example, you could delete a hook, which will cause the corresponding instructions to be not logged anymore. Or you could add a counter to keep track of how often particular kinds of instructions are executed. After each modification, re-run the `buildBrowserTest.sh` command from above, and then re-load the page in your browser.

Congratulations! If you reach this, you have successfully implemented and executed your first Wasabi analysis. 

