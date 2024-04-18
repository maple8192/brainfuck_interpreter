use crate::ast::{Node, NodeType, Program};
use crate::error::ProgramError;
use crate::token::{Token, TokenType};

pub fn parse(tokens: Vec<Token>, src: &str) -> Result<Program, ProgramError> {
    let mut nodes = Vec::new();

    let mut brackets = Vec::new();
    let mut pos = 0;
    while pos < tokens.len() {
        let typ = match tokens[pos].typ {
            TokenType::Inc => Some(NodeType::Inc),
            TokenType::Dec => Some(NodeType::Dec),
            TokenType::Shr => Some(NodeType::Shr),
            TokenType::Shl => Some(NodeType::Shl),
            TokenType::Out => Some(NodeType::Out),
            TokenType::In => Some(NodeType::In),
            TokenType::Jmp => {
                brackets.push(pos);
                None
            }
            TokenType::Ret => {
                let Some(p) = brackets.pop() else { return Err(ProgramError::new(src, pos, "pair bracket not found")) };

                nodes[p] = Some(Node { typ: NodeType::Jmp(pos + 1), token: tokens[p] });
                Some(NodeType::Ret(p))
            }
        };

        nodes.push(typ.map(|typ| Node { typ, token: tokens[pos] }));

        pos += 1;
    }

    if let Some(rem) = brackets.last() {
        return Err(ProgramError::new(src, *rem, "pair bracket not found"));
    }

    let nodes = nodes.into_iter().map(|node| node.unwrap()).collect();

    Ok(Program(nodes))
}