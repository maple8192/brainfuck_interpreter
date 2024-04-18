use std::io::Read;

use crate::error::ProgramError;
use crate::executor::execute;
use crate::lexer::tokenize;
use crate::parser::parse;

pub fn interpret(src: &str, input: impl Read) -> Result<(), ProgramError> {
    let tokens = tokenize(src);
    let ast = parse(tokens, src)?;
    execute(ast, input, src)?;
    Ok(())
}