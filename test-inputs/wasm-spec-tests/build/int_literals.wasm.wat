(module
  (type $t0 (func (result i32)))
  (type $t1 (func (result i64)))
  (func $i32.test (type $t0) (result i32)
    i32.const 195940365
    return)
  (func $i32.umax (type $t0) (result i32)
    i32.const -1
    return)
  (func $i32.smax (type $t0) (result i32)
    i32.const 2147483647
    return)
  (func $i32.neg_smax (type $t0) (result i32)
    i32.const -2147483647
    return)
  (func $i32.smin (type $t0) (result i32)
    i32.const -2147483648
    return)
  (func $i32.alt_smin (type $t0) (result i32)
    i32.const -2147483648
    return)
  (func $i32.inc_smin (type $t0) (result i32)
    i32.const -2147483648
    i32.const 1
    i32.add
    return)
  (func $i32.neg_zero (type $t0) (result i32)
    i32.const 0
    return)
  (func $i32.not_octal (type $t0) (result i32)
    i32.const 10
    return)
  (func $i32.unsigned_decimal (type $t0) (result i32)
    i32.const -1
    return)
  (func $i32.plus_sign (type $t0) (result i32)
    i32.const 42
    return)
  (func $i64.test (type $t1) (result i64)
    i64.const 913028331277281902
    return)
  (func $i64.umax (type $t1) (result i64)
    i64.const -1
    return)
  (func $i64.smax (type $t1) (result i64)
    i64.const 9223372036854775807
    return)
  (func $i64.neg_smax (type $t1) (result i64)
    i64.const -9223372036854775807
    return)
  (func $i64.smin (type $t1) (result i64)
    i64.const -9223372036854775808
    return)
  (func $i64.alt_smin (type $t1) (result i64)
    i64.const -9223372036854775808
    return)
  (func $i64.inc_smin (type $t1) (result i64)
    i64.const -9223372036854775808
    i64.const 1
    i64.add
    return)
  (func $i64.neg_zero (type $t1) (result i64)
    i64.const 0
    return)
  (func $i64.not_octal (type $t1) (result i64)
    i64.const 10
    return)
  (func $i64.unsigned_decimal (type $t1) (result i64)
    i64.const -1
    return)
  (func $i64.plus_sign (type $t1) (result i64)
    i64.const 42
    return)
  (func $i32-dec-sep1 (type $t0) (result i32)
    i32.const 1000000)
  (func $i32-dec-sep2 (type $t0) (result i32)
    i32.const 1000)
  (func $i32-hex-sep1 (type $t0) (result i32)
    i32.const 168755353)
  (func $i32-hex-sep2 (type $t0) (result i32)
    i32.const 109071)
  (func $i64-dec-sep1 (type $t1) (result i64)
    i64.const 1000000)
  (func $i64-dec-sep2 (type $t1) (result i64)
    i64.const 1000)
  (func $i64-hex-sep1 (type $t1) (result i64)
    i64.const 3078696982321561)
  (func $i64-hex-sep2 (type $t1) (result i64)
    i64.const 109071)
  (export "i32.test" (func $i32.test))
  (export "i32.umax" (func $i32.umax))
  (export "i32.smax" (func $i32.smax))
  (export "i32.neg_smax" (func $i32.neg_smax))
  (export "i32.smin" (func $i32.smin))
  (export "i32.alt_smin" (func $i32.alt_smin))
  (export "i32.inc_smin" (func $i32.inc_smin))
  (export "i32.neg_zero" (func $i32.neg_zero))
  (export "i32.not_octal" (func $i32.not_octal))
  (export "i32.unsigned_decimal" (func $i32.unsigned_decimal))
  (export "i32.plus_sign" (func $i32.plus_sign))
  (export "i64.test" (func $i64.test))
  (export "i64.umax" (func $i64.umax))
  (export "i64.smax" (func $i64.smax))
  (export "i64.neg_smax" (func $i64.neg_smax))
  (export "i64.smin" (func $i64.smin))
  (export "i64.alt_smin" (func $i64.alt_smin))
  (export "i64.inc_smin" (func $i64.inc_smin))
  (export "i64.neg_zero" (func $i64.neg_zero))
  (export "i64.not_octal" (func $i64.not_octal))
  (export "i64.unsigned_decimal" (func $i64.unsigned_decimal))
  (export "i64.plus_sign" (func $i64.plus_sign))
  (export "i32-dec-sep1" (func $i32-dec-sep1))
  (export "i32-dec-sep2" (func $i32-dec-sep2))
  (export "i32-hex-sep1" (func $i32-hex-sep1))
  (export "i32-hex-sep2" (func $i32-hex-sep2))
  (export "i64-dec-sep1" (func $i64-dec-sep1))
  (export "i64-dec-sep2" (func $i64-dec-sep2))
  (export "i64-hex-sep1" (func $i64-hex-sep1))
  (export "i64-hex-sep2" (func $i64-hex-sep2)))
