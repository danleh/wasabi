(module
  (import "print" (func $0 (param i32) (result)))
  (func $1 (param i32 i32) (result)
    get_local 0
    get_local 1
    i32.add
    call $0)
  (export "addAndPrint" (func $1))
)
