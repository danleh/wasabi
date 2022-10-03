(module
  (func $0 (param) (result i64)
    i64.const 1337000000000
    return)
  (func $start (param) (result)
    call $0
    drop)
  (start $start)
)
