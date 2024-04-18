use std::io::{stdin, stdout};

use crate::interpreter::interpret;

mod lexer;
mod token;
mod parser;
mod ast;
mod executor;
mod memory;
mod error;
mod interpreter;

const PROGRAM: &str = "
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
";

fn main() {
    let ret = interpret(PROGRAM, stdin(), stdout());
    if let Err(err) = ret {
        println!("{err}");
    }
}
