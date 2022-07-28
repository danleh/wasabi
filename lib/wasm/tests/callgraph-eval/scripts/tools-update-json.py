import os, sys
from subprocess import PIPE, Popen
import json, re 
import time
from itertools import count, groupby

# Note: If you aren't Michelle, change variable 'wassail_path' in line {TODO} to your wassail binary path 
# or see if the wassail command runs on the shell without any path being specified.  

# TODO: tests :) 

TEST_SUITE_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/test-suite" 

DATA_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/library_data"

JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/data.json"

OURTOOL_DIR = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm"
OURTOOL_BINARY_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/target/release/dce"
    
DOT_FILE_EDGE_RE = "^(?:\"f(\d+)\"|\"(.+)\"|node(\d+)) ?-> ?(?:\"f(\d+)\"|\"(.+)\"|node(\d+));$"

METADCE_ROOT_RE = "root: (?:func\$(\d+)\$\d+|(.+))$"
METADCE_NODE_RE = "node: (?:func\$(\d+)\$\d+|(.+))$"
METADCE_REACHES_RE = "reaches: (?:importId\$fimport\$(\d+)\$\d+|func\$(\d+)\$\d+)$"
METADCE_UNUSED_RE = "unused: (.+)$"

TWIGGY_IR_LINE_RE = "^Id\((\d+), (\d+)\) \| (.+) \| (.+)$"
TWIGGY_IR_REACHES_RE = "Id\((\d+), (\d+)\)"
TWIGGY_META_ROOT_ID = 4294967295
TWIGGY_FUNC_RE = "code\[(\d+)\]"
    
def extract_lib(path): 
	path = path[:path.rfind("/")]
	return path[path.rfind("/")+1:]

def extract_wasm_paths(): 
    wasm_file_paths = []
    # Extract paths of the wasm files to be tested
    for item1 in os.listdir(TEST_SUITE_PATH):
        item1_path = os.path.join(TEST_SUITE_PATH, item1)
        if os.path.isdir(item1_path): 
            for item2 in os.listdir(item1_path):
                item2_path = os.path.join(item1_path, item2)
                if os.path.splitext(item2_path)[1] == ".wasm":
                    wasm_file_paths.append(item2_path)
    return wasm_file_paths 

def execute_command(command, program, output_file, write_stdout=True): 
    MAX_PROGRAM_LEN = 24
    output_file_pretty = output_file.split("/")
    output_file_pretty = "/".join(output_file_pretty[len(output_file_pretty)-4:])
    
    time_started = time.time()
    p = Popen(command, shell=True, stdout=PIPE, stderr=PIPE)
    exec_time = (time.time() - time_started)*1000 #time in milliseconds
    
    stdout, stderr = p.communicate()
    stdout, stderr = stdout.decode("utf-8"), stderr.decode("utf-8") 
    
    # The aWsm compiler produces stderr even when the compliation is successful.
    # So, only report error if main thread panics  
    if program == 'aWsm': flag_stderr = 'thread \'main\' panicked' in stderr
    else: 
        if 'warning: ' in stderr : stderr = ''
        flag_stderr = not not stderr

    if (not not stdout and write_stdout) or flag_stderr: 
        if os.path.exists(output_file): os.system('rm {}'.format(output_file))
        with open(output_file, "a") as f:
            if not not stdout and write_stdout: f.write("{} output: {}\n".format(program, stdout))
            if flag_stderr: f.write("{} error: {}\n".format(program, stderr))
        
    program = "Executing {}".format(program)
    program = program + " "*(MAX_PROGRAM_LEN-len(program))
    if flag_stderr: print("{} .....ERROR Error recorded in {}.".format(program, output_file_pretty))
    else: print("{} ...SUCCESS. {:.2f}ms".format(program, exec_time))
        
    return (not flag_stderr, stdout, exec_time)    

def run_ourtool(wasm_file, lib, lib_obj): 
    # Generate 'exported_funcs.csv file for ourtool
    # Only functions are made reachable 
    # Remove duplicated export names because our tool complains 
    # FIXME: fix bug in our tool: constraint not generated for export with two names, for the second name   
    exports_path = '{}/{}/exported_funcs.csv'.format(TEST_SUITE_PATH, lib) 
    with open(exports_path, 'w') as f_exports: 
        exp_str = ''
        for exp in lib_obj['static_info']['exports']['names']: 
            #if exp['type'] == 'function' and exp['internal_id'] not in exp_internal_ids:
            exp_str += '{},{},{}\n'.format(exp['name'], exp['type'], exp['internal_id'])
        f_exports.write(exp_str) 

    # Our Tool  
          
    dce_path = "{}/{}/CG_tools_data/our_tool/dce.wasm".format(DATA_PATH, lib)
    cg_path = "{}/{}/CG_tools_data/our_tool/callgraph.dot".format(DATA_PATH, lib)
    tool_shell_output = "{}/{}/CG_tools_data/our_tool/output.txt".format(DATA_PATH, lib)
    ourtool_command = "{} {} {} {} -cg {}".format(OURTOOL_BINARY_PATH, wasm_file, exports_path, dce_path, cg_path)
    status, _, exec_time = execute_command(ourtool_command, "ourtool", tool_shell_output)
    return (status, exec_time)
    
def run_wassail(wasm_file, lib):
    # Wassail 
    wassail_path = "/home/michelle/Documents/sa-for-wasm/wassail/_build/default/main.exe"
    cg_path = "{}/{}/CG_tools_data/wassail/callgraph.dot".format(DATA_PATH, lib)
    tool_shell_output = "{}/{}/CG_tools_data/wassail/output.txt".format(DATA_PATH, lib)
    wassail_command = "{} callgraph {} {}".format(wassail_path, wasm_file, cg_path)
    status, _, exec_time = execute_command(wassail_command, "wassail", tool_shell_output)
    return (status, exec_time) 

def run_metadce(wasm_file, lib, lib_obj): 
    # Generate reachablity graph.
    # All exports (memory, variables, functions, tables) are made reachable. 
    reachability_graph_path = '{}/{}/reachability-graph.json'.format(TEST_SUITE_PATH, lib)
    reachability_graph = []
    for export in lib_obj['static_info']['exports']['names']: 
        reachability_graph.append({
            'name': export['name'],
            'root': True,
            'export': export['name']  
        })
    json.dump(reachability_graph, open(reachability_graph_path, 'w'), indent=2)
    
    # Metadce
    dce_path = '{}/{}/CG_tools_data/metadce/dce.wasm'.format(DATA_PATH, lib)
    new_graph_path = '{}/{}/CG_tools_data/metadce/new-graph.txt'.format(DATA_PATH, lib)
    tool_shell_output = '{}/{}/CG_tools_data/metadce/output.txt'.format(DATA_PATH, lib)
    metadce_command = 'wasm-metadce --dump -f {} {} -o {}'.format(reachability_graph_path, wasm_file, dce_path)
    #print(metadce_command)
    status, output, exec_time = execute_command(metadce_command, "wasm-metadce", tool_shell_output, False)
    with open(new_graph_path, 'w') as f: 
        f.write(output)
    return (status, exec_time)

def run_twiggy(wasm_file, lib): 
    # Twiggy 
    internal_ir_path = '{}/{}/CG_tools_data/twiggy/internal_ir.txt'.format(DATA_PATH, lib)
    garbage_path = '{}/{}/CG_tools_data/twiggy/garbage.txt'.format(DATA_PATH, lib)
    tool_shell_output = '{}/{}/CG_tools_data/twiggy/output.txt'.format(DATA_PATH, lib)
    # dump internal IR 
    status_ir, dominators_output, exec_time = execute_command('twiggy dominators {}'.format(wasm_file), "twiggy IR", tool_shell_output, False)
    internal_ir = dominators_output[:dominators_output.index(" Retained Bytes")]
    with open(internal_ir_path, "w") as ir_f: ir_f.write(internal_ir)
    # dump garbage.csv 
    status_garbage, garbage_output, _ = execute_command('twiggy garbage {}'.format(wasm_file), "twiggy garbage", tool_shell_output, False)
    with open(garbage_path, "w") as garbage_f: garbage_f.write(garbage_output) 
    return (status_ir and status_garbage, exec_time)

def run_awsm(wasm_file, lib):    
    # LLVM - aWsm: save .ll file and .bc file 
    awsm_bc_path = '{}/{}/CG_tools_data/aWsm/awsm.bc'.format(DATA_PATH, lib)
    awsm_ll_path = '{}/{}/CG_tools_data/aWsm/awsm.ll'.format(DATA_PATH, lib)
    tool_shell_output = '{}/{}/CG_tools_data/aWsm/output.txt'.format(DATA_PATH, lib)
    status, _, exec_time = execute_command('awsm {} -o {}'.format(wasm_file, awsm_bc_path), "aWsm", tool_shell_output)
    if status: execute_command('llvm-dis-12 {} -o {}'.format(awsm_bc_path, awsm_ll_path), "llvm-dis", tool_shell_output)
    return (status, exec_time ) 

def run_wavm(wasm_file, lib): 
    # LLVM - WAVM: save .ll file and .bc file 
    wavm_bc_path = '{}/{}/CG_tools_data/WAVM/wavm.bc'.format(DATA_PATH, lib)
    wavm_ll_path = '{}/{}/CG_tools_data/WAVM/wavm.ll'.format(DATA_PATH, lib)
    tool_shell_output = '{}/{}/CG_tools_data/WAVM/output.txt'.format(DATA_PATH, lib)
    status, _, exec_time = execute_command('wavm compile --format=unoptimized-llvmir {} {}'.format(wasm_file, wavm_ll_path), "wavm", tool_shell_output)
    if status: execute_command('llvm-as-12 {} -o {}'.format(wavm_ll_path, wavm_bc_path), "llvm-as", tool_shell_output)
    return (status, exec_time ) 

def get_reachable_funcs_from_dot(cg_path, lib):
    reachable_funcs = set()
    with open(cg_path) as dot_file: 
        for line in dot_file:
            line = line.strip()
            line_search = re.search(DOT_FILE_EDGE_RE, line)
            if line_search: 
                matches = [match for match in line_search.groups() if match != None]
                for ind in range(0, len(matches)): 
                    if matches[ind].isdigit(): matches[ind] = int(matches[ind])
                reachable_funcs.update(matches)
    return reachable_funcs

def process_metadce(new_graph_path):
    graph_lines, graph_lines_ind = [], -1
    with open(new_graph_path) as graph:
        for line in graph: 
            if '=' in line: continue
            indentation_count = line.rstrip().split(' ').count('')
            line = line.strip()
            if indentation_count == 0: 
                graph_lines_ind += 1
                graph_lines.append([line])                        
            elif indentation_count == 2: graph_lines[graph_lines_ind].append(line)
            else: sys.exit("Indentation level that's not 0 or 2 not expected.")
    
    reachable_funcs = set()
    garbage_funcs = set()
    for lines in graph_lines: 
        if len(lines) == 1: 
            root = re.search(METADCE_ROOT_RE, lines[0])
            if root:
                root = [node for node in root.groups() if node != None][0]
                reachable_funcs.update(root)
            else: 
                unused = re.search(METADCE_UNUSED_RE, lines[0])                
                # TODO: don't report everything to be garbage. Make sure it's a function
                if unused: garbage_funcs.update(unused.groups()[0])
                else: sys.exit("unknown line(s) in metadce new graph: {}".format(lines))
        else:   
            node = [node for node in re.search(METADCE_NODE_RE, lines[0]).groups() if node != None][0]
            if node in reachable_funcs:
            # They record if the target is a imported function or an internal function 
            # I am not differenciating between those two 
                for targets in lines[2:]:
                    target = re.search(METADCE_REACHES_RE, targets)
                    if target:
                        target = [node for node in target.groups() if node != None][0]                        
                        reachable_funcs.update(target)
    
    return (list(reachable_funcs), list(garbage_funcs))
         
def process_twiggy(internal_ir_path, garbage_path):
    IR = {}
    with open(internal_ir_path) as ir_f: 
        for line in ir_f: 
            id_num1, id_num2, name, reaches = re.search(TWIGGY_IR_LINE_RE, line).groups()
            targets = []
            if reaches != "None": 
                # When you want matches of a pattern that is recursive (uses *, +), 
                # don't use re.search because it won't return all matches in * or +. 
                # Instead, match on the individual recursive component 
                # and split on it to get all elements.  
                targets_ = re.split(TWIGGY_IR_REACHES_RE,reaches[6:-2])
                targets_ = [x for x in targets_ if len(x) != 0 and x != ", "]
                if len(targets_)%2 != 0: sys.exit("ID should have two numbers associated with it: {}".format(reaches))
                for ind in range(0, len(targets_), 2): 
                    targets.append((int(targets_[ind]), int(targets_[ind+1])))
            IR[(int(id_num1), int(id_num2))] = {
                'name': name, 
                'targets': targets
            }
    
    # Not set because you get an 'int' object is not iterable error when you try to update it with func:int  
    # TODO: make set and figure out why the error shows up?
    reachable_funcs = [] 
    worklist = [(TWIGGY_META_ROOT_ID, TWIGGY_META_ROOT_ID)]
    processed_nodes = [] # we don't want any cycles, or repeated computations  
    while len(worklist) != 0: 
        node = worklist.pop()
        name, targets = IR[node].values()
        processed_nodes.append(node)
        for target in targets: 
            if target not in processed_nodes: worklist.append(target)
            func = re.search(TWIGGY_FUNC_RE, name)
            if func: 
                func = int(func.groups()[0])
                reachable_funcs.append(func)            
    reachable_funcs = set(reachable_funcs)            

    #TODO remove garbage funcs from reachable_funcs
    garbage_funcs = []
    with open(garbage_path) as garbage_f: 
        garbage_lines = garbage_f.readlines()[2:]
        for line in garbage_lines: 
            _byte, _size, item = line.strip().split(" ┊ ")
            func = re.search(TWIGGY_FUNC_RE, item)
            if func:
                func = int(func.groups()[0])
                garbage_funcs.append(func)
    garbage_funcs = set(garbage_funcs)

    return (list(reachable_funcs), list(garbage_funcs))

def pretty_print_reachable_funcs(counts):
    tools = ["ourtool", "wassail", "metadce", "twiggy"]
    MAX_COUNT_LEN = len(str(max(counts)))
    for tool, count in zip(tools, counts):
        count = str(count)
        count_pretty = ' '*(MAX_COUNT_LEN-len(count)) + count
        print("{} reachable functions computed for {}.".format(count_pretty, tool))

def main():

    args = sys.argv[1:]
    if len(args)>0: # This script should not be passed any arguments. Also incase user sends --help or -h
        print("Usage: tools-update-json.py")
        print("This script runs all the tools that are being evaluated.")
        print("Each tools reachability graph as well as stdout and stderr are located in data/library_data/lib/CG_tools_data/tool/.")
        print("The set of reachable functions is extracted from the reachability graph for each tool")
        print("and data.json is updated to include this set for each tool.")
        sys.exit()

    # build ourtool 
    os.system('RUSTFLAGS=-Awarnings cargo build -q --package wasm --bin dce --release')

    data = json.load(open(JSON_PATH))
    wasm_file_paths = extract_wasm_paths() 
    for wasm_file in wasm_file_paths:
        
        lib = extract_lib(wasm_file)        
        print(lib)

        lib_obj = [l for l in data['library_data'] if l['library_name'] == lib][0]
        lib_obj["tools"] = []

        ourtool_status, ourtool_time   = run_ourtool(wasm_file, lib, lib_obj)
        wassail_status, wassail_time   = run_wassail(wasm_file, lib)
        metadce_status, metadce_time   = run_metadce(wasm_file, lib, lib_obj)
        twiggy_status,  twiggy_time    = run_twiggy (wasm_file, lib)
        awsm_status,    awsm_time      = run_awsm   (wasm_file, lib)
        wavm_status,    wavm_time      = run_wavm   (wasm_file, lib) 
        
        ourtool_status = False
        wassail_status = False
        metadce_status = False
        twiggy_status = False
        awsm_status = False
        wavm_status = False

        reachable_funcs_count = [0]*4

        if ourtool_status: 
            cg_path = "{}/{}/CG_tools_data/our_tool/callgraph.dot".format(DATA_PATH, lib)
            reachable_funcs = get_reachable_funcs_from_dot(cg_path, lib)
            lib_obj["tools"].append({
                "name": "ourtool",
                "callgraph_construction": True, 
                "dce" : False,
                "execution_time": ourtool_time, 
                "reachable_functions": {
                    "names": list(reachable_funcs), 
                    "count": len(reachable_funcs)
                }
            })
            reachable_funcs_count.append(len(reachable_funcs))
                            
        if wassail_status: 
            cg_path = "{}/{}/CG_tools_data/wassail/callgraph.dot".format(DATA_PATH, lib)
            reachable_funcs = get_reachable_funcs_from_dot(cg_path, lib)
            lib_obj["tools"].append({
                "name": "wassail",
                "callgraph_construction": True, 
                "dce" : False,
                "execution_time": wassail_time, 
                "reachable_functions": {
                    "names": list(reachable_funcs), 
                    "count": len(reachable_funcs)
                }
            })
            reachable_funcs_count.append(len(reachable_funcs))
        
        # FIXME: opencv execution gives different ouputs for each execution!?
        # - dump input to metadce from binaryen? and figure out the right way to make a reachability graph 
        # - commit to metadce tool help to have better documententation 
        # FIXME: metadce obviously not giving us the right reachability graph. the input graph is probably wrong. 
        if metadce_status and 'opencv' not in lib:
            new_graph_path = '{}/{}/CG_tools_data/metadce/new-graph.txt'.format(DATA_PATH, lib)
            reachable_funcs, garbage_funcs = process_metadce(new_graph_path)
            lib_obj["tools"].append({
                "name": "metadce",
                "callgraph_construction": False,
                "dce" : True,
                "execution_time": metadce_time, # TODO 
                "reachable_functions": {
                    "names": list(reachable_funcs), 
                    "count": len(reachable_funcs)
                },
                "garbage_functions": {
                    "names": list(garbage_funcs), 
                    "count": len(garbage_funcs)
                }
            })
            reachable_funcs_count.append(len(reachable_funcs))
        
        if twiggy_status: 
            internal_ir_path = '{}/{}/CG_tools_data/twiggy/internal_ir.txt'.format(DATA_PATH, lib)
            garbage_path = '{}/{}/CG_tools_data/twiggy/garbage.txt'.format(DATA_PATH, lib)
            reachable_funcs, garbage_funcs = process_twiggy(internal_ir_path, garbage_path)
            lib_obj["tools"].append({
                "name": "twiggy",
                "callgraph_construction": False,
                "dce" : True,
                "execution_time": twiggy_time, # TODO 
                "reachable_functions": {
                    "names": list(reachable_funcs), 
                    "count": len(reachable_funcs)
                },
                "garbage_functions": {
                    "names": list(garbage_funcs), 
                    "count": len(garbage_funcs)
                }
            })
            reachable_funcs_count.append(len(reachable_funcs))

        #pretty_print_reachable_funcs(reachable_funcs_count)
        print()
    
    json.dump(data, open(JSON_PATH, 'w'), indent=2)
 
if __name__ == "__main__":
    main()