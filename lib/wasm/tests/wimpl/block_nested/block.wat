(module
    (func $1
        block (result i32)
            i32.const 42
            block 
            	i32.const 0 
            	drop  
            end 
            i32.const 1 
            i32.add            
        end
        drop         
    )
)
