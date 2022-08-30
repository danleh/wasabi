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
    
    def multirow(num, data): return "\multirow{"+str(num)+"}{*}{"+str(data)+"}"
    def make_bold_red(data): return "\\textcolor{red}{\\textbf{"+str(data)+"}}"

    table_rows = []

    f.write("\\newcommand{\BFt}{{$\\textbf{F}_{\\textbf{t}}$}}\n")
    f.write("\\newcommand{\BFs}{{$\\textbf{F}_{\\textbf{s}}$}}\n")
    f.write("\\newcommand{\BFd}{{$\\textbf{F}_{\\textbf{d}}$}}\n")
    f.write("\\newcommand{\BFm}{{$\\textbf{F}_{\\textbf{m}}$}}\n")
    f.write("\\newcommand{\BFr}{{$\\textbf{F}_{\\textbf{r}}$}}\n")
    f.write("\\newcommand{\Ft}{{$\\text{F}_\\text{t}$}}\n")
    f.write("\\newcommand{\Fs}{{$\\text{F}_\\text{s}$}}\n")
    f.write("\\newcommand{\Fd}{{$\\text{F}_\\text{d}$}}\n")
    f.write("\\newcommand{\Fm}{{$\\text{F}_\\text{m}$}}\n")
    f.write("\\newcommand{\Fr}{{$\\text{F}_\\text{r}$}}\n")
    
    f.write("\setlength{\extrarowheight}{0.2em}\n")
    
    f.write("\\begin{table*}[t]\n")
    f.write("\\small\n")
    f.write("\centering\n")
    f.write("\captionsetup{justification=centering}\n")

    f.write("\\begin{tabular}{l|r|r|\n")
    f.write("    rrr|\n")
    f.write("    rrr|\n")
    f.write("    rrr|\n")
    f.write("    rrr}\n")
    f.write("    \\toprule\n")

    f.write("    \multirow{2}{*}{\\textbf{Library}} & \multirow{2}{*}{\BFt} & \multirow{2}{*}{\BFd} &\n")
    f.write("    \multicolumn{3}{c|}{\\textbf{Wassail}} &\n")
    f.write("    \multicolumn{3}{c|}{\\textbf{Metadce}} & \n")
    f.write("    \multicolumn{3}{c|}{\\textbf{Twiggy}} & \n")
    f.write("    \multicolumn{3}{c}{\\textbf{WAVM + LLVM opt}}\\\\\n")

    f.write("    & & & \n")
    f.write("    \BFs & \BFm & \BFr & \n")
    f.write("    \BFs & \BFm & \BFr & \n")
    f.write("    \BFs & \BFm & \BFr & \n")
    f.write("    \BFs & \BFm & \BFr \\\\\n")

    f.write("    \midrule\n")
    
    counter = 0
    for lib in data['library_data']:
        lib_total = lib["static_info"]["count_functions"] + lib["static_info"]["imports"]["count_imported_funcs"]
        lib_dyn = lib["dyn_total_reachable_functions"]["count"]

        tool_recall_data = ""
        tool_percent_data = ""
        tool_ascii_data = []
        for tool in lib["tools"]:
            if "ourtool" in tool["name"]: continue 
            
            if tool["callgraph"] == None:
                tool_recall_data += "& & & "
                tool_percent_data += "\multicolumn{3}{c|}{\multirow{-2}{*}{Did Not Execute}} & "
                tool_ascii_data.append("DNE")
            else:
                missing_funcs = tool['callgraph']['missing_functions']['count']
                missing_percent = "({:.0f}\%)".format(tool['callgraph']['missing_functions']['percent'])

                if tool['callgraph']['missing_functions']['count'] > 0: 
                    missing_funcs = make_bold_red(missing_funcs)
                    missing_percent = make_bold_red(missing_percent)
                    
                tool_recall_data += ("& "
                                     f"{missing_funcs} & "
                                     f"{tool['callgraph']['removed_functions']['count']} & ")
                tool_percent_data += "{} & {} & ({:.0f}\%) & ".format(
                    multirow(-2, tool['callgraph']['reachable_functions']['count']),
                    missing_percent, 
                    tool['callgraph']['removed_functions']['percent'])
                
                tool_ascii_data.append("{:.2f}".format(tool["recall"]))

        tool_recall_data = tool_recall_data[:-3]
        tool_percent_data = tool_percent_data[:-3]

        gray = ""
        if counter%2 != 0: gray = "\\rowcolor{gray!20}" 
        counter += 1
        f.write("    {} & & & {}\\\\\n".format(gray, tool_recall_data))
        f.write("    {} {} & {} & {} & {}\\\\\n".format(
            gray, 
            multirow(-2, lib["pretty_name"]), 
            multirow(-2, lib_total), 
            multirow(-2, lib_dyn),
            tool_percent_data))
        table_rows.append([lib["pretty_name"], lib_total, lib_dyn] + tool_ascii_data)
    
    f.write("    \\bottomrule\n")
    f.write("\end{tabular}\n")
    f.write("\caption{Comparison of the reachable functions reported by each tool.\\\\The number of missing functions (\Fm = \Fd-\Fs) and removed functions (\Fr = \Ft-\Fs) are also reported.}\n")
    f.write("\label{recall}\n")
    f.write("\end{table*}\n")

    return table_rows

def coverage_latex_table(f, data):
    table_rows = []

    f.write("\\begin{table}[h]\n")
    f.write("\centering\n")
    f.write("\caption{Overview of the real-world programs in our dataset.}\n")
    f.write("\label{tab:real-world-programs}\n")
    f.write("\\begin{tabular}{llrr}\n")
    f.write("    \\toprule\n")
    f.write("    \\textbf{Library} & \\textbf{Test Name} & \\thead{\\textbf{\%Reachable}\\\\\\textbf{Exports}} & \\thead{\\textbf{\%Reachable}\\\\\\textbf{Funcs}}\\\\\n")
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
    f.write("\caption{Evaluation of the tools on our microbenchmarks.}\n")
    f.write("\label{tab:microbenchmarks}\n")
    f.write("\small\n")
    f.write("\centering\n")
    f.write("\captionsetup{justification=centering}\n")
    
    f.write("\\begin{tabular}{l|l|rrr|rrrr|rrr|rrr|rrrr}\n")
    f.write("    \\toprule\n")
    f.write("    \multirow{2}{*}{\\textbf{Name}} & \multirow{2}{*}{\\textbf{Challenge}} & \n")
    f.write("    \multicolumn{3}{c|}{\\textbf{Ground Truth}} & \n")
    f.write("    \multicolumn{4}{c|}{\\textbf{Wassail}} &\n")
    f.write("    \multicolumn{3}{c|}{\\textbf{Metadce}} & \n")
    f.write("    \multicolumn{3}{c|}{\\textbf{Twiggy}} &\n")
    f.write("    \multicolumn{4}{c}{\\textbf{WAVM+LLVM opt}}\\\\\n")
    f.write("    & & \n")
    f.write("    \\textbf{$|F_{all}|$} & \\textbf{$|F_{r}|$} & \\textbf{$|E|$} &\n")
    f.write("    \\textbf{$|F_{r}|$} & \\textbf{$|E|$} & \\textbf{S} & \\textbf{P} & \n")
    f.write("    \\textbf{$|F_{r}|$} & \\textbf{S} & \\textbf{P} &\n")
    f.write("    \\textbf{$|F_{r}|$} & \\textbf{S} & \\textbf{P} & \n")
    f.write("    \\textbf{$|F_{r}|$} & \\textbf{$|E|$} & \\textbf{S} & \\textbf{P} \\\\ \n")
    f.write("    \midrule\n")
    
    benchmark_counter = 0
    rows_data = []
    for microbench in data: 
        gray = "" 
        if benchmark_counter%2 != 0: gray = "\\rowcolor{gray!20}"
        
        challenges = ""
        if data[microbench]["challenges"]: 
            for challenge in data[microbench]["challenges"]: tools_data += f"{challenge} "
        challenges = challenges[:-1]

        tools_data = ""
        pretty_row = []
        for tool in data[microbench]["tools"]:
            
            if 'ourtool' in tool['name']: 
                pretty_row.append(tool['soundness']['sound'])
                continue 

            if tool["callgraph"] == None:
                if "wassail" in tool['name'] or "wavm" in tool['name']: tools_data += "\multicolumn{4}{c|}{Did Not Execute} & "
                else: tools_data += "\multicolumn{3}{c|}{Did Not Execute} & "
                pretty_row.append("-")

            else:
                sound = "\\xmark"
                precise = "\\xmark"
                if tool['soundness']['sound']: sound = "\cmark"
                if tool['precision']['precise']: precise = "\cmark"
                
                if "wassail" in tool['name'] or "wavm" in tool['name']: 
                    tools_data += (f"{tool['callgraph']['reachable_functions']['count']} & {tool['callgraph']['count_edges']} & {sound} & {precise} & ")
                else: tools_data += (f"{tool['callgraph']['reachable_functions']['count']} & {sound} & {precise} & ")

                pretty_row.append(tool['soundness']['sound'])
                

        tools_data = tools_data[:-3]
        
        #print(data[microbench]['precise_callgraph'])

        f.write("    {} {} & {} & {} & {} & {} & {}\\\\\n".format(
            gray, microbench[:10], 
            challenges, 
            data[microbench]["ground_truth"]["count_funcs"],
            data[microbench]["ground_truth"]["reachable_functions"]["count"],
            data[microbench]["ground_truth"]["count_edges"],
            tools_data
        ))        
        
        benchmark_counter += 1
        rows_data.append([microbench] + pretty_row) 

    f.write("    \\bottomrule\n")
    f.write("\end{tabular}\n")
    f.write("\end{table*}\n")

    return rows_data

def recall_precision_pretty_table(data):
    table = PrettyTable()
    table.title = "Recall of every tool on each library"
    table.field_names = ["Library", "F_total", "F_dyn", 
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
    table.field_names = ["Name", "CBA", "Wassail", "Metadce", "Twiggy", "WAVM"]
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
