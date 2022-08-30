import os, sys
import json 
import re
from subprocess import PIPE, Popen
from collections import Counter


WASMBENCH_FILTERED_JSON = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/WasmBench/dataset-metadata/filtered.pretty.json"
WASMBENCH_FILTERED_BINARIES = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/WasmBench/filtered-binaries-metadata/filtered"
WASMBENCH_NUM_BINARIES = 8461

WASMBENCH_ANALYSIS_JSON = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/wasmbench-data.json"

import_re = "\(import (\".+\") (\".+\") (?:\(global \(;\d+;\) (?:i|f)(?:32|64)\)|\(memory \(;\d+;\) \d+ \d+\)|\(func (?:\(;[0-9]+;\)|\$.*) \(type [0-9]+\)?\)|\(table \(;\d+;\) \d+(?: \d+)? (?:funcref|anyref)\))\)$"
export_re = "\(export \"(.+)\" \((memory|func|table) (?:\d+|\$.+)\)\)"
elem_re = "\(elem \(;[0-9]+;\) (?:\((global\.get) (\d+)\)|\((i(?:32|64)\.const) (\d+)\)) func (?:(?:\d+ )*\d+|(?:\$.+))\)"
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

def extract_static_info(data):
    i = 0
    
    for wasm_path in extract_wasm_paths(WASMBENCH_FILTERED_BINARIES):
        
        i += 1

        if i%500 == 0: print(f"{i} binaries processed.") 

        wat_path = wasm_path+".wat"
        wasm2wat_status, stderr = execute_command('wasm2wat --ignore-custom-section-errors {} -o {}'.format(wasm_path, wat_path), print_stdout=False)	

        if not wasm2wat_status: print(f"{wasm_path.split('/')[-1]} cannot be converted to wat file. {stderr}"); continue

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
            
            FUNC_RE = re.compile(r'^\(func ([^ ]+) \(type ([^ ]+)\)')
            FUNC_NUMBER_RE = re.compile(r'\(;(\d+);\)')

            # Configuration and command line arguments.
            SPLIT_LOAD_ADDR = True

            # State for each function (reset when a new function is found).
            func = None
            body = []

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
                
        percent_direct_calls, percent_indirect_calls = 0, 0
        if direct_call_num != 0:
            percent_direct_calls = (direct_call_num/(direct_call_num+indirect_call_num))*100
            percent_indirect_calls = (indirect_call_num/(direct_call_num+indirect_call_num))*100
        
        # make percentages 
        for instr, count in call_indirect_indices.most_common():
            call_indirect_indices[instr] = (count/indirect_call_num)*100
        
        data[wasm_path] = {
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
        }

        #print(data[wasm_path])

        json.dump(data, open(WASMBENCH_ANALYSIS_JSON, 'w'), indent=2)


        #for n in dict(call_indirect_indices):
        #    print(f"{n} -< {call_indirect_indices[n]}")
 

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

def main():
    
    #args = 
    #print(args[0])
    extract_static_info(json.load(open(WASMBENCH_FILTERED_JSON)))
    
    #ind_calls_over_50_percent = 0
    #for wasm_bin in data: 
    #    if data[wasm_bin]["indirect_calls"]["percent"] >= 50: ind_calls_over_50_percent += 1
                
    #print(f"{ind_calls_over_50_percent} binaries have over 50% of their total calls as indirect calls.")

if __name__ == "__main__":
    main()


#   Percentage of calls that are indirect? 
#   Percentage of binaries that have at least 1 call indirect and have at least 1 table?

#   What is the element section initialized with? Make distribution  
#   Analyze data sections, how often is offset expression i32.const vs. something else?
#   % of binaries with an import table
#   % of binaries with an export table

#   How many imports have WASI in their name?
#   How many imports have emscripten/emcc in their name?
# How many binaries do have a start section?
#   How many binaries do have a WASI _start exported function?

#   Of all binaries with at least one call_indirect, how many of those have a memory section
#   Measure binaries with at least one store

# 7. C_TypesLowLevel
# (low prio): for binaries with debug info (.debug section)
#    - extract DWARF info (source types) of all functions in the binary
#    - measure how many unique DWARF function types there are
#    - measure how many unique Wasm function types there are
#    - expect #DWARF types >> #Wasm types
#    - Python library for parsing DWARF sections


# Measure distribution of index expressions

# 6. venn diagram?
# sets:
# all binaries = 100%
# binaries with a table = x%
# binaries with an imported table = <x%
# binaries with an exported table = <x%

