use std::io::{self, Bytes, Read, Write};

use thiserror::Error;

use crate::{
    lexer::lex,
    memory::{Memory, NegPtrError},
    token::Token,
    token_kind::TokenKind,
};

/// Represents a executor of a brainfuck program.
pub struct Executor<R: Read, W: Write> {
    program: Vec<Token>,
    input: Bytes<R>,
    output: W,
    memory: Memory,
    pc: usize,
}

impl<R: Read, W: Write> Executor<R, W> {
    /// Creates a new executor for the given program
    pub fn new(src: &str, input: R, output: W) -> Self {
        Self {
            program: lex(src),
            input: input.bytes(),
            output,
            memory: Memory::default(),
            pc: 0,
        }
    }

    /// Returns the executing program.
    pub fn program(&self) -> &[Token] {
        &self.program
    }

    /// Returns the current memory.
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Returns the current program counter.
    pub fn pc(&self) -> usize {
        self.pc
    }

    /// Executes a single step of the program.
    /// Returns `Ok(false)` if the program has finished executing.
    pub fn step(&mut self) -> Result<bool, ExecutorError> {
        let Some(token) = self.program.get(self.pc) else {
            return Ok(false);
        };
        self.pc += 1;

        match token.kind() {
            TokenKind::Inc => self.memory.inc(),
            TokenKind::Dec => self.memory.dec(),
            TokenKind::Shr => self.memory.shr(),
            TokenKind::Shl => self
                .memory
                .shl()
                .map_err(|err| ExecutorError(err.into(), token.row(), token.col()))?,
            TokenKind::Out => {
                self.output
                    .write_all(&[self.memory.get()])
                    .map_err(|err| ExecutorError(err.into(), token.row(), token.col()))?;
                self.output
                    .flush()
                    .map_err(|err| ExecutorError(err.into(), token.row(), token.col()))?;
            }
            TokenKind::Inp => self.memory.set(
                self.input
                    .next()
                    .transpose()
                    .map_err(|err| ExecutorError(err.into(), token.row(), token.col()))?
                    .unwrap_or(0),
            ),
            TokenKind::Jmp => {
                if self.memory.get() == 0 {
                    return Ok(true);
                }

                let mut depth = 1;
                while depth > 0 {
                    let cur = self.program.get(self.pc).ok_or_else(|| {
                        ExecutorError(ExecErrKind::UnclosedBkt, token.row(), token.col())
                    })?;
                    match cur.kind() {
                        TokenKind::Jmp => depth += 1,
                        TokenKind::Ret => depth -= 1,
                        _ => {}
                    }
                    self.pc += 1;
                }
            }
            TokenKind::Ret => {
                if self.memory.get() != 0 {
                    return Ok(true);
                }

                let mut depth = 1;
                while depth > 0 {
                    self.pc -= 1;
                    let cur = self.program.get(self.pc.wrapping_sub(1)).ok_or_else(|| {
                        ExecutorError(ExecErrKind::UnopenedBkt, token.row(), token.col())
                    })?;
                    match cur.kind() {
                        TokenKind::Jmp => depth -= 1,
                        TokenKind::Ret => depth += 1,
                        _ => {}
                    }
                }
            }
        }
        Ok(true)
    }
}

#[derive(Debug, Error)]
#[error("Error occurred in ({1}:{2}): {0}")]
pub struct ExecutorError(ExecErrKind, usize, usize);

/// Represents an error when executing a program.
#[derive(Debug, Error)]
pub enum ExecErrKind {
    #[error(transparent)]
    NegPtr(#[from] NegPtrError),

    #[error(transparent)]
    Io(#[from] io::Error),

    /// Represents an error when square brackets are not closed.
    #[error("Not found matching `]`")]
    UnclosedBkt,

    /// Represents an error when square brackets are not opened.
    #[error("Not found matching `[`")]
    UnopenedBkt,
}
