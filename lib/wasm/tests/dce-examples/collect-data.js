const fs = require("fs");

console.log(calledFuncs)

if (get_reachable_exports) {
    let reachable_exports = ""; 
    let count = 0; 
    calledFuncs.forEach(func_info => {
        count+=1; 
        reachable_exports += JSON.parse(func_info).export; 
        reachable_exports += "\n";        
    })
    console.log(count+" exported functions are reachable.")
    fs.writeFileSync("reachable-exports.txt", reachable_exports);
} else if (get_callsite_sensitive_cg) {
    
    let call_site_cg = ""; 
    let count = 0; 
    [...call_sensitive_cg].sort().forEach(callsite => {
        count+=1; 
        call_site_cg += callsite + "\n"; 
    })
    console.log(count+" callsites have been analyzed.")
    fs.writeFileSync("callsite-sensitive-cg.txt", call_site_cg);

} else {
    let lowerbound = ""; 
    let count = 0; 
    calledFuncs.forEach(func_info => {
        count += 1; 
        lowerbound += JSON.parse(func_info).function_idx; 
        lowerbound += "\n";
    })
    console.log(count+" functions are the lower bound for the analysis.")
    fs.writeFileSync("lowerbound-reachable-functions.txt", lowerbound);
}
