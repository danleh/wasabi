(module
  (type $t0 (func (param f32 f32) (result f32)))
  (type $t1 (func (param f64 f64) (result f64)))
  (func $f32.no_fold_conditional_inc (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p0
    f32.const 0x1p+0 (;=1;)
    f32.add
    local.get $p1
    f32.const 0x0p+0 (;=0;)
    f32.lt
    select)
  (func $f64.no_fold_conditional_inc (type $t1) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p0
    f64.const 0x1p+0 (;=1;)
    f64.add
    local.get $p1
    f64.const 0x0p+0 (;=0;)
    f64.lt
    select)
  (export "f32.no_fold_conditional_inc" (func $f32.no_fold_conditional_inc))
  (export "f64.no_fold_conditional_inc" (func $f64.no_fold_conditional_inc)))
