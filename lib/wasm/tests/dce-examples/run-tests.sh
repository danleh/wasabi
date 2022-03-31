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

for lib in */ ; 
do  
    cd $lib
    echo "$lib"
    
    if [ $lib = "hpcc-lib/" ]
    then 
    	cd ..
        continue        
    fi

    if [ $lib = "sql.js/" ]
    then 
    	cd ..
        continue        
    fi
    
    wasm_file=`find . -maxdepth 1 -type f -name "*.wasm"`
    
    for test in */ ; 
    do 
        cd $test 
        
        printf "\n$lib$test\n"

        echo -n "node index.js"
        node index.js > index.js.stdout.txt 
        exit_status
        
        echo -n "node index.js --reachable-exports"
        node index.js --reachable-exports >> index.js.stdout.txt 
        exit_status
        
        echo -n "RUSTFLAGS=-Awarnings cargo -q run --release --bin dce ../${wasm_file:2} reachable-exports.txt dce.wasm > analysis.stdout.txt"
        RUSTFLAGS=-Awarnings cargo -q run --release --bin dce ../${wasm_file:2} reachable-exports.txt dce.wasm > analysis.stdout.txt
        exit_status
        
        cd .. 

    done
    cd ..

done

python3 analysis-aggregate.py