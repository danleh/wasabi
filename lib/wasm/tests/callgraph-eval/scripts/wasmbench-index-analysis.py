import os, sys
import json 
import re
from subprocess import PIPE, Popen
from collections import Counter

WASMBENCH_FILTERED_BINARIES = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/WasmBench/filtered-binaries-metadata/filtered"
WASMBENCH_NUM_BINARIES = 8461

WASMBENCH_INDEX = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/wasmbench_index_distribution.json"

def extract_wasm_paths(TEST_DIR): 
    paths = [] 
    for item1 in os.listdir(TEST_DIR):
        item1_path = os.path.join(TEST_DIR, item1)
        if os.path.splitext(item1_path)[1] == ".wasm": paths.append(item1_path)
    return paths

def execute_command(command, print_stdout=True):
    if print_stdout: p = Popen(command, shell=True, stderr=PIPE)
    else: p = Popen(command, shell=True, stdout=PIPE, stderr=PIPE)
    _, stderr = p.communicate()
    stderr = stderr.decode("utf-8").strip()
    if "warning" in stderr: stderr = ""
    if stderr != "": return (False, stderr)
    else: return (True, "")

def main():

    data = json.load(open(WASMBENCH_INDEX))
    index_expr_count = {}
    total_idx = 0
    for binary in data:
        for idx in data[binary]["index_distribution"]:
            count = data[binary]["index_distribution"][idx]['count']
            total_idx += count
            if idx not in index_expr_count.keys(): index_expr_count[idx] = 0
            index_expr_count[idx] += count
    
    buckets = {
        "affected by memory": {
            "atleast one load in idx": 0, 
            "address is constant": 0, 
            "address from load": 0, 
            "address involves arithmetic": 0, 
            "address from call": 0, 
            "address from local": 0, 
            "address from param": 0
        }, 
        "local variable": {
            "local get/tee from local variable": 0,
        }, 
        "interprocedural": {
            "local get/tee from parameter": 0,
            "calls": 0, 
        }, 
        "constant": 0,  # any special constants? wasabi didn't report the specific constant :(
        "arithmetic": 0, 
        "misc": 0
    }
    constants_counter = Counter()

    arth_operations = ["add", "sub", "mul", "div", "and", "shr_u"]
    for index, count in index_expr_count.items(): 
        load_flag, local_flag, interprocedural_flag, const_flag, arth_flag = [0]*5 

        if "load" in index: 
            load_flag = True           
            buckets["affected by memory"]["atleast one load in idx"] += count
            addr_pattern = index[:18]
            
            if "i32.load" in index[:8]: 
                addr = index[8:]

                if "<local>" in addr: buckets["affected by memory"]["address from local"] += count
                if "<param>" in addr: buckets["affected by memory"]["address from param"] += count
                if "load"    in addr: buckets["affected by memory"]["address from load"] += count
                if "call"    in addr: buckets["affected by memory"]["address from call"] += count
                if "const" in addr[:10]: buckets["affected by memory"]["address is constant"] += count
            
            arth = [1 for x in arth_operations if x in addr_pattern]
            if len(arth)>0: buckets["affected by memory"]["address involves arithmetic"] += count

        if "<local>" in index: local_flag = True; buckets["local variable"]["local get/tee from local variable"] += count
  
        if "<param>" in index: interprocedural_flag = True; buckets["interprocedural"]["local get/tee from parameter"] += count    
        if "call" in index: interprocedural_flag = True; buckets["interprocedural"]["calls"] += count 

        if "const" in index[:9]: const_flag = True; buckets["constant"] += count

        arth = [x for x in arth_operations if x in index]
        if len(arth)>0: arth_flag = True; buckets["arithmetic"] += count

        if not load_flag and not local_flag and not interprocedural_flag and not const_flag and not arth_flag: 
            print(f"{index} -> {index_expr_count[index]}")
            buckets["misc"] += count
        
    print(f"{total_idx} total indirect calls")
    for b in buckets: 
        if type(buckets[b]) == int:
            print(f"{(buckets[b]/total_idx)*100:.0f}% {buckets[b]} {b}")
        else:
            for s in buckets[b]: 
                print(f"{(buckets[b][s]/total_idx)*100:.0f}% {buckets[b][s]} {s}")
      
if __name__ == "__main__":
    main()