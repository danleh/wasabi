import os, sys
import json 
import re
from subprocess import PIPE, Popen
from collections import Counter
import matplotlib.pyplot as plt
import numpy as np
import statistics

WASMBENCH_FILTERED_JSON = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/WasmBench/dataset-metadata/filtered.pretty.json"
WASMBENCH_FILTERED_BINARIES = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/WasmBench/filtered-binaries-metadata/filtered"
WASMBENCH_NUM_BINARIES = 8461

WASMBENCH_ANALYSIS_JSON = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/wasmbench-data.json"

WASMBENCH_CLEAN_ANALYSIS_JSON = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/wasmbench-clean-data.json"

import_re = "\(import (\".+\") (\".+\") (?:\(global \(;\d+;\) (?:i|f)(?:32|64)\)|\(memory \(;\d+;\) \d+ \d+\)|\(func (?:\(;[0-9]+;\)|\$.*) \(type [0-9]+\)?\)|\(table \(;\d+;\) \d+(?: \d+)? (?:funcref|anyref)\))\)$"
import_func_re = "\(import \".+\" \".+\" \(func (\(;[0-9]+;\)|\$.*) \(type [0-9]+\)?\)\)$"

func_re = "\(func (\(;[0-9]+;\)|\$.*) \(type (?:[0-9]+|\$.+)\)(?: \(param (?:((?:i|f)(?:32|64)|\$.*) ?)+\))?(?: \(result (?:((?:i|f)(?:32|64)|\$.*) ?)+\))?$"

export_re = "\(export \"(.+)\" \((memory|func|table) (?:\d+|\$.+)\)\)"
export_func_re = "\(export \"(.+)\" \(func (\d+|\$.+)\)\)"

elem_re = "\(elem \(;[0-9]+;\) (?:\((global\.get) (\d+)\)|\((i(?:32|64)\.const) (\d+)\)) func (?:(?:\d+ )*\d+|(?:\$.+))\)"
elem_funcs_re = "\(elem \(;[0-9]+;\) (?:\(global\.get \d+\)|\(i(?:32|64)\.const \d+\)) func ((?:(?:\d+ )*\d+|(?:\$.+)))\)"

table_re = "\(table \(;\d+;\) \d+( \d+)? (funcref|anyref)\)$"
memory_re = "\(memory \(;\d+;\) \d+( ?:\d+)?\)$"
store_re = "(?:i|f)(?:32|64)\.store"
call_re = "call (?:\d+|\$.+)"
call_ind_re = "call_indirect \(type \d+\)"
data_re = "\(data \(;\d+;\) \((i32.const \d+|global.get \d+)\) \".+\"\)"

def process_filtered_json():
    # Does the binary have a .name section? 
    # Does the binary have a .debug section?
    # Distribution of possible source languages
    
    wasmbench_data = json.load(open(WASMBENCH_FILTERED_JSON))
    name_section  = []
    debug_section = []
    language_distribution = {}
    for binary in wasmbench_data:
        name_section.append("name" in wasmbench_data[binary]["custom_sections"])
        debug_section.append(".debug_info" in wasmbench_data[binary]["custom_sections"])
        for language in wasmbench_data[binary]["possible_source_languages"]:
            if language not in language_distribution.keys(): language_distribution[language] = 0
            language_distribution[language] += 1 

    count_binaries_with_names = (name_section.count(True)/len(name_section))*100
    count_binaries_with_debug = (debug_section.count(True)/len(debug_section))*100
    for language in language_distribution: language_distribution[language] = (language_distribution[language]/WASMBENCH_NUM_BINARIES)*100
    language_distribution = {k: v for k, v in sorted(language_distribution.items(), key=lambda item: item[1], reverse=True)}

    print("{:.2f}% of binaries have a name section.".format(count_binaries_with_names))
    print("{:.2f}% of binaries have a debug section.".format(count_binaries_with_debug))
    print()

    for language in language_distribution:
        percentage = "{:.2f}".format(language_distribution[language])
        percentage = ' '*(5-len(percentage)) + percentage + "%"
        print(f"{percentage} of binaries have {language} as a possible source language.")

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

def extract_static_data_pt2():  

    atleast_one_import_count = 0
    explicit_start_function = 0
    explicit_wasi_start_function = 0
    import_funcs_percents = []
    export_funcs_percents = []
    elem_funcs_percents = []

    for i, wasm_path in enumerate(extract_wasm_paths(WASMBENCH_FILTERED_BINARIES), 1):
        if i%500 == 0: print(f"{i} binaries processed.")
        
        wat_path = wasm_path+".wat"
        wasm2wat_status, stderr = execute_command('wasm2wat --ignore-custom-section-errors {} -o {}'.format(wasm_path, wat_path), print_stdout=False)	
        if not wasm2wat_status: print(f"{i}: {wasm_path.split('/')[-1]} cannot be converted to wat file. {stderr}"); continue

        with open(wat_path) as wat_file:
        
            atleast_one_import_flag = False
            explicit_start_function_flag = False
            explicit_wasi_start_function_flag = False
            new_funcs_count = 0
            import_funcs = set()
            export_funcs = set()
            elem_funcs = set()

            for line in wat_file:
                line = line.strip()
                
                if "(start " in line: explicit_start_function_flag = True 

                func_line       = re.search(func_re, line)
                import_func     = re.search(import_func_re, line)
                export_func     = re.search(export_func_re, line)
                elem_funcs_line = re.search(elem_funcs_re, line)
                
                if func_line: new_funcs_count += 1
                if import_func: atleast_one_import_flag = True; import_funcs.add(import_func.groups()[0])
                if export_func: 
                    export_funcs.add(export_func.groups()[1])
                    if "_start" == export_func.groups()[0]: 
                        explicit_wasi_start_function_flag = True                
                if elem_funcs_line: 
                    elem_funcs.update(elem_funcs_line.groups()[0].split(" "))
            
            print(wasm_path.split('/')[-1])

            if atleast_one_import_flag: atleast_one_import_count += 1
            total_funcs = new_funcs_count + len(import_funcs)
            if total_funcs == 0:
                import_funcs_percents.append(0)
                export_funcs_percents.append(0)
                elem_funcs_percents.append(0)
            else:
                import_funcs_percents.append((len(import_funcs)/total_funcs)*100)
                export_funcs_percents.append((len(export_funcs)/total_funcs)*100)
                elem_funcs_percents.append((len(elem_funcs)/total_funcs)*100)
            if explicit_start_function_flag: explicit_start_function += 1
            if explicit_wasi_start_function_flag: explicit_wasi_start_function += 1

    print(f"{atleast_one_import_count} atleast_one_import_count")
    print(f"{explicit_start_function} explicit_start_function")
    print(f"{explicit_wasi_start_function} explicit_wasi_start_function")
    print()
    print(f"{statistics.mean(import_funcs_percents):.2f} import_funcs_percents mean")
    print(f"{statistics.mean(export_funcs_percents):.2f} export_funcs_percents mean")
    print(f"{statistics.mean(elem_funcs_percents):.2f} elem_funcs_percents mean")
    print(f"{statistics.median(import_funcs_percents):.2f} import_funcs_percents median")
    print(f"{statistics.median(export_funcs_percents):.2f} export_funcs_percents median")
    print(f"{statistics.median(elem_funcs_percents):.2f} elem_funcs_percents median")
print()    
    



def extract_static_info(data):

    for wasm_path in extract_wasm_paths(WASMBENCH_FILTERED_BINARIES):
        
        i += 1

        if i%500 == 0: 
            print(f"{i} binaries processed.")
            json.dump(data, open(WASMBENCH_ANALYSIS_JSON, 'w'), indent=2) 
        
        wat_path = wasm_path+".wat"
        wasm2wat_status, stderr = execute_command('wasm2wat --ignore-custom-section-errors {} -o {}'.format(wasm_path, wat_path), print_stdout=False)	

        if not wasm2wat_status: print(f"{i}: {wasm_path.split('/')[-1]} cannot be converted to wat file. {stderr}"); continue

        #total = sum(call_indirect_indices.values())
        #print()
        #for instr, count in call_indirect_indices.most_common():
        #    print(f"{count:6} ({count/total:5.1%}) {instr}")

        direct_call_num = 0
        indirect_call_num = 0
        import_table_exists, export_table_exists = False, False
        memory_section_exists, import_memory_exists, export_memory_exists = False, False, False 
        store_exists = False
        emcc_in_import_name, wasi_in_import_name = False, False
        start_in_export_name = False
        elem_initialization = []
        data_initialization = []
        call_indirect_indices = Counter()

        with open(wat_path) as wat_file:
            
            for line in wat_file:
                line = line.strip()
                
                call_line     = re.search(call_re, line)
                call_ind_line = re.search(call_ind_re, line)
                import_line   = re.search(import_re, line) 
                export_line   = re.search(export_re, line)
                elem_line     = re.search(elem_re, line)
                data_line     = re.search(data_re, line)
                
                if not memory_section_exists: memory_section_exists = re.search(memory_re, line) != None            
                if not store_exists: store_exists = re.search(store_re, line) != None
                
                if call_line: direct_call_num += 1
                if call_ind_line: indirect_call_num += 1
                if import_line:
                    name = ".".join([x[1:-1] for x in import_line.groups()])
                    if not import_memory_exists: import_memory_exists = re.search("\(import (\".+\") (\".+\") \(memory \(;\d+;\) \d+ \d+\)\)", line) != None
                    if not import_table_exists: import_table_exists = re.search("\(table \(;\d+;\) \d+(?: \d+)? (?:funcref|anyref)\)", line) != None
                    if not emcc_in_import_name: emcc_in_import_name = "emscripten" in name
                    if not wasi_in_import_name: wasi_in_import_name = "wasi" in name 
                if export_line:
                    name, export_type = export_line.groups()
                    if not export_memory_exists: export_memory_exists = "memory" in export_type
                    if not export_table_exists: export_table_exists = "table" in export_type
                    if not start_in_export_name: start_in_export_name = "_start" in name
                if elem_line:
                    type, value = [x for x in elem_line.groups() if x != None]
                    elem_initialization.append((type, value))
                if data_line:
                    data_initialization.append(data_line.groups()[0])

        
        
        percent_direct_calls, percent_indirect_calls = 0, 0
        if direct_call_num != 0:
            percent_direct_calls = (direct_call_num/(direct_call_num+indirect_call_num))*100
            percent_indirect_calls = (indirect_call_num/(direct_call_num+indirect_call_num))*100
        
        # make percentages 
        for instr, count in call_indirect_indices.most_common():
            call_indirect_indices[instr] = (count/indirect_call_num)*100
        
        wasm_path = wasm_path.split("/")[-1].split(".")[0]
        data[wasm_path].update({
            "direct_calls" : {
                "count": direct_call_num,
                "percent": percent_direct_calls
            },
            "indirect_calls": {
                "count": indirect_call_num, 
                "percent": percent_indirect_calls
            },
            "call_indirect_indices": dict(call_indirect_indices),
            "import_table_exists": import_table_exists,
            "import_memory_exists": import_memory_exists,
            "export_table_exists": export_table_exists,
            "export_memory_exists": export_memory_exists,
            "memory_section_exists": memory_section_exists, 
            "store_exists": store_exists,
            "emcc_in_import_name": emcc_in_import_name, 
            "wasi_in_import_name": wasi_in_import_name,
            "start_in_export_name": start_in_export_name,
            "element_initialization": elem_initialization,
            "data_initialization": data_initialization
        })
    
    json.dump(data, open(WASMBENCH_ANALYSIS_JSON, 'w'), indent=2) 

def call_ind(wasm_path):

    FUNC_RE = re.compile(r'^\(func ([^ ]+) \(type ([^ ]+)\)')
    FUNC_NUMBER_RE = re.compile(r'\(;(\d+);\)')

    # Configuration and command line arguments.
    SPLIT_LOAD_ADDR = True

    # Global variables.
    types = {}  # TODO
    call_direct_count = 0
    call_indirect_indices = Counter()

    wat_path = wasm_path+".wat"
    wasm2wat_status, stderr = execute_command('wasm2wat --ignore-custom-section-errors {} -o {}'.format(wasm_path, wat_path), print_stdout=False)	

    input = open(wat_path).readlines()


    # State for each function (reset when a new function is found).
    func = None
    func_type = None
    body = []

    for i, line in enumerate(input):
        line = line.strip()
        
        # Record current function, its type, and instructions in the body (quick&dirty).
        match = re.search(FUNC_RE, line)
        if match:
            func = match.group(1)
            func_num_match = re.search(FUNC_NUMBER_RE, func)
            if func_num_match:
                func = int(func_num_match.group(1))
            func_type = match.group(2)
            body = []
        else:
            body.append(line)

        # "Analysis" of the index expression that flows into call_indirect.
        if line.startswith("call_indirect"):
            # print(f"function {func} (type {func_type})")
            # last_n = body[-2:]
            # for j, instr in enumerate(last_n):
            #     print(f"{i - len(last_n) + j + 2:5}: {instr}")

            def abstract_instr(instr):
                split = instr.split()
                instr = split[0]
                if len(split) > 1:
                    arg = split[1]

                if instr.startswith('local.get'):
                    if arg.startswith('$p'):
                        return f"{instr} (parameter)"
                    elif arg.startswith('$l'):
                        return f"{instr} (local)"
                    else:
                        return f"{instr} (unknown)"
                else:
                    return instr

            index_instr = body[-2]
            index_instr = abstract_instr(index_instr)
            if SPLIT_LOAD_ADDR and index_instr.startswith('i32.load'):
                addr_instr = abstract_instr(body[-3])
                index_instr += f", from addr: {addr_instr}"
                if addr_instr.startswith('i32.load'):
                    index_instr += f", from addr: {abstract_instr(body[-4])}"
            if index_instr.startswith('i32.add'):
                arg1_instr = abstract_instr(body[-3])
                arg2_instr = abstract_instr(body[-4])
                index_instr += f", args: {arg1_instr}, {arg2_instr}"
                if arg2_instr.startswith('i32.and'):
                    arg1_instr = abstract_instr(body[-5])
                    arg2_instr = abstract_instr(body[-6])
                    index_instr += f", args: {arg1_instr}, {arg2_instr}"
            call_indirect_indices[index_instr] += 1
        elif line.startswith('call'):
            call_direct_count += 1

    # Output of global stats.
    print('call_indirect index instruction:')
    total = sum(call_indirect_indices.values())
    for instr, count in call_indirect_indices.most_common():
        print(f"{count:6} ({count/total:5.1%}) {instr}")
    print(f"call_indirect: {total} total")
    print(f"(regular direct) calls: {call_direct_count}")

def main1():
    
    # TODO: add flag to recompute data from scratch. Warning: takes a long time 
    #extract_static_info(json.load(open(WASMBENCH_FILTERED_JSON)))
    
    data = json.load(open(WASMBENCH_CLEAN_ANALYSIS_JSON))
    call_indirect_index_exprs = {}
    multi_pl = Counter()
    bool_data = {
        '10% of calls are indirect': 0, 
        'table is imported': 0, 
        'memory is imported': 0, 
        'table is exported': 0, 
        'memory is exported': 0, 
        'binary has a .debug section': 0, 
        'binary has a name section': 0, 
        'binary has .dylink section': 0,
        'element section initialized by variable': 0, 
        'data section initialized by variable': 0, 
        'wasi in import name': 0, 
        'store_exists': 0
    }
    
    i = -1
    hist_data = []
    size_counter = []
    call_ind_total = 0 
    call_ind_percents = []
    call_ind_counter = {
        "0": 0,
        ">0-1": 0,
        "1-2": 0,
        "2-4": 0,
        "4-6": 0,
        "6-8": 0,
        "8-10": 0,
        "10-12": 0,
        "12-14":0, 
        "14-16": 0,
        "16-18": 0,
        "18-20": 0,
        "20-22": 0,
        "22-24": 0,
        "24-26": 0,
        "26-28": 0,
        "28-30": 0,
        "30-35": 0,
        "35-40": 0,
        "40-50": 0,
        ">50": 0
    }
    
    for binary in data:
        
        if len([1 for sec in data[binary]["custom_sections"] if ".debug" in sec]) > 0: 
            bool_data['binary has a .debug section'] += 1
        if len([1 for sec in data[binary]["custom_sections"] if "name" in sec]) > 0: 
            bool_data['binary has a name section'] += 1
        if len([1 for sec in data[binary]["custom_sections"] if "dylink" in sec]) > 0: 
            bool_data['binary has .dylink section'] += 1

        multi_pl.update(data[binary]["possible_source_languages"])

        if "call_indirect_indices" in data[binary].keys(): 
            i += 1 
            
            call_ind_total += data[binary]["indirect_calls"]['count']
            p = data[binary]["indirect_calls"]["percent"]
            call_ind_percents.append(p)

            if p == 0 : call_ind_counter["0"] += 1
            elif p>0 and p<=1: call_ind_counter[">0-1"] += 1
            elif p>1 and p<=2: call_ind_counter["1-2"] += 1

            elif p>2 and p<=4 : call_ind_counter["2-4"] += 1
            elif p>4 and p<=6 : call_ind_counter["4-6"] += 1
            elif p>6 and p<=8 : call_ind_counter["6-8"] += 1
            elif p>8 and p<=10 : call_ind_counter["8-10"] += 1
            elif p>10 and p<=12 : call_ind_counter["10-12"] += 1
            elif p>12 and p<=14 : call_ind_counter["12-14"] += 1
            elif p>14 and p<=16 : call_ind_counter["14-16"] += 1
            elif p>16 and p<=18 : call_ind_counter["16-18"] += 1
            elif p>18 and p<=20 : call_ind_counter["18-20"] += 1
            elif p>20 and p<=22 : call_ind_counter["20-22"] += 1
            elif p>22 and p<=24 : call_ind_counter["22-24"] += 1
            elif p>24 and p<=26 : call_ind_counter["24-26"] += 1
            elif p>26 and p<=28 : call_ind_counter["26-28"] += 1
            elif p>28 and p<=30 : call_ind_counter["28-30"] += 1

            elif p>30 and p<=35: call_ind_counter["30-35"] += 1
            elif p>35 and p<=40: call_ind_counter["35-40"] += 1
            elif p>40 and p<=50: call_ind_counter["40-50"] += 1
            elif p>50 and p<=100: call_ind_counter[">50"] += 1

                    
            not_processed = list(call_indirect_index_exprs.keys())
            for ind in data[binary]["call_indirect_indices"]:
                if ind not in call_indirect_index_exprs.keys(): 
                    call_indirect_index_exprs[ind] = [0]*i
                    not_processed.append(ind)
                not_processed.remove(ind)
                call_indirect_index_exprs[ind].append(data[binary]["call_indirect_indices"][ind])
            for key in not_processed: call_indirect_index_exprs[key].append(0)

            if data[binary]['store_exists']: bool_data['store_exists'] += 1
            if data[binary]["indirect_calls"]["count"] > 0 : bool_data['10% of calls are indirect'] += 1
            if data[binary]["import_table_exists"]: bool_data['table is imported'] += 1
            if data[binary]["import_memory_exists"]: bool_data['memory is imported'] += 1
            if data[binary]["export_table_exists"]: bool_data['table is imported'] += 1
            if data[binary]["export_memory_exists"]: bool_data['memory is exported'] += 1
            if data[binary]["wasi_in_import_name"]: bool_data["wasi in import name"] += 1
            
            if len([1 for init in data[binary]["element_initialization"] if ".get" in init[0]])>0: bool_data['element section initialized by variable'] += 1
            if len([1 for init in data[binary]["data_initialization"] if ".get" in init])>0: bool_data['data section initialized by variable'] += 1

    ##plt.hist(size_counter)
    #plt.show()
    #plt.clf()

    #for pl, cnt in multi_pl.most_common(): 
    #    print(f"{(cnt/8461)*100}% {pl}")

    #print(f"{call_ind_total} total number of indirect calls")
    #print(f"Mean of call indirect percentages is {statistics.mean(call_ind_percents)}")
    #print(f"Median of call indirect percentages is {statistics.median(call_ind_percents)}")
    
    print(bool_data['table is imported'])
    print(bool_data['table is imported']/8392)
    
    print(8392-bool_data['binary has a .debug section'])
    print((8392-bool_data['binary has a .debug section'])/8392)
    
    print(8392-bool_data['binary has a name section'])
    print((8392-bool_data['binary has a name section'])/8392)
    

    '''
    plt.rcParams.update({'font.size': 30})
    plt.rcParams["figure.figsize"] = (20,11)
    x, y1, y2 = [], [], []
    for n in call_ind_counter: 
        x.append(n)
        y1.append(call_ind_counter[n])
        y2.append((call_ind_counter[n]/8392)*100)
    
    fig, ax1 = plt.subplots()
    ax2 = ax1.twinx()
    ax1.bar(x[1:], y1[1:], color = (0, 0, 0.5))
    ax2.bar(x[1:], y2[1:], color = (0, 0, 0.5))
    
    labels = []
    for p in y2: 
        if p == 0: labels.append("0%")
        elif p>0 and p<1: labels.append(f"{p:.1f}%")
        else: labels.append(f"{p:.0f}%")

    rects = ax1.patches
    for rect, label in zip(rects, labels):
        height = rect.get_height()
        ax1.text(
            rect.get_x() + rect.get_width() / 2, height + 5, label, ha="center", va="bottom", rotation=90
        )

    ax1.margins(y=0.15)
    ax2.margins(y=0.15)
    
    ax1.set_xlabel("Percentage of Indirect Calls")
    ax1.set_ylabel("Number of binaries")
    ax2.set_ylabel("Percentage of binaries")
    for tick in ax1.get_xticklabels():
        tick.set_rotation(45)
    plt.savefig('/home/michelle/Documents/sa-for-wasm/wasm-call-graph-challenges-paper/figures/call_ind_distribution.pdf', bbox_inches='tight')
    #plt.show()
    '''
    #call_indirect_index_exprs = {k: v for k, v in sorted(call_indirect_index_exprs.items(), key=lambda item: len(item[1])-item[1].count(0), reverse=True)}

    #for n, cnt in call_ind_counter.most_common(): 
    #    print(f"{n}% -> {cnt}")
    
    #for d in bool_data:
    #    print(f"{(bool_data[d]/8392)*100: .0f} & {bool_data[d]} -> {d} ->  -> ")
    # set the font globally
    #plt.rcParams.update({'font.family':'sans-serif'})
    
    '''
    # barplot for call indirect index distribution
    x, y = [], []
    for key, value in call_indirect_index_exprs.items(): 
        x.append(key)
        y.append(len(value)-value.count(0)) 
    plt.bar(x[:20], y[:20])
    plt.xlabel("pattern")
    plt.ylabel("#binaries")
    plt.ylim([0, 9000])
    #plt.title("Distribution of call indirect index patterns in WasmBench binaries")
    plt.xticks(rotation=30, ha='right')
    plt.savefig('../data/index_barplot_distribution.pdf', bbox_inches='tight')
    #plt.show()
    plt.clf()
    
    # boxplot for call indirect index distribution
    #fig, ax = plt.subplots()
    #ax.boxplot(call_indirect_index_top_10.values())
    #ax.set_xticklabels(call_indirect_index_top_10.keys())
    #plt.xticks(rotation=30, ha='right')
    #plt.savefig('call_indirect_index_distribution.png', bbox_inches='tight')
    #plt.show()
    
    # barplot for challenges
    x = list(bool_data.keys())
    y = list(bool_data.values())
    plt.bar(x, y)
    plt.xlabel("Challenge")
    plt.ylabel("#binaries")
    plt.ylim([0, 9000])
    #plt.title("Distribution of challenges in WasmBench binaries")
    plt.xticks(rotation=30, ha='right')
    plt.savefig('../data/challenges_distribution.pdf', bbox_inches='tight')
    #plt.show()
    
    '''

def main(): 
    #print(sys.argv)
    extract_static_data_pt2()

if __name__ == "__main__":
    main()
