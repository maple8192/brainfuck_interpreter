use std::fmt::{self, Display, Formatter};

/// Represents a kind of token in the brainfuck language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// Represents a '+' token.
    Inc,

    /// Represents a '-' token.
    Dec,

    /// Represents a '>' token.
    Shr,

    /// Represents a '<' token.
    Shl,

    /// Represents a '.' token.
    Out,

    /// Represents a ',' token.
    Inp,

    /// Represents a '[' token.
    Jmp,

    /// Represents a ']' token.
    Ret,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Inc => write!(f, "+"),
            TokenKind::Dec => write!(f, "-"),
            TokenKind::Shr => write!(f, ">"),
            TokenKind::Shl => write!(f, "<"),
            TokenKind::Out => write!(f, "."),
            TokenKind::Inp => write!(f, ","),
            TokenKind::Jmp => write!(f, "["),
            TokenKind::Ret => write!(f, "]"),
        }
    }
}
