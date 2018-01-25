#ocamlopt -w -40 -I ./lib/ /usr/lib/ocaml/bigarray.cmxa wasm.cmx hello.ml -o hello
ocamlopt -w -40 -I ./lib/ /usr/lib/ocaml/bigarray.cmxa wasm.cmx -pp "refmt -p ml" -impl hello.re -o hello
# TODO use reopt instead, for that: fix installation issue of reason!