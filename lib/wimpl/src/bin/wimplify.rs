fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("args: {:?}\n", args);

    let wasm_path = &args[1];
    let wimpl = wimpl::Module::from_wasm_file(wasm_path).unwrap();

    // DEBUG
    // println!("{:#?}", wimpl);

    if let Some(out_path) = args.get(2) {
        std::fs::write(out_path, wimpl.to_string()).unwrap();
    }
}
