use std::io::stdin;
use crate::executor::execute;
use crate::lexer::tokenize;
use crate::parser::parse;

mod lexer;
mod token;
mod parser;
mod ast;
mod executor;
mod memory;
mod error;

const PROGRAM: &str = "
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
";

fn main() -> Result<(), ()> {
    let tokens = tokenize(PROGRAM);
    let ast = parse(tokens, PROGRAM).map_err(|e| println!("{e}"))?;
    execute(ast, stdin(), PROGRAM).map_err(|e| println!("{e}"))?;
    Ok(())
}
