use std::io::stdin;

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
    let ret = interpret(PROGRAM, stdin());
    if let Err(err) = ret {
        println!("{err}");
    }
}
