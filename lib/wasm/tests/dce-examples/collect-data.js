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
    
    console.log(count+" exported functions are reachable. Exported functions in reachable-exports.txt")
    fs.writeFileSync("reachable-exports.txt", reachable_exports);
} 

if (get_callsite_sensitive_cg) {
    
    let sorted_line_nums = [...call_sensitive_cg.keys()].sort(function(a, b) {
        a = a.split(",")
        b = b.split(",")
        if (a[0] === b[0]) {
            return a[1] - b[1]
        } else {
            return a[0] - b[0]
        }

    })
    
    let callsite_cg = ""; 
    let count = 0; 
    
    sorted_line_nums.forEach(key => {
        count += 1
        let callsite = call_sensitive_cg.get(key)
        callsite_cg += key.split(",")[0]+":"+callsite+"\n"
    })
    
    console.log(count+" callsites have been analyzed. Callsite info in callsite_cg_dynamic.txt")
    fs.writeFileSync("callsite_cg_dynamic.txt", callsite_cg);

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
    
    console.log(count+" functions are the lower bound for the analysis. Lowerbound of reachable functions in lowerbound-reachable-functions.txt")
    fs.writeFileSync("lowerbound-reachable-functions.txt", lowerbound);
}
