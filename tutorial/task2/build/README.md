# Task 2: Call Graph Analysis on a 3D Game

Now, we want to make our usage of Wasabi more interesting in two dimensions:

1. We will analyse a "real-world"/more useful program: A simple 3D game, written in C, that we have already compiled to WebAssembly and that runs smoothly in the browser. (See https://github.com/kripken/BananaBread for more information about it.)
2. A useful dynamic analysis that is an important building block for many other analyses: Getting the dynamic call graph of a program.

In this directory, you find the zipped resources for said game.
As a baseline, make sure you can run it without any instrumentation or analysis.
For that, unzip the bb.zip file, and serve the contents of that directory as before:

```
$ unzip bb.zip -d bb
$ pushd bb/
$ python -m SimpleHTTPServer
```

When you open http://localhost:8000/bb.html in a webbrowser, you will see the Emscripten logo and a small canvas on which - after a short loading time - a 3D game will appear.
You can move around with WASD and the mouse.

## Instrumenting with Wasabi

Before we can dynamically analyse the WebAssembly binary, we need to instrument it (that is, rewrite the binary and insert some "analysis glue code") using Wasabi.

For that, change into the game directory `bb/` and see that there is a ca. 2MB large `bb.wasm` file.
Since we want to only perform a call graph analysis, we can select to insert code for WebAssembly `call` instructions only, by using the `--hooks=` option.
(Note that this option must be given with the equals sign.)
That is, inside `bb/` execute:

```
wasabi --hooks=call bb.wasm .
```

This does three things:
- `--hooks=call` will instrument only call instructions, nothing else, which produces less code size increase and runtime overhead.
- `.` means that the outputs shall be placed in the current directory.
- One output is the instrumented file. Since it will also be named bb.wasm, it overwrites the original, uninstrumented file.
- The second output is `bb.wasabi.js`, a generated JavaScript file that contains glue code and statically extracted information about the binary.

## Adding the Wasabi runtime

If you open http://localhost:8000/bb.html now, you will see an error similar to

```
WebAssembly.instantiate(): Import #926 module="__wasabi_hooks" error: module is not an object or function
```

The reason is that while we have instrumented the binary, we have not added the Wasabi runtime to the program harness.
We will fix this, by inserting the following line into `bb.html` before the closing `</head>` tag (line 99):

```
<script src="bb.wasabi.js"></script>
```

When you open http://localhost:8000/bb.html now in your browser and check the JavaScript console (e.g., by opening the developer tools in Firefox and Chromium with F12), you will see (among many debug messages produced by Emscripten, which we ignore) something like

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

## Adding a Wasabi Analysis in JavaScript

Now, let's write the actual analysis with Wasabi.
You find a minimal template in `analysis-template.js`.
It shows that a Wasabi analysis is a JavaScript object, where properties are the Wasabi hooks that perform the actual analysis.
As it is a template, the `call_pre` hook is left to fill out:

```
Wasabi.analysis = {
    call_pre(location, targetFunc, args, indirectTableIdx) {
        // TODO
    },
};
```

Copy that file `analysis.js` to the `bb/` directory and let's start from there.

```
cp analysis-template.js bb/analysis.js
```

Also, add that analysis to `bb.html` by inserting the following line (i.e., adding a new script tag) to the `<head>` of the website.

```
<script src="analysis.js"></script>
```

## Obtaining a call graph at runtime

Subtasks:

- Assume, we want to represent the call graph as a set of edges. Add a global variable with an empty JavaScript `Set` to the analysis file.
- An edge consists of the current function and the target function. In WebAssembly, functions are identified by their index. How to obtain the current function, and how to obtain the target function in the `call_pre` hook?
- Add an edge as a string of the form `callerIndex + " -> " + calleeIndex` to the edge set inside the hook.
- Inspect the call graph edge set by opening the JavaScript console in the DevTools and typing the name of the edge set variable.

## Making the call graph nicer

### Subtask 1: Add function names, where available.

- WebAssembly functions only have a name when they are imported, exported, or when we have debug information (in WebAssembly lingo: a "name section" in the binary).
- Wasabi saves the import and export names for a function `f` in `Wasabi.module.info.functions[f].export[]` and `Wasabi.module.info.functions[f].import`.
- Retrieve the function names, if present from this information and put them instead of the index into the edge string.

### Subtask 2: Generate a plot from the edge set with Graphviz.

While this is not really related to Wasabi itself, it gives us a more comprehensible impression of the call graph.
Ideally, we would like something like `call-graph-example.pdf`.

Steps:

- Transform the edge set into a Graphviz `.dot` file format. Check out Graphviz for more information about that: https://www.graphviz.org/documentation/. In particular, this means:
    * Converting the edge set into a list (array) of lines, separated by semicolons.
    * Avoiding special characters in the node name by replacing them with a regex (for named functions).
    * Surrounding everything by `digraph { <edges...> }`
- "Exporting" the so produced dot file from the browser.
See, e.g., https://stackoverflow.com/questions/11849562/how-to-save-the-output-of-a-console-logobject-to-a-file
- Generating a PDF from the .dot through graphviz. Useful options are `-Ksfdp` (a particular graph layout algorithm), `-Goverlap=false` (avoiding overlapping nodes and edges), and `-Tpdf`.