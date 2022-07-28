
// analysis.js 

global.get_lower_bound = false; 
global.get_reachable_exports = false; 
global.check_if_consts_are_written_to = false; 
global.get_callsite_sensitive_cg = false; 

// TODO getopts for nodejs 
// command line options 
for (const arg of process.argv.slice(2)) {
    console.log(arg)
    if (arg == '--lower-bound') {
        get_lower_bound = true;           
    } else if (arg == '--reachable-exports') {
        get_reachable_exports = true;           
    } else if (arg == "--check-constants") {
        check_if_consts_are_written_to = true; 
    } else if (arg == "--callsite-sensitive-cg") {
        get_callsite_sensitive_cg = true; 
    } else {
    console.log("\
OPTIONS:\n\
      --reachable-exports\t\tto get the exported functions that reachable\n\
      --check-constants\t\t\tto check that the constants in file call_indirect_load_const_addr.json are never written to\n\
      --callsite-sensitive-cg\tget data about callsites of every call\n\
      --lower-bound\t\t\tget the lower bound of all reachable functions\
      ");
      process.exit(0); 
  }
}

// Read array of constant addresses from which call_indirects load their function pointer.
let call_indirect_load_const_addrs;
if (check_if_consts_are_written_to) {
  call_indirect_load_const_addrs = JSON.parse(fs.readFileSync("./call_indirect_load_const_addr.json")); 
}

global.lowerbound_funcs = new Set();
global.called_funcs_exports = new Set();
global.call_sensitive_cg = new Map(); 

function add_to_call_graph (func_idx) {
    //console.log(func_idx)
    let func_info = function_info(func_idx); 
    //console.log(func_info)
    if (get_reachable_exports && func_info.export != null) {
        called_funcs_exports.add(JSON.stringify(func_info)); 
    } 
    if (get_lower_bound) {
        lowerbound_funcs.add(JSON.stringify(func_info)); 
    }
}

function function_info(func_idx) {
    const fct = Wasabi.module.info.functions[func_idx];
    return {
        function_idx: func_idx,
        export: fct.export[0] || null,
        import: fct.import,
    };
}

Wasabi.analysis = {
    call_pre(location, targetFunc, args, indirectTableIdx) {
        //console.log(targetFunc)
        if (get_callsite_sensitive_cg) {
            call_sensitive_cg.set(
                `${location.instr},${location.func}`, 
                `f${targetFunc}`)
        }
        // const caller = function_info(location.func);
        // console.log(`${caller} -> ${callee}`);
        // calledFuncs.add(JSON.stringify(caller));
        // calledFuncs.add(JSON.stringify(callee));
        add_to_call_graph(targetFunc); 
    },

    begin({func, instr}, type) {
        // if we enter the function, add this function to the call graph 
        if (instr == -1) {
            add_to_call_graph(func); 
            //let funcName = function_info(func);
            //calledFuncs.add(JSON.stringify(funcName));
        }
    },
    
    store(location, op, memarg, value) {
        // Ensure that no store goes to these addresses above
        if (check_if_consts_are_written_to && call_indirect_load_const_addrs.includes(memarg.addr)) {
            console.error(`${location.func}:${location.instr}: violated assumption, address of function pointer is written to! addr: ${memarg.addr}`);
            console.error(function_info(location.func));
            console.error(op, JSON.stringify(memarg), value);
        }
    },
    
};

