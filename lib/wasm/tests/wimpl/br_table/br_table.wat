(module
    (func $1
  		(i32.const 1)  ;; value used to select a branch
  		(br_table 0))
)

func f1 [] -> [] @label0 : {		//@label0 
	br_table default=@ 
}


s0 = 0 // s0 holds the return value of @lab0 
block {

	s0 = s7 //return of lab0 
	br 
}


Vec<(Instr, Option<Comment>)>


