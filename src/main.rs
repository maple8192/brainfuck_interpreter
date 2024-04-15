use crate::executor::execute;
use crate::lexer::tokenize;
use crate::parser::parse;

mod lexer;
mod token;
mod parser;
mod ast;
mod parser_error;
mod executor;

const PROGRAM: &str = "
>+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]
>++++++++[<++++>-] <.>+++++++++++[<++++++++>-]<-.--------.+++
.------.--------.[-]>++++++++[<++++>-]<+.[-]++++++++++.
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
    execute(ast);
}
