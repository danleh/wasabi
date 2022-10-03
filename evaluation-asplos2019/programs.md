# Potential Benchmark Programs for Evaluating Wasabi Overhead

## synthetic/toy programs/kernels

- [PolyBenc/C, version 4.2.1-beta](http://web.cse.ohio-state.edu/~pouchet.2/software/polybench/)
    * (seems to be) mostly for benchmarking polyhedral compiler optimizations (loop tiling, SIMD, etc.)
    * 30 programs
    * pure computation, e.g., matrix operations, different solvers, image processing
    * used in PLDI 2017 WebAssembly paper to compare Wasm runtimes vs. native, and code size vs. asm.js and native
    * got it running with clang (native) and emscripten
- [SciMark 2.0, ANSI C version](https://math.nist.gov/scimark2/download_c.html)
    * from Website: "Java benchmark for scientific and numerical computing. It measures several computational kernels and reports a composite score in approximate Mflops"
    * single program ("driver") but multiple kernels
    * used in PLDI 2017 WebAssembly paper to compare code size against asm.js and native
- [The Computer Language Benchmarks Game: C or C++ or Rust versions?](https://benchmarksgame-team.pages.debian.net/benchmarksgame/faster/c.html)
    * there has been some precedent uses in papers, e.g., http://www.dcs.gla.ac.uk/~wingli/jvm_language_study/jvmlanguages.pdf, http://design.cs.iastate.edu/vmil/2013/papers/p05-Sarimbekov.pdf, http://www.softlab.ntua.gr/research/techrep/CSD-SW-TR-8-09.pdf
    * 10 programs, see https://benchmarksgame-team.pages.debian.net/benchmarksgame/download/benchmarksgame-sourcecode.zip
- [Ostrich Benchmark Suite](http://www.sable.mcgill.ca/mclab/projects/ostrich/)
    * from Website: "Ostrich is a benchmark suite developed in the Sable Lab at McGill University with the objective of studying the performance of JavaScript and WebCL for numerical code. Version of the benchmarks are available in C/C++, JavaScript, OpenCL and WebCL."
    * 12 programs, e.g., back propagation, bfs, fft, page rank, ...
    * used in WebAssembly performance technical report (from same University): http://www.sable.mcgill.ca/publications/techreports/2018-2/techrep.pdf
- SPEC CPU 2006 (?) or 2017: did not try, need to buy their inputs

## bigger applications/"real programs"

- [PSPDFKit WASM Benchmark](https://pspdfkit.com/webassembly-benchmark/)
    * very recent (2018-07-05), see announcement under https://pspdfkit.com/blog/2018/a-real-world-webassembly-benchmark/
    * real product (not synthetic): PDF rendering and annotation in the browser
    * Problem: How to instrument with Wasabi?
        A) Download and instrument offline
        B) Use mitmproxy to on-the-fly call Wasabi and rewrite Website (tricky)
- [Epic Zen Garden (Unreal Engine 4 Demo/Benchmark)](https://s3.amazonaws.com/mozilla-games/ZenGarden/EpicZenGarden.html)
    * big, real world game engine (i.e., a "realistic" use case for Wasm)
    * lots of rendering, shaders, etc., so not "just" WebAssembly
    * with parameters one can turn of rendering, Audio, etc., then it is in "benchmark mode": https://s3.amazonaws.com/mozilla-games/ZenGarden/EpicZenGarden.html?playback&novsync&noaudio&fakegl&webgl1
    * there IS an offline version: https://s3.amazonaws.com/mozilla-games/ZenGarden/2017-03-16-ZenGarden.zip
    (found in a Firefox bugtracker comment https://bugzilla.mozilla.org/show_bug.cgi?id=1347950#c0)
- [Unity WebGL benchmark](https://beta.unity3d.com/jonas/benchmark2015/)
    * also a "real-world game engine"
    * don't know how to download it yet
    * used in PLDI 2017 WebAssembly paper to compare code size of Wasm vs. asm.js