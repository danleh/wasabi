
// analysis.js 

global.get_reachable_exports = false; 
global.check_if_consts_are_written_to = false; 

for (const arg of process.argv.slice(2)) {
    if (arg == '--reachable-exports') {
        get_reachable_exports = true;           
    } else if (arg == "--check-constants") {
        check_if_consts_are_written_to = true; 
    } else if (arg == "--help") {
      console.log("\
OPTIONS:\n\
      --reachable-exports\tto get the exported functions that reachable\n\
      --check-constants\t\tto check that the constants in file call_indirect_load_const_addr.json are never written to\
      "); 
      process.exit(0);
  } else {
    console.log("\
OPTIONS:\n\
      --reachable-exports\tto get the exported functions that reachable\n\
      --check-constants\t\tto check that the constants in file call_indirect_load_const_addr.json are never written to\
      ");
      process.exit(0); 
  }
}

// Read array of constant addresses from which call_indirects load their function pointer.
let call_indirect_load_const_addrs;
if (check_if_consts_are_written_to) {
  call_indirect_load_const_addrs = JSON.parse(fs.readFileSync("./call_indirect_load_const_addr.json")); 
}

global.calledFuncs = new Set();

function add_to_call_graph (func_idx) {
    //console.log(func_idx)
    let func_info = function_info(func_idx); 
    //console.log(func_info)
    if (get_reachable_exports && func_info.export != null) {
        calledFuncs.add(JSON.stringify(func_info)); 
    } else if (!get_reachable_exports) {
        calledFuncs.add(JSON.stringify(func_info)); 
    }
}

function function_info(func_idx) {
    const fct = Wasabi.module.info.functions[func_idx];
    if (fct.export[0] != undefined) {
        return {
            function_idx: func_idx,
            export: fct.export[0],
            import: fct.import,
        };
    } 
    return {
        function_idx: func_idx,
        export: null,
        import: null,
    };  
}

Wasabi.analysis = {
    call_pre(location, targetFunc, args, indirectTableIdx) {
        // const caller = function_info(location.func);
        /// const callee = function_info(targetFunc);
        // console.log(`${caller} -> ${callee}`);
        // calledFuncs.add(JSON.stringify(caller));
        /// calledFuncs.add(JSON.stringify(callee));
        add_to_call_graph(location.func); 
    },

    begin({func, instr}, type) {
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

