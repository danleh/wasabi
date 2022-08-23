import os, sys
from subprocess import PIPE, Popen
import re

REAL_TEST_SUITE_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/test-suite" 
MICRO_TEST_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/micro-benchmarks"

SCRIPTS_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/scripts/"

TEST_SUITE_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/test-suite.json"
TEST_SUITE_DATA_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/test-suite-data.json"

MICROBENCH_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/micro-benchmarks.json"
MICROBENCH_DATA_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/microbench-data.json"


def extract_name(path): 
    return path.split("/")[len(path.split("/"))-1]

def extract_wasm_and_test_paths(TEST_DIR): 
    paths = {} # {lib -> {testpaths -> [], wasmpath -> ""}}
    for item1 in os.listdir(TEST_DIR):
        item1_path = os.path.join(TEST_DIR, item1)
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
    print("--all-fresh      Run a fresh evaluation for realworld and microbenchmarks.")
    print("--real           Run evaluation on real world binaries dataset")
    print("  --fresh        Run a fresh evaluation from the start, re-evaluating static, tools and dynamic information.")
    print("  --eval-static  Re-run only the static binary evaluation.")
    print("  --eval-tools   Re-run only the tools evaluation.")
    print("  --eval-dyn     Re-run only the dynamic evalution using Wasabi.")
    print("--micro          Run evaulation on microbenchmarks")
    print("  --fresh        Run a fresh evaluation from the start, re-evaluating static and tools information.")
    print("  --eval-static  Re-run only the static binary evaluation")
    print("  --eval-tools   Re-run only the tools evaluation")
    print("If you do not pass in any options, none of the evaluations are re-run. The results are recomputed for data in data.json.")

def main():

    args = sys.argv[1:]
    if len(args)==0 or args[0] == "-h" or args[0] == "--help": 
        help_message()
        sys.exit()
    
    # flag_eval[0] indicates static binary evaluation (ie, get-static-data.py)
    # flag_eval[1] indicates tools evaluation (ie, get-tools-data.py)
    # flag_eval[2] indicates dynamic evalution using Wasabi (ie, get-dyn-data.py)
    flag_real, flag_micro = False, False
    flag_real_fresh, flag_real_eval = False, [0, 0, 0] 
    flag_micro_fresh, flag_micro_eval = False, [0, 0]

    if args[0] == "--all-fresh":
        flag_real, flag_micro = True, True
        flag_real_fresh, flag_real_eval = True, [1, 1, 1]
        flag_micro_fresh, flag_micro_eval = True, [1, 1, 1] 
     
    elif args[0] == "--real":
        flag_real = True
        for arg in args[1:]:
            if arg == "--fresh": 
                flag_real_fresh = True
                flag_real_eval = [1, 1, 1]
            elif arg == "--eval-static": flag_real_eval[0] = 1
            elif arg == "--eval-tools" : flag_real_eval[1] = 1
            elif arg == "--eval-dyn"   : flag_real_eval[2] = 1
            else: 
                help_message()
                sys.exit()

    elif args[0] == "--micro":
        flag_micro = True
        for arg in args[1:]:
            if arg == "--fresh": 
                flag_micro_fresh = True
                flag_micro_eval = [1, 1, 1]
            elif arg == "--eval-static": flag_micro_eval[0] = 1
            elif arg == "--eval-tools" : flag_micro_eval[1] = 1
            else: 
                help_message()
                sys.exit()

    # Start "fresh" by removing the old results and copying test-suite.json to data.json.
    if flag_real_fresh:
        execute_command("rm {}".format(TEST_SUITE_DATA_JSON_PATH))
        execute_command("cp {} {}".format(TEST_SUITE_JSON_PATH, TEST_SUITE_DATA_JSON_PATH))

    if flag_micro_fresh:
        execute_command("rm {}".format(MICROBENCH_DATA_JSON_PATH))
        execute_command("cp {} {}".format(MICROBENCH_JSON_PATH, MICROBENCH_DATA_JSON_PATH))

    # Build ourtool 
    if flag_real_fresh:
        execute_command('RUSTFLAGS=-Awarnings cargo build -q --package wasm --bin dce --release')
    
    if flag_micro: 
        paths = extract_wasm_and_test_paths(MICRO_TEST_PATH)
        for lib in paths:
            wasm_file = paths[lib]["wasm_path"]
            if flag_micro_eval[0]: execute_command("python3 get-static-data.py --micro-update-json {}".format(wasm_file)); print("\n") 
            if flag_micro_eval[0]: execute_command("python3 get-tools-data.py --micro-update-json {}".format(wasm_file)); print("\n")
        execute_command("python3 analysis.py --micro"); print("\n")

    if flag_real: 
        paths = extract_wasm_and_test_paths(REAL_TEST_SUITE_PATH)    
        for lib in paths:
            wasm_file = paths[lib]["wasm_path"]
            if flag_real_eval[0]: execute_command("python3 get-static-data.py --real-update-json {}".format(wasm_file)); print("\n") 
            if flag_real_eval[1]: execute_command("python3 get-tools-data.py  --real-update-json {}".format(wasm_file)); print("\n")

            if flag_real_eval[2]: 
                
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
                    results = [re.search("(\d+) ",x).groups()[0].strip() for x in stdout[len(stdout)-6:] if x != '']
                    print("{} exported functions are reachable.".format(results[0]))  
                    print("{} callsites have been analyzed.".format(results[1]))          
                    print("{} functions are the lower bound for the analysis.".format(results[2]))
                    print("\n")

            print("\n")

            # Get reachability sets for each test and update data.json
            if flag_real_eval[2]:
                execute_command("python3 get-dyn-data.py"); print("\n")

        execute_command("python3 analysis.py --real"); print("\n")

    # Latexify results into latex tables and figures. Report results on stdout.
    execute_command("python3 latexify.py")

if __name__ == "__main__":
    main()