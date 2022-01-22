(module
    (func $1
        block (result i32)
            i32.const 42
            block (param i32) (result i32) 
            	i32.const 0
            	i32.add 
            end 
            i32.const 1 
            i32.add            
        end
        drop         
    )
)
