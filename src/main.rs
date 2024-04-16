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
>++++>+++++>++++++[[-]<]
";

fn main() {
    let tokens = tokenize(PROGRAM);
    let ast = match parse(tokens, PROGRAM) {
        Ok(ast) => ast,
        Err(err) => {
            println!("{err}");
            return;
        }
    };
    println!("{ast:?}");
    let Err(err) = execute(ast, stdin(), PROGRAM) else { return };
    println!("{err}");
}
