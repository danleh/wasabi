# Task 2: Call Graph Analysis on a 3D Game

Now, we want to make our usage of Wasabi more interesting in two dimensions:

1. We will analyse a "real-world"/more useful program: A simple 3D game, written in C, that we have already compiled to WebAssembly and that runs smoothly in the browser. (See https://github.com/kripken/BananaBread for more information about it.)
2. A useful dynamic analysis that is an important building block for many other analyses: Getting the dynamic call graph of a program.

## Step 1: Make Sure the Original Program Runs

In this directory, you find the zipped resources for said game.
As a baseline, make sure you can run it without any instrumentation or analysis.
For that, unzip the bb.zip file, and serve the contents of that directory as before:

```
$ unzip bb.zip -d bb
$ cd bb/
$ python -m SimpleHTTPServer
```

When you open http://localhost:8000/bb.html in a webbrowser, you will see the Emscripten logo and a small canvas on which - after a short loading time - a 3D game will appear.
You can move around with WASD and the mouse.

## Step 2: Instrumenting with Wasabi

Before we can dynamically analyse the WebAssembly binary, we need to instrument it (that is, rewrite the binary and insert some "analysis glue code") using Wasabi.

For that, confirm that in the game directory (`bb/`) there is a ca. 2MB large `bb.wasm` file.
Since we want to only perform a call graph analysis, we can select to insert code for WebAssembly `call` instructions only, by using the `--hooks=` option.
(Note that this option must be given with the equals sign.)
That is, inside `bb/` execute:

```
wasabi --hooks=call bb.wasm .
```

This does three things:
- `--hooks=call` will instrument only call instructions, nothing else, which results in not quite as much code size increase and runtime overhead as instrumenting every instruction.
- `.` means that the outputs shall be placed in the current directory.
- One output is the instrumented file. Since it will also be named bb.wasm, it overwrites the original, uninstrumented file.
- The second output is `bb.wasabi.js`, a generated JavaScript file that contains glue code and statically extracted information about the binary.

## Step 3: Adding the Wasabi Runtime

We previously mentioned that Wasabi has also a runtime component (in the form of the generated `bb.wasabi.js` file).
What happens if we do not add that?
Since it is a common mistake to forget adding the runtime, let's first find out what happens without it.

Serve the `bb/` directory again with

```
python -m SimpleHTTPServer
```

If you open http://localhost:8000/bb.html now, you will see an error similar to `WebAssembly.instantiate(): Import #926 module="__wasabi_hooks" error: module is not an object or function` (Chrome) or `TypeError: import object field '__wasabi_hooks' is not an Object` (Firefox).

That is, the instrumented binary imports some functions with the name `__wasabi_hooks` that are not provided to it.
Since non-existent imported functions cannot be executed, running the instrumented binary without the Wasabi runtime fails.

Let's fix this by inserting the following line into `bb.html` before the closing `</head>` tag (line 99):

```
<script src="bb.wasabi.js"></script>
```

When you open http://localhost:8000/bb.html again and check the JavaScript console (e.g., by opening the developer tools in Firefox and Chromium with F12), you will see (among many debug messages produced by Emscripten, which we ignore) something like

```
start hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
if_ hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
br hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
br_if hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
br_table hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
begin hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
end hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
nop hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
unreachable hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
drop hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
select hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
call_pre hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
call_post hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
return_ hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
const_ hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
unary hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
binary hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
load hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
store hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
memory_size hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
memory_grow hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
local hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
global hook not provided by Wasabi.analysis, add empty function as fallback bb.wasabi.js:141:25
```

and the game should be working as before.

## Step 4: Template for a Simple Call Graph Analysis

Now, we want to start writing the actual analysis with Wasabi.
You find a minimal template in `analysis-template.js`.
Copy that file to `bb/analysis.js` and let's have a look inside.
For recording WebAssembly function calls, we need only a single analysis hook, namely `call_pre`.
As the analysis is only a template so far, the hook body is left to fill out:

```
Wasabi.analysis = {
    call_pre(location, targetFunc, args, indirectTableIdx) {
        // TODO
    },
};
```

Also, add that analysis to `bb.html` by inserting the following line (i.e., adding a new script tag) to the `<head>` of the website.

```
<script src="analysis.js"></script>
```

## Step 5: Implementing the Analysis

Implementation step by step:

- For simplicity, let's represent the call graph as a set of edges. Add a global variable `edgeSet` at the top of `analysis.js` and initialize it with an empty [JavaScript Set](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set).
- An edge consists of the current function (caller) and the target function (callee). We will add such an edge inside the `call_pre` hook, which is executed just before every WebAssembly `call` instruction in the analyzed program.
- In WebAssembly, functions are identified by their index. The current function index (i.e., the caller) can be obtained from `location` argument inside the `call_pre` hook (via `location.func`). The target function index (the callee) is given as the second argument to the hook (`targetFunc` in the analysis template).
- Again, to keep things simple, we will represent a single edge as a string of the format "caller -> callee". Concatenate the two function indices with a literal "->" string in between them, then `add()` that string to the global `edgeSet`.

A very first prototype of the analysis is now done (and should be less then 10 lines of JavaScript).
We can test it by opening http://localhost:8000/bb.html again.
Once the game has started, go to the JavaScript console and type `edgeSet` (i.e., inspect the call graph as the set of its edges).
You will get output similar to:

```
Set(3557) [ "3064 -> 1226", "1226 -> 4096", "4096 -> 4183", "4183 -> 2", "3064 -> 1378", "1378 -> 3934", "1378 -> 4096", "1378 -> 4052", "4052 -> 4053", "4053 -> 4182", … ]
```

## Step 6: Visualizing the Call Graph

Getting a list of edges is sufficient for further programmatic analysis of the call graph.
But for the purpose of this demo, let us visualize this edge set in a nice plot, generated with the Graphviz tool.
Since this is orthogonal to Wasabi and has not much to do with learning how to write dynamic analyses, we have already prepared a script in `save-call-graph.js`.
Copy that into the game directory `bb/` and add the following line to the `head` section of `bb.html`:

```
<script src="save-call-graph.js"></script>
```

Save `bb.html`, re-open http://localhost:8000/bb.html, wait until the game starts, and open the JavaScript console.
Type the following in the JavaScript console and hit enter:

```
saveCallGraph(edgeSet)
```

This should open a download dialog with a `call-graph.dot` file.
Save it to the `task2/` directory.

To generate a nice plot from the .dot file, we will use Graphviz.
We have placed a very simple script to do so in `dot2pdf.sh`, which you can invoke like so

```
./dot2pdf.sh call-graph.dot
```

Check the generated `call-graph.pdf` file for a first overview of the functions and their call relation.

## Step 7: Add Function Names, Where Available

While the structure of the call graph is interesting to look at, names instead of indices would be even more useful.
Unfortunately, as WebAssembly is quite a low-level format, functions only have a name when they are imported, exported, or when we have debug information (in WebAssembly lingo: a "name section" in the binary).

For exported and imported functions, Wasabi saves their import/export name into `Wasabi.module.info.functions[idx]`.
We can make use of this information in the call graph analysis, by writing a function `functionName` that takes an `index` as argument and:

- If `Wasabi.module.info.functions[idx].export[0] != undefined` (i.e., it is exported at least once), return that value instead of the index.
- If `Wasabi.module.info.functions[idx].import != undefined` (i.e., it is imported), return that value instead of the index.
- As a fallback, return the index.

Then, use `functionName` on the caller and callee index in `call_pre` to have call graph edges with names (if available).

Finally, re-open http://localhost:8000/bb.html, open the JS console, download the call graph via `saveCallGraph(edgeSet)`, generate a PDF from the .dot file via `./dot2pdf.sh call-graph.dot`, and hopefully see that some of the nodes now have names in the plot.
