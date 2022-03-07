(module
  (type (;0;) (func (param i32 i32) ))
  (func $1 (type 0) (param i32 i32) 
    (loop 
    	local.get 0
    	call $2    	
		i32.const 42
		local.set 0 
	)
  )
  (func $2 (param i32) 
  	nop
  )
)


