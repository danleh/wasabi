import csv 
import sys

path_csv_file = sys.argv[1]

# get index of function, table, elem, etc 
#  code[0] -> 0 
# table[1] -> 1
#  elem[3] -> 3 
def get_index(name): 
	return name[name.index('[')+1:name.index(']')]

row_headings = []
row_data = []
with open(path_csv_file, newline='') as csvfile: 
	csvreader = csv.reader(csvfile, delimiter=',')
	row_headings = next(csvreader)
	for row in csvreader: 
		row_data.append(row)
		
index_of_path = row_headings.index('Path')
index_of_name = row_headings.index('Name')

callgraph_edges = set()
funcs = set()

for row in row_data:
	path = row[index_of_path]
	name = row[index_of_name]
	
	
	if len(path) == 0 : 
		continue
	
	# all functions will flow into type
	# even the ones that are removed 
	# so record them and add them to the graph 
	if 'type' in name : 
		path = path.split(' -> ')
		[funcs.add(func) for func in path[:-2]] 
		continue

	if 'elem' in name : continue 
	if 'export' in path : continue 
	
	# the paths with elem in them are of two kinds
	#
	# 1. elem[0] -> code[1] 
	#    this indicates that code[1] is in the elem section 
	#
	# 2. elem[0] -> code[0] -> code[4]
	#    this is a weird one. 
	#    in the text format this is represented as, 
	#    	⬑ code[4]
    #      		⬑ elem[0]
    #      		⬑ code[0]
	#    so both flow into code[4] independently 
	#    there might not be a code[0] -> code[4] edge again
	#    so we need to add it into the list of edges now 
	
	if 'elem' in path: 
	    arrow_count = path.count(' -> ')
	    if arrow_count == 1 : continue
	    elif arrow_count > 1 :
	    	path = ' -> '.join(path.split(' -> ')[1:])
	
	callgraph_edges.add(path)		 
		
dot_file = open('callgraph.dot', 'w')
dot_file.write("digraph G {\n")

for func in funcs: 
	dot_file.write('\tf'+get_index(func)+"\n")	

for edge in callgraph_edges: 
	edge = ' -> '.join(['f'+get_index(x) for x in edge.split(' -> ')])
	dot_file.write("\t"+edge+"\n")

dot_file.write("}")

