import sys
import json
from numpy import gradient
from prettytable import PrettyTable

REAL_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/test-suite-data.json"
MICRO_JSON_PATH = "/home/michelle/Documents/sa-for-wasm/wasabi/lib/wasm/tests/callgraph-eval/data/microbench-data.json"

LATEX_RECALL_TABLE = "/home/michelle/Documents/sa-for-wasm/wasm-call-graph-challenges-paper/sections/recall-table.tex"
LATEX_COVERAGE_TABLE = "/home/michelle/Documents/sa-for-wasm/wasm-call-graph-challenges-paper/sections/real-world-coverage.tex"
LATEX_MICRO_EVAL_TABLE = "/home/michelle/Documents/sa-for-wasm/wasm-call-graph-challenges-paper/sections/micro-eval.tex"

def recall_precision_latex_table(f, data):
    table_rows = []

    f.write("\\newcommand{\Ft}{{$\\textbf{F}_{\\textbf{t}}$}}\n")
    f.write("\\newcommand{\Fs}{{$\\textbf{F}_{\\textbf{s}}$}}\n")
    f.write("\\newcommand{\Fd}{{$\\textbf{F}_{\\textbf{d}}$}}\n")

    f.write("\\begin{table*}[t]\n")
    f.write("\\small\n")
    f.write("\centering\n")
    f.write("\captionsetup{justification=centering}\n")
    
    f.write("\\begin{tabular}{m{3.2em}|m{2.4em}|m{1.7em}|\n")
    f.write("    m{1.8em}m{1.3em}m{2em}|\n")
    f.write("    m{1.5em}m{1.7em}m{2em}|\n")
    f.write("    m{1.4em}m{1.3em}m{2em}|\n")
    f.write("    m{1.4em}m{1.3em}m{2em}|\n")
    f.write("    m{1em}m{1.7em}m{2em}}\n")
    f.write("    \\toprule\n")
    f.write("    \\textbf{Library} & \Ft & \Fd &\n")
    f.write("    \multicolumn{3}{c|}{\\textbf{Ourtool}} & \n")
    f.write("    \multicolumn{3}{c|}{\\textbf{Wassail}} &\n")
    f.write("    \multicolumn{3}{c|}{\\textbf{Metadce}} & \n")
    f.write("    \multicolumn{3}{c|}{\\textbf{Twiggy}} & \n")
    f.write("    \multicolumn{3}{c}{\\textbf{WAVM}}\\\\\n")
    f.write("    & & & \n")
    f.write("    \Fs & \\thead{\Fd-\\Fs} & \\thead{\Ft-\\Fs} & \n")
    f.write("    \Fs & \\thead{\Fd-\\Fs} & \\thead{\Ft-\\Fs} & \n")
    f.write("    \Fs & \\thead{\Fd-\\Fs} & \\thead{\Ft-\\Fs} & \n")
    f.write("    \Fs & \\thead{\Fd-\\Fs} & \\thead{\Ft-\\Fs} & \n")
    f.write("    \Fs & \\thead{\Fd-\\Fs} & \\thead{\Ft-\\Fs} \\\\\n")    
    f.write("    \midrule\n")
    
    counter = 0
    for lib in data['library_data']:
        lib_total = lib["static_info"]["count_functions"] + lib["static_info"]["imports"]["count_imported_funcs"]
        lib_dyn = lib["dyn_total_reachable_functions"]["count"]

        tool_recall_data = ""
        tool_ascii_data = []
        for tool in lib["tools"]:
            if tool["reachable_functions"] == None:
                tool_recall_data += "\multicolumn{3}{c|}{Did Not Execute} & "
                tool_ascii_data.append("DNE")
            else:
                f_stat = f"{tool['reachable_functions']['count']}"
                f_missing = "\\thead{" + f"{tool['missing_funcs']['number']}\\\\"+"({:.0f}\%)".format(tool['missing_funcs']['percent'])+"}"
                f_removed = "\\thead{" + f"{tool['removed_funcs']['number']}\\\\"+"({:.0f}\%)".format(tool['removed_funcs']['percent'])+"}"
                tool_recall_data += "{} & {} & {} & ".format(
                    f_stat, f_missing, f_removed
                )
                tool_ascii_data.append("{:.2f}".format(tool["recall"]))
        tool_recall_data = tool_recall_data[:-3]
        gray = ""
        if counter%2 != 0: gray = "\\rowcolor{gray!20}" 
        counter += 1
        f.write("    {} {} & {} & {} & {}\\\\\n".format(
            gray,
            lib["pretty_name"], lib_total, lib_dyn,
            tool_recall_data
        ))
        table_rows.append([lib["pretty_name"], lib_total, lib_dyn] + tool_ascii_data)
    
    f.write("    \\bottomrule\n")
    f.write("\end{tabular}\n")
    f.write("\caption{Comparison of dynamically reachable functions $\\text{F}_\\text{DYN}$ of each library\\\\with the statically reachable functions $\\text{F}_\\text{STAT}$ and recall of each tool.}\n")
    f.write("\label{recall}\n")
    f.write("\end{table*}\n")

    return table_rows

def coverage_latex_table(f, data):
    table_rows = []

    f.write("\\begin{table}[h]\n")
    f.write("\centering\n")
    f.write("\\begin{tabular}{cccc}\n")
    f.write("    \\toprule\n")
    f.write("    \\textbf{Library} & \\textbf{Test Name} & \\thead{\\textbf{\%Reachable}\\\\textbf{Exports}} & \\thead{\\textbf{\%Reachable}\\\\textbf{Funcs}}\\\\\n")
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
                    "*", lib["pretty_name"])
            test_counter += 1
            f.write("    {} {} & {} & {:.2f} & {:.2f}\\\\\n".format(
                gray,
                multi_row, 
                test["test_name"], 
                test["coverage"]["exports_covered"],
                test["coverage"]["funcs_covered"]
            ))
            table_rows.append([lib["pretty_name"], test["test_name"], test["coverage"]["exports_covered"], test["coverage"]["funcs_covered"]])
        lib_counter += 1 

    f.write("    \\bottomrule\n")
    f.write("\end{tabular}\n")
    f.write("\end {table}\n")

    return table_rows

def micro_eval_latex_table(f, data):
    f.write("\\begin{table*}[t]\n")
    f.write("\small\n")
    f.write("\centering\n")
    f.write("\captionsetup{justification=centering}\n")
    f.write("\\begin{tabular}{ccccccc}\n")
    f.write("    \\toprule\n")
    f.write("    \\textbf{Name} & \\textbf{Challenge} & \\textbf{Ourtool} & \\textbf{Wassail} & \\textbf{Metadce} & \\textbf{Twiggy} & \\textbf{WAVM}\\\\\n")
    f.write("    \midrule\n")
    
    benchmark_counter = 0
    rows_data = []
    for test in data: 
        benchmark_name = str(test)+ "-" + data[test]["name"]
        gray = "" 
        if benchmark_counter%2 != 0: gray = "\\rowcolor{gray!20}"
        
        challenges = ""
        if data[test]["challenges"]: 
            for challenge in data[test]["challenges"]: row_data += f"{challenge} "
        challenges = challenges[:-1]

        row_data = ""
        pretty_row = []
        for tool in data[test]["tools"]:
            if tool["sound"]: 
                row_data += "\\cmark & "
                pretty_row.append("✓")
            else: 
                row_data += "\\xmark & "
                pretty_row.append("✗")
        row_data = row_data[:-3]
        
        f.write(f"    {gray} {test} & {challenges} & {row_data}\\\\\n")        
        
        benchmark_counter += 1
        rows_data.append([test] + pretty_row) 

    f.write("    \\bottomrule\n")
    f.write("\end{tabular}\n")
    f.write("\caption{Evaluation of tools on microbenchmarks}\n")
    f.write("\label{micro:eval}\n")
    f.write("\end{table*}\n")

    return rows_data

def recall_precision_pretty_table(data):
    table = PrettyTable()
    table.title = "Recall of every tool on each library"
    table.field_names = ["Library", "F_total", "F_dyn", 
                         "Ourtool Recall", 
                         "Wassail Recall", 
                         "Metadce Recall", 
                         "Twiggy Recall",
                         "WAVM Recall"] 
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

def micro_eval_pretty_table(data): 
    table = PrettyTable()
    table.title = "Evaluation of each tool against the microbenchmarks"
    table.field_names = ["Name", "Ourtool", "Wassail", "Metadce", "Twiggy", "WAVM"]
    table.add_rows(data)
    print(table)

def main():
    
    args = sys.argv[1:]
    if len(args) > 0:
        print("Usage: latexify.py")
        print("Generates LaTeX tables that are saved in data/latex-tables.txt")
        print("Also prints ASCII tables in stdout of the computed results.")
        sys.exit()

    real_data = json.load(open(REAL_JSON_PATH))
    micro_data = json.load(open(MICRO_JSON_PATH))

    with open(LATEX_MICRO_EVAL_TABLE, "w") as f_tab:
        row_data = micro_eval_latex_table(f_tab, micro_data)
        micro_eval_pretty_table(row_data)
        print("\n")
    
    with open(LATEX_RECALL_TABLE, "w") as f_tab:
        row_data = recall_precision_latex_table(f_tab, real_data)        
        recall_precision_pretty_table(row_data)
        print("\n")

    with open(LATEX_COVERAGE_TABLE, "w") as f_tab:
        row_data = coverage_latex_table(f_tab, real_data)
        coverage_pretty_table(row_data)

    print("The LaTeX tables can be found in the paper repo.")
    


if __name__ == "__main__":
    main()
