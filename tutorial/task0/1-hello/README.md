# Task 0.1: A Minimal WebAssembly Program

## Step 1: Understanding the Text Format

Read and understand `hello.wat` and copy it to an empty directory.

```
;; hello.wat
(module
  (import "env" "print"
    (func $print (param i32))
  )
  (func $start
    i32.const 42
    call $print
  )
  (start $start)
)
```

## Step 2: Converting between Text and Binary Format

Assemble the text file to a binary module with `wat2wasm`:

```
$ wat2wasm hello.wat
$ ls
hello.wat  hello.wasm
```

Check that `hello.wasm` is actually a binary (and with 50 bytes more compact than the text file):

```
$ hexdump -C hello.wasm
00000000  00 61 73 6d 01 00 00 00  01 08 02 60 01 7f 00 60  |.asm.......`...`|
00000010  00 00 02 0d 01 03 65 6e  76 05 70 72 69 6e 74 00  |......env.print.|
00000020  00 03 02 01 01 08 01 01  0a 08 01 06 00 41 2a 10  |.............A*.|
00000030  00 0b                                             |..|
00000032
```

You can also disassemble `hello.wasm` again. Note how the "names" of the indices are actually lost in the binary and how types are defined separately. `(; bla ;)` is the syntax for inline comments.

```
$ wasm2wat hello.wasm
(module
  (type (;0;) (func (param i32)))
  (type (;1;) (func))
  (import "env" "print" (func (;0;) (type 0)))
  (func (;1;) (type 1)
    i32.const 42
    call 0)
  (start 1))
```

For a more low-level view into the binary, you can also use `wasm-objdump` (similar to `objdump` for native binaries):

```
$ wasm-objdump -hxd hello.wasm

hello.wasm:     file format wasm 0x1

Sections:

     Type start=0x0000000a end=0x00000012 (size=0x00000008) count: 2
   Import start=0x00000014 end=0x00000021 (size=0x0000000d) count: 1
 Function start=0x00000023 end=0x00000025 (size=0x00000002) count: 1
    Start start=0x00000027 end=0x00000028 (size=0x00000001) start: 1
     Code start=0x0000002a end=0x00000032 (size=0x00000008) count: 1

Section Details:

Type[2]:
 - type[0] (i32) -> nil
 - type[1] () -> nil
Import[1]:
 - func[0] sig=0 <print> <- env.print
Function[1]:
 - func[1] sig=1
Start:
 - start function: 1

Code Disassembly:

00002b func[1]:
 00002d: 41 2a                      | i32.const 42
 00002f: 10 00                      | call 0 <print>
 000031: 0b                         | end
```

## Step 3: Embedding WebAssembly into a Website

Copy `hello.html` to the directory and look inside to see that:

- The two level import on the WebAssembly side corresponds to a nested `importObject` in JavaScript.
- The `hello.wasm` file is dynamically requested with the `fetch()` API and loaded into an `ArrayBuffer`.
- That buffer and the `importObject` is passed to `WebAssembly.instantiate()`.

## Step 4: Run WebAssembly in the Browser 

Start a webserver in the directory with:

```
python -m SimpleHTTPServer
```

(The webserver is necessary due to browser security policies. Otherwise, fetch won't be allowed to access local files.)

Open http://localhost:8000/hello.html in a browser and click on the button to see the output `42` appear.
