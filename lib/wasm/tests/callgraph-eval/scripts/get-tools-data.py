import os, sys
from subprocess import PIPE, Popen
import json, re 
import time
from itertools import count, groupby

# Note: If you aren't Michelle, change variable 'wassail_path' in function run_wassail() to your wassail binary path 
# or see if the wassail command runs on the shell without any path being specified.  

# TODO: tests :) 

TEST_SUITE_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/test-suite" 
MICROBENCH_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/micro-benchmarks"
DATA_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/library_data"

TEST_SUITE_DATA_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/test-suite-data.json"
MICROBENCH_DATA_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/microbench-data.json"

OURTOOL_DIR = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm"
OURTOOL_BINARY_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/target/release/dce"
    
DOT_FILE_EDGE_RE = "^(?:\"f(\d+)\"|\"(.+)\"|node(\d+)) ?-> ?(?:\"f(\d+)\"|\"(.+)\"|node(\d+));$"

METADCE_ROOT_FUNC_RE = "root: func\$(\d+)\$\d+$"
METADCE_ROOT_EXPORT_RE = "root: (.+)$"
METADCE_NODE_FUNC_RE = "node: func\$(\d+)\$\d+$"
METADCE_NODE_EXPORT_RE = "node: (.+)$"
METADCE_REACHES_IMPORT_RE = "reaches: importId\$fimport\$(\d+)\$\d+$"
METADCE_REACHES_FUNC_RE = "reaches: func\$(\d+)\$\d+$"
METADCE_UNUSED_RE = "unused: (.+)$"

TWIGGY_IR_LINE_RE = "^Id\((\d+), (\d+)\) \| (.+) \| (.+)$"
TWIGGY_IR_REACHES_RE = "Id\((\d+), (\d+)\)"
TWIGGY_META_ROOT_ID = 4294967295
TWIGGY_FUNC_RE = "code\[(\d+)\]"
    
WAVM_DOT_NODE_IMPORT_RE = "Node(0x[\w]+) \[.*label=\"{functionImport(\d+)}\"\];"
WAVM_DOT_NODE_FUNC_RE = "Node(0x[\w]+) \[.*label=\"{functionDef(\d+)}\"\];"
WAVM_DOT_EDGE_RE = "Node(0x[\w]+) -> Node(0x[\w]+);"

REAL_BIN, MICRO_BIN = False, False

def extract_lib(path): 
	path = path[:path.rfind("/")]
	return path[path.rfind("/")+1:]

def execute_command(command, program, output_file, write_stdout=True): 
    MAX_PROGRAM_LEN = 24
    output_file_pretty = output_file.split("/")
    output_file_pretty = "/".join(output_file_pretty[len(output_file_pretty)-4:])
    
    time_started = time.time()
    p = Popen(command, shell=True, stdout=PIPE, stderr=PIPE)
    exec_time = (time.time() - time_started)*1000 #time in milliseconds
    
    stdout, stderr = p.communicate()
    if "opt" != program:
        stdout, stderr = stdout.decode("utf-8"), stderr.decode("utf-8") 
    else:
        stdout, stderr = "", "" #TODO: fix decoding error for opt 
    
    # The aWsm compiler produces stderr even when the compliation is successful.
    # So, only report error if main thread panics  
    if 'aWsm' in program: flag_stderr = 'thread \'main\' panicked' in stderr
    elif "metadce" in program: flag_stderr = "=============" not in stdout
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
    if flag_stderr: print("{} .....ERROR. Error recorded in {}.".format(program, output_file_pretty))
    else: print("{} ...SUCCESS. {:.2f}ms".format(program, exec_time))
        
    return (not flag_stderr, stdout, exec_time)    

def run_ourtool(bin_type, wasm_file, lib, lib_obj): 
    # Generate 'exported_funcs.csv file for ourtool
    # Only functions are made reachable 
    # Remove duplicated export names because our tool complains 
    # FIXME: fix bug in our tool: constraint not generated for export with two names, for the second name   
    exports_path = ""
    if bin_type == "real": exports_path = '{}/{}/exported_funcs.csv'.format(TEST_SUITE_PATH, lib) 
    if bin_type == "micro": exports_path = '{}/{}/exported_funcs.csv'.format(MICROBENCH_PATH, lib)
    with open(exports_path, 'w') as f_exports: 
        exp_str = ''
        for exp in lib_obj['static_info']['exports']['names']: 
            #if exp['type'] == 'function' and exp['internal_id'] not in exp_internal_ids:
            exp_str += '{},{},{}\n'.format(exp['name'], exp['type'], exp['internal_id'])
        f_exports.write(exp_str) 

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

def run_metadce(bin_type, wasm_file, lib_name, lib_obj): 
    # Generate reachablity graph.
    # All exports (memory, variables, functions, tables) are made reachable. 
    reachability_graph_path = ""
    if bin_type == "real": 
        reachability_graph_path = '{}/{}/reachability-graph.json'.format(TEST_SUITE_PATH, lib_name)
        reachability_graph = []
        for export in lib_obj['static_info']['exports']['names']: 
            reachability_graph.append({
                'name': export['name'],
                'root': True,
                'export': export['name']  
            })
        json.dump(reachability_graph, open(reachability_graph_path, 'w'), indent=2)
    if bin_type == "micro": 
        reachability_graph_path = '{}/{}/reachability-graph.json'.format(MICROBENCH_PATH, lib_name)
        export_names = {item['internal_id'] : item['name'] for item in lib_obj["static_info"]["exports"]["names"]}    
        reachability_graph = []
        print(export_names)
        for export in json.load(open(MICROBENCH_DATA_JSON_PATH))[lib_name]['ground_truth']['entry_points']:
            reachability_graph.append({
                'name': export_names[int(export)],
                'root': True,
                'export': export_names[int(export)]  
            })
        json.dump(reachability_graph, open(reachability_graph_path, 'w'), indent=2)        
    
    # Metadce
    dce_path = '{}/{}/CG_tools_data/metadce/dce.wasm'.format(DATA_PATH, lib_name)
    new_graph_path = '{}/{}/CG_tools_data/metadce/new-graph.txt'.format(DATA_PATH, lib_name)
    tool_shell_output = '{}/{}/CG_tools_data/metadce/output.txt'.format(DATA_PATH, lib_name)
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
    if status_ir:
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
    if status: 
        execute_command('llvm-as-12 {} -o {}'.format(wavm_ll_path, wavm_bc_path), "llvm-as", tool_shell_output)
        status, _, _ = execute_command('opt-12 {} --dot-callgraph'.format(wavm_bc_path), "opt", tool_shell_output)
    return (status, exec_time) 

# replace names in a list
def replace_names_with_internal_ids(funcs, export_names, import_names, debug_names):
    funcs_replaced  = set()
    export_names = {item['name']: {"type":item["type"], "internal_id": item['internal_id']} for item in export_names}
    import_names = {item["module_name"]+"."+item["export_name_within_module"]: {"type":item["type"], "internal_id": item['internal_id']} for item in import_names}
    for f in funcs:
        if not str(f).isdigit(): #it is an exported or imported function index and has to be replaced with its internal id 
            if f in export_names.keys(): 
                if export_names[f]["type"] == "function": funcs_replaced.add(int(export_names[f]["internal_id"]))
            elif f in import_names.keys(): 
                if import_names[f]["type"] == "function": funcs_replaced.add(int(import_names[f]["internal_id"]))
            elif f in debug_names.keys():
                return funcs_replaced.add(int(debug_names[f]))
            else: sys.exit("Unknown identifier for function found {}".format(f))
        elif str(f).isdigit(): funcs_replaced.add(int(f))
    return list(funcs_replaced) 

# replace names through a graph 
def replace_graph_nodes_with_id(old_graph, lib_obj):
    
    export_names = {item['name']: {"type":item["type"], "internal_id": item['internal_id']} for item in lib_obj["static_info"]["exports"]["names"]}
    import_names = {item["module_name"]+"."+item["export_name_within_module"]: {"type":item["type"], "internal_id": item['internal_id']} for item in lib_obj["static_info"]["imports"]["names"]}
    debug_names = lib_obj["static_info"]["debug_names"]

    def replace_name_with_id(name):
        if name in export_names.keys(): 
            if export_names[name]["type"] == "function": 
                return export_names[name]["internal_id"]
        elif name in import_names.keys(): 
            if import_names[name]["type"] == "function": return import_names[name]["internal_id"]
        elif name in debug_names.keys():
            return debug_names[name]
        else: sys.exit("Unknown identifier for function found {}".format(name))
    
    new_graph = {}
    for from_node in old_graph:
        from_node_id = -1
        if not str(from_node).isdigit(): 
            #it is an exported or imported function index and has to be replaced with its internal id 
            from_node_id = replace_name_with_id(from_node)
            if from_node_id != None: 
                from_node_id = int(from_node_id)
                new_graph[from_node_id] = []
        else: 
            from_node_id = int(from_node)
            new_graph[int(from_node)] = []

        for to_node in old_graph[from_node]:
            if from_node_id != None:
                if not str(to_node).isdigit(): 
                    to_node = replace_name_with_id(to_node)
                if to_node != None:
                    new_graph[from_node_id].append(int(to_node))
    return new_graph

def get_reachable_funcs_and_edges(graph, initially_reachable_funcs):
    reachable_funcs = initially_reachable_funcs 
    edges = set()
    worklist = set(reachable_funcs)
    processed_list = set()
    while len(worklist) != 0:
        node = worklist.pop()
        reachable_funcs.add(node)
        if node not in processed_list and node in graph.keys(): #it is not a leaf
            processed_list.add(node)
            for target in list(graph[node]):
                if target not in processed_list: 
                    edges.add((node, target))
                    worklist.add(target)
    return set(reachable_funcs), edges

def get_reachable_funcs_from_dot(cg_path, lib_obj):

    def get_graph(path):    
        graph = {}
        with open(path) as dot_file:    
            for line in dot_file:
                line = line.strip()
                line_search = re.search(DOT_FILE_EDGE_RE, line)
                if line_search: 
                    matches = [match for match in line_search.groups() if match != None]
                    for ind in range(0, len(matches)): 
                        if matches[ind].isdigit(): matches[ind] = int(matches[ind])
                    from_node, to_node = matches
                    if from_node in graph.keys(): graph[from_node].add(to_node)
                    else: 
                        graph[from_node] = set()
                        graph[from_node].add(to_node)
        return replace_graph_nodes_with_id(graph, lib_obj)

    graph = get_graph(cg_path)
    reachable_funcs = set([int(e["internal_id"]) for e in lib_obj["static_info"]["exports"]["names"] if e["type"] == "function"])
    
    reachable_funcs, reachable_edges = get_reachable_funcs_and_edges(graph, reachable_funcs)
    
    return (graph, reachable_funcs, reachable_edges)

def process_metadce(new_graph_path, lib):
    import_funcs_num = int(lib['static_info']['imports']['count_imported_funcs'])
    
    export_names = {item['name']: {"type":item["type"], "internal_id": item['internal_id']} for item in lib["static_info"]["exports"]["names"]}
    import_names = {item["module_name"]+"."+item["export_name_within_module"]: {"type":item["type"], "internal_id": item['internal_id']} for item in lib["static_info"]["imports"]["names"]}
    debug_names = lib["static_info"]["debug_names"]

    def replace_name_with_id(name):
        if name in export_names.keys(): 
            if export_names[name]["type"] == "function": 
                return export_names[name]["internal_id"]
        elif name in import_names.keys(): 
            if import_names[name]["type"] == "function": return import_names[name]["internal_id"]
        elif name in debug_names.keys():
            return int(debug_names[name])
        else: sys.exit("Unknown identifier for function found {}".format(name))

    def parse_graph_into_nodes(graph_path):
        graph_lines, graph_lines_ind = [], -1
        with open(graph_path) as graph:
            for line in graph: 
                if '=' in line: continue
                indentation_count = line.rstrip().split(' ').count('')
                line = line.strip()
                if indentation_count == 0: 
                    graph_lines_ind += 1
                    graph_lines.append([line])                        
                elif indentation_count == 2: graph_lines[graph_lines_ind].append(line)
                else: sys.exit("Indentation level that's not 0 or 2 not expected.")
        return graph_lines

    def get_graph(graph_path):
        graph_lines = parse_graph_into_nodes(graph_path)
        roots = set() 
        graph = {} 
        garbage = [] 
        for lines in graph_lines: 
            if len(lines) == 1: 
                func_root = re.search(METADCE_ROOT_FUNC_RE, lines[0])
                if func_root:
                    func_root = int([node for node in func_root.groups() if node != None][0]) + import_funcs_num
                    roots.add(func_root)
                else: 
                    export_root = re.search(METADCE_ROOT_EXPORT_RE, lines[0])
                    if export_root:
                        export_root = "".join([node for node in export_root.groups() if node != None][0])
                        roots.add(export_root)
                    else: 
                        unused = re.search(METADCE_UNUSED_RE, lines[0])                
                        # TODO: don't report everything to be garbage. Make sure it's a function
                        if unused: 
                            unused = unused.groups()[0]
                            garbage.append(unused)
                        else: sys.exit("unknown line(s) in metadce new graph: {}".format(lines))
            else:   
                node = re.search(METADCE_NODE_FUNC_RE, lines[0])
                if node: 
                    node = [n for n in node.groups() if n != None][0]
                    node = int(node) + import_funcs_num
                else: 
                    node_ = re.search(METADCE_NODE_EXPORT_RE, lines[0])
                    node = [node for node in node_.groups() if node != None][0]

                # add to map 
                targets = []
                for t in lines[2:]:
                    target = re.search(METADCE_REACHES_IMPORT_RE, t)
                    if target: 
                        target = int([node for node in target.groups() if node != None][0])
                        targets.append(target)
                    else: 
                        target = re.search(METADCE_REACHES_FUNC_RE, t)
                        if target: 
                            target = int([node for node in target.groups() if node != None][0]) + import_funcs_num
                            targets.append(target)        
                graph[node] = targets
        return (graph, roots, garbage)
        
    graph, reachable_funcs, garbage = get_graph(new_graph_path)
    reachable_funcs, reachable_edges = get_reachable_funcs_and_edges(graph, reachable_funcs)
    
    reachable_edges_normalized = set()
    for node, child in reachable_edges: 
        node_is_export = False
        if not str(node).isdigit(): 
            node_is_export = node in export_names
            node = replace_name_with_id(node)
        if not str(child).isdigit(): 
            child = replace_name_with_id(child)
        # node == child is when their export nodes point to their internal function id nodes 
        # What about recursive nodes! node_is_export makes sure that recursive edges are not removed
        if not node_is_export:
            reachable_edges_normalized.add((node, child))  
    
    graph = {from_node: graph[from_node] for from_node in graph if str(from_node).isdigit()}
    
    return (graph, 
            replace_names_with_internal_ids(
                reachable_funcs, 
                lib['static_info']['exports']['names'],
                lib['static_info']['imports']['names'], 
                lib['static_info']['debug_names']), 
            reachable_edges_normalized, 
            list(garbage))
    
def process_twiggy(internal_ir_path, garbage_path, lib):
    
    def get_IR_graph(path):
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
            return IR

    def get_graph(IR):
        import_funcs_num = int(lib['static_info']['imports']['count_imported_funcs'])

        graph = {}
        for node in IR:
            
            name, targets = IR[node].values()
            graph_from_node = False

            if "code[" in name: 
                graph_from_node = node[1] + import_funcs_num
                
            if graph_from_node != False and graph_from_node not in graph.keys(): 
                graph[graph_from_node] = []
        
            if graph_from_node:
                for target in targets:

                    target_name = IR[target]['name']
                    if "code[" in target_name: 
                        f_id = int(re.search(r'\d+', target_name).group()) + import_funcs_num
                        graph[graph_from_node].append(f_id)
                    
                    if "import" in target_name and "section" not in target_name: 
                        import_name = ".".join(target_name.split(" ")[1].split("::"))
                        import_id = [x['internal_id'] for x in lib['static_info']['imports']['names'] if x["module_name"]+"."+x["export_name_within_module"] == import_name][0]
                        graph[graph_from_node].append(int(import_id))

        return graph

    IR = get_IR_graph(internal_ir_path)
    import_funcs_num = int(lib['static_info']['imports']['count_imported_funcs'])
    reachable_funcs = set() 
    reachable_edges = set()
    worklist = [(TWIGGY_META_ROOT_ID, TWIGGY_META_ROOT_ID)]
    processed_nodes = set() # we don't want any cycles, or repeated computations  
    while len(worklist) != 0: 
        node = worklist.pop()
        name, targets = IR[node].values()
        
        graph_from_node = False

        if "code[" in name: 
            graph_from_node = node[1] + import_funcs_num
            reachable_funcs.add(graph_from_node)
        
        if "import" in name and "section" not in name:
            graph_from_node = ".".join(name.split(" ")[1].split("::"))
            reachable_funcs.add(graph_from_node)

        if node not in processed_nodes:
            processed_nodes.add(node)
            for target in targets: 
                
                if target not in processed_nodes: 
                    worklist.append(target)

                    if graph_from_node != False:

                        target_name = IR[target]['name']
                        if "code[" in target_name: 
                            f_id = int(re.search(r'\d+', target_name).group()) + import_funcs_num
                            reachable_edges.add((graph_from_node, f_id))
                        
                        if "import" in target_name and "section" not in target_name: 
                            import_name = ".".join(target_name.split(" ")[1].split("::"))
                            import_id = [x['internal_id'] for x in lib['static_info']['imports']['names'] if x["module_name"]+"."+x["export_name_within_module"] == import_name][0]
                            reachable_edges.add((graph_from_node, int(import_id)))

    
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

    graph = replace_graph_nodes_with_id(get_graph(IR), lib)
    #graph = replace_graph_nodes_with_id({f : graph[f] for f in graph.keys() if len(graph[f]) != 0}, lib)
    
    return (graph, 
            replace_names_with_internal_ids(
                reachable_funcs, 
                lib['static_info']['exports']['names'],
                lib['static_info']['imports']['names'], 
                lib['static_info']['debug_names']),
            reachable_edges,
            list(garbage_funcs))

def process_wavm_dot(dot_path, lib):
    import_funcs_num = int(lib['static_info']['imports']['count_imported_funcs'])
    
    def get_graph(path):
        
        def get_dot_graph(path):
            function_nodes = {}
            dot_graph = {}
            with open(path) as graph_file:
                for line in graph_file:
                    import_node = re.search(WAVM_DOT_NODE_IMPORT_RE, line)
                    if import_node:
                        node_id, function_id = import_node.groups() 
                        function_id = int(function_id) 
                        function_nodes[node_id] = function_id
                        dot_graph[node_id] = []
                    else: 
                        func_node = re.search(WAVM_DOT_NODE_FUNC_RE, line)
                        if func_node:
                            node_id, function_id = func_node.groups()
                            function_id = int(function_id) + import_funcs_num     
                            function_nodes[node_id] = function_id
                            dot_graph[node_id] = []
                        
                        edge = re.search(WAVM_DOT_EDGE_RE, line)
                        if edge: 
                            from_node, to_edge = edge.groups()
                            dot_graph[from_node].append(to_edge)
            return (function_nodes, dot_graph)
            
        function_nodes, dot_graph = get_dot_graph(path)
        graph = {}
        for from_node in dot_graph: 
            from_node_id = function_nodes[from_node]
            if from_node_id not in graph: graph[from_node_id] = []
            for to_node in dot_graph[from_node]:
                if to_node in function_nodes.keys():
                    graph[from_node_id].append(function_nodes[to_node])
        return graph
    
    graph = get_graph(dot_path)
    reachable_funcs = set([int(e["internal_id"]) for e in lib["static_info"]["exports"]["names"] if e["type"] == "function"])
    reachable_funcs, reachable_edges = get_reachable_funcs_and_edges(graph, reachable_funcs)
    
    return (graph, reachable_funcs, reachable_edges)
    
def pretty_print_reachable_funcs(counts):
    tools = ["ourtool", "wassail", "metadce", "twiggy", "wavm"]
    count_no_None = [count for count in counts if count!= None and type(count)==int]
    if len(count_no_None) == 0: return
    MAX_COUNT_LEN = len(str(max(count_no_None)))
    for tool, count in zip(tools, counts):
        if count != None:
            count = str(count)
            count_pretty = ' '*(MAX_COUNT_LEN-len(count)) + count
            print("{} reachable functions computed for {}.".format(count_pretty, tool))

def main():

    def help_message():
        print("Usage: get-tools-data.py [OPTION] WASM_FILE")
        print("This script runs all the tools that are being evaluated on the wasm file that is passed in.")
        print("Each tools reachability graph as well as stdout and stderr are located in data/library_data/lib/CG_tools_data/tool/.")
        print("The set of reachable functions is extracted from the reachability graph for each tool.")
        print("--real-update-json\tUpdate data.json with the set of reachable functions for each tool information.")
        print("--micro-update-json\tUpdate data.json with the set of reachable functions for each tool information.")

    args = sys.argv[1:]
    if args[0] == "-h" or args[0] == "--help": 
        help_message()
        sys.exit()

    real_update_json = False
    micro_update_json = True
    bin_type = ""
    wasm_file = "" 
    if len(args) == 1: 
        if re.search(".*\.wasm", args[0]): wasm_file = args[0]
        else: print("WebAssembly binary (.wasm) expected. {} found.".format(args[0]))
    elif len(args) == 2: 
        if args[0] == '--real-update-json': 
            bin_type = "real"
            real_update_json = True
            if re.search(".*\.wasm", args[1]): wasm_file = args[1]
            else: print("WebAssembly binary (.wasm) expected. {} found.".format(args[1]))
        elif args[0] == '--micro-update-json':
            bin_type = "micro"
            micro_update_json = True
            if re.search(".*\.wasm", args[1]): wasm_file = args[1]
            else: print("WebAssembly binary (.wasm) expected. {} found.".format(args[1]))
        else:
            help_message()
            sys.exit()
    
    print("Computing set of reachable functions for each tool being evaluated...")

    data = {}
    lib_name, lib_obj = "", {}
    if real_update_json:
        data = json.load(open(TEST_SUITE_DATA_JSON_PATH))
        lib_name = extract_lib(wasm_file)        
        lib_obj = [l for l in data['library_data'] if l['library_name'] == lib_name][0]
    elif micro_update_json:
        data = json.load(open(MICROBENCH_DATA_JSON_PATH))
        lib_name = extract_lib(wasm_file)
        lib_obj = data[lib_name]
    
    lib_obj["tools"] = []

    ourtool_status, ourtool_time   = run_ourtool(bin_type, wasm_file, lib_name, lib_obj)
    wassail_status, wassail_time   = run_wassail(wasm_file, lib_name)
    metadce_status, metadce_time   = run_metadce(bin_type, wasm_file, lib_name, lib_obj)
    twiggy_status,  twiggy_time    = run_twiggy (wasm_file, lib_name)
    awsm_status,    awsm_time      = run_awsm   (wasm_file, lib_name)
    wavm_status,    wavm_time      = run_wavm   (wasm_file, lib_name) 
    
    #ourtool_status = False
    #wassail_status = False
    #metadce_status = False
    #twiggy_status = False
    #awsm_status = False 
    #wavm_status = False
    
    reachable_funcs_count = [None]*5

    print("Processing ourtool...")
    if ourtool_status: 
        cg_path = "{}/{}/CG_tools_data/our_tool/callgraph.dot".format(DATA_PATH, lib_name)
        graph, reachable_funcs, reachable_edges = get_reachable_funcs_from_dot(cg_path, lib_obj)
        if real_update_json:
            lib_obj["tools"].append({
                "name": "ourtool",
                "callgraph_construction": True, 
                "dce" : False,
                "execution_time": ourtool_time, 
                "callgraph": {
                    "reachable_functions": {
                        "names": list(reachable_funcs), 
                        "count": len(reachable_funcs)
                    }
                }
            })
        elif micro_update_json:
            lib_obj["tools"].append({
                "name": "ourtool",
                "callgraph_construction": True, 
                "dce" : False,
                "execution_time": ourtool_time, 
                "callgraph": {
                    "graph": graph,
                    "reachable_functions": {
                        "names": list(reachable_funcs), 
                        "count": len(reachable_funcs)
                    }, 
                    "reachable_edges": {
                        "names": list(reachable_edges), 
                        "count": len(reachable_edges)
                    }
                }
            })
        reachable_funcs_count[0] = len(reachable_funcs)
    else:
        lib_obj["tools"].append({
            "name": "ourtool",
            "callgraph_construction": True, 
            "dce" : False,
            "execution_time": None, 
            "callgraph": None
        })

    print("Processing wassail...")                        
    if wassail_status: 
        cg_path = "{}/{}/CG_tools_data/wassail/callgraph.dot".format(DATA_PATH, lib_name)
        graph, reachable_funcs, reachable_edges  = get_reachable_funcs_from_dot(cg_path, lib_obj)
        if real_update_json:
            lib_obj["tools"].append({
                "name": "wassail",
                "callgraph_construction": True, 
                "dce" : False,
                "execution_time": wassail_time, 
                "callgraph": {
                    "reachable_functions": {
                        "names": list(reachable_funcs), 
                        "count": len(reachable_funcs)
                    }
                }
            })
        elif micro_update_json:
            lib_obj["tools"].append({
                "name": "wassail",
                "callgraph_construction": True, 
                "dce" : False,
                "execution_time": wassail_time, 
                "callgraph": {
                    "graph": graph,
                    "reachable_functions": {
                        "names": list(reachable_funcs), 
                        "count": len(reachable_funcs)
                    }, 
                    "reachable_edges": {
                        "names": list(reachable_edges), 
                        "count": len(reachable_edges)
                    }
                }
            })
        reachable_funcs_count[1] = len(reachable_funcs)
    else:
        lib_obj["tools"].append({
            "name": "wassail",
            "callgraph_construction": True, 
            "dce" : False,
            "callgraph": None, 
            "execution_time": None, 
        })        

    print("Processing metadce...")                        
    if metadce_status:
        new_graph_path = '{}/{}/CG_tools_data/metadce/new-graph.txt'.format(DATA_PATH, lib_name)
        graph, reachable_funcs, reachable_edges, garbage_funcs = process_metadce(new_graph_path, lib_obj)
        if real_update_json:
            lib_obj["tools"].append({
                "name": "metadce",
                "callgraph_construction": False,
                "dce" : True,
                "execution_time": metadce_time,  
                "callgraph": {
                    "reachable_functions": {
                        "names": list(reachable_funcs), 
                        "count": len(reachable_funcs)
                    }
                },
                "garbage_functions": {
                    "names": list(garbage_funcs), 
                    "count": len(garbage_funcs)
                }
            })
        elif micro_update_json:
            lib_obj["tools"].append({
                "name": "metadce",
                "callgraph_construction": False,
                "dce" : True,
                "execution_time": metadce_time,  
                "callgraph": {
                    "graph": graph,
                    "reachable_functions": {
                        "names": list(reachable_funcs), 
                        "count": len(reachable_funcs)
                    }, 
                    "reachable_edges": {
                        "names": list(reachable_edges), 
                        "count": len(reachable_edges)
                    }
                },
                "garbage_functions": {
                    "names": list(garbage_funcs), 
                    "count": len(garbage_funcs)
                }
            })
        reachable_funcs_count[2] = len(reachable_funcs)
    else:
        lib_obj["tools"].append({
            "name": "metadce",
            "callgraph_construction": False,
            "dce" : True,
            "execution_time": None, 
            "callgraph": None,
        })
 
    print("Processing twiggy...")                        
    if twiggy_status: 
        internal_ir_path = '{}/{}/CG_tools_data/twiggy/internal_ir.txt'.format(DATA_PATH, lib_name)
        garbage_path = '{}/{}/CG_tools_data/twiggy/garbage.txt'.format(DATA_PATH, lib_name)
        graph, reachable_funcs, reachable_edges, garbage_funcs = process_twiggy(internal_ir_path, garbage_path, lib_obj)
        if real_update_json:
            lib_obj["tools"].append({
                "name": "twiggy",
                "callgraph_construction": False,
                "dce" : True,
                "execution_time": twiggy_time,  
                "callgraph": {
                    "reachable_functions": {
                        "names": list(reachable_funcs), 
                        "count": len(reachable_funcs)
                    }
                },
                "garbage_functions": {
                    "names": list(garbage_funcs), 
                    "count": len(garbage_funcs)
                }
            })
        elif micro_update_json:
            lib_obj["tools"].append({
                "name": "twiggy",
                "callgraph_construction": False,
                "dce" : True,
                "execution_time": twiggy_time,  
                "callgraph": {
                    "graph": graph,
                    "reachable_functions": {
                        "names": list(reachable_funcs), 
                        "count": len(reachable_funcs)
                    }, 
                    "reachable_edges": {
                        "names": list(reachable_edges), 
                        "count": len(reachable_edges)
                    }
                },
                "garbage_functions": {
                    "names": list(garbage_funcs), 
                    "count": len(garbage_funcs)
                }
            })
        reachable_funcs_count[3] = len(reachable_funcs)
    else:
        lib_obj["tools"].append({
            "name": "twiggy",
            "callgraph_construction": False,
            "dce" : True,
            "execution_time": None,  
            "callgraph": None,
            "reachable_functions": None,
            "garbage_functions": None
        })

    print("Processing wavm...")                        
    if wavm_status:
        wavm_dot_path = '{}/{}/CG_tools_data/WAVM/wavm.bc.callgraph.dot'.format(DATA_PATH, lib_name)
        graph, reachable_funcs, reachable_edges = process_wavm_dot(wavm_dot_path, lib_obj)
        if real_update_json:
            lib_obj["tools"].append({
                "name": "wavm",
                "callgraph_construction": True,
                "dce" : False,
                "execution_time": wavm_time,  
                "callgraph": {
                    "reachable_functions": {
                        "names": list(reachable_funcs), 
                        "count": len(reachable_funcs)
                    }
                }
            })
        elif micro_update_json:
            lib_obj["tools"].append({
                "name": "wavm",
                "callgraph_construction": True,
                "dce" : False,
                "execution_time": wavm_time,  
                "callgraph": {
                    "graph": graph,
                    "reachable_functions": {
                        "names": list(reachable_funcs), 
                        "count": len(reachable_funcs)
                    }, 
                    "reachable_edges": {
                        "names": list(reachable_edges), 
                        "count": len(reachable_edges)
                    }
                }
            })
        reachable_funcs_count[4] = len(reachable_funcs)
    else:
        lib_obj["tools"].append({
            "name": "wavm",
            "callgraph_construction": True,
            "dce" : False,
            "callgraph": None,
            "execution_time": None,  
            "reachable_functions": None
        })
    
    pretty_print_reachable_funcs(reachable_funcs_count)
    
    if real_update_json:
        print("Updating test-suite-data.json...")
        json.dump(data, open(TEST_SUITE_DATA_JSON_PATH, 'w'), indent=2)
    elif micro_update_json:
        print("Updating microbench-data.json...")
        json.dump(data, open(MICROBENCH_DATA_JSON_PATH, 'w'), indent=2)


if __name__ == "__main__":
    main()