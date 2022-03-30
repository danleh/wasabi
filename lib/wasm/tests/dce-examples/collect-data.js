const fs = require("fs");

console.log(calledFuncs)

if (get_reachable_exports) {
    let reachable_exports = ""; 
    let count = 0; 
    calledFuncs.forEach(func_info => {
        if (func_info != undefined) {
            count+=1; 
            reachable_exports += JSON.parse(func_info).export; 
            reachable_exports += "\n";
        }
    })
    console.log(count+" exported functions are reachable.")
    let f = fs.writeFileSync("reachable-exports.txt", reachable_exports);
} else {
    let lowerbound = ""; 
    let count = 0; 
    calledFuncs.forEach(func_info => {
        if (func_info != undefined) {
            count += 1; 
            lowerbound += JSON.parse(func_info).function_idx; 
            lowerbound += "\n";
        }
    })
    console.log(count+" functions are the lower bound for the analysis.")
    let f = fs.writeFileSync("lowerbound-reachable-functions.txt", lowerbound);
}
