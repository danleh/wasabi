open Wasm;;

(* All files xyz.ml, regardless of directory structure
 * are available as submodules with capital letters.
 * E.g., /syntax/ast.ml -> open Wasm.Ast
 *
 * If there is an interface file xyz.mli, only its symbols
 * are available. Everything else is "private". 
 * E.g., /script/run.mli
 *)

let file = "ackermann.wasm" in
let in_channel = open_in_bin file in
let num_bytes = in_channel_length in_channel in
let bytes = Bytes.create num_bytes in
really_input in_channel bytes 0 num_bytes;
close_in in_channel;
let ast = Decode.decode "ackermann" bytes in
print_endline (Ast.string_of_name (List.hd ast.it.exports).it.name);;
(* Print.module_ stdout 80 ast;; *)