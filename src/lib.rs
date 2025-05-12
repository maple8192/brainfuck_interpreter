use std::io::{BufRead, Write};

use executor::Executor;
use lexer::lex;

mod executor;
mod lexer;
mod memory;
mod position;
mod token;
mod token_kind;

/// Runs the given source code and writes results to the provided output destination.
pub fn run(src: &str, input: impl BufRead, output: impl Write) -> anyhow::Result<()> {
    let tokens = lex(src);
    let mut executor = Executor::new(tokens, input, output);
    executor.run()?;
    Ok(())
}
