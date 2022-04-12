#!/bin/bash

# script to automate tests for Wasm DCE tool 

# checks the exit code of the program that was just run 
# (exit code is stored in $?)
# and either declares success or 
# alerts user to an stderr and exits bash script 
exit_status () {
    if [ $? -eq 0 ]
    then 
        echo "  ...SUCCESS"
    else 
        echo "  ...ERROR: check stdout.txt for output"
        exit 1 
    fi 
}

# same as above but does not exit bash script
exit_status_no_exit () {
    if [ $? -eq 0 ]
    then 
        echo "  ...SUCCESS"
    else 
        echo "  ...ERROR: check stdout.txt for output"
    fi 
}

# options
#   --get-callsite-info gets the dynamic callsite info 
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
    
    # find the wasm file and remember the name  
    wasm_file=`find . -maxdepth 1 -type f -name "*.wasm"`
    
    for test in */ ; 
    do 
        if [ $test = "operations/" ]
        then 
            continue        
        fi

        cd $test 
                
        # remove old analysis and stdout files 
        rm -r output_files > /dev/null
        rm -r analysis_data > /dev/null

        # make directories to save analysis and stdout files 
        mkdir output_files
        mkdir analysis_data
        
        printf "\n$lib$test\n"

        # get the lowerbound of the analysis and exported functions 
        echo -n "node instrumented_index.js --reachable-exports --lower-bound"
        node instrumented_index.js --reachable-exports --lower-bound &> ./output_files/index.js.stdout.txt 
        exit_status
        
        # if user wants to, get the dynamic callsite information 
        if [ "$GET_CALLSITE_INFO" = 1 ]; 
        then 
            echo -n "node instrumented_index.js --get-callsite-sensitive-cg"
            node instrumented_index.js --get-callsite-sensitive-cg > /dev/null
            exit_status 
        fi 
        
        # run the dce tool on the wasm file with the reachable exports as entry points  
        echo -n "RUSTFLAGS=-Awarnings cargo -q run --release --bin dce ../${wasm_file:2} reachable-exports.txt dce.wasm > analysis.stdout.txt"
        RUSTFLAGS=-Awarnings cargo -q run --release --bin dce ../${wasm_file:2} ./analysis_data/reachable-exports.txt dce.wasm &> ./output_files/analysis.stdout.txt
        exit_status
        
        # now we want to test if the dce'd wasm file is sound 
        # for this we will have to replace the instrumented wasm file with the dce'd file 
        # and then run the non-instrumented test  
        
        # save the current path because we want to get back here to run the test 
        curr_path=`pwd`

        # save path of the dce.wasm file 
        wasm_dce_file=`realpath dce.wasm` 

        # get the directory where we have to replace the wasm file 
        # it is always with a .wasabi file, which is going to be unique in every test 
        # find on the wasabi file will give out two results, there will always be a result in a dir named out 
        # the directory where we have to replace the wasm file is in parent of out 
        wasabi_file=`find . -type f -name "*.wasabi.js" | grep out`
        wasabi_file_dir=${wasabi_file/out*/}

        # cd to where we have to replace the file 
        cd $wasabi_file_dir

        # wasabi instrumented files are present in out so no need to save them 
        rm $wasm_file
        rm $(find . -maxdepth 1 -type f -name "*.wasabi.js")
        rm ./long.js

        # copy the dce file to this directory and rename it to the original wasm file 
        # since it has to be found by the test 
        cp $wasm_dce_file .
        mv dce.wasm ${wasm_file:2}

        # go back to where the non-instrumented test is 
        cd $curr_path
        
        # run the test 
        echo "Running test with DCE'd wasm file"
        echo -n "node index.js"            
        node index.js &> ./output_files/dce_test.stdout.txt       
        exit_status_no_exit

        # we want to replace the dce'd wasm file since we have to get back to the state we were when 
        # we started this script 
        # TODO: automate the wasabi script and incorporate into this? 
        cd $wasabi_file_dir
        rm $wasm_file
        cp -v ./out/* . > /dev/null
        cd $curr_path 
        
        cd .. 

    done

    cd ..

done

# aggregate the results 
echo ""
echo "Aggregating the results - check analysis-aggregate.csv"
python3 analysis-aggregate.py
