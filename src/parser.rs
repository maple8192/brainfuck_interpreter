use crate::ast::{Node, NodeType, Program};
use crate::parser_error::ParserError;
use crate::token::{Token, TokenType};

pub fn parse(tokens: Vec<Token>, src: &str) -> Result<Program, ParserError> {
    let mut pos = 0;
    Ok(Program(_parse(&tokens, &mut pos, None, src)?))
}

fn _parse<'a>(tokens: &[Token], pos: &mut usize, bracket: Option<usize>, src: &'a str) -> Result<Vec<Node>, ParserError<'a>> {
    let mut commands = Vec::new();

    while *pos < tokens.len() {
        let p = *pos;
        let typ = match tokens[p].typ {
            TokenType::Inc => NodeType::Inc,
            TokenType::Dec => NodeType::Dec,
            TokenType::Shr => NodeType::Shr,
            TokenType::Shl => NodeType::Shl,
            TokenType::Out => NodeType::Out,
            TokenType::In => NodeType::In,
            TokenType::Jmp => {
                let start = p;
                *pos += 1;
                NodeType::Bracket(_parse(tokens, pos, Some(start), src)?)
            }
            TokenType::Ret => {
                if bracket.is_none() {
                    return Err(ParserError { src, pos: tokens[p].pos, message: "Open bracket not found" });
                }

                return Ok(commands);
            }
        };
        commands.push(Node { typ, token: tokens[p] });

        *pos += 1;
    }

    if let Some(st) = bracket {
        return Err(ParserError { src, pos: tokens[st].pos, message: "Close bracket not found" })
    }

    Ok(commands)
}