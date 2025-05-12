/// Represents the kind of token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    /// Represents '+'.
    Inc,

    /// Represents '-'.
    Dec,

    /// Represents '>'.
    Shr,

    /// Represents '<'.
    Shl,

    /// Represents '.'.
    Out,

    /// Represents ','.
    In,

    /// Represents '['.
    Jmp,

    /// Represents ']'.
    Ret,
}
