(module
  (type (;0;) (func))
  (type (;1;) (func (param i32)))
  (import "a" "A" (func (;0;) (type 1)))
  (func (;1;) (type 0) 
    i32.const 42
    drop
  )
  (table (;0;) 4 funcref)
  (export "main" (func 1))
  (export "tab" (table 0))
  (elem (i32.const 0) 1)
)
