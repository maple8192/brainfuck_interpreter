use std::io;
use std::io::Read;

use crate::ast::{NodeType, Program};
use crate::error::ProgramError;
use crate::memory::Memory;

pub fn execute(ast: Program, mut input: impl Read, src: &str) -> Result<(), ProgramError> {
    let Program(program) = ast;

    let mut memory = Memory::new();

    let mut stack = vec![(&program, 0, false)];
    while let Some((nodes, mut p, is_bracket)) = stack.pop() {
        if is_bracket && memory.get() == 0 {
            continue;
        }
        while p < nodes.len() {
            println!("{memory:?}");
            match &nodes[p].typ {
                NodeType::Inc => memory.increment(),
                NodeType::Dec => memory.decrement(),
                NodeType::Shr => memory.shift_right(),
                NodeType::Shl => memory.shift_left().map_err(|_| ProgramError::new(src, nodes[p].token.pos, "memory out of range"))?,
                NodeType::Out => print!("{}", memory.get() as char),
                NodeType::In => memory.set(read_u8(&mut input).map_err(|_| ProgramError::new(src, nodes[p].token.pos, "io error"))?),
                NodeType::Bracket(body) => {
                    stack.push((nodes, p + 1, is_bracket));
                    stack.push((body, 0, true));
                    break;
                }
            }

            p += 1;
        }
        if is_bracket {
            stack.push((nodes, 0, true));
        }
    }

    Ok(())
}

fn read_u8(input: &mut impl Read) -> io::Result<u8> {
    let mut buf = [0; 1];
    input.read_exact(&mut buf)?;
    Ok(buf[0])
}