use io::Result;
use std::{fs, io};
use std::path::Path;

pub fn read_files<P: AsRef<Path>>(code_path_str: P, input_path_str: Option<P>) -> (Result<String>, Option<Result<String>>) {
    let code = get_content(code_path_str);
    let input = match input_path_str {
        Some(p) => Some(get_content(p)),
        None => None,
    };

    (code, input)
}

fn get_content<P: AsRef<Path>>(file_path: P) -> Result<String> {
    fs::read_to_string(file_path)
}