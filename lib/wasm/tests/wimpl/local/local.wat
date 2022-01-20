(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func $1 (type 0) (param i32 i32) (result i32)
    (i32.add
      (local.get 0)
      (local.get 1))
    (local.set 0)
    (i32.const 3)
    (local.tee 1)  
  )
)
