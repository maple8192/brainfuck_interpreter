#[derive(Debug)]
pub enum Node {
    Inc,
    Dec,
    Shr,
    Shl,
    Out,
    In,
    Bracket(Vec<Node>)
}

#[derive(Debug)]
pub struct Program(pub Vec<Node>);