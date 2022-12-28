(module
  (type $t0 (func (param f64) (result f64)))
  (type $t1 (func (param f64 f64) (result f64)))
  (func $abs (type $t0) (param $p0 f64) (result f64)
    local.get $p0
    f64.abs)
  (func $neg (type $t0) (param $p0 f64) (result f64)
    local.get $p0
    f64.neg)
  (func $copysign (type $t1) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.copysign)
  (export "abs" (func $abs))
  (export "neg" (func $neg))
  (export "copysign" (func $copysign)))
