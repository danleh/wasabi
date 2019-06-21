;; .wat is the WebAssembly text format.
;; The syntax is s-expressions.
;; Double semicolons are comment lines.

;; Every WebAssembly file is a "module".
(module
  ;; Imports have "two-level names", where the first level is often "env".
  ;; This is often written as "env.print".
  (import "env" "print"
    ;; The import is a function import.
    ;; $print is actually not a name, but a placeholder for an index (= the number of the function).
    ;; But for convenience in the text format you can name indices.
    ;; You can now refer to this number somewhere else with $print.
    (func $print
      ;; Functions are typed. Here: takes a single 32-bit integer argument.
      (param i32)
    )
  )
  ;; Define a new function with index $start 
  ;; and no arguments or return values (no type signature).
  (func $start
    ;; Place a 32 bit integer constant on the stack.
    i32.const 42
    ;; Call the previously imported function $print with 
    ;; 42 as the argument on the stack.
    call $print
  )
  ;; Declare $start to be the start function.
  ;; (Which is automatically run when the module is "instantiated".)
  (start $start)
)
