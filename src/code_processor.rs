use crate::common::Token;

pub fn extract_code(raw_code: String) -> Vec<Token> {
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
