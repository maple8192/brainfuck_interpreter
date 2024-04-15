use crate::token::{Token, TokenType};

pub fn tokenize(src: &str) -> Vec<Token> {
    let src = split_to_chars(src);

    let mut tokens = Vec::new();
    for (p, c) in src.into_iter().enumerate() {
        let typ = match c {
            '+' => TokenType::Inc,
            '-' => TokenType::Dec,
            '>' => TokenType::Shr,
            '<' => TokenType::Shl,
            '.' => TokenType::Out,
            ',' => TokenType::In,
            '[' => TokenType::Jmp,
            ']' => TokenType::Ret,
            _ => continue
        };
        tokens.push(Token { typ, pos: p });
    }

    tokens
}

fn split_to_chars(src: &str) -> Vec<char> {
    src.chars().collect()
}