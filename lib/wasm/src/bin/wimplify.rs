use wasm::wimpl::{wimplify::wimplify, self};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    println!("args: {:?}\n", args);

    let wasm_path = &args[1];
    let wimpl = wimpl::Module::from_wasm_file(wasm_path).unwrap();

    // println!("{}", wimpl);
    for stmt in &wimpl.functions[0].body.0 {
        println!("{}", stmt);
    }
}
