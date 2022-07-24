mod file_reader;
mod code_processor;
mod common;
mod interpreter;
mod machine;

use std::env;
use crate::common::Token;
use crate::interpreter::Interpreter;
use crate::machine::Machine;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (code_content, input_content) = file_reader::read_files(
        args.get(1).unwrap(),
        if args.len() >= 2 { Some(args.get(2).unwrap()) } else { None }
    );

    let code_content = code_content.unwrap();
    let input_content = input_content.unwrap().unwrap();

    let code = code_processor::extract_code(code_content);

    let machine = Machine::new(code, input_content);

    Interpreter::new(machine).run();
}
