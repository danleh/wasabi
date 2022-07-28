#!/bin/bash

WASM_FILE=$1

LIB=`echo $WASM_FILE | grep -Eo "/.*/" `
LIB="${LIB:1:-1}"

WAT_FILE="$WASM_FILE.wat"

if ! test -f "$WAT_FILE"; then
	`wasm2wat $WASM_FILE -o $WAT_FILE`
fi

# save the names of all the exported functions 

