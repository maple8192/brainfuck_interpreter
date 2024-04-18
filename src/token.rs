#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    Inc,
    Dec,
    Shr,
    Shl,
    Out,
    In,
    Jmp,
    Ret 
}

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub pos: usize
}