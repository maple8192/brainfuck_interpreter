use std::str::Chars;

use crate::{position::Position, token::Token, token_kind::TokenKind};

/// Lexes the given source code and returns tokens.
pub fn lex(src: &str) -> Vec<Token> {
    Lexer::new(src).collect()
}

/// Represents a lexer.
/// This struct is for `lex` function.
#[derive(Debug)]
struct Lexer<'a> {
    src: Chars<'a>,
    pos: Position,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer instance with the given source code.
    fn new(src: &'a str) -> Self {
        Self {
            src: src.chars(),
            pos: Position::default(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    /// Returns the next token.
    /// Only characters in [`TokenKind`] are recognized and others are ignored as comments.
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.src.next()?;
        let ret = match ch {
            '+' => Token::new(TokenKind::Inc, self.pos),
            '-' => Token::new(TokenKind::Dec, self.pos),
            '>' => Token::new(TokenKind::Shr, self.pos),
            '<' => Token::new(TokenKind::Shl, self.pos),
            '.' => Token::new(TokenKind::Out, self.pos),
            ',' => Token::new(TokenKind::In, self.pos),
            '[' => Token::new(TokenKind::Jmp, self.pos),
            ']' => Token::new(TokenKind::Ret, self.pos),
            '\n' => {
                self.pos.next_line();
                return self.next();
            }
            _ => {
                self.pos.advance();
                return self.next();
            }
        };
        self.pos.advance();
        Some(ret)
    }
}
