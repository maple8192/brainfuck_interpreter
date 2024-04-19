use std::fs::{File, read_to_string};
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
mod logger;

#[derive(Parser)]
#[command(version)]
struct Args {
    source: Option<String>,

    #[arg(short, long)]
    file: Option<PathBuf>,

    #[arg(short, long)]
    log: Option<PathBuf>
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

    let log_file = match args.log {
        Some(path) => match File::create(path) {
            Ok(file) => Some(file),
            Err(err) => {
                println!("{err}");
                return;
            }
        }
        None => None
    };

    let ret = interpret(&source, stdin(), stdout(), log_file);
    if let Err(err) = ret {
        println!("{err}");
    }
}
