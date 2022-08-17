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

def execute_command(command, print_stdout=True):
    if print_stdout: p = Popen(command, shell=True, stderr=PIPE)
    else: p = Popen(command, shell=True, stdout=PIPE, stderr=PIPE)
    stdout, stderr = p.communicate()
    if stderr.decode("utf-8").strip() != "": sys.exit(stderr.decode("utf-8").strip())
    if stdout == None: return stdout
    else: return stdout.decode("utf-8")

def help_message(): 
    print("Usage: run-eval.py [OPTIONS]")
    print("")
    print("This script does the following things:")
    print("  - Extracts static information for each wasm library.")
    print("  - Computes sets of reachable functions for each tool being evaluated, for each wasm library.")
    print("  - Reinstruments and computes sets of reachable functions for each test using Wasabi.")
    print("  - Runs analysis on all the extracted data.")
    print("  - Latexifies all the data and reports results on stdout.")
    print("")
    print("Options:")
    print("--fresh          Run a fresh evaluation from the start, re-evaluating static, tools and dynamic information.")
    print("--eval-static    Re-run only the static binary evaluation.")
    print("--eval-tools     Re-run only the tools evaluation.")
    print("--eval-dyn       Re-run only the dynamic evalution using Wasabi.")
    print("If you do not pass in any options, none of the evaluations are re-run. The results are recomputed for data in data.json.")

def main():

    args = sys.argv[1:]
    if args[0] == "-h" or args[0] == "--help": 
        help_message()
        sys.exit()
    
    # flag_eval[0] indicates static binary evaluation (ie, get-static-data.py)
    # flag_eval[1] indicates tools evaluation (ie, get-tools-data.py)
    # flag_eval[2] indicates dynamic evalution using Wasabi (ie, get-dyn-data.py)
    flag_eval = [0, 0, 0] 
    flag_fresh = False
    for arg in args:
        if arg == "--fresh": 
            flag_fresh = True
            flag_eval = [1, 1, 1]
        elif arg == "--eval-static": flag_eval[0] = 1
        elif arg == "--eval-tools" : flag_eval[1] = 1
        elif arg == "--eval-dyn"   : flag_eval[2] = 1
        else: 
            help_message()
            sys.exit()

    
    # Start "fresh" by removing the old results and copying test-suite.json to data.json.
    if flag_fresh:
        execute_command("rm {}".format(DATA_JSON_PATH))
        execute_command("cp {} {}".format(TEST_SUITE_JSON_PATH, DATA_JSON_PATH))

    # Build ourtool 
    if flag_fresh:
        execute_command('RUSTFLAGS=-Awarnings cargo build -q --package wasm --bin dce --release')
    
    paths = extract_wasm_and_test_paths() 
    for lib in paths:
        
        wasm_file = paths[lib]["wasm_path"]
        
        # Extract static information of the binary and update data.json
        if flag_eval[0]: execute_command("python3 get-static-data.py --update-json {}".format(wasm_file)); print() 
        
        # Compute Reachablity sets for all the tools and update data.json
        if flag_eval[1]: execute_command("python3 get-tools-data.py  --update-json {}".format(wasm_file)); print()

        if flag_eval[2]: 
            
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
                stdout = execute_command(run_instrumented_tests_command, print_stdout=False)
                stdout = stdout.split("\n")
                results = [re.search("(\d+) ",x)[0].strip() for x in stdout[len(stdout)-6:] if x != '']
                print("{} exported functions are reachable.".format(results[0]))  
                print("{} callsites have been analyzed.".format(results[1]))          
                print("{} functions are the lower bound for the analysis.".format(results[2]))
                print()

        print()

    # Get reachability sets for each test and update data.json
    execute_command("python3 get-dyn-data.py"); print()

    # Run analysis on data.json 
    execute_command("python3 analysis.py"); print()

    # Latexify results into latex tables and figures. Report results on stdout.
    execute_command("python3 latexify.py")

if __name__ == "__main__":
    main()