#!/bin/bash

echo -n '' > empty-file.wasm
echo -ne 'Xasm' > magic-number-invalid.wasm 
echo -ne '\x00asm' > version-missing.wasm
echo -ne '\x00asm\x02\x00\x00\x00' > version-invalid.wasm

# last byte: section id 0 == custom section
echo -ne '\x00asm\x01\x00\x00\x00\x00' > custom-section-size-missing.wasm
# last byte: section length in bytes (as LEB128)
echo -ne '\x00asm\x01\x00\x00\x00\x00\x01' > custom-section-size-too-large.wasm
# last byte: custom section name length in bytes (as LEB128)
# invalid because section size in bytes is zero, so there cannot be enough space for the section name length alone
echo -ne '\x00asm\x01\x00\x00\x00\x00\x00\x00' > custom-section-size-0-name-empty.wasm

echo -ne '\x00asm\x01\x00\x00\x00\xFF' > section-id-invalid.wasm
# last three bytes:
# section id 1 == type section
# section size: 1 byte
# type count: 2
echo -ne '\x00asm\x01\x00\x00\x00\x01\x01\x02' > type-section-too-many-elements-for-size.wasm
# two function types (both starting with 0x60 and having 0 args and results), 
echo -ne '\x60\x00\x00\x60\x00\x00' >> type-section-too-many-elements-for-size.wasm
# section size: 2 bytes
# type count: 1
echo -ne '\x00asm\x01\x00\x00\x00\x01\x02\x01' > type-section-elements-missing.wasm

echo -ne '\x00asm\x01\x00\x00\x00' > only-code-no-function-section.wasm
# section id: 0xa = 10 == code section
# size of whole section in bytes: 11
# number of code entries: 3
echo -ne '\x0a\x0b\x03' >> only-code-no-function-section.wasm
# code entry 1
# size in bytes: 2
# number of locals: 0
# expression: just end (0xb)
echo -ne '\x02\x00\x0b' >> only-code-no-function-section.wasm
# code entry 2
# size in bytes: 3
# number of locals: 0
# expression: nop (0x1); end (0xb)
echo -ne '\x03\x00\x01\x0b' >> only-code-no-function-section.wasm
# code entry 3
# size in bytes: 2
# number of locals: 0
# expression: just end (0xb)
echo -ne '\x02\x00\x0b' >> only-code-no-function-section.wasm

echo -ne '\x00asm\x01\x00\x00\x00' > code-element-size-too-small.wasm
# section id: 0xa = 10 == code section
# size of whole section in bytes: 5 (correct)
# number of code entries: 1
echo -ne '\x0a\x05\x01' >> code-element-size-too-small.wasm
# code entry 1
# size in bytes: 2 (one too short!)
# number of locals: 0
# expression: nop (0x1); end (0xb)
echo -ne '\x02\x00\x01\x0b' >> code-element-size-too-small.wasm

echo -ne '\x00asm\x01\x00\x00\x00' > code-element-size-too-large.wasm
# section id: 0xa = 10 == code section
# size of whole section in bytes: 5 (correct)
# number of code entries: 1
echo -ne '\x0a\x05\x01' >> code-element-size-too-large.wasm
# code entry 1
# size in bytes: 4 (one too long!)
# number of locals: 0
# expression: nop (0x1); end (0xb)
echo -ne '\x04\x00\x01\x0b' >> code-element-size-too-large.wasm

# check against official wasm-validate from WABT to make sure all of these are really invalid
# start with smallest first
for file in $(ls -Sr *.wasm)
do
    echo ""
    echo "$file"
    wasm-validate "$file"
    cargo run -q -- "$file"
    # wasabi "$file"
done
