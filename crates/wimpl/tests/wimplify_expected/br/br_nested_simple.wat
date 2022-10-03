(module
   (func $1 (result i32) 
        block (result i32) 
            i32.const 42	
            block 			
				i32.const 0
				br 1 
			end 
        end					
    )
)
