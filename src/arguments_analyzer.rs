use clap::{App, Arg};

pub struct ArgsInfo {
    pub code_file_path: String,
    pub input_file_path: Option<String>,
    pub is_debug_mode: bool,
}

pub fn analyze_args() -> Result<ArgsInfo, String> {
    let code_file_path_arg = Arg::new("code_file_path").required(true);
    let input_file_path_arg = Arg::new("input_file_path").required(false);
    let debug_mode = Arg::new("debug_mode")
        .short('d')
        .long("debug")
        .required(false);

    let app = App::new("Brainfuck Interpreter")
        .arg(code_file_path_arg)
        .arg(input_file_path_arg)
        .arg(debug_mode);

    match app.try_get_matches() {
        Ok(m) => {
            let code_file_path = match m.value_of("code_file_path") { Some(v) => v.to_string(), None => return Err("Invalid arguments.".to_string()), };
            let input_file_path = match m.value_of("input_file_path") { Some(s) => Some(s.to_string()), None => None, };
            let is_debug_mode = m.is_present("debug_mode");

            Ok(ArgsInfo { code_file_path, input_file_path, is_debug_mode })
        }
        Err(_) => {
            Err("Invalid arguments.".to_string())
        }
    }
}