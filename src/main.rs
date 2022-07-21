use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    let code_path = PathBuf::from(args.get(1).unwrap());
    let input_path = if args.len() >= 2 { Some(PathBuf::from(args.get(2).unwrap())) } else { None };

    let code_content = fs::read_to_string(code_path.as_path()).unwrap();
    let input_content = match input_path {
        Some(p) => fs::read_to_string(p.as_path()).unwrap(),
        None => "".to_string(),
    };

    let extension = code_path.extension().unwrap().to_string_lossy().to_string();
    if extension != "bf" && extension != "b" { panic!("This isn't brainfuck source file."); }

    let code = extract_code(code_content);

    run(code, input_content);
}

enum Token {
    Inc,
    Dec,
    IncPtr,
    DecPtr,
    LoopIn,
    LoopOut,
    Print,
    Read,
}

fn extract_code(raw_code: String) -> Vec<Token> {
    let mut code: Vec<Token> = Vec::new();

    let mut in_comment = false;

    for c in raw_code.chars() {
        if c == '\n' {
            in_comment = false;
            continue;
        }
        if in_comment { continue; }
        match c {
            '+' => code.push(Token::Inc),
            '-' => code.push(Token::Dec),
            '>' => code.push(Token::IncPtr),
            '<' => code.push(Token::DecPtr),
            '[' => code.push(Token::LoopIn),
            ']' => code.push(Token::LoopOut),
            '.' => code.push(Token::Print),
            ',' => code.push(Token::Read),
            '#' => in_comment = true,
            _ => (),
        }
    }

    code
}

fn run(code: Vec<Token>, input: String) {
    let mut program_pointer: isize = 0;
    let mut pointer: isize = 0;
    let mut memory: Vec<u8> = vec![0];

    let input_vec: Vec<char> = input.chars().collect();

    let mut input_counter: usize = 0;

    println!("Output:");
    print!("  ");

    while program_pointer < code.len() as isize {
        let current = &code[program_pointer as usize];

        match current {
            Token::Inc => memory[pointer as usize] = memory[pointer as usize].wrapping_add(1),
            Token::Dec => memory[pointer as usize] = memory[pointer as usize].wrapping_sub(1),
            Token::IncPtr => {
                pointer += 1;
                if pointer >= memory.len() as isize {
                    memory.push(0);
                }
            },
            Token::DecPtr => {
                pointer -= 1;
                if pointer < 0 {
                    panic!("Pointer cannot be less than zero. {}", program_pointer);
                }
            },
            Token::LoopIn => {
                if memory[pointer as usize] == 0 {
                    let mut count = 0;
                    for i in (program_pointer + 1)..=(code.len() as isize) {
                        if i == code.len() as isize {
                            panic!("No corresponding bracket. {}", program_pointer);
                        }

                        match code[i as usize] {
                            Token::LoopIn => {
                                count += 1;
                            }
                            Token::LoopOut => {
                                if count == 0 {
                                    program_pointer = i;
                                    break;
                                } else {
                                    count -= 1;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            },
            Token::LoopOut => {
                if memory[pointer as usize] != 0 {
                    let mut count = 0;
                    for i in (0..=program_pointer).rev() {
                        if i == 0 {
                            panic!("No corresponding bracket. {}", program_pointer);
                        }

                        match code[(i - 1) as usize] {
                            Token::LoopIn => {
                                if count == 0 {
                                    program_pointer = i - 1;
                                    break;
                                } else {
                                    count -= 1;
                                }
                            }
                            Token::LoopOut => {
                                count += 1;
                            }
                            _ => (),
                        }
                    }
                }
            },
            Token::Print => {
                let c = memory[pointer as usize] as char;

                print!("{}", c);
                if c == '\n' {
                    print!("  ");
                }
            },
            Token::Read => {
                memory[pointer as usize] = input_vec[input_counter] as u8;
                input_counter += 1;
            },
        }

        program_pointer += 1;
    }

    println!();
}
