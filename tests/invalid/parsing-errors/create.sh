#!/bin/bash

echo -n '' > empty-file.wasm
echo -ne 'Xasm' > magic-number-invalid.wasm 
echo -ne '\x00asm' > version-missing.wasm
echo -ne '\x00asm\x02\x00\x00\x00' > version-invalid.wasm

# last byte: section id 0 == custom section
echo -ne '\x00asm\x01\x00\x00\x00\x00' > custom-section-size-missing.wasm
# last byte: section length in bytes (as LEB128)
echo -ne '\x00asm\x01\x00\x00\x00\x00\x00' > custom-section-size-0-name-missing.wasm
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
echo -ne '\x00asm\x01\x00\x00\x00\x01\x02\x01' > type-section-elements-missing.wasm


# check against official wasm-validate from WABT to make sure all of these are really invalid
# start with smallest first
for file in $(ls -Sr *.wasm)
do
    echo "$file"
    wasm-validate "$file"
    cargo run -q -- "$file"
done
