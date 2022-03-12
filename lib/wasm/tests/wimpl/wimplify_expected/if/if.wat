(module
    (func $0
        i32.const 0 
		(if
		  (then
		    i32.const 1
		    call $1
		  )		  
		)
    )
    (func $1 (param i32) 
    	nop 
    )
)
