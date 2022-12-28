(module
  (type $t0 (func (param f64 f64) (result f64)))
  (type $t1 (func (param f64) (result f64)))
  (func $add (type $t0) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.add)
  (func $sub (type $t0) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.sub)
  (func $mul (type $t0) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.mul)
  (func $div (type $t0) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.div)
  (func $sqrt (type $t1) (param $p0 f64) (result f64)
    local.get $p0
    f64.sqrt)
  (func $min (type $t0) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.min)
  (func $max (type $t0) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.max)
  (func $ceil (type $t1) (param $p0 f64) (result f64)
    local.get $p0
    f64.ceil)
  (func $floor (type $t1) (param $p0 f64) (result f64)
    local.get $p0
    f64.floor)
  (func $trunc (type $t1) (param $p0 f64) (result f64)
    local.get $p0
    f64.trunc)
  (func $nearest (type $t1) (param $p0 f64) (result f64)
    local.get $p0
    f64.nearest)
  (export "add" (func $add))
  (export "sub" (func $sub))
  (export "mul" (func $mul))
  (export "div" (func $div))
  (export "sqrt" (func $sqrt))
  (export "min" (func $min))
  (export "max" (func $max))
  (export "ceil" (func $ceil))
  (export "floor" (func $floor))
  (export "trunc" (func $trunc))
  (export "nearest" (func $nearest)))
