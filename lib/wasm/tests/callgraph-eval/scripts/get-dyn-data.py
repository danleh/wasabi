import os
import json 

# This script reads the Wasabi output files and saves the data in data.json by updating it  


def get_file_data(path): 
	return [line.strip() for line in open(path, "r").readlines()]	

				
dyn_data = dict() # {library -> {test_name -> {reachable ->, lowerbound -> }}}


# Extract the reachable exports and reachable functions sets from the data 
for root, dirs, files in os.walk("../data/library_data"):
	for f in files: 
		
		if "exports" in f or "lowerbound" in f: 
			
			lib, _, test = root[21:].split("/")
			if lib not in dyn_data.keys(): 
				dyn_data[lib] = {}
				dyn_data[lib]["tests"] = {}
			if test not in dyn_data[lib]["tests"].keys():
				dyn_data[lib]["tests"][test] = {} 
			
			if "exports" in f:
				dyn_data[lib]["tests"][test]["reachable_exports"] = set(get_file_data(os.path.join(root, f)))
				 			
			if "lowerbound" in f: 
				funcs = set(get_file_data(os.path.join(root, f)))
				funcs = [int(x) for x in funcs]
				dyn_data[lib]["tests"][test]["reachable_functions"] = funcs 


# Union all the reachable functions sets together to get the total_reachable functions of the library 
for lib in dyn_data: 
	total_reachable = set()

	for test in dyn_data[lib]["tests"]:
		total_reachable.update(dyn_data[lib]["tests"][test]["reachable_functions"])

	dyn_data[lib]["total_reachable_funcs"] = total_reachable



# Update the data.json file 
data = json.load(open("../data/data.json"))

for lib in dyn_data.keys(): 	 
	lib_obj = [l for l in data['library_data'] if l['library_name'] == lib][0]
		
	for test in dyn_data[lib]["tests"]: 
		
		ind_test = [i for i in range(len(lib_obj["tests"])) if lib_obj["tests"][i]['test_name'] == test][0]
		
		lib_obj["tests"][ind_test]["dyn_reachable_exports"] = {
			"names": list(dyn_data[lib]["tests"][test]["reachable_exports"]), 
			"number": len(dyn_data[lib]["tests"][test]["reachable_exports"])
		}
		
		lib_obj["tests"][ind_test]["dyn_reachable_functions"] = {
			"names": list(dyn_data[lib]["tests"][test]["reachable_functions"]), 
			"number": len(dyn_data[lib]["tests"][test]["reachable_functions"])
		}
		
		lib_obj["dyn_total_reachable_functions"] = {
			"names" : list(dyn_data[lib]["total_reachable_funcs"]), 
			"number" : len(dyn_data[lib]["total_reachable_funcs"])
		}	
					
json.dump(data, open("../data/data.json", "w"), indent=2)



