(module
  (type (;0;) (func (param i32 i32)))
  (func $1 (type 0) (param i32 i32)
    (i32.add
      (local.get 0)
      (local.get 1))
    (local.set 0)
    (global.get 0) 
    (local.tee 1)
    (global.set 0) 
  )
  (global $0 (mut i32) (i32.const 42))  
)
