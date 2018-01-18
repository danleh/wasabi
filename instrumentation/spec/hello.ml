open Wasm

(* All files xyz.ml, regardless of directory structure
 * are available as submodules with capital letters.
 * E.g., /syntax/ast.ml -> open Wasm.Ast
 *
 * If there is an interface file xyz.mli, only its symbols
 * are available. Everything else is "private". 
 * E.g., /script/run.mli
 *)

let () = Flags.trace := true

let () = Run.trace "hello world"