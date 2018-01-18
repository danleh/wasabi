open Wasm;;

(* The library itself is a module of same name.
 * E.g., wasm.cmx -> open Wasm
 *
 * All files xyz.ml, regardless of directory structure
 * are available as submodules within in the library one.
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
let export_to_name (export : Ast.export) = Ast.string_of_name export.it.name in
List.iter (fun e -> print_endline (export_to_name e)) ast.it.exports;;
(* Print.module_ stdout 80 ast;; *)