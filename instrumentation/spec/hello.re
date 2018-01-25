open Wasm;

{
  /* The library itself is a module of same name.
   * E.g., wasm.cmx -> open Wasm
   *
   * All files xyz.ml, regardless of directory structure
   * are available-pp refmt  as submodules within in the library one.
   * E.g., /syntax/ast.ml -> open Wasm.Ast
   *
   * If there is an interface file xyz.mli, only its symbols
   * are available. Everything else is "private".
   * E.g., /script/run.mli
   */
  let file = "ackermann.wasm";
  let in_channel = open_in_bin(file);
  let num_bytes = in_channel_length(in_channel);
  let bytes = Bytes.create(num_bytes);
  really_input(in_channel, bytes, 0, num_bytes);
  close_in(in_channel);
  let ast = Decode.decode("ackermann", bytes);
  let export_to_name = (export: Ast.export) =>
    Ast.string_of_name(export.it.name);
  List.iter(e => print_endline(export_to_name(e)), ast.it.exports);
};
/* Print.module_ stdout 80 ast;; */
