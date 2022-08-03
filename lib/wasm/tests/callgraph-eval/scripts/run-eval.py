import os, sys
from subprocess import PIPE, Popen
import re

TEST_SUITE_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/test-suite" 

SCRIPTS_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/scripts/"

DATA_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/data.json"
TEST_SUITE_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/test-suite.json"

def extract_name(path): 
    return path.split("/")[len(path.split("/"))-1]

def extract_wasm_and_test_paths(): 
    paths = {} # {lib -> {testpaths -> [], wasmpath -> ""}}
    for item1 in os.listdir(TEST_SUITE_PATH):
        item1_path = os.path.join(TEST_SUITE_PATH, item1)
        if os.path.isdir(item1_path): 
            paths[extract_name(item1_path)] = {"test_paths" : [], "wasm_path" : ""}
            for item2 in os.listdir(item1_path):
                item2_path = os.path.join(item1_path, item2)
                if os.path.splitext(item2_path)[1] == ".wasm":
                    paths[extract_name(item1_path)]["wasm_path"] = item2_path
                elif os.path.isdir(item2_path):
                    paths[extract_name(item1_path)]["test_paths"].append(item2_path) 
    return paths 

def execute_command(command):
    p = Popen(command, shell=True, stdout=PIPE, stderr=PIPE)
    stdout, stderr = p.communicate()
    stdout, stderr = stdout.decode("utf-8").split("\n"), stderr.decode("utf-8").strip()
    if stderr != "": sys.exit(stderr)
    return stdout

def main():

    args = sys.argv[1:]
    if len(args) > 0: 
        print("Usage: run-eval.py")
        print("This script does the following things:")
        print("  - Extracts static information for each wasm library.")
        print("  - Computes sets of reachable functions for each tool being evaluated, for each wasm library.")
        print("  - Reinstruments and computes sets of reachable functions for each test using Wasabi.")
        print("  - Runs analysis on all the extracted data.")
        print("  - Latexifies all the data and reports results on stdout.")
        sys.exit()

    # Start "fresh" by removing the old results and copying test-suite.json to data.json.
    os.system("rm {}".format(DATA_JSON_PATH))
    os.system("cp {} {}".format(TEST_SUITE_JSON_PATH, DATA_JSON_PATH))

    # Build ourtool 
    #os.system('RUSTFLAGS=-Awarnings cargo build -q --package wasm --bin dce --release')
    
    paths = extract_wasm_and_test_paths() 
    for lib in paths:
        
        wasm_file = paths[lib]["wasm_path"]
        
        # Extract static information of the binary and update data.json
        os.system("python3 get-static-data.py --update-json {}".format(wasm_file)); print() 
        
        # Compute Reachablity sets for all the tools and update data.json
        os.system("python3 get-tools-data.py  --update-json {}".format(wasm_file)); print()

        # ./node_modules/blake3-wasm/dist/wasm/nodejs/blake3_js_bg.wasm
        print("Running tests for {}...\n".format(lib))
        for test in paths[lib]['test_paths']:
            
            print(lib+"/"+extract_name(test))

            # Uninstrument test 
                # rm -rf instrumented-index.js node_modules
                # npm i

            # Run test
                # node index.js  

            # Instrument test 
                # instrument-test.sh PATH-TO-WASM-FILE
                # test specific instrumentation -> add collect.js in the right place - use Babble?

            # Run instrumented tests
            run_instrumented_tests_command = "node {} --reachable-exports --callsite-sensitive-cg --lower-bound".format(test+"/instrumented-index.js")
            stdout = execute_command(run_instrumented_tests_command)
            results = [re.search("(\d+) ",x)[0].strip() for x in stdout[len(stdout)-6:] if x != '']
            print("{} exported functions are reachable.".format(results[0]))  
            print("{} callsites have been analyzed.".format(results[1]))          
            print("{} functions are the lower bound for the analysis.".format(results[2]))
            print()

        print()

    # Get reachability sets for each test and update data.json
    os.system("python3 get-dyn-data.py"); print()

    # Run analysis on data.json 
    os.system("python3 analysis.py"); print()

    # Latexify results into latex tables and figures. Report results on stdout.
    os.system("python3 latexify.py")

if __name__ == "__main__":
    main()