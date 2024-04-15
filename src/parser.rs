use crate::ast::{Node, Program};
use crate::parser_error::ParserError;
use crate::token::{Token, TokenType};

pub fn parse(tokens: Vec<Token>, src: &str) -> Result<Program, ParserError> {
    let mut pos = 0;
    Ok(Program(_parse(&tokens, &mut pos, None, src)?))
}

fn _parse<'a>(tokens: &[Token], pos: &mut usize, bracket: Option<usize>, src: &'a str) -> Result<Vec<Node>, ParserError<'a>> {
    let mut commands = Vec::new();

    while *pos < tokens.len() {
        let node = match tokens[*pos].typ {
            TokenType::Inc => Node::Inc,
            TokenType::Dec => Node::Dec,
            TokenType::Shr => Node::Shr,
            TokenType::Shl => Node::Shl,
            TokenType::Out => Node::Out,
            TokenType::In => Node::In,
            TokenType::Jmp => {
                let start = *pos;
                *pos += 1;
                Node::Bracket(_parse(tokens, pos, Some(start), src)?)
            },
            TokenType::Ret => {
                if bracket.is_none() {
                    return Err(ParserError { src, pos: tokens[*pos].pos, message: "Open bracket not found" });
                }

                return Ok(commands);
            }
        };
        commands.push(node);

        *pos += 1;
    }

    if let Some(st) = bracket {
        return Err(ParserError { src, pos: tokens[st].pos, message: "Close bracket not found" })
    }

    Ok(commands)
}