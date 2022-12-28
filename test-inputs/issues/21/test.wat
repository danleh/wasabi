(module 
  (import "env" "print" (func $print (param i32)))
  (table $interp 4 funcref)
  (elem $interp (i32.const 0) $print)
  (elem $interp (i32.const 1) $f1)
  (elem $interp (i32.const 2) $f2)
  (func $f1
    i32.const 1
    call $print
  )
  (func $f2
    i32.const 2
    call $print
  )
  (func $main (export "main")
    ;; Calls $print
    i32.const 0
    i32.const 0
    call_indirect (param i32)

    ;; Calls $f1
    i32.const 1
    call_indirect (param)

    ;; Calls $f2
    i32.const 2
    call_indirect (param)

    ;; Fails at runtime due to undefined table entry
    i32.const 3
    call_indirect (param)
  )
)
