use wasm::wimpl;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("args: {:?}\n", args);

    let wasm_path = &args[1];
    let wimpl = wimpl::Module::from_wasm_file(wasm_path).unwrap();

    if let Some(out_path) = args.get(2) {
        std::fs::write(out_path, wimpl.to_string()).unwrap();
    }
    // for stmt in &wimpl.functions[0].body.0 {
    //     println!("{}", stmt);
    // }
}
