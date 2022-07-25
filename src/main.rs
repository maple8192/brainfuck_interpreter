mod file_reader;
mod code_processor;
mod common;
mod interpreter;
mod machine;
mod error;
mod arguments_analyzer;
mod debugger;

use crate::arguments_analyzer::analyze_args;
use crate::common::Token;
use crate::debugger::Debugger;
use crate::interpreter::Interpreter;
use crate::machine::Machine;

fn main() {
    let args = match analyze_args() {
        Ok(v) => v,
        Err(e) => { println!("{}", e); return; }
    };

    let (code_content, input_content) = file_reader::read_files(
        args.code_file_path,
        args.input_file_path
    );
    let is_debug_mode = args.is_debug_mode;

    let code_content = code_content.unwrap();
    let input_content = input_content.unwrap().unwrap();

    let code = code_processor::extract_code(code_content);

    let machine = Machine::new(code, input_content);

    if is_debug_mode {
        Debugger::new(machine).debug_run();
    } else {
        Interpreter::new(machine).run();
    }
}
