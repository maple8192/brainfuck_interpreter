use std::fs;

const CODE_PATH: &str = "code.bf";

fn main() {
    let content = fs::read_to_string(CODE_PATH).unwrap();

    println!("{}", content);
}
