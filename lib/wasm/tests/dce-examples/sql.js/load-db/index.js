const fs = require("fs");
global.Wasabi = require('./node_modules/sql.js/dist/sql-wasm.wasabi.js');

require('./../../analysis.js');

const initSqlJs = require('./node_modules/sql.js/dist/sql-wasm.js');

initSqlJs().then(function (SQL) {
    const db = new SQL.Database();

    let sqlstr = "CREATE TABLE hello (a int, b char); \
INSERT INTO hello VALUES (0, 'hello'); \
INSERT INTO hello VALUES (1, 'world');";

    db.run(sqlstr);
    const binaryArray = db.export();

    fs.writeFileSync("db.sqlite", binaryArray)

    require("./../../collect-data.js")
    
});
