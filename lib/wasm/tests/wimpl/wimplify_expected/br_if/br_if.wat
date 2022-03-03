(module
    (func $1 (param i32 i32) 
        (block
			local.get 0 ;; get 0th local, and put into stack
			i32.eqz     ;; pop stack and check if it is zero, put result(boolean) into stack
			br_if 0     ;; branch out of 0th `block` if top item in stack is true
			i32.const 42
			local.set 1
        )
    )
)
