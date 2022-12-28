(module
  (type $t0 (func (param f32 f32) (result f32)))
  (type $t1 (func (param f32) (result f32)))
  (type $t2 (func (param f64 f64) (result f64)))
  (type $t3 (func (param f64) (result f64)))
  (func $f32.add (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.add)
  (func $f32.sub (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.sub)
  (func $f32.mul (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.mul)
  (func $f32.div (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.div)
  (func $f32.sqrt (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.sqrt)
  (func $f32.abs (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.abs)
  (func $f32.neg (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.neg)
  (func $f32.copysign (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.copysign)
  (func $f32.ceil (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.ceil)
  (func $f32.floor (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.floor)
  (func $f32.trunc (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.trunc)
  (func $f32.nearest (type $t1) (param $p0 f32) (result f32)
    local.get $p0
    f32.nearest)
  (func $f32.min (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.min)
  (func $f32.max (type $t0) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.max)
  (func $f64.add (type $t2) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.add)
  (func $f64.sub (type $t2) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.sub)
  (func $f64.mul (type $t2) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.mul)
  (func $f64.div (type $t2) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.div)
  (func $f64.sqrt (type $t3) (param $p0 f64) (result f64)
    local.get $p0
    f64.sqrt)
  (func $f64.abs (type $t3) (param $p0 f64) (result f64)
    local.get $p0
    f64.abs)
  (func $f64.neg (type $t3) (param $p0 f64) (result f64)
    local.get $p0
    f64.neg)
  (func $f64.copysign (type $t2) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.copysign)
  (func $f64.ceil (type $t3) (param $p0 f64) (result f64)
    local.get $p0
    f64.ceil)
  (func $f64.floor (type $t3) (param $p0 f64) (result f64)
    local.get $p0
    f64.floor)
  (func $f64.trunc (type $t3) (param $p0 f64) (result f64)
    local.get $p0
    f64.trunc)
  (func $f64.nearest (type $t3) (param $p0 f64) (result f64)
    local.get $p0
    f64.nearest)
  (func $f64.min (type $t2) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.min)
  (func $f64.max (type $t2) (param $p0 f64) (param $p1 f64) (result f64)
    local.get $p0
    local.get $p1
    f64.max)
  (export "f32.add" (func $f32.add))
  (export "f32.sub" (func $f32.sub))
  (export "f32.mul" (func $f32.mul))
  (export "f32.div" (func $f32.div))
  (export "f32.sqrt" (func $f32.sqrt))
  (export "f32.abs" (func $f32.abs))
  (export "f32.neg" (func $f32.neg))
  (export "f32.copysign" (func $f32.copysign))
  (export "f32.ceil" (func $f32.ceil))
  (export "f32.floor" (func $f32.floor))
  (export "f32.trunc" (func $f32.trunc))
  (export "f32.nearest" (func $f32.nearest))
  (export "f32.min" (func $f32.min))
  (export "f32.max" (func $f32.max))
  (export "f64.add" (func $f64.add))
  (export "f64.sub" (func $f64.sub))
  (export "f64.mul" (func $f64.mul))
  (export "f64.div" (func $f64.div))
  (export "f64.sqrt" (func $f64.sqrt))
  (export "f64.abs" (func $f64.abs))
  (export "f64.neg" (func $f64.neg))
  (export "f64.copysign" (func $f64.copysign))
  (export "f64.ceil" (func $f64.ceil))
  (export "f64.floor" (func $f64.floor))
  (export "f64.trunc" (func $f64.trunc))
  (export "f64.nearest" (func $f64.nearest))
  (export "f64.min" (func $f64.min))
  (export "f64.max" (func $f64.max)))
