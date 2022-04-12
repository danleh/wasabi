const fs = require("fs");
global.Wasabi = require('./node_modules/sql.js/dist/sql-wasm.wasabi.js');

let analysis = require('./../../analysis.js');

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

  require("./../../collect-data.js")

});
