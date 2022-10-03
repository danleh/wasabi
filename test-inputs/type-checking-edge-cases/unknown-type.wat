(module
;; WebAssembly doesn't have "principle types", i.e., there is not one, unique "most general" type for each instruction that would be expressible as a function type.
;; Not even for instruction _sequences_!
(func $single_value_unknown (result i32) ;; this is because of value polymorphism + stack polymorphism
    unreachable ;; [t*] -> [t*]  => [] -> [i32, i32, ?]
    drop        ;; [t] -> []     => [?] -> []
    i32.add     ;; [i32, i32] -> [i32]
)
(func $arity_unknown (result i32) ;; this is because of stack polymorphism + stack polymorphism
    unreachable ;; [t*] -> [t*]  => ?
    unreachable ;; [t*] -> [t*]  => ?
    i32.add
    ;; one way to solve the above case is just choose any possible type, e.g.
    ;; the first unreachable always produces all arguments, and the second one
    ;; is just "empty", i.e., [] -> [].
)
)