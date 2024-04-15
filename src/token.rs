#[derive(Debug)]
pub enum TokenType {
    Inc,    // +
    Dec,    // -
    Shr,    // >
    Shl,    // <
    Out,    // .
    In,     // ,
    Jmp,    // [
    Ret     // ]
}

#[derive(Debug)]
pub struct Token {
    pub typ: TokenType,
    pub pos: usize
}