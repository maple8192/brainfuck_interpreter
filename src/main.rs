use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let code_path = &args[1];
    let content = fs::read_to_string(code_path).unwrap();

    println!("{}", extract_code(content));
}

fn extract_code(raw_code: String) -> String {
    let mut code: Vec<char> = Vec::new();

    let mut in_comment = false;

    for c in raw_code.chars() {
        if c == '\n' {
            in_comment = false;
            continue;
        }
        if in_comment {
            continue;
        }
        match c {
            '>' | '<' | '+' | '-' | '[' | ']' | '.' | ',' => code.push(c),
            '#' => in_comment = true,
            _ => (),
        }
    }

    code.iter().collect()
}
