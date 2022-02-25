(module
    (func $1 (result i32)
    	(i32.const 42) 
  		(i32.const 1)  ;; value used to select a branch
  		(br_table 0))
)


