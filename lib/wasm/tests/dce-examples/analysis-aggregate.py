from pathlib import Path
import json 

csv = open("analysis-aggregate.csv", "w")

csv.write("wasm-file-analyzed"+", "+\
    "#total-exports"+", "+\
    "#reachable-exports"+", "+\
    "#functions"+", "+\
    "#retained-functions"+", "+\
    "#removed-functions"+", "+\
    "#%removed"+", "+\
    "size-reduction(bytes)"+", "+\
    "%size-reduction"+", "+\
    "highest-expr-for-idx"+", "+\
    "#%highest-expr-fox-idx-percent\n")


for path in Path('../dce-examples').rglob('analysis-stats.json'):
    f = open(path, mode='r')
    data = f.read()
    
    json_obj = json.loads(data)

    file_analyzed = json_obj["wasm-file-analyzed"]
    file_analyzed = file_analyzed[file_analyzed.rindex("/")+1:]

    total_exports = json_obj["#total-exports"]
    reachable_exports = json_obj["#reachable-exports"]
    functions = json_obj[ "#functions"]
    retained_functions = json_obj["#retained-functions"]
    removed_functions = json_obj["#removed-functions"]
    removed_percent = json_obj["#%removed"]
    size_reduction = json_obj["size-reduction(bytes)"]
    size_reduction_percent = json_obj["%size-reduction"]
    highest_expr_for_idx = json_obj["highest-expr-for-idx"]
    highest_expr_for_idx_percent = json_obj["#%highest-expr-fox-idx-percent"]

    csv.write(str(file_analyzed) + ", " + \
        str(total_exports) + ", " + \
        str(reachable_exports) + ", " + \
        str(functions) + ", " + \
        str(retained_functions) + ", " + \
        str(removed_functions) + ", " + \
        str(removed_percent) + ", " + \
        str(size_reduction) + ", " + \
        str(size_reduction_percent) + ", " + \
        str(highest_expr_for_idx) + ", " + \
        str(highest_expr_for_idx_percent) + "\n")

    f.close()

csv.close()