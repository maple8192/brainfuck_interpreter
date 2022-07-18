use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let code_path = &args[1];
    let content = fs::read_to_string(code_path).unwrap();

    println!("{}", content);
}
