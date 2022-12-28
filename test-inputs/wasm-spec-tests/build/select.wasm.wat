(module
  (type $t0 (func (result i32)))
  (type $t1 (func))
  (func $f0 (type $t0) (result i32)
    unreachable
    select)
  (func $f1 (type $t0) (result i32)
    unreachable
    select
    nop)
  (func $f2 (type $t0) (result i32)
    unreachable
    select
    select)
  (func $f3 (type $t0) (result i32)
    unreachable
    select
    select)
  (func $f4 (type $t0) (result i32)
    unreachable
    select
    select
    select)
  (func $f5 (type $t0) (result i32)
    unreachable
    select (result i32))
  (func $f6 (type $t0) (result i32)
    unreachable
    select (result i32))
  (func $f7 (type $t0) (result i32)
    unreachable
    select (result i32)
    select)
  (func $f8 (type $t0) (result i32)
    unreachable
    select (result i32)
    select (result i32))
  (func $f9 (type $t0) (result i32)
    unreachable
    select
    call_indirect $T0 (type $t1))
  (func $f10 (type $t0) (result i32)
    unreachable
    select
    call_indirect $T0 (type $t1)
    select)
  (table $T0 1 funcref))
