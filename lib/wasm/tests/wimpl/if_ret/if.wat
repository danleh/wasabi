(module
    (func $0
        i32.const 0 
		(if
		  (then
		    i32.const 1
		  )		  
		)
		i32.const 42 
		drop                  
    )
    (func $1 (param i32) 
    	nop 
    )
)
