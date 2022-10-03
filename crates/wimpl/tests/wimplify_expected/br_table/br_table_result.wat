(module
    (func $1 (result i64)
		(block (result i64)
			(i64.const 42) ;; result value
			(i32.const 1)  ;; value used to select a branch
			(br_table 1 0 1) ;; last is default
		)
	)
)
