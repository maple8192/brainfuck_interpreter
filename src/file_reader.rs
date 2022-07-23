use io::Result;
use std::{fs, io};
use std::path::Path;

pub fn read_files(code_path_str: String, input_path_str: Option<String>) -> (Result<String>, Option<Result<String>>) {
    let code = get_content(code_path_str);
    let input = match input_path_str {
        Some(p) => Some(get_content(p)),
        None => None,
    };

    (code, input)
}

fn get_content(file_path: String) -> Result<String> {
    let path = Path::new(&file_path);
    fs::read_to_string(path)
}