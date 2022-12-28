(module
  (func $0 (param) (result i64) (i64.const 0))
  (export "i64_function" (func $0))
)
;; fails when invoking form JS API with TypeError: i64 result
