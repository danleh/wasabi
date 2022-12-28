(module
  (type $t0 (func))
  (type $t1 (func))
  (type $t2 (func (param i64 i64 f64 i64 f64 i64 f32 i32)))
  (type $t3 (func (param f64 i64 f64 i64 f64 i64 f32 i32)))
  (func $f0 (type $t0))
  (func $f1 (type $t3) (param $p0 f64) (param $p1 i64) (param $p2 f64) (param $p3 i64) (param $p4 f64) (param $p5 i64) (param $p6 f32) (param $p7 i32))
  (func $f2 (type $t0))
  (func $f3 (type $t3) (param $p0 f64) (param $p1 i64) (param $p2 f64) (param $p3 i64) (param $p4 f64) (param $p5 i64) (param $p6 f32) (param $p7 i32))
  (func $f4 (type $t3) (param $p0 f64) (param $p1 i64) (param $p2 f64) (param $p3 i64) (param $p4 f64) (param $p5 i64) (param $p6 f32) (param $p7 i32))
  (func $f5 (type $t2) (param $p0 i64) (param $p1 i64) (param $p2 f64) (param $p3 i64) (param $p4 f64) (param $p5 i64) (param $p6 f32) (param $p7 i32))
  (func $f6 (type $t2) (param $p0 i64) (param $p1 i64) (param $p2 f64) (param $p3 i64) (param $p4 f64) (param $p5 i64) (param $p6 f32) (param $p7 i32))
  (func $signature-explicit-reused (type $t0)
    i32.const 1
    call_indirect $T0 (type $t0)
    i32.const 4
    call_indirect $T0 (type $t0))
  (func $signature-implicit-reused (type $t0)
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 0
    call_indirect $T0 (type $t3)
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 2
    call_indirect $T0 (type $t3)
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 3
    call_indirect $T0 (type $t3))
  (func $signature-explicit-duplicate (type $t0)
    i32.const 1
    call_indirect $T0 (type $t1))
  (func $signature-implicit-duplicate (type $t0)
    i64.const 0
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 5
    call_indirect $T0 (type $t2)
    i64.const 0
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 6
    call_indirect $T0 (type $t2))
  (table $T0 7 7 funcref)
  (export "signature-explicit-reused" (func $signature-explicit-reused))
  (export "signature-implicit-reused" (func $signature-implicit-reused))
  (export "signature-explicit-duplicate" (func $signature-explicit-duplicate))
  (export "signature-implicit-duplicate" (func $signature-implicit-duplicate))
  (elem $e0 (i32.const 0) func $f4 $f2 $f1 $f4 $f0 $f5 $f6))
