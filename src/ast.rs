use crate::token::Token;

#[derive(Debug)]
pub enum NodeType {
    Inc,
    Dec,
    Shr,
    Shl,
    Out,
    In,
    Jmp(usize),
    Ret(usize)
}

#[derive(Debug)]
pub struct Node {
    pub typ: NodeType,
    pub token: Token
}

#[derive(Debug)]
pub struct Program(pub Vec<Node>);