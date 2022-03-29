const Wasabi = require('./node_modules/sql.js/dist/sql-wasm.wasabi.js');

const initSqlJs = require('./node_modules/sql.js/dist/sql-wasm.js');

// Read array of constant addresses from which call_indirects load their function pointer.
const fs = require("fs");
let call_indirect_load_const_addrs = JSON.parse(fs.readFileSync("./call_indirect_load_const_addr.json")); 

// analysis.js 

const calledFuncs = new Set();

function function_info(func_idx) {
    const fct = Wasabi.module.info.functions[func_idx];
    return {
        function_idx: func_idx,
        export: fct.export,
        import: fct.import,
    };
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
        if (call_indirect_load_const_addrs.includes(memarg.addr)) {
            console.error(`${location.func}:${location.instr}: violated assumption, address of function pointer is written to! addr: ${memarg.addr}`);
            console.error(function_info(location.func));
            console.error(op, JSON.stringify(memarg), value);
        }
    },
};

initSqlJs().then(function (SQL) {
    const db = new SQL.Database();

    let sqlstr = "CREATE TABLE hello (a int, b char); \
INSERT INTO hello VALUES (0, 'hello'); \
INSERT INTO hello VALUES (1, 'world');";

    db.run(sqlstr);
    const binaryArray = db.export();

    fs.writeFileSync("db.sqlite", binaryArray)

    //console.log(calledFuncs)
});
