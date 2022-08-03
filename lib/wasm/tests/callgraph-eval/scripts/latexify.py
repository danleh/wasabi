import sys
import json
from numpy import gradient
from prettytable import PrettyTable

JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/data.json"

LATEX_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/latex-tables.txt"

# LaTeX table that compares precision and recall for each tool, per library 
def recall_precision_latex_table(f, data):
    table_rows = []

    f.write("\\begin{table}[h]\n")
    f.write("\centering\n")
    f.write("\\begin{tabular}{cccccccccc}\n")
    f.write("    \\toprule\n")
    f.write("    \\textbf{Library} & \multicolumn{2}{c}{\\textbf{Ourtool}} & \multicolumn{2}{c}{\\textbf{Wassail}} & \multicolumn{2}{c}{\\textbf{Metadce}} & \multicolumn{2}{c}{\\textbf{Twiggy}}\\\\\n")
    f.write("    & \\textbf{P} & \\textbf{R} & \\textbf{P} & \\textbf{R} & \\textbf{P} & \\textbf{R} & \\textbf{P} & \\textbf{R}\\\\\n")
    f.write("    \midrule\n")

    counter = 0
    for lib in data['library_data']:
        for tool in lib["tools"]:
            if tool["name"] == "ourtool": ourtool_p, ourtool_r = tool["precision"], tool["recall"]
            if tool["name"] == "wassail": wassail_p, wassail_r = tool["precision"], tool["recall"]
            if tool["name"] == "metadce": metadce_p, metadce_r = tool["precision"], tool["recall"]
            if tool["name"] == "twiggy":  twiggy_p,  twiggy_r  = tool["precision"], tool["recall"]               
        gray = ""
        if counter%2 != 0: gray = "\\rowcolor{gray!20}" 
        counter += 1
        f.write("    {} {} & {:.2f} & {:.2f} & {:.2f} & {:.2f} & {:.2f} & {:.2f} & {:.2f} & {:.2f}\\\\ \n".format(
            gray,
            lib["library_name"],
            ourtool_p, ourtool_r, 
            wassail_p, wassail_r, 
            metadce_p, metadce_r, 
            twiggy_p,  twiggy_r
        ))
        table_rows.append([lib["library_name"], ourtool_p, ourtool_r, wassail_p, wassail_r, metadce_p, metadce_r, twiggy_p,  twiggy_r])

    f.write("    \\bottomrule\n")
    f.write("\end{tabular}\n")
    f.write("\end {table}\n")

    return table_rows

# LaTeX table that reports on coverage per library per testcase 
def coverage_latex_table(f, data):
    table_rows = []

    f.write("\\begin{table}[h]\n")
    f.write("\centering\n")
    f.write("\\begin{tabular}{cccc}\n")
    f.write("    \\toprule\n")
    f.write("    \\textbf{Library} & \\textbf{Test Name} & \\textbf{\%Reachable Exports} & \\textbf{\%Reachable Funcs}\\\\\n")
    f.write("    \midrule\n")

    lib_counter = 0
    for lib in data['library_data']:
        test_counter = 0
        for test in lib["tests"]:
            gray, multi_row = "", ""
            if lib_counter%2 != 0: gray = "\\rowcolor{gray!20}" 
            if test_counter == len(lib["tests"])-1: 
                multi_row = "\multirow{{{}{}}}{{{}}}{{{}}}".format(
                    "-", test_counter+1, 
                    "*", lib["library_name"])
            test_counter += 1
            f.write("    {} {} & {} & {:.2f} & {:.2f}\\\\\n".format(
                gray,
                multi_row, 
                test["test_name"], 
                test["coverage"]["exports_covered"],
                test["coverage"]["funcs_covered"]
            ))
            table_rows.append([lib["library_name"], test["test_name"], test["coverage"]["exports_covered"], test["coverage"]["funcs_covered"]])
        lib_counter += 1 

    f.write("    \\bottomrule\n")
    f.write("\end{tabular}\n")
    f.write("\end {table}\n")

    return table_rows

def recall_precision_pretty_table(data):
    table = PrettyTable()
    table.title = "Precision and Recall of every tool on each library"
    table.field_names = ["Library", "Ourtool P", "Ourtool R", "Wassail P", "Wassail R", "Metadce P", "Metadce R", "Twiggy P", "Twiggy R"] 
    table.add_rows(data)
    table.float_format = '.2'
    print(table)

def coverage_pretty_table(data):
    table = PrettyTable()
    table.title = "Coverage of each testcase for its library"
    table.field_names = ["Library", "Test", "%ReachableExports", "%ReachableFunctions"] 
    table.add_rows(data)
    table.float_format = '.2'
    print(table)

def main():
    
    args = sys.argv[1:]
    if len(args) > 0:
        print("Usage: latexify.py")
        print("")
        sys.exit()

    data = json.load(open(JSON_PATH))

    with open(LATEX_PATH, "w") as f_tab:

        row_data = recall_precision_latex_table(f_tab, data)        
        recall_precision_pretty_table(row_data)
        
        f_tab.write("\n\n\n")
        print("\n")

        row_data = coverage_latex_table(f_tab, data)
        coverage_pretty_table(row_data)

        print("The LaTeX tables can be found in data/latex-tables.txt")

if __name__ == "__main__":
    main()
