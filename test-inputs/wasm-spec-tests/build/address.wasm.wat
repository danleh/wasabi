(module
  (type $t0 (func (param i32) (result f64)))
  (type $t1 (func (param i32)))
  (func $64_good1 (type $t0) (param $p0 i32) (result f64)
    local.get $p0
    f64.load)
  (func $64_good2 (type $t0) (param $p0 i32) (result f64)
    local.get $p0
    f64.load align=1)
  (func $64_good3 (type $t0) (param $p0 i32) (result f64)
    local.get $p0
    f64.load offset=1 align=1)
  (func $64_good4 (type $t0) (param $p0 i32) (result f64)
    local.get $p0
    f64.load offset=2 align=2)
  (func $64_good5 (type $t0) (param $p0 i32) (result f64)
    local.get $p0
    f64.load offset=18)
  (func $64_bad (type $t1) (param $p0 i32)
    local.get $p0
    f64.load offset=4294967295
    drop)
  (memory $M0 1)
  (export "64_good1" (func $64_good1))
  (export "64_good2" (func $64_good2))
  (export "64_good3" (func $64_good3))
  (export "64_good4" (func $64_good4))
  (export "64_good5" (func $64_good5))
  (export "64_bad" (func $64_bad))
  (data $d0 (i32.const 0) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\f4\7f\01\00\00\00\00\00\fc\7f"))
