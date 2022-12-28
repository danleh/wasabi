(module
  (type $t0 (func (param i32) (result i32)))
  (type $t1 (func (param i64) (result i64)))
  (func $i32.no_fold_div_neg1 (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.const -1
    i32.div_s)
  (func $i64.no_fold_div_neg1 (type $t1) (param $p0 i64) (result i64)
    local.get $p0
    i64.const -1
    i64.div_s)
  (export "i32.no_fold_div_neg1" (func $i32.no_fold_div_neg1))
  (export "i64.no_fold_div_neg1" (func $i64.no_fold_div_neg1)))
