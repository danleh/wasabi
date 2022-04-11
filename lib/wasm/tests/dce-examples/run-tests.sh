#!/bin/bash

exit_status () {
    if [ $? -eq 0 ]
    then 
        echo "  ...SUCCESS"
    else 
        echo "  ...ERROR: check stdout.txt for output"
        exit 1 
    fi 
}

GET_CALLSITE_INFO=0
if [ "$1" = "--get-callsite-info" ]; 
then 
    GET_CALLSITE_INFO=1 
elif [ "$1" = "--help" -o "$1" = "-h" ]; 
then 
    echo "--get-callsite-info : get static and dynamic callsite info for tests"
    exit 0
fi

for lib in */ ; 
do  
    cd $lib
    echo "$lib"
    
    if [ $lib = "hpcc-lib/" ]
    then 
    	cd ..
        continue        
    fi

    wasm_file=`find . -maxdepth 1 -type f -name "*.wasm"`
    
    for test in */ ; 
    do 
        cd $test 
        
        rm -r output_files > /dev/null
        rm -r analysis_data > /dev/null

        mkdir output_files
        mkdir analysis_data
        
        printf "\n$lib$test\n"

        echo -n "node index.js --reachable-exports --lower-bound"
        node index.js --reachable-exports --lower-bound > ./output_files/index.js.stdout.txt 
        exit_status
        
        if [ "$GET_CALLSITE_INFO" = 1 ]; 
        then 
            echo -n "node index.js --get-callsite-sensitive-cg"
            node index.js --get-callsite-sensitive-cg > /dev/null
            exit_status 
        fi 
        
        echo -n "RUSTFLAGS=-Awarnings cargo -q run --release --bin dce ../${wasm_file:2} reachable-exports.txt dce.wasm > analysis.stdout.txt"
        RUSTFLAGS=-Awarnings cargo -q run --release --bin dce ../${wasm_file:2} ./analysis_data/reachable-exports.txt dce.wasm > ./output_files/analysis.stdout.txt
        exit_status
        
        cd .. 

    done
    cd ..

done

python3 analysis-aggregate.py