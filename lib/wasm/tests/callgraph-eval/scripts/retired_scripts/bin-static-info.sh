#!/bin/bash

# report the number of types, functions, elements in the elem section, exports, imports, calls and call_indirects in the binary passed as input 

# find . -maxdepth 2 -type f -name *.wasm -exec bin-static-info.sh {} \;

if [[ "$1" == "--help" || "$1" == "-h" ]]; then 
	echo "Usage: bin-static-info.sh"
	echo "Report the number of types, functions, elements in the elem section, exports, imports, calls and call_indirects in the test binaries."  
	exit 1
fi

# TODO: What happens with multiple tables? multiple elem sections in table? multiple sections in multiple tables? 

WASM_FILE=$1

LIB=`echo $WASM_FILE | grep -Eo "/.*/" `
LIB="${LIB:1:-1}"

WAT_FILE="$WASM_FILE.wat"

if ! test -f "$WAT_FILE"; then
	`wasm2wat $WASM_FILE -o $WAT_FILE`
fi

TYPES_NUM=`wasm-objdump -h $WASM_FILE | grep -Eo "Type(.*)count:.*" | sed 's/Type.*count//' | sed 's/[^0-9]*//g'`

FUNC_NUM=`wasm-objdump -h $WASM_FILE | grep -Eo "Function(.*)count:.*" | sed 's/Function.*count//' | sed 's/[^0-9]*//g'` 

EXPORT_NUM=`wasm-objdump -h $WASM_FILE | grep -Eo "Export(.*)count:.*" | sed 's/Export.*count//' | sed 's/[^0-9]*//g'` 

IMPORT_NUM=`wasm-objdump -h $WASM_FILE | grep -Eo "Import(.*)count:.*" | sed 's/Import.*count//' | sed 's/[^0-9]*//g'` 

CALL_NUM=`grep -cEo "call ([0-9]+)" $WAT_FILE`

UNIQUE_CALL_NUM=`grep -Eo "call ([0-9]+)" $WAT_FILE | sort --unique | wc -l`

CALLIND_NUM=`grep -cEo "call_indirect \(" $WAT_FILE`

ELEM_NUM=`wasm-objdump -x $WASM_FILE | grep -cEo "elem\["`

DATA_PATH="/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/library_data/$LIB"

eval "rm $DATA_PATH/static-data.csv"
eval "touch $DATA_PATH"

#EXPORT_LINENUM=`wasm-objdump -x sql-wasm.wasm | grep -nEo "Export\[[0-9]+\]" | cut -f1 -d:`
#let "TILL_LINE = $EXPORT_LINENUM + $EXPORT_NUM"
#eval "wasm-objdump -x sql-wasm.wasm | sed '$EXPORT_LINENUM,$TILL_LINE!d'" >> "$DATA_PATH/exported-functions.txt"

echo "$WASM_FILE, #types:$TYPES_NUM, #funcs:$FUNC_NUM, #elem: $ELEM_NUM, #exports: $EXPORT_NUM, #imports: $IMPORT_NUM, #calls: $CALL_NUM, #uniq_calls: $UNIQUE_CALL_NUM, #callinds: $CALLIND_NUM" >> "$DATA_PATH/static-data.csv"


