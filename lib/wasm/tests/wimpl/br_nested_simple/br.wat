(module
    (func $1
        block (result i32) ;;@label0 
            i32.const 42	;; [s0]
            block 			
				i32.const 0 	;;[s0, s1] 
				br 1 			;;[s0, s1] 	
			end 
        end					;; [s1]
        drop         
    )
)
