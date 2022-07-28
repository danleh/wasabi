import json, os
import csv

DATA_DIR = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/library_data"

def get_num(data): 
	return int(data.split(":")[1])

static_data = {} # {lib -> {}}

for root, dirs, files in os.walk(DATA_DIR):
	for f in files:
		if "static-data" in f : 
			file_path = os.path.join(root, f)
			f = open(file_path)
			csvreader = csv.reader(f)
			flag = False
			for row in csvreader: 
				
				if flag: 
					print("ERROR") #TODO real message 
					os._exit()
									
				name, types, funcs, elems, exports, imports, calls, uniq_calls, call_inds = row
	
				lib = name[2:][:name[2:].index("/")]
				
				static_data[lib] = {
					"types": get_num(types), 
					"funcs": get_num(funcs), 
					"elems" : get_num(elems), 
					"exports": get_num(exports), 
					"imports": get_num(imports), 
					"calls" : get_num(calls), 
					"uniq_calls" : get_num(uniq_calls),
					"call_inds": get_num(call_inds)
				}	
				
				flag = True	
			

# Update the data.json file 
data = json.load(open("../data/data.json"))

for lib in static_data.keys(): 	 
	lib_obj = [l for l in data['library_data'] if l['library_name'] == lib][0]
	print(lib)
	lib_obj["static_info"] = {
		"num_types": static_data[lib]["types"], 
		"num_functions": static_data[lib]["funcs"],
		"num_indirectly_callable_functions": static_data[lib]["elems"],
		"num_exported_functions": static_data[lib]["exports"], 
		"num_imported_functions": static_data[lib]["imports"], 
		"num_call_instructions": static_data[lib]["calls"],
		"num_unique_call_instructions": static_data[lib]["uniq_calls"],
		"num_call_indirect_instructions": static_data[lib]["call_inds"],  
	}	
		

json.dump(data, open("../data/data.json", "w"), indent=2)

