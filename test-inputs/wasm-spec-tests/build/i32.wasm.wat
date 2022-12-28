(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func (param i32) (result i32)))
  (func $add (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.add)
  (func $sub (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.sub)
  (func $mul (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.mul)
  (func $div_s (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.div_s)
  (func $div_u (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.div_u)
  (func $rem_s (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.rem_s)
  (func $rem_u (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.rem_u)
  (func $and (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.and)
  (func $or (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.or)
  (func $xor (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.xor)
  (func $shl (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.shl)
  (func $shr_s (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.shr_s)
  (func $shr_u (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.shr_u)
  (func $rotl (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.rotl)
  (func $rotr (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.rotr)
  (func $clz (type $t1) (param $p0 i32) (result i32)
    local.get $p0
    i32.clz)
  (func $ctz (type $t1) (param $p0 i32) (result i32)
    local.get $p0
    i32.ctz)
  (func $popcnt (type $t1) (param $p0 i32) (result i32)
    local.get $p0
    i32.popcnt)
  (func $extend8_s (type $t1) (param $p0 i32) (result i32)
    local.get $p0
    i32.extend8_s)
  (func $extend16_s (type $t1) (param $p0 i32) (result i32)
    local.get $p0
    i32.extend16_s)
  (func $eqz (type $t1) (param $p0 i32) (result i32)
    local.get $p0
    i32.eqz)
  (func $eq (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.eq)
  (func $ne (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.ne)
  (func $lt_s (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.lt_s)
  (func $lt_u (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.lt_u)
  (func $le_s (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.le_s)
  (func $le_u (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.le_u)
  (func $gt_s (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.gt_s)
  (func $gt_u (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.gt_u)
  (func $ge_s (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.ge_s)
  (func $ge_u (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    local.get $p1
    i32.ge_u)
  (export "add" (func $add))
  (export "sub" (func $sub))
  (export "mul" (func $mul))
  (export "div_s" (func $div_s))
  (export "div_u" (func $div_u))
  (export "rem_s" (func $rem_s))
  (export "rem_u" (func $rem_u))
  (export "and" (func $and))
  (export "or" (func $or))
  (export "xor" (func $xor))
  (export "shl" (func $shl))
  (export "shr_s" (func $shr_s))
  (export "shr_u" (func $shr_u))
  (export "rotl" (func $rotl))
  (export "rotr" (func $rotr))
  (export "clz" (func $clz))
  (export "ctz" (func $ctz))
  (export "popcnt" (func $popcnt))
  (export "extend8_s" (func $extend8_s))
  (export "extend16_s" (func $extend16_s))
  (export "eqz" (func $eqz))
  (export "eq" (func $eq))
  (export "ne" (func $ne))
  (export "lt_s" (func $lt_s))
  (export "lt_u" (func $lt_u))
  (export "le_s" (func $le_s))
  (export "le_u" (func $le_u))
  (export "gt_s" (func $gt_s))
  (export "gt_u" (func $gt_u))
  (export "ge_s" (func $ge_s))
  (export "ge_u" (func $ge_u)))
