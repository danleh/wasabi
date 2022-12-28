(module
  (type $t0 (func (param f32) (result f32)))
  (type $t1 (func (param f32 f32) (result f32)))
  (func $abs (type $t0) (param $p0 f32) (result f32)
    local.get $p0
    f32.abs)
  (func $neg (type $t0) (param $p0 f32) (result f32)
    local.get $p0
    f32.neg)
  (func $copysign (type $t1) (param $p0 f32) (param $p1 f32) (result f32)
    local.get $p0
    local.get $p1
    f32.copysign)
  (export "abs" (func $abs))
  (export "neg" (func $neg))
  (export "copysign" (func $copysign)))
