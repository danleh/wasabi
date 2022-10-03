| Benchmark Program | Browser | Emscripten | Optimization | Instrumented Instructions | Low-level hooks | Analysis | Time | Factor vs Uninstrumented |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| PolyBench/C atax | N/A (native) | clang 5.0.0-3 | -O3 | N/A | N/A | N/A | 0.006s | 0.6x (i.e. native is obviously faster) |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | None | N/A | N/A | 0.01s | 1x |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | All | Full | Null | 1.72s | 172x |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | All | Empty | Null | 1.31s | 131x |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | All, except ```*_local```, ```*.const```, and numeric instructions | Full | Null | 0.39s | 39x |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | All, except ```*_local```, ```*.const```, and numeric instructions | Empty | Null | 0.29s | 29x |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | All, except ```*_local```, ```*.const```, ```*.load_*```, ```*.store_*```, and numeric instructions | Full | Null | 0.20s | 20x |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | All, except ```*_local```, ```*.const```, ```*.load_*```, ```*.store_*```, and numeric instructions | Empty | Null | 0.12s | 12x |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | All | Full | Statement Coverage | 13s | 1300x |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | All | Full | Instruction Counting | 11.7s | 1170x |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | All | Full | Branch Coverage | 2.61s | 261x |
| PolyBench/C atax | Firefox 61.0.1 (Ubuntu 17.10 x64 stable) | 1.37.22 (AFAIK LLVM 4.0) | -O3 | All | Full | Call Graph (named [=imported/exported] functions only) | 2.23s | 223x |

On atax, FF 61, emcc 1.37.22: What is the influence of A) instrumenting less instructions and B) having empty low-level hooks?

| Instructions / Low-level Hooks | Empty | Full |
| ---: | ---: | ---: |
| None | 0.01 | 0.01 |
| No local, const, numeric, load, store (i.e., almost control-flow only) | 0.12 (12x to above) | 0.20 (20x to above, 1.66x to left) |
| No local, const, numeric (i.e., add load/store cf. with above) | 0.29 (2.42x to above) | 0.39 (1.95x to above, 1.34x to left) |
| All | 1.31 (4.51x to above) | 1.72 (4.41x to above, 1.31x to left) |

From this, we can conclude:
- adding the low-level hook bodies adds 30-70% overhead
- adding load and store instrumentation adds 200-240% overhead
- adding local, const, numeric instrumentation adds 440-450% overhead