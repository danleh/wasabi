(module
;;  (type $0 (func))
;;  (import "env" "func" (func $0 (type $0)))
;;  (func $0 (type $0) (param) (result) (i32.const 0 (drop)))

  (import "env" "glob" (global $0 i32))
  (global $0 i32 (i32.const 0))
)
