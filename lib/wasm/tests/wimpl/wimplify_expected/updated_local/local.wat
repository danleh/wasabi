(module
  (type (;0;) (func (param i32 i32) ))
  (func $1 (type 0) (param i32 i32) 
    local.get 0
    i32.const 42
    local.set 0 
    call $2  
  )
  (func $2 (param i32) 
  	nop
  )
)


