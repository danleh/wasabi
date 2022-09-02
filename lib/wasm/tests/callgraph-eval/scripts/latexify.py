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

    f.write("\setlength{\extrarowheight}{0.2em}\n")
    
    f.write("\\begin{table*}[t]\n")
    f.write("\\small\n")
    f.write("\centering\n")
    f.write("\captionsetup{justification=centering}\n")

    f.write("\setlength{\\tabcolsep}{2pt}")

    f.write("\\begin{tabular}{l|r|r|\n")
    f.write("    rr@{~}rr@{~}r|\n")
    f.write("    rr@{~}rr@{~}r|\n")
    f.write("    rr@{~}rr@{~}r|\n")
    f.write("    rr@{~}rr@{~}r}\n")
    f.write("    \\toprule\n")

    f.write("    \multirow{2}{*}{\\textbf{Library}} & \multirow{2}{*}{$|F_{\\text{all}}|$} & \multirow{2}{*}{$|F_{\\text{dyn}}|$} &\n")
    f.write("    \multicolumn{5}{c|}{\\textbf{Wassail}} &\n")
    f.write("    \multicolumn{5}{c|}{\\textbf{WAVM+LLVM}} & \n")
    f.write("    \multicolumn{5}{c|}{\\textbf{MetaDCE}} & \n")
    f.write("    \multicolumn{5}{c}{\\textbf{Twiggy}}\\\\\n")

    f.write("    & & & \n")
    f.write("    $|F_\\text{r}|$ & \multicolumn{2}{c}{$|F_{\\text{unsound}}|$} & \multicolumn{2}{c|}{$|F_{\\text{del}}|$} & \n")
    f.write("    $|F_\\text{r}|$ & \multicolumn{2}{c}{$|F_{\\text{unsound}}|$} & \multicolumn{2}{c|}{$|F_{\\text{del}}|$} & \n")
    f.write("    $|F_\\text{r}|$ & \multicolumn{2}{c}{$|F_{\\text{unsound}}|$} & \multicolumn{2}{c|}{$|F_{\\text{del}}|$} & \n")
    f.write("    $|F_\\text{r}|$ & \multicolumn{2}{c}{$|F_{\\text{unsound}}|$} & \multicolumn{2}{c} {$|F_{\\text{del}}|$} \\\\\n")

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
                print(None)
                #tool_recall_data += "& & & "
                tool_recall_data += "\multicolumn{5}{c|}{Crash} & "
                tool_ascii_data.append("DNE")
            else:
                missing_funcs = tool['callgraph']['missing_functions']['count']
                missing_percent = "({:.0f}\%)".format(tool['callgraph']['missing_functions']['percent'])

                if tool['callgraph']['missing_functions']['count'] > 0: 
                    missing_funcs = make_bold_red(missing_funcs)
                    missing_percent = make_bold_red(missing_percent)
                    
                tool_recall_data += (f"{tool['callgraph']['reachable_functions']['count']} & "
                                     f"{missing_funcs} & "
                                     f"{missing_percent} &"
                                     f"{tool['callgraph']['removed_functions']['count']} & "
                                     f"({tool['callgraph']['removed_functions']['percent']:0.0f}\%) & ")
                
                #tool_percent_data += "{} & {} & ({:.0f}\%) & ".format(
                #    multirow(-2, tool['callgraph']['reachable_functions']['count']),
                #    missing_percent, 
                #    tool['callgraph']['removed_functions']['percent'])
                
                tool_ascii_data.append("{:.2f}".format(tool["recall"]))

        tool_recall_data = tool_recall_data[:-3]
        #tool_percent_data = tool_percent_data[:-3]

        gray = ""
        if counter%2 != 0: gray = "\\rowcolor{gray!20}" 
        counter += 1
        #f.write("    {} & & & {}\\\\\n".format(gray, tool_recall_data))
        f.write("    {} {} & {} & {} & {}\\\\\n".format(
            gray, 
            lib["pretty_name"], 
            lib_total, 
            lib_dyn,
            tool_recall_data))
        table_rows.append([lib["pretty_name"], lib_total, lib_dyn] + tool_ascii_data)
    
    f.write("    \\bottomrule\n")
    f.write("\end{tabular}\n")
    f.write("\caption{Comparison of the reachable functions reported by each tool.\\\\The number of missing functions ($F_{\\text{unsound}} = F_{\\text{dyn}}-F_{\\text{r}}$) and removed functions ($F_{\\text{del}} = F_{\\text{all}} - F_{\\text{r}}$) are also reported.}\n")
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
    f.write("\\newcommand{\BFall}{{$|\\textbf{F}_{\\textbf{all}}|$}}\n")
    f.write("\\newcommand{\BFreach}{{$|\\textbf{F}_{\\textbf{r}}|$}}\n")
    f.write("\\newcommand{\BE}{{$|\\textbf{E}|$}}\n")
    f.write("\\newcommand{\Fall}{{$|\\text{F}_{\\text{all}}|$} }\n")
    f.write("\\newcommand{\Freach}{{$|\\text{F}_{\\text{r}}|$} }\n")
    f.write("\\newcommand{\E}{{$|\\text{E}|$} }\n")
    
    f.write("\\begin{table*}[t]\n")
    
    f.write("\captionsetup{justification=centering}\n")
    f.write("\caption{\n")
    f.write("    Overview of the microbenchmarks and results of different call graph analyses on them.\n")
    f.write("    \Fall is the total number of functions in the binary.\n")
    f.write("    \Freach is the number of functions reachable from the given entry point(s).\n")
    f.write("    \E is the number of edges in the call graph.\n")
    f.write("    S indicates if the analysis is sound and P indicates if the analysis is precise, compared to the ground truth.\n")    
    f.write("}\n")

    f.write("\label{tab:microbenchmarks}\n")
    f.write("\small\n")
    f.write("\centering\n")
    f.write("\captionsetup{justification=centering}\n")
    f.write("\setlength{\\tabcolsep}{2pt}\n")
    f.write("\\begin{tabular}{@{}r|p{19em}|p{5.6em}|rrr|rrrr|rrrr|rrr|rrr@{}}\n")
    f.write("    \\toprule\n")
    f.write("    \multirow{2}{*}{\\textbf{\#}} & \n")
    f.write("    \multirow{2}{*}{\\textbf{Description}} & \n")
    f.write("    \multirow{2}{*}{\\textbf{Challenges}} & \n")
    f.write("    \multicolumn{3}{@{}c|@{}}{\\textbf{Ground Truth}} & \n")
    f.write("    \multicolumn{4}{@{}c|@{}}{\\textbf{Wassail}} &\n")
    f.write("    \multicolumn{4}{@{}c|@{}}{\\textbf{WAVM+LLVM opt}} &\n")
    f.write("    \multicolumn{3}{@{}c|@{}}{\\textbf{Metadce}} & \n")
    f.write("    \multicolumn{3}{@{}c@{}}{\\textbf{Twiggy}} \\\\\n")
    f.write("    & & & \n")
    f.write("    \BFall & \BFreach & \BE &\n")
    f.write("    \BFreach & \BE & \\textbf{S} & \\textbf{P} & \n")
    f.write("    \BFreach & \BE & \\textbf{S} & \\textbf{P} & \n")
    f.write("    \BFreach & \\textbf{S} & \\textbf{P} &\n")
    f.write("    \BFreach & \\textbf{S} & \\textbf{P} \\\\\n")
    f.write("    \midrule\n")
    
    benchmark_counter = 0
    rows_data = []
    summary = {
        "ground_truth": { "f_all": 0, "f_r": 0, "e": 0}, 
        "wassail"     : { "f_r": 0,   "e": 0, "s": 0, "p": 0},
        "metadce"     : { "f_r": 0,   "s": 0, "p": 0},
        "twiggy"      : { "f_r": 0,   "s": 0, "p": 0},
        "wavm"        : { "f_r": 0,   "e": 0, "s": 0, "p": 0},
    }
    total_tests = len(data.keys())
    for microbench in data: 
        gray = "" 
        if benchmark_counter%2 != 0: gray = "\\rowcolor{gray!15}"
        
        microbench_description = data[microbench]["description"]
        #for word in data[microbench]["description"].split(" "):

        microbench_challenges = ""
        if data[microbench]["challenges"] != None:
            for challenge in data[microbench]["challenges"]:
                microbench_challenges += "\challenge{" + f"{challenge}" +"}, "
            microbench_challenges = microbench_challenges[:-2]

        tools_data = ""
        pretty_row = []

        summary["ground_truth"]["f_all"] +=  data[microbench]["ground_truth"]["count_funcs"]
        summary["ground_truth"]["f_r"] += data[microbench]["ground_truth"]["reachable_functions"]["count"]
        summary["ground_truth"]["e"] += data[microbench]["ground_truth"]["count_edges"]

        for tool in data[microbench]["tools"]:
            
            if 'ourtool' in tool['name']: 
                pretty_row.append(tool['soundness']['sound'])
                continue 

            if tool["callgraph"] == None:
                dne = "Crash"
                if "wassail" in tool['name'] or "wavm" in tool['name']: tools_data += "\multicolumn{4}{c|}{"+dne+"} & "
                elif "twiggy" in tool['name']: 
                    tools_data += "\multicolumn{3}{c}{"+dne+"} & "
                else: tools_data += "\multicolumn{3}{c|}{"+dne+"} & "
                pretty_row.append("-")

            else:
                sound = "\\xmark"
                precise = "\\xmark"

                if tool['soundness']['sound']: sound = "\cmark"; summary[tool["name"]]["s"] += 1
                if tool['precision']['precise']: precise = "\cmark"; summary[tool["name"]]["p"] += 1
                
                summary[tool["name"]]["f_r"] += tool['callgraph']['reachable_functions']['count']
                
                
                if "wassail" in tool['name'] or "wavm" in tool['name']: 
                    tools_data += (f"{tool['callgraph']['reachable_functions']['count']} & {tool['callgraph']['count_edges']} & {sound} & {precise} & ")
                    summary[tool["name"]]["e"] += tool['callgraph']['count_edges']                    
                else: tools_data += (f"{tool['callgraph']['reachable_functions']['count']} & {sound} & {precise} & ")

                pretty_row.append(tool['soundness']['sound'])
                

        tools_data = tools_data[:-3]
        
        #print(data[microbench]['precise_callgraph'])

        f.write("    {} {} & {} & {} & {} & {} & {} & {}\\\\\n".format(
            gray, data[microbench]["id"],
            microbench_description, 
            microbench_challenges,
            data[microbench]["ground_truth"]["count_funcs"],
            data[microbench]["ground_truth"]["reachable_functions"]["count"],
            data[microbench]["ground_truth"]["count_edges"],
            tools_data
        ))        
        
        benchmark_counter += 1
        rows_data.append([microbench] + pretty_row) 

    f.write("    \\midrule\n")
    print(summary)
    summary_line = ("    \multicolumn{3}{l|}{\\textit{Summary}} & "
                   f"{summary['ground_truth']['f_all']} & {summary['ground_truth']['f_r']} & {summary['ground_truth']['e']} & "
                   f"{summary['wassail']['f_r']} & {summary['wassail']['e']} & {summary['wassail']['s']} & {summary['wassail']['p']} &"
                   f"{summary['wavm']['f_r']} & {summary['wavm']['e']} & {summary['wavm']['s']} & {summary['wavm']['p']} &"
                   f"{summary['metadce']['f_r']} & {summary['metadce']['s']} & {summary['metadce']['p']} & "
                   f"{summary['twiggy']['f_r']} & {summary['twiggy']['s']} & {summary['twiggy']['p']} \\\\\n"
                   )
    f.write(summary_line)

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

    #with open(LATEX_MICRO_EVAL_TABLE, "w") as f_tab:
    #    row_data = micro_eval_latex_table(f_tab, micro_data)
    #    micro_eval_pretty_table(row_data)
    #    print("\n")
    
    #with open(LATEX_RECALL_TABLE, "w") as f_tab:
    #    row_data = recall_precision_latex_table(f_tab, real_data)        
    #    recall_precision_pretty_table(row_data)
    #    print("\n")

    #with open(LATEX_COVERAGE_TABLE, "w") as f_tab:
    #    row_data = coverage_latex_table(f_tab, real_data)
    #    coverage_pretty_table(row_data)
    
    print("The LaTeX tables can be found in the paper repo.")
    
if __name__ == "__main__":
    main()
