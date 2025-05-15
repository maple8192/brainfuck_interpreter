use crate::token_kind::TokenKind;

/// Represents a token.
pub struct Token {
    kind: TokenKind,
    row: usize,
    col: usize,
}

impl Token {
    /// Creates a new token of the given kind, with the given row and column.
    pub fn new(kind: TokenKind, row: usize, col: usize) -> Self {
        Self { kind, row, col }
    }

    /// Returns the kind of the token.
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Returns the row where the token was found.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the column where the token was found.
    pub fn col(&self) -> usize {
        self.col
    }
}
