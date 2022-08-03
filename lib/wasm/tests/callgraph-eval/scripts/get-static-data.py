import os, re, sys
import json 


TEST_SUITE_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/test-suite" 

DATA_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/library_data"

JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/data.json"


#TODO: Modularize  
#TODO: make groups and extract output via groups! 
type_re = "\(type \(;[0-9]+;\) \(func( \(param ((i|f)(32|64) ?)+\))?( \(result ((i|f)(32|64) ?)+\))?\)\)"
func_re = "\(func \(;[0-9]+;\) \(type [0-9]+\)( \(param ((i|f)(32|64) ?)+\))?( \(result ((i|f)(32|64) ?)+\))?$"
import_re = "\(import \".+\" (\(global \(;\d+;\) (i|f)(32|64)\)|\(memory \(;\d+;\) \d+ \d+\)|\(func \(;[0-9]+;\) \(type [0-9]+\)?\)|\(table \(;\d+;\) \d+( \d+)? (funcref|anyref)\))\)$"
export_re = "\(export \".+\" \((memory|func|table) \d+\)\)$"
elem_re = "\(elem \(;[0-9]+;\) (\(global\.get \d+\)|\(i(32|64)\.const \d+\)) func (\d+ )*\d+\)$"
table_re = "\(table \(;\d+;\) \d+( \d+)? (funcref|anyref)\)$"
call_re = "call \d+"
call_ind_re = "call_indirect \(type \d+\)"
get_ind_re = "\(;\d+;\)"


def extract_lib(path): 
	path = path[:path.rfind("/")]
	return path[path.rfind("/")+1:]

args = sys.argv[1:]
if args[0] == "-h" or args[0] == "--help": 
	print("Usage: get-static-data.py [OPTION] WASM_FILE")
	print("This script analyzes the given WebAssembly binary and report static information about the binary.")
	print("--update-json\tUpdate data.json with the binaries static information.")
	sys.exit()

update_json = False
wasm_file = "" 
if len(args) == 1: 
	if re.search(".*\.wasm", args[0]): wasm_file = args[0]
	else: print("WebAssembly binary (.wasm) expected. {} found.".format(args[0]))
elif len(args) == 2: 
	if args[0] == '--update-json': 
		update_json = True
		if re.search(".*\.wasm", args[1]): wasm_file = args[1]
		else: print("WebAssembly binary (.wasm) expected. {} found.".format(args[1]))
	else:
		print("Usage: static-update-json.py [OPTION] WASM_FILE")
		print("This script analyzes the given WebAssembly binary and report static information about the binary.")
		print("--update-json\tUpdate data.json with the binaries static information.")
		sys.exit()

r_data = {} # raw data 
p_data = {} # processed data 

os.system('wasm2wat {} -o {}.wat'.format(wasm_file, wasm_file))	

lib = extract_lib(wasm_file)

wasm_file_pretty = wasm_file.split("/")
wasm_file_pretty = wasm_file_pretty[len(wasm_file_pretty)-1]
print("Extracting static information from {}".format(wasm_file_pretty))

r_data[lib] = {
	'types': [],
	'funcs': [],
	'imports': [],
	'exports': [],
	'tables': [],
	'elems': [],
	'calls': [],
	'call_inds': []
} 

# TODO: why go through all the lines first and then processes them and update the json file? 
# Maybe updatation of the JSON file should be pushed to the end since it is an IO operation 
# The rest should be done in order: Match, process, update data dictionary
with open(wasm_file+".wat") as wat_file:
	for line in wat_file: 
		line = line.strip()
		if re.search(type_re, line): r_data[lib]['types'].append(line) 
		elif re.search(func_re, line): 
			r_data[lib]['funcs'].append(line)
		elif re.search(import_re, line): r_data[lib]['imports'].append(line)
		elif re.search(export_re, line): r_data[lib]['exports'].append(line)
		elif re.search(elem_re, line): r_data[lib]['elems'].append(line)		
		elif re.search(table_re, line): r_data[lib]['tables'].append(line)
		elif re.search(call_re, line): r_data[lib]['calls'].append(line)
		elif re.search(call_ind_re, line): r_data[lib]['call_inds'].append(line)


p_data[lib] = {
	'types': 0,
	'funcs': 0,
	'imports': {		
		'names': [], 
		'count_total': 0,
		'count_imported_funcs': 0, 
		'imported_table': False, 
		'imported_memory': False,  
	},
	'exports': {
		'names': [], 
		'count_total': 0,
		'count_exported_funcs': 0, 
		'exported_memory': False, 
		'exported_table': False 
	},
	#'tables': { 
	# 	ind : {
	# 		'size': 0, 
	# 		'elems' : {
	# 			'start': {
	# 				'type' : '', # global variable or constant
	#				'value' : 0,
	# 			}, 
	#			'names': [], 
	# 			'count': 0
	# 		}
	# 	} 
	# },
	'tables': {}, 
	'calls': [],
	'call_inds': [],  
}


# Imports: 
# We need process the imports before Tables because 
# an imported table is a valid table for which an elem section might be defined  
# Calculate 
# - the total number of imports, 
# - the total number of imported functions, 
# - if a table is imported or not, 
# - if memory is imported or not, 
# - names of each import as (type, module name, export name, internal name)
imp_names = []
flag_imp_mem, flag_imp_tab = False, False 
count_imp_funcs = 0
for imp in r_data[lib]['imports']: 

	tab_re_search = re.search("\(table \(;\d+;\) \d+( \d+)? (funcref|anyref)\)", imp)
	if tab_re_search: 
		flag_imp_tab = True
		imp_type = 'table'
		count_imp_funcs -= 1
		tab_ind = re.search(get_ind_re, imp)[0][2:-2]
		tab_size = int(tab_re_search.groups()[0])
		tab_type = tab_re_search.groups()[1]
		p_data[lib]['tables'][tab_ind] = {
			'size': tab_size, 
			'type': tab_type, 
		}


	elif re.search("\(memory \(;\d+;\) \d+ \d+\)", imp): 
		flag_imp_mem = True
		count_imp_funcs -= 1
		imp_type = 'memory'
	
	elif re.search("\(global \(;\d+;\) (i|f)(32|64)\)", imp): 
		count_imp_funcs -= 1
		imp_type = 'global var'
	
	elif re.search("\(func \(;[0-9]+;\) \(type [0-9]+\)?\)", imp): imp_type = 'function' 
	
	else: sys.exit("Unknown import type!!! {}".format(imp))

	imp_ind = re.search(get_ind_re, imp)[0][2:-2]
	imp_module, imp_name = re.search("\".*\"", imp)[0].split(" ")
	imp_module, imp_name = imp_module[1:-1], imp_name[1:-1]
	imp_names.append({
		'type': imp_type, 
		'module_name': imp_module, 
		'export_name_within_module': imp_name, 
		'internal_name': imp_ind, 
	})
	

count_imp_funcs += len(imp_names)
p_data[lib]['imports'] = {
	'names': imp_names, 
	'count_total': len(imp_names),
	'count_imported_funcs': count_imp_funcs, 
	'imported_table': flag_imp_tab, 
	'imported_memory': flag_imp_mem,  
}


# Types: Right now we only want to report the number of types
p_data[lib]['types'] = len(r_data[lib]['types'])


# Functions: Right now, we only want to report on the number of functions 
p_data[lib]['funcs'] = len(r_data[lib]['funcs'])


# Exports: for each line extract the name, number of exports, 
# and whether the table and memory are exported 
exp_names = []
count_exported_funcs = 0
flag_exp_mem, flag_exp_tab = False, False 
for exp in r_data[lib]['exports']: 
	exp_split = exp.split(" ")
	exp_name = exp_split[1][1:-1]
	exp_internal_id = int(exp_split[3][:-2])
	exp_name = re.search("\".*\"", exp)[0][1:-1]
	if "table " in exp: 
		exp_type = 'table'
		flag_exp_tab = True
	elif "memory " in exp: 
		exp_type = 'memory'
		flag_exp_mem = True
	elif "func " in exp: 
		count_exported_funcs += 1
		exp_type = 'function'
	else: sys.exit('Unknown export: {}'.format(exp))
	exp_names.append({
		'type': exp_type, 
		'name': exp_name, 
		'internal_id': exp_internal_id
	})

p_data[lib]['exports'] = {
	'names': exp_names, 
	'count': len(exp_names),
	'count_exported_funcs': count_exported_funcs,
	'exported_table': flag_exp_tab, 'exported_memory': flag_exp_mem,  
}		


# Tables: get table index and table size 
for tab in r_data[lib]['tables']: 
	tab_split = tab.split(" ") 
	tab_ind = tab_split[1][2:-2]
	tab_size = int(tab_split[2])
	tab_type = tab_split[len(tab_split)-1][:-1]
	if len(tab_split) == 5: 
		if tab_size != int(tab_split[3]): 
			sys.exit("Assumption violated: The two numbers in the table initialization are always equal: {}".format(tab))				
	elif len(tab_split) != 4: sys.exit("Did not expect so many things defined in table: {}".format(tab))
	p_data[lib]['tables'][tab_ind] = {
		'size': tab_size, 
		'type': tab_type, 
	}


# Element Section 
for elem in r_data[lib]['elems']:
	elem_funcs = re.search("func (\d+ )*\d+", elem)[0]
	elem_funcs = elem_funcs[5:].split(" ")
	elem_ind = re.search(get_ind_re, elem)[0][2:-2]
	
	elem_start_type = ''
	if re.search("\(global\.get \d+\)", elem): 
		elem_start_type = 'global variable'

	elif re.search("\(i(32|64)\.const \d+\)", elem): 
		elem_start_type = 'constant'
	
	else: sys.exit("Element section start index initialization is not with a constant or a variable: {}".format(elem))

	elem_start = int(elem.split(" ")[3][:-1])
	
	if elem_ind not in p_data[lib]['tables'].keys(): 
		sys.exit("Assumption violated: Elem sections are matched up to their tables using their index: {}".format(elem))
	
	p_data[lib]['tables'][elem_ind]['elem']= {
		'start': {
			'type': elem_start_type, # variable or constant  
			'value': elem_start,
		},
		'entries': elem_funcs, 
		'count': len(elem_funcs)
	}


# Calls
for call in r_data[lib]['calls']: 
	if ")" in call: call=call[:-1]
	p_data[lib]['calls'].append(int(call.strip().split(" ")[1]))


#Call Indirect 
for call in r_data[lib]['call_inds']: 
	if "))" in call: call=call[:-1]
	p_data[lib]['call_inds'].append("type "+call.strip().split(" ")[2][:-1])
	

# Update the data.json file 
if update_json:
	print("Updating data.json...")
	data = json.load(open(JSON_PATH))
	lib_obj = [l for l in data['library_data'] if l['library_name'] == lib][0]
	lib_obj['static_info'] = {
		'count_types': p_data[lib]['types'], 
		'count_functions': p_data[lib]['funcs'],		
		'tables': p_data[lib]['tables'], 
		'imports': p_data[lib]['imports'], 
		'exports': p_data[lib]['exports'], 
		'calls': {
			'names': p_data[lib]['calls'], 
			'count_total_calls': len(p_data[lib]['calls']),
			'count_unique_calls': len(set(p_data[lib]['calls'])),
		}, 
		'call_indirects': {
			'count_total_call_indirects': len(p_data[lib]['call_inds']),
			'types_called_indirectly': list(set(p_data[lib]['call_inds']))
		}
	}	
	json.dump(data, open(JSON_PATH, 'w'), indent=2)
else:
	MAX_LEFT_LEN = 25
	print("#types:{}{}".format(" "*(MAX_LEFT_LEN-7),p_data[lib]['types'],)) 
	print("#functions:{}{}".format(" "*(MAX_LEFT_LEN-11),p_data[lib]['funcs'],)) 
	print("#imports:{}{}".format(" "*(MAX_LEFT_LEN-9),p_data[lib]['imports']['count_total']))
	print("#imported functions:{}{}".format(" "*(MAX_LEFT_LEN-20),p_data[lib]['imports']['count_imported_funcs']))
	print("#exports:{}{}".format(" "*(MAX_LEFT_LEN-9),p_data[lib]['exports']['count']))
	for ind in p_data[lib]['tables']:
		print("table{} size:{}{}".format(ind, " "*(MAX_LEFT_LEN-(11+len(str(ind)))), p_data[lib]['tables'][ind]['size']))
	print("#calls:{}{}".format(" "*(MAX_LEFT_LEN-7), len(p_data[lib]['calls'])))
	print("#unique calls:{}{}".format(" "*(MAX_LEFT_LEN-14), len(set(p_data[lib]['calls']))))
	print("#call_indirects:{}{}".format(" "*(MAX_LEFT_LEN-16), len(p_data[lib]['call_inds'])))
	#print("types called indirectly: {}".format(list(set(p_data[lib]['call_inds']))))
