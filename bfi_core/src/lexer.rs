use std::str::Chars;

use crate::{token::Token, token_kind::TokenKind};

/// Lexes the given source code into tokens.
pub fn lex(src: &str) -> Vec<Token> {
    Lexer::new(src).collect()
}

/// Represents a lexer.
/// This struct is for [`lex`] function.
#[derive(Debug)]
struct Lexer<'a> {
    src: Chars<'a>,
    row: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer for the given source code.
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars(),
            row: 1,
            col: 1,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = match self.src.next()? {
            '+' => Token::new(TokenKind::Inc, self.row, self.col),
            '-' => Token::new(TokenKind::Dec, self.row, self.col),
            '>' => Token::new(TokenKind::Shr, self.row, self.col),
            '<' => Token::new(TokenKind::Shl, self.row, self.col),
            '.' => Token::new(TokenKind::Out, self.row, self.col),
            ',' => Token::new(TokenKind::Inp, self.row, self.col),
            '[' => Token::new(TokenKind::Jmp, self.row, self.col),
            ']' => Token::new(TokenKind::Ret, self.row, self.col),
            '\n' => {
                self.row += 1;
                self.col = 1;
                return self.next();
            }
            _ => {
                self.col += 1;
                return self.next();
            }
        };
        self.col += 1;
        Some(tok)
    }
}
