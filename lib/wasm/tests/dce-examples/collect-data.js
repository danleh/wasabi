const fs = require("fs");


if (get_reachable_exports) {
    
    console.log(called_funcs_exports)
    
    let reachable_exports = ""; 
    let count = 0; 
    
    called_funcs_exports.forEach(func_info => {
        count+=1; 
        reachable_exports += JSON.parse(func_info).export; 
        reachable_exports += "\n";        
    })
    
    console.log(count+" exported functions are reachable. Exported functions in analysis_data/reachable-exports.txt")
    fs.writeFileSync("./analysis_data/reachable-exports.txt", reachable_exports);
} 

if (get_callsite_sensitive_cg) {
    
    let sorted_line_nums = [...call_sensitive_cg.keys()].sort(function(a, b) {
        a = a.split(",")
        b = b.split(",")
        if (a[1] === b[1]) {
            return a[0] - b[0]
        } else {
            return a[1] - b[1]
        }

    })
    
    let callsite_cg = ""; 
    let count = 0; 
    
    sorted_line_nums.forEach(key => {
        count += 1
        let instr_loc = key.split(",")[0]
        let src = key.split(",")[1] 
        let target = call_sensitive_cg.get(key)
        callsite_cg += `f`+src+":"+instr_loc+" -> "+target+"\n"
    })
    
    console.log(count+" callsites have been analyzed. Callsite info in analysis_data/callsite_cg_dynamic.txt")
    fs.writeFileSync("./analysis_data/callsite_cg_dynamic.txt", callsite_cg);

} 

if (get_lower_bound) {
    
    console.log(lowerbound_funcs)
    
    let lowerbound = ""; 
    let count = 0; 
    lowerbound_funcs.forEach(func_info => {
        count += 1; 
        lowerbound += JSON.parse(func_info).function_idx; 
        lowerbound += "\n";
    })
    
    console.log(count+" functions are the lower bound for the analysis. Lowerbound of reachable functions in analysis_data/lowerbound-reachable-functions.txt")
    fs.writeFileSync("./analysis_data/lowerbound-reachable-functions.txt", lowerbound);
}