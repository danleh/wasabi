const fs = require("fs");
const Wasabi = require('./node_modules/sql.js/dist/sql-wasm.wasabi.js');

// analysis.js 

let get_reachable_exports = false; 
let check_if_consts_are_written_to = false; 
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

const calledFuncs = new Set();

function function_info(func_idx) {
    const fct = Wasabi.module.info.functions[func_idx];
    if (get_reachable_exports && fct.export[0] !== undefined) {
        return {
            function_idx: func_idx,
            export: fct.export,
            import: fct.import,
        };
    } else if (!get_reachable_exports) {
        return {
            function_idx: func_idx,
            export: fct.export,
            import: fct.import,
        };
    }
}

Wasabi.analysis = {
    call_pre(location, targetFunc, args, indirectTableIdx) {
        // const caller = function_info(location.func);
        const callee = function_info(targetFunc);
        // console.log(`${caller} -> ${callee}`);
        // calledFuncs.add(JSON.stringify(caller));
        calledFuncs.add(JSON.stringify(callee));
    },

    begin({func, instr}, type) {
        if (instr == -1) {
            let funcName = function_info(func);
            calledFuncs.add(JSON.stringify(funcName));
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

const initSqlJs = require('./node_modules/sql.js/dist/sql-wasm.js');
const { exit } = require("process");

const filebuffer = fs.readFileSync('db.sqlite');

initSqlJs().then(function(SQL){

  const db = new SQL.Database(filebuffer);

  // create new table 
  let sqlstr = "CREATE TABLE new_student (id int, name char, age int); \
INSERT INTO new_student VALUES (100, 'michelle', 34); \
INSERT INTO new_student VALUES (101, 'daniel', 42); \
INSERT INTO new_student VALUES (102, 'rachel', 23);";

  db.run(sqlstr);

  //print things  
  var stmt = db.prepare("SELECT * FROM HELLO;");
  stmt.step(); 
  console.log(stmt.getAsObject());   
  
  var stmt = db.prepare("SELECT * FROM new_student;");
  stmt.step(); 
  console.log(stmt.getAsObject()); 
  
  // insert into existing table
  let sqlstr_ = "INSERT INTO hello VALUES (10, 'hii'); \
  INSERT INTO hello VALUES (11, 'hey'); ";
  db.run(sqlstr_);
  
  const binaryArray = db.export();
  
  fs.writeFileSync("db_new.sqlite", binaryArray)

  if (get_reachable_exports) {
    let reachable_exports = ""; 
    let count = 0; 
    calledFuncs.forEach(func_info => {
        if (func_info != undefined) {
            count+=1; 
            reachable_exports += JSON.parse(func_info).export[0]; 
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

});
