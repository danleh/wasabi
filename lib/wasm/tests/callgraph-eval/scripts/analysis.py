import sys
import json 

REAL_DATA_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/test-suite-data.json"
MICRO_DATA_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/microbench-data.json"


def graph_names_normalize(graph, export_names, import_names):
    new_graph = {}
    def sort_targets(g):
        for f in g: g[f].sort()
        return g

    def replace_name_with_id(name):
        if name in export_names.keys(): 
            if export_names[name]["type"] == "function": return export_names[name]["internal_id"]
        elif name in import_names.keys(): 
            if import_names[name]["type"] == "function": return import_names[name]["internal_id"]
        else: sys.exit("Unknown identifier for function found {}".format(name))
    
    if not graph: return {} # return an empty graph if graph is an empty map

    for from_node in graph:
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
        
        for to_node in graph[from_node]:
            if from_node_id != None:
                if not str(to_node).isdigit(): 
                    to_node = replace_name_with_id(to_node) 
                new_graph[from_node_id].append(int(to_node))

    return sort_targets(new_graph)

def main():
    args = sys.argv[1:]
    
    def help_message():
        print("Usage: analysis.py OPTION")
        print("--all     Run analysis for real world and microbenchmarks")
        print("--real    Compute precision, recall for each library and coverage for each library test case")
        print("--micro   Compare callgraphs produced by each tool with the precise callgraph")
        sys.exit()

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

    flag_real, flag_micro = False, False
    if len(args) == 0: help_message()
    else: 
        for arg in args:
            if arg == "--all": flag_real, flag_micro = True, True
            elif arg == "--real": flag_real = True
            elif arg == "--micro": flag_micro = True
            else: help_message()

    if flag_micro: 
        data = json.load(open(MICRO_DATA_JSON_PATH))

        for microbench in data: 
            
            microbench = data[microbench]
            
            export_names = {item['name']: {"type":item["type"], "internal_id": item['internal_id']} for item in microbench["static_info"]["exports"]["names"]}
            import_names = {item["module_name"]+"."+item["export_name_within_module"]: {"type":item["type"], "internal_id": item['internal_id']} for item in microbench["static_info"]["imports"]["names"]}

            precise_callgraph = microbench["ground_truth"]["precise_callgraph"]
            reachable_funcs = set([int(e["internal_id"]) for e in microbench["static_info"]["exports"]["names"] if e["type"] == "function"])
            precise_callgraph = graph_names_normalize(precise_callgraph, export_names, import_names)
            precise_reachable_funcs, precise_edges = get_reachable_funcs_and_edges(precise_callgraph, reachable_funcs) 
            
            E_all = 0
            F_all = set()
            for node in precise_callgraph:
                F_all.add(node)
                F_all.update(precise_callgraph[node])
                E_all += len(precise_callgraph[node])

            microbench["ground_truth"]["count_edges"] = E_all
            microbench["ground_truth"]["count_funcs"] = len(F_all)

            microbench["ground_truth"]["reachable_functions"] = {
                "names": list(precise_reachable_funcs), 
                "count": len(precise_reachable_funcs)
            }
            microbench["ground_truth"]["reachable_edges"] = {
                "names": list(precise_edges), 
                "count": len(precise_edges)
            }
            
            for tool in microbench["tools"]:
                #print(tool['name'])
                if tool["callgraph"] == None: 
                    tool["sound"] = None
                else: 
                    tool_reachable_functions = set(tool["callgraph"]["reachable_functions"]["names"])
                    tool_reachable_edges = set()
                    for node, child in tool["callgraph"]["reachable_edges"]["names"]:
                        tool_reachable_edges.add((node, child))

                    edges_missing = precise_edges.difference(tool_reachable_edges)
                    superflous_edges = tool_reachable_edges.difference(precise_edges)
                    
                    if "wassail" in tool['name'] or "wavm" in tool['name']:   
                        tool_graph = tool["callgraph"]["graph"]
                        E_all = 0
                        for node in tool_graph:
                            E_all += len(tool_graph[node])
                        tool["callgraph"]["count_edges"] = E_all

                    tool["soundness"] = {
                        # the precise reachable functions should be a subset of the tool reachable functions for the analysis to be sound 
                        "sound": tool_reachable_functions.intersection(precise_reachable_funcs) == precise_reachable_funcs, 
                        "missing_edges": {
                            "names": list(edges_missing), 
                            "count": len(edges_missing), 
                        }
                    }
                    tool["precision"] = {
                        "precise": tool_reachable_edges == precise_edges,
                        "superflous_edges": {
                            "names": list(superflous_edges), 
                            "count": len(superflous_edges), 
                        }
                    }
        
        json.dump(data, open(MICRO_DATA_JSON_PATH, 'w'), indent=2)

    if flag_real:         
        data = json.load(open(REAL_DATA_JSON_PATH))

        # for each tool precision = |Mdyn \cap Mstat| / |Mstat|
        # for each tool recall    = |Mdyn \cap Mstat| / |Mdyn|
        # coverage for each test_case = {
        #   exported_covered = (|dyn_exports| / |total_exports|)*100
        #   funcs_convered = (|dyn_funcs| / |total_funcs|)*100
        # } 
        for lib in data["library_data"]:
            lib_total = lib["static_info"]["count_functions"] + lib["static_info"]["imports"]["count_imported_funcs"]
            M_total = set(range(0, lib_total))
            M_dyn  = set(lib["dyn_total_reachable_functions"]["names"])
            for tool in lib["tools"]:            
                if tool["callgraph"] == None:
                    tool["precision"] = None
                    tool["recall"] = None
                else:
                    M_stat = set(tool["callgraph"]["reachable_functions"]["names"])
                    cap_set = M_stat.intersection(M_dyn)                                
                    recall = len(cap_set)/len(M_dyn)            
                    M_missing = M_dyn.difference(M_stat)
                    
                    #print(f"{lib['library_name']} {tool['name']} {M_missing}")
                    
                    M_removed = M_total.difference(M_stat)
                    
                    tool["recall"] = recall
                    tool['callgraph']["missing_functions"] = {
                        "names": list(M_missing),
                        "count": len(M_missing), 
                        "percent": (len(M_missing)/len(M_dyn))*100
                    }
                    tool['callgraph']["removed_functions"] = {
                        "names": list(M_removed), 
                        "count": len(M_removed), 
                        "percent": (len(M_removed)/len(M_total))*100
                    }

            M_stat_exports = lib["static_info"]["exports"]["count_exported_funcs"]
            M_stat_funcs = lib["static_info"]["count_functions"]
            for microbench in lib["tests"]:
                M_dyn_exports = microbench["dyn_reachable_exports"]["count"]
                M_dyn_funcs =  microbench["dyn_reachable_functions"]["count"]
                percent_exports = (M_dyn_exports/M_stat_exports)*100
                percent_funcs = (M_dyn_funcs/M_stat_funcs)*100
                microbench["coverage"] = {
                    "exports_covered": percent_exports,
                    "funcs_covered": percent_funcs
                } #TODO instructions covered 

        json.dump(data, open(REAL_DATA_JSON_PATH, 'w'), indent=2)

if __name__ == "__main__":
    main()
