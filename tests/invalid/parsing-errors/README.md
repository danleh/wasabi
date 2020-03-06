Testing error handling of the Wasm binary parser.
Each of the files is invalid in some way, which can usually be told from the filename.
Since invalid binaries cannot be produced from the text format, we create them by hand (hex editor on valid binaries or manually writing all bytes).
