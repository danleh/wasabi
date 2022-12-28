(module
  (type $t0 (func (param f32 f32) (result f32)))
  (type $t1 (func (param f32) (result f32)))
  (func $add (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.add)
  (func $sub (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.sub)
  (func $mul (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.mul)
  (func $div (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.div)
  (func $sqrt (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.sqrt)
  (func $min (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.min)
  (func $max (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.max)
  (func $ceil (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.ceil)
  (func $floor (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.floor)
  (func $trunc (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.trunc)
  (func $nearest (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.nearest)
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
