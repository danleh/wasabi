(module 
	(func $f
    	block $outer
    		block $inner0
    			i32.const 0
    			br_if 2
    			;; branch is not taken, check that NO end hook is called
    			nop
    		end

    		block $inner1
    			i32.const 1
    			br_if $inner1
    			;; check that $inner1 end hook is called
    			nop ;; dead code
    		end
    		
    		block $inner2
    			i32.const 1
    			br_if $outer
    			;; check that 2 end hooks are called: for $inner2 and $outer
    			nop ;; dead code
    		end
    	end

    	block $outer2
    		return
    		;; check that $outer2 end hook is called
    		nop ;; dead code
    	end
	)
    (start $f)
)
