import sys
import pygraphviz as pgv

num_files = len(sys.argv) - 1
files = sys.argv[1:]

num_edges = []
num_nodes = [] 

for file in files: 
    G = pgv.AGraph(file)
    num_nodes.append(len(G.nodes()))    
    num_edges.append(len(G.edges()))    

# report on extra edges added?  
# the functions might be all called different things but they still have the same indicies 
# iterate through all edges and strip characters to just leave behind numbers 
# if its a llvm file, you can pass in a flag/ indicate it in the name and it would strip the function name appropriately

print("FILE, #FUNCS, #EDGES")
for ind in range(0,num_files): 
    print(files[ind] + ', ' + str(num_nodes[ind]) + ', ' + str(num_edges[ind]))

