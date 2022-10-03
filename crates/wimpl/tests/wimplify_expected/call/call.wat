(module
    (table $table 1 anyfunc)
	(type (;0;) (func)) 
    (type (;1;) (func (param i32 i32) (result i32)))
    (func $1 (type 0) 
        i32.const 0
        drop
        call $2
        i32.const 3 
        i32.const 42
        call $3
        i32.const 56 
        i32.add
        drop 			;; return   
    )
    (func $2 (type 0)  
    	nop
	)
    (func $3 (type 1) (param i32 i32) (result i32)
		(i32.add
		  (local.get 0)
		  (local.get 1))    
    )
    (elem (i32.const 0) $2)
    (export "f" (func $1))
)

