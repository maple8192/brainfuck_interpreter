use crate::{position::Position, token_kind::TokenKind};

/// Represents a token in the source code.
#[derive(Debug, Clone, Copy)]
pub struct Token {
    kind: TokenKind,
    pos: Position,
}

impl Token {
    /// Creates a new token with the given kind and position.
    pub fn new(kind: TokenKind, pos: Position) -> Self {
        Self { kind, pos }
    }

    /// Returns the kind of the token.
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Returns the position of the token.
    pub fn pos(&self) -> Position {
        self.pos
    }
}
