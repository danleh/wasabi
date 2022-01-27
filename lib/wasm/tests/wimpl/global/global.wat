(module
  (type (;0;) (func (param i32 f32)))
  (func $1 (type 0) (param i32 f32)
    (local.get 1)
    (i32.add
      (local.get 0)
      (i32.const 0))
    (local.set 0)
    (global.get 0) 
    (i32.const 42) 
    (global.set 0)
    (global.get 1)    
    drop
    drop
    drop 
  )
  (global $0 (mut i32) (i32.const 42)) 
  (global $1 (mut f32) (f32.const 42))    
)
