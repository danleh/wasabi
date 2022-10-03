(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    i32.const 0
    if
      return
    else
      ;; this was incorrectly instrumented
      return
    end)
  (export "main" (func 0)))
