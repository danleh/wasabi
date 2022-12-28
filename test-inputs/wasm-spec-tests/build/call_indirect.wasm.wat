(module
  (type $t0 (func))
  (type $t1 (func (result i32)))
  (func $f0 (type $t0)
    unreachable
    call_indirect $T0 (type $t0))
  (func $f1 (type $t0)
    unreachable
    call_indirect $T0 (type $t0)
    nop)
  (func $f2 (type $t0)
    unreachable
    call_indirect $T0 (type $t0)
    call_indirect $T0 (type $t0))
  (func $f3 (type $t0)
    unreachable
    call_indirect $T0 (type $t0)
    call_indirect $T0 (type $t0))
  (func $f4 (type $t0)
    unreachable
    call_indirect $T0 (type $t0)
    call_indirect $T0 (type $t0)
    call_indirect $T0 (type $t0))
  (func $f5 (type $t0)
    unreachable
    call_indirect $T0 (type $t0))
  (func $f6 (type $t0)
    unreachable
    call_indirect $T0 (type $t0))
  (func $f7 (type $t0)
    unreachable
    call_indirect $T0 (type $t0)
    call_indirect $T0 (type $t0))
  (func $f8 (type $t0)
    unreachable
    call_indirect $T0 (type $t0)
    call_indirect $T0 (type $t0))
  (func $f9 (type $t1) (result i32)
    unreachable
    call_indirect $T0 (type $t0)
    select)
  (func $f10 (type $t1) (result i32)
    unreachable
    call_indirect $T0 (type $t0)
    select
    call_indirect $T0 (type $t0))
  (table $T0 1 funcref))
