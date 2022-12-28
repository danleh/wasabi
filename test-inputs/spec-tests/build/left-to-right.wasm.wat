(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func (param i64 i64) (result i32)))
  (type $t2 (func (param f32 f32) (result i32)))
  (type $t3 (func (param f64 f64) (result i32)))
  (type $t4 (func))
  (type $t5 (func (result i32)))
  (type $t6 (func (result i64)))
  (type $t7 (func (result f32)))
  (type $t8 (func (result f64)))
  (type $t9 (func (param i32 i32)))
  (type $t10 (func (param i64 i64)))
  (type $t11 (func (param f32 f32)))
  (type $t12 (func (param f64 f64)))
  (func $f0 (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    i32.const -1)
  (func $f1 (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    i32.const -2)
  (func $f2 (type $t1) (param $p0 i64) (param $p1 i64) (result i32)
    i32.const -1)
  (func $f3 (type $t1) (param $p0 i64) (param $p1 i64) (result i32)
    i32.const -2)
  (func $f4 (type $t2) (param $p0 f32) (param $p1 f32) (result i32)
    i32.const -1)
  (func $f5 (type $t2) (param $p0 f32) (param $p1 f32) (result i32)
    i32.const -2)
  (func $f6 (type $t3) (param $p0 f64) (param $p1 f64) (result i32)
    i32.const -1)
  (func $f7 (type $t3) (param $p0 f64) (param $p1 f64) (result i32)
    i32.const -2)
  (func $f8 (type $t4)
    i32.const 8
    i32.const 0
    i32.store)
  (func $f9 (type $t4)
    i32.const 11
    i32.const 10
    i32.load8_u
    i32.store8
    i32.const 10
    i32.const 9
    i32.load8_u
    i32.store8
    i32.const 9
    i32.const 8
    i32.load8_u
    i32.store8
    i32.const 8
    i32.const -3
    i32.store8)
  (func $f10 (type $t5) (result i32)
    i32.const 8
    i32.load)
  (func $f11 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 1
    i32.store8
    i32.const 0)
  (func $f12 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 2
    i32.store8
    i32.const 1)
  (func $f13 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 3
    i32.store8
    i32.const 1)
  (func $f14 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 4
    i32.store8
    i32.const 0)
  (func $f15 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 5
    i32.store8
    i32.const 0)
  (func $f16 (type $t6) (result i64)
    call $f9
    i32.const 8
    i32.const 1
    i32.store8
    i64.const 0)
  (func $f17 (type $t6) (result i64)
    call $f9
    i32.const 8
    i32.const 2
    i32.store8
    i64.const 1)
  (func $f18 (type $t6) (result i64)
    call $f9
    i32.const 8
    i32.const 3
    i32.store8
    i64.const 1)
  (func $f19 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 4
    i32.store8
    i32.const 2)
  (func $f20 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 5
    i32.store8
    i32.const 0)
  (func $f21 (type $t7) (result f32)
    call $f9
    i32.const 8
    i32.const 1
    i32.store8
    f32.const 0x0p+0 (;=0;))
  (func $f22 (type $t7) (result f32)
    call $f9
    i32.const 8
    i32.const 2
    i32.store8
    f32.const 0x1p+0 (;=1;))
  (func $f23 (type $t7) (result f32)
    call $f9
    i32.const 8
    i32.const 3
    i32.store8
    f32.const 0x1p+0 (;=1;))
  (func $f24 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 4
    i32.store8
    i32.const 4)
  (func $f25 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 5
    i32.store8
    i32.const 0)
  (func $f26 (type $t8) (result f64)
    call $f9
    i32.const 8
    i32.const 1
    i32.store8
    f64.const 0x0p+0 (;=0;))
  (func $f27 (type $t8) (result f64)
    call $f9
    i32.const 8
    i32.const 2
    i32.store8
    f64.const 0x1p+0 (;=1;))
  (func $f28 (type $t8) (result f64)
    call $f9
    i32.const 8
    i32.const 3
    i32.store8
    f64.const 0x1p+0 (;=1;))
  (func $f29 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 4
    i32.store8
    i32.const 6)
  (func $f30 (type $t5) (result i32)
    call $f9
    i32.const 8
    i32.const 5
    i32.store8
    i32.const 0)
  (func $f31 (type $t9) (param $p0 i32) (param $p1 i32))
  (func $f32 (type $t10) (param $p0 i64) (param $p1 i64))
  (func $f33 (type $t11) (param $p0 f32) (param $p1 f32))
  (func $f34 (type $t12) (param $p0 f64) (param $p1 f64))
  (func $i32_add (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.add
    drop
    call $f10)
  (func $i32_sub (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.sub
    drop
    call $f10)
  (func $i32_mul (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.mul
    drop
    call $f10)
  (func $i32_div_s (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.div_s
    drop
    call $f10)
  (func $i32_div_u (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.div_u
    drop
    call $f10)
  (func $i32_rem_s (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.rem_s
    drop
    call $f10)
  (func $i32_rem_u (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.rem_u
    drop
    call $f10)
  (func $i32_and (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.and
    drop
    call $f10)
  (func $i32_or (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.or
    drop
    call $f10)
  (func $i32_xor (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.xor
    drop
    call $f10)
  (func $i32_shl (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.shl
    drop
    call $f10)
  (func $i32_shr_u (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.shr_u
    drop
    call $f10)
  (func $i32_shr_s (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.shr_s
    drop
    call $f10)
  (func $i32_eq (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.eq
    drop
    call $f10)
  (func $i32_ne (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.ne
    drop
    call $f10)
  (func $i32_lt_s (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.lt_s
    drop
    call $f10)
  (func $i32_le_s (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.le_s
    drop
    call $f10)
  (func $i32_lt_u (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.lt_u
    drop
    call $f10)
  (func $i32_le_u (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.le_u
    drop
    call $f10)
  (func $i32_gt_s (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.gt_s
    drop
    call $f10)
  (func $i32_ge_s (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.ge_s
    drop
    call $f10)
  (func $i32_gt_u (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.gt_u
    drop
    call $f10)
  (func $i32_ge_u (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.ge_u
    drop
    call $f10)
  (func $i32_store (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.store
    call $f10)
  (func $i32_store8 (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.store8
    call $f10)
  (func $i32_store16 (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    i32.store16
    call $f10)
  (func $i32_call (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    call $f31
    call $f10)
  (func $i32_call_indirect (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    call $f14
    call_indirect $T0 (type $t0)
    drop
    call $f10)
  (func $i32_select (type $t5) (result i32)
    call $f8
    call $f11
    call $f12
    call $f15
    select
    drop
    call $f10)
  (func $i64_add (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.add
    drop
    call $f10)
  (func $i64_sub (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.sub
    drop
    call $f10)
  (func $i64_mul (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.mul
    drop
    call $f10)
  (func $i64_div_s (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.div_s
    drop
    call $f10)
  (func $i64_div_u (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.div_u
    drop
    call $f10)
  (func $i64_rem_s (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.rem_s
    drop
    call $f10)
  (func $i64_rem_u (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.rem_u
    drop
    call $f10)
  (func $i64_and (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.and
    drop
    call $f10)
  (func $i64_or (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.or
    drop
    call $f10)
  (func $i64_xor (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.xor
    drop
    call $f10)
  (func $i64_shl (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.shl
    drop
    call $f10)
  (func $i64_shr_u (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.shr_u
    drop
    call $f10)
  (func $i64_shr_s (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.shr_s
    drop
    call $f10)
  (func $i64_eq (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.eq
    drop
    call $f10)
  (func $i64_ne (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.ne
    drop
    call $f10)
  (func $i64_lt_s (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.lt_s
    drop
    call $f10)
  (func $i64_le_s (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.le_s
    drop
    call $f10)
  (func $i64_lt_u (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.lt_u
    drop
    call $f10)
  (func $i64_le_u (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.le_u
    drop
    call $f10)
  (func $i64_gt_s (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.gt_s
    drop
    call $f10)
  (func $i64_ge_s (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.ge_s
    drop
    call $f10)
  (func $i64_gt_u (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.gt_u
    drop
    call $f10)
  (func $i64_ge_u (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    i64.ge_u
    drop
    call $f10)
  (func $i64_store (type $t5) (result i32)
    call $f8
    call $f11
    call $f17
    i64.store
    call $f10)
  (func $i64_store8 (type $t5) (result i32)
    call $f8
    call $f11
    call $f17
    i64.store8
    call $f10)
  (func $i64_store16 (type $t5) (result i32)
    call $f8
    call $f11
    call $f17
    i64.store16
    call $f10)
  (func $i64_store32 (type $t5) (result i32)
    call $f8
    call $f11
    call $f17
    i64.store32
    call $f10)
  (func $i64_call (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    call $f32
    call $f10)
  (func $i64_call_indirect (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    call $f19
    call_indirect $T0 (type $t1)
    drop
    call $f10)
  (func $i64_select (type $t5) (result i32)
    call $f8
    call $f16
    call $f17
    call $f20
    select
    drop
    call $f10)
  (func $f32_add (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.add
    drop
    call $f10)
  (func $f32_sub (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.sub
    drop
    call $f10)
  (func $f32_mul (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.mul
    drop
    call $f10)
  (func $f32_div (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.div
    drop
    call $f10)
  (func $f32_copysign (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.copysign
    drop
    call $f10)
  (func $f32_eq (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.eq
    drop
    call $f10)
  (func $f32_ne (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.ne
    drop
    call $f10)
  (func $f32_lt (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.lt
    drop
    call $f10)
  (func $f32_le (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.le
    drop
    call $f10)
  (func $f32_gt (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.gt
    drop
    call $f10)
  (func $f32_ge (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.ge
    drop
    call $f10)
  (func $f32_min (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.min
    drop
    call $f10)
  (func $f32_max (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    f32.max
    drop
    call $f10)
  (func $f32_store (type $t5) (result i32)
    call $f8
    call $f11
    call $f22
    f32.store
    call $f10)
  (func $f32_call (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    call $f33
    call $f10)
  (func $f32_call_indirect (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    call $f24
    call_indirect $T0 (type $t2)
    drop
    call $f10)
  (func $f32_select (type $t5) (result i32)
    call $f8
    call $f21
    call $f22
    call $f25
    select
    drop
    call $f10)
  (func $f64_add (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.add
    drop
    call $f10)
  (func $f64_sub (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.sub
    drop
    call $f10)
  (func $f64_mul (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.mul
    drop
    call $f10)
  (func $f64_div (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.div
    drop
    call $f10)
  (func $f64_copysign (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.copysign
    drop
    call $f10)
  (func $f64_eq (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.eq
    drop
    call $f10)
  (func $f64_ne (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.ne
    drop
    call $f10)
  (func $f64_lt (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.lt
    drop
    call $f10)
  (func $f64_le (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.le
    drop
    call $f10)
  (func $f64_gt (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.gt
    drop
    call $f10)
  (func $f64_ge (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.ge
    drop
    call $f10)
  (func $f64_min (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.min
    drop
    call $f10)
  (func $f64_max (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    f64.max
    drop
    call $f10)
  (func $f64_store (type $t5) (result i32)
    call $f8
    call $f11
    call $f27
    f64.store
    call $f10)
  (func $f64_call (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    call $f34
    call $f10)
  (func $f64_call_indirect (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    call $f29
    call_indirect $T0 (type $t3)
    drop
    call $f10)
  (func $f64_select (type $t5) (result i32)
    call $f8
    call $f26
    call $f27
    call $f30
    select
    drop
    call $f10)
  (func $br_if (type $t5) (result i32)
    block $B0 (result i32)
      call $f8
      call $f11
      call $f12
      i32.const 0
      i32.and
      br_if $B0
      drop
      call $f10
    end)
  (func $br_table (type $t5) (result i32)
    block $B0 (result i32)
      call $f8
      block $B1 (result i32)
        call $f11
        call $f12
        br_table $B0 $B1
      end
      drop
      call $f10
    end)
  (table $T0 8 8 funcref)
  (memory $M0 1)
  (export "i32_add" (func $i32_add))
  (export "i32_sub" (func $i32_sub))
  (export "i32_mul" (func $i32_mul))
  (export "i32_div_s" (func $i32_div_s))
  (export "i32_div_u" (func $i32_div_u))
  (export "i32_rem_s" (func $i32_rem_s))
  (export "i32_rem_u" (func $i32_rem_u))
  (export "i32_and" (func $i32_and))
  (export "i32_or" (func $i32_or))
  (export "i32_xor" (func $i32_xor))
  (export "i32_shl" (func $i32_shl))
  (export "i32_shr_u" (func $i32_shr_u))
  (export "i32_shr_s" (func $i32_shr_s))
  (export "i32_eq" (func $i32_eq))
  (export "i32_ne" (func $i32_ne))
  (export "i32_lt_s" (func $i32_lt_s))
  (export "i32_le_s" (func $i32_le_s))
  (export "i32_lt_u" (func $i32_lt_u))
  (export "i32_le_u" (func $i32_le_u))
  (export "i32_gt_s" (func $i32_gt_s))
  (export "i32_ge_s" (func $i32_ge_s))
  (export "i32_gt_u" (func $i32_gt_u))
  (export "i32_ge_u" (func $i32_ge_u))
  (export "i32_store" (func $i32_store))
  (export "i32_store8" (func $i32_store8))
  (export "i32_store16" (func $i32_store16))
  (export "i32_call" (func $i32_call))
  (export "i32_call_indirect" (func $i32_call_indirect))
  (export "i32_select" (func $i32_select))
  (export "i64_add" (func $i64_add))
  (export "i64_sub" (func $i64_sub))
  (export "i64_mul" (func $i64_mul))
  (export "i64_div_s" (func $i64_div_s))
  (export "i64_div_u" (func $i64_div_u))
  (export "i64_rem_s" (func $i64_rem_s))
  (export "i64_rem_u" (func $i64_rem_u))
  (export "i64_and" (func $i64_and))
  (export "i64_or" (func $i64_or))
  (export "i64_xor" (func $i64_xor))
  (export "i64_shl" (func $i64_shl))
  (export "i64_shr_u" (func $i64_shr_u))
  (export "i64_shr_s" (func $i64_shr_s))
  (export "i64_eq" (func $i64_eq))
  (export "i64_ne" (func $i64_ne))
  (export "i64_lt_s" (func $i64_lt_s))
  (export "i64_le_s" (func $i64_le_s))
  (export "i64_lt_u" (func $i64_lt_u))
  (export "i64_le_u" (func $i64_le_u))
  (export "i64_gt_s" (func $i64_gt_s))
  (export "i64_ge_s" (func $i64_ge_s))
  (export "i64_gt_u" (func $i64_gt_u))
  (export "i64_ge_u" (func $i64_ge_u))
  (export "i64_store" (func $i64_store))
  (export "i64_store8" (func $i64_store8))
  (export "i64_store16" (func $i64_store16))
  (export "i64_store32" (func $i64_store32))
  (export "i64_call" (func $i64_call))
  (export "i64_call_indirect" (func $i64_call_indirect))
  (export "i64_select" (func $i64_select))
  (export "f32_add" (func $f32_add))
  (export "f32_sub" (func $f32_sub))
  (export "f32_mul" (func $f32_mul))
  (export "f32_div" (func $f32_div))
  (export "f32_copysign" (func $f32_copysign))
  (export "f32_eq" (func $f32_eq))
  (export "f32_ne" (func $f32_ne))
  (export "f32_lt" (func $f32_lt))
  (export "f32_le" (func $f32_le))
  (export "f32_gt" (func $f32_gt))
  (export "f32_ge" (func $f32_ge))
  (export "f32_min" (func $f32_min))
  (export "f32_max" (func $f32_max))
  (export "f32_store" (func $f32_store))
  (export "f32_call" (func $f32_call))
  (export "f32_call_indirect" (func $f32_call_indirect))
  (export "f32_select" (func $f32_select))
  (export "f64_add" (func $f64_add))
  (export "f64_sub" (func $f64_sub))
  (export "f64_mul" (func $f64_mul))
  (export "f64_div" (func $f64_div))
  (export "f64_copysign" (func $f64_copysign))
  (export "f64_eq" (func $f64_eq))
  (export "f64_ne" (func $f64_ne))
  (export "f64_lt" (func $f64_lt))
  (export "f64_le" (func $f64_le))
  (export "f64_gt" (func $f64_gt))
  (export "f64_ge" (func $f64_ge))
  (export "f64_min" (func $f64_min))
  (export "f64_max" (func $f64_max))
  (export "f64_store" (func $f64_store))
  (export "f64_call" (func $f64_call))
  (export "f64_call_indirect" (func $f64_call_indirect))
  (export "f64_select" (func $f64_select))
  (export "br_if" (func $br_if))
  (export "br_table" (func $br_table))
  (elem $e0 (i32.const 0) func $f0 $f1 $f2 $f3 $f4 $f5 $f6 $f7))
