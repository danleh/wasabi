(module
    (func $0
        i32.const 0 
		(if (result i32)
		  (then 
		    i32.const 1
		  )	
		  (else 
		  	i32.const 42
		  )	  
		)
		i32.const 42 
		i32.add 
		drop                  
    )
    (func $1 (param i32) 
    	nop 
    )
)
