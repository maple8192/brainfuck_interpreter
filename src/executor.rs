use std::io::{BufRead, Bytes, Write};

use anyhow::anyhow;

use crate::{memory::Memory, token::Token, token_kind::TokenKind};

/// Represents a executor of brainfuck program.
pub struct Executor<In: BufRead, Out: Write> {
    program: Vec<Token>,
    input: Bytes<In>,
    output: Out,
    memory: Memory,
    pc: usize,
}

impl<In: BufRead, Out: Write> Executor<In, Out> {
    /// Creates a new executor instance with the given program and output destination.
    pub fn new(program: Vec<Token>, input: In, output: Out) -> Self {
        Self {
            program,
            input: input.bytes(),
            output,
            memory: Memory::default(),
            pc: 0,
        }
    }

    /// Executes a single step of the program.
    /// Returns `Ok(false)` if the program has finished.
    pub fn step(&mut self) -> anyhow::Result<bool> {
        if self.pc >= self.program.len() {
            return Ok(false);
        }

        let token = &self.program[self.pc];
        match token.kind() {
            TokenKind::Inc => self.memory.increment(),
            TokenKind::Dec => self.memory.decrement(),
            TokenKind::Shr => self.memory.shift_right(),
            TokenKind::Shl => self.memory.shift_left().map_err(|err| {
                anyhow!(
                    "Error occurred in ({}:{}): {err}",
                    token.pos().row(),
                    token.pos().col()
                )
            })?,
            TokenKind::Out => {
                self.output.write_all(&[self.memory.get()])?;
                self.output.flush()?;
            }
            TokenKind::In => {
                let ch = self
                    .input
                    .next()
                    .ok_or_else(|| anyhow!("Input stream ended"))??;
                dbg!(ch);
                self.memory.set(ch);
            }
            TokenKind::Jmp => {
                if self.memory.get() == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        self.pc += 1;
                        if self.pc >= self.program.len() {
                            return Err(anyhow!(
                                "Error occurred in ({}:{}): Not found matching `]`",
                                token.pos().row(),
                                token.pos().col()
                            ));
                        }
                        match self.program[self.pc].kind() {
                            TokenKind::Jmp => depth += 1,
                            TokenKind::Ret => depth -= 1,
                            _ => (),
                        }
                    }
                }
            }
            TokenKind::Ret => {
                if self.memory.get() != 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        if self.pc == 0 {
                            return Err(anyhow!(
                                "Error occurred in ({}:{}): Not found matching `[`",
                                token.pos().row(),
                                token.pos().col()
                            ));
                        }
                        self.pc -= 1;
                        match self.program[self.pc].kind() {
                            TokenKind::Jmp => depth -= 1,
                            TokenKind::Ret => depth += 1,
                            _ => (),
                        }
                    }
                }
            }
        }

        self.pc += 1;
        Ok(true)
    }

    /// Executes the entire program.
    pub fn run(&mut self) -> anyhow::Result<()> {
        while self.step()? {}
        Ok(())
    }
}
