sort inputs into
  - "testsuite" for Wasm features, very small files, mostly generated from text format
  - "integration" style end-to-end tests: real-world/large Wasm binaries

write tests to automatically traverse ALL wasm files in these directories

new output dirs
  - just validating that wasm files can be parsed (needs no dir)
  - that writing them out again (uninstrumented) are still wasm-validate OK
  - wasm-validate after instrumentation
