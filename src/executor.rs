use std::io;
use std::io::{Read, Write};

use crate::ast::{NodeType, Program};
use crate::error::ProgramError;
use crate::logger::{IO, Logger};
use crate::memory::Memory;

pub fn execute(ast: Program, mut input: impl Read, mut output: impl Write, log: Option<impl Write>, src: &str) -> Result<(), ProgramError> {
    let Program(program) = ast;

    let mut logger = log.map(|log| Logger::new(log, src));

    let mut memory = Memory::new();

    let mut step = 1;
    let mut p = 0;
    while p < program.len() {
        match program[p].typ {
            NodeType::Inc => memory.increment(),
            NodeType::Dec => memory.decrement(),
            NodeType::Shr => memory.shift_right(),
            NodeType::Shl => memory.shift_left().map_err(|_| ProgramError::new(src, program[p].token.pos, "memory out of range"))?,
            NodeType::Out => write!(&mut output, "{}", memory.get() as char).map_err(|err| ProgramError::new(src, program[p].token.pos, err.to_string().leak()))?,
            NodeType::In => memory.set(read_u8(&mut input).map_err(|err| ProgramError::new(src, program[p].token.pos, err.to_string().leak()))?),
            NodeType::Jmp(to) => if memory.get() == 0 { p = to }
            NodeType::Ret(to) => if memory.get() != 0 { p = to }
        }

        if let Some(logger) = &mut logger {
            logger.log(step, program[p].token, &memory, match program[p].typ {
                NodeType::Out => Some(IO::Out),
                NodeType::In => Some(IO::In),
                _ => None
            }).map_err(|err| ProgramError::new(src, program[p].token.pos, err.to_string().leak()))?;
        }

        step += 1;
        p += 1;
    }

    Ok(())
}

fn read_u8(input: &mut impl Read) -> io::Result<u8> {
    let mut buf = [0; 1];
    input.read_exact(&mut buf)?;
    Ok(buf[0])
}