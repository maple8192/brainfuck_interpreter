use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let code_path = &args[1];
    let content = fs::read_to_string(code_path).unwrap();

    let code = extract_code(content);

    for i in 0..code.len() {
        match code[i] {
            Token::Inc => println!("increment"),
            Token::Dec => println!("decrement"),
            Token::IncPtr => println!("increment pointer"),
            Token::DecPtr => println!("decrement pointer"),
            Token::LoopIn => println!("enter loop"),
            Token::LoopOut => println!("return or exit loop"),
            Token::Print => println!("print character"),
            Token::Read => println!("read character"),
        }
    }
}

enum Token {
    Inc,
    Dec,
    IncPtr,
    DecPtr,
    LoopIn,
    LoopOut,
    Print,
    Read,
}

fn extract_code(raw_code: String) -> Vec<Token> {
    let mut code: Vec<Token> = Vec::new();

    let mut in_comment = false;

    for c in raw_code.chars() {
        if c == '\n' {
            in_comment = false;
            continue;
        }
        if in_comment { continue; }
        match c {
            '+' => code.push(Token::Inc),
            '-' => code.push(Token::Dec),
            '>' => code.push(Token::IncPtr),
            '<' => code.push(Token::DecPtr),
            '[' => code.push(Token::LoopIn),
            ']' => code.push(Token::LoopOut),
            '.' => code.push(Token::Print),
            ',' => code.push(Token::Read),
            '#' => in_comment = true,
            _ => (),
        }
    }

    code
}
