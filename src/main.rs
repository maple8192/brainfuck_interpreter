use std::fs::read_to_string;
use std::io::{stdin, stdout};
use std::path::PathBuf;
use clap::{arg, command, Parser};

use crate::interpreter::interpret;

mod lexer;
mod token;
mod parser;
mod ast;
mod executor;
mod memory;
mod error;
mod interpreter;

#[derive(Parser)]
#[command(version)]
struct Args {
    source: Option<String>,

    #[arg(short, long)]
    file: Option<PathBuf>
}

fn main() {
    let args = Args::parse();
    let source = match args.source {
        Some(source) => source,
        None => match args.file {
            Some(path) => match read_to_string(path) {
                Ok(source) => source,
                Err(err) => {
                    println!("{err}");
                    return;
                }
            },
            None => {
                println!("needs a code or a file path");
                return;
            }
        }
    };

    let ret = interpret(&source, stdin(), stdout());
    if let Err(err) = ret {
        println!("{err}");
    }
}
