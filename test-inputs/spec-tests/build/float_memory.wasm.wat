(module
  (type $t0 (func (result f64)))
  (type $t1 (func (result i64)))
  (type $t2 (func))
  (func $f64.load (type $t0) (result f64)
    i32.const 0
    f64.load)
  (func $i64.load (type $t1) (result i64)
    i32.const 0
    i64.load)
  (func $f64.store (type $t2)
    i32.const 0
    f64.const nan:0xc000000000001 (;=nan;)
    f64.store)
  (func $i64.store (type $t2)
    i32.const 0
    i64.const 9222246136947933185
    i64.store)
  (func $reset (type $t2)
    i32.const 0
    i64.const 0
    i64.store)
  (memory $M0 1 1)
  (export "f64.load" (func $f64.load))
  (export "i64.load" (func $i64.load))
  (export "f64.store" (func $f64.store))
  (export "i64.store" (func $i64.store))
  (export "reset" (func $reset))
  (data $d0 (i32.const 0) "\01\00\00\00\00\00\fc\7f"))
