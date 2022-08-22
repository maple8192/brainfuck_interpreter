use std::cmp::min;
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::{Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crate::debugger::renderer::terminal_char::TerminalChar;
use crate::{Machine, Token};
use crate::debugger::renderer::highlight::HighLight;

mod terminal_char;
mod highlight;

pub struct Renderer {
    cache: Vec<Vec<TerminalChar>>,
    terminal_col: u16,
    terminal_row: u16,
    rerender: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer { cache: Vec::new(), terminal_col: 0, terminal_row: 0, rerender: true }
    }

    pub fn set_terminal_size(&mut self, terminal_col: u16, terminal_row: u16) {
        self.terminal_col = terminal_col;
        self.terminal_row = terminal_row;

        self.rerender = true;
    }

    pub fn render(&mut self, machine: &Machine, auto_step_level: u32, output: String, error: Option<String>) {
        let width_limit = self.terminal_col as usize - 6;

        let mut display = Vec::<Vec<TerminalChar>>::new();

        // デバッグモードの説明
        display.push(create_str(format!("\"Right\" : Next Step, Auto Stepper({} steps/sec)[\"Up\" : Speed Up, \"Down\" : Speed Down], \"Esc\" : Exit", match auto_step_level {
            1 => 2,
            2 => 4,
            3 => 8,
            4 => 16,
            5 => 50,
            6 => 100,
            7 => 200,
            8 => 400,
            9 => 800,
            10 => 5000,
            _ => 0,
        }).as_str(), None));

        display.push(Vec::new());

        // ソースコード
        display.push(create_str("Code:", None));

        let mut code_lines = Vec::<Vec<TerminalChar>>::new();
        if machine.code.len() <= width_limit * 7 {
            for i in 0..machine.code.len() {
                if i % width_limit == 0 {
                    code_lines.push(create_str("  ", None));
                }

                code_lines.last_mut().unwrap().push(TerminalChar { char: token_to_char(&machine.code[i]) as u8, highlight: if i == machine.program_pointer { Some(HighLight::Code) } else { None } });
            }
        } else {
            if machine.program_pointer < width_limit * 6 {
                for i in 0..width_limit * 6 {
                    if i % width_limit == 0 {
                        code_lines.push(create_str("  ", None));
                    }

                    code_lines.last_mut().unwrap().push(TerminalChar { char: token_to_char(&machine.code[i]) as u8, highlight: if i == machine.program_pointer { Some(HighLight::Code) } else { None } });
                }
                code_lines.push(create_str("  ...", None));
            } else {
                let line = machine.program_pointer / width_limit;
                let block = (line - 1) / 5;

                let last_line = (machine.code.len() - 1) / width_limit;
                let last_block_temp = (last_line - 1) / 5;
                let last_block = if (last_block_temp * 5) + 1 == last_line { last_block_temp - 1 } else { last_block_temp };

                if block == last_block {
                    code_lines.push(create_str("  ...", None));
                    for i in (block * 5 + 1) * width_limit..machine.code.len() {
                        if i % width_limit == 0 {
                            code_lines.push(create_str("  ", None));
                        }

                        code_lines.last_mut().unwrap().push(TerminalChar { char: token_to_char(&machine.code[i]) as u8, highlight: if i == machine.program_pointer { Some(HighLight::Code) } else { None } });
                    }
                } else {
                    code_lines.push(create_str("  ...", None));
                    for i in (block * 5 + 1) * width_limit..((block + 1) * 5 + 1) * width_limit {
                        if i % width_limit == 0 {
                            code_lines.push(create_str("  ", None));
                        }

                        code_lines.last_mut().unwrap().push(TerminalChar { char: token_to_char(&machine.code[i]) as u8, highlight: if i == machine.program_pointer { Some(HighLight::Code) } else { None } });
                    }
                    code_lines.push(create_str("  ...", None));
                }
            }
        }

        for itr in code_lines.iter() {
            display.push(itr.clone());
        }

        display.push(Vec::new());

        // メモリ
        display.push(create_str("Memory:", None));

        let memory_width_limit = width_limit / 5;

        let mut memory_lines = Vec::<Vec<TerminalChar>>::new();
        if machine.memory.len() <= memory_width_limit * 7 {
            for i in 0..machine.memory.len() {
                if i % memory_width_limit == 0 {
                    memory_lines.push(create_str("", None));
                }

                push_str(memory_lines.last_mut().unwrap(), "  ", None);
                push_str(memory_lines.last_mut().unwrap(), format!("{:>3}", machine.memory[i]).as_str(), if i == machine.pointer as usize { Some(HighLight::Memory) } else { None })
            }
        } else {
            if (machine.pointer as usize) < memory_width_limit * 6 {
                for i in 0..memory_width_limit * 6 {
                    if i % memory_width_limit == 0 {
                        memory_lines.push(create_str("", None));
                    }

                    push_str(memory_lines.last_mut().unwrap(), "  ", None);
                    push_str(memory_lines.last_mut().unwrap(), format!("{:>3}", machine.memory[i]).as_str(), if i == machine.pointer as usize { Some(HighLight::Memory) } else { None })
                }
                memory_lines.push(create_str("  ...", None));
            } else {
                let line = machine.pointer as usize / memory_width_limit;
                let block = (line - 1) / 5;

                // メモリは可変長のため、コード部分のように最終ブロックだけを6行にはしない。(表示が少しおかしくなるため)
                let last_line = (machine.memory.len() - 1) / memory_width_limit;
                let last_block = (last_line - 1) / 5;

                if block == last_block {
                    memory_lines.push(create_str("  ...", None));
                    for i in ((block * 5) + 1) * memory_width_limit..machine.memory.len() {
                        if i % memory_width_limit == 0 {
                            memory_lines.push(create_str("", None));
                        }

                        push_str(memory_lines.last_mut().unwrap(), "  ", None);
                        push_str(memory_lines.last_mut().unwrap(), format!("{:>3}", machine.memory[i]).as_str(), if i == machine.pointer as usize { Some(HighLight::Memory) } else { None })
                    }
                } else {
                    memory_lines.push(create_str("  ...", None));
                    for i in ((block * 5) + 1) * memory_width_limit..min(((block + 1) * 5 + 1) * memory_width_limit, machine.memory.len()) {
                        if i % memory_width_limit == 0 {
                            memory_lines.push(create_str("", None));
                        }

                        push_str(memory_lines.last_mut().unwrap(), "  ", None);
                        push_str(memory_lines.last_mut().unwrap(), format!("{:>3}", machine.memory[i]).as_str(), if i == machine.pointer as usize { Some(HighLight::Memory) } else { None })
                    }
                    memory_lines.push(create_str("  ...", None));
                }
            }
        }

        for itr in memory_lines.iter() {
            display.push(itr.clone());
        }

        display.push(Vec::new());

        // 入力
        display.push(create_str("Input:", None));

        let mut input_lines = Vec::<Vec<TerminalChar>>::new();
        input_lines.push(create_str("  ", None));
        for itr in machine.input.iter() {
            if *itr == '\n' {
                if input_lines.len() == 6 {
                    input_lines.push(create_str("  ...", None));
                    break;
                } else {
                    input_lines.push(create_str("  ", None));
                }
            } else {
                input_lines.last_mut().unwrap().push(TerminalChar { char: *itr as u8, highlight: None });
            }
        }

        for itr in input_lines.iter() {
            display.push(itr.clone());
        }

        display.push(Vec::new());

        // 出力
        display.push(create_str("Output:", None));

        let mut output_lines = Vec::<Vec<TerminalChar>>::new();
        output_lines.push(create_str("  ", None));
        for itr in output.chars() {
            if itr == '\n' {
                output_lines.push(create_str("  ", None));
            } else {
                output_lines.last_mut().unwrap().push(TerminalChar { char: itr as u8, highlight: None });
            }
        }

        if output_lines.len() <= 7 {
            for itr in output_lines.iter() {
                display.push(itr.clone());
            }
        } else {
            display.push(create_str("  ...", None));
            for i in output_lines.len() - 6..output_lines.len() {
                display.push(output_lines[i].clone());
            }
        }

        display.push(Vec::new());

        // エラー
        display.push(create_str("Error:", None));

        if let Some(str) = error {
            display.push(create_str(format!("  {}", str).as_str(), None));
        }

        // 表示
        if self.rerender {
            execute!(
                stdout(),
                Clear(ClearType::All),
            ).unwrap();

            for i in 0..min(self.terminal_row as usize, display.len()) {
                execute!(
                    stdout(),
                    MoveTo(0, i as u16),
                ).unwrap();

                for j in display[i].iter() {
                    if let Some(c) = j.highlight {
                        execute!(
                            stdout(),
                            SetBackgroundColor(HighLight::get_color(c).0),
                            SetForegroundColor(HighLight::get_color(c).1),
                        ).unwrap();
                    } else {
                        execute!(
                            stdout(),
                            ResetColor,
                        ).unwrap();
                    }

                    execute!(
                        stdout(),
                        Print(format!("{}", j.char as char)),
                    ).unwrap();
                }
            }

            self.cache = display;
            self.rerender = false;
        } else {
            for i in 0..self.terminal_row as usize {
                for j in 0..self.terminal_col as usize {
                    if display.len() <= i {
                        if self.cache.len() > i {
                            execute!(
                                stdout(),
                                MoveTo(0, i as u16),
                                Clear(ClearType::CurrentLine),
                            ).unwrap();
                        }
                        continue;
                    }
                    if display[i].len() <= j {
                        if self.cache.len() > i && self.cache[i].len() > j {
                            execute!(
                                stdout(),
                                MoveTo(j as u16, i as u16),
                                ResetColor,
                                Print(" "),
                            ).unwrap();
                        }
                        continue;
                    }
                    if self.cache.len() <= i || self.cache[i].len() <= j || self.cache[i][j] != display[i][j] {
                        execute!(
                            stdout(),
                            MoveTo(j as u16, i as u16),
                        ).unwrap();

                        if let Some(c) = display[i][j].highlight {
                            execute!(
                                stdout(),
                                SetBackgroundColor(HighLight::get_color(c).0),
                                SetForegroundColor(HighLight::get_color(c).1),
                            ).unwrap();
                        } else {
                            execute!(
                                stdout(),
                                ResetColor,
                            ).unwrap();
                        }

                        execute!(
                            stdout(),
                            Print(format!("{}", display[i][j].char as char)),
                        ).unwrap();
                    }
                }
            }

            self.cache = display.clone();
        }
    }
}

fn push_str(origin: &mut Vec<TerminalChar>, str: &str, highlight: Option<HighLight>) {
    for c in str.chars() {
        origin.push(TerminalChar { char: c as u8, highlight });
    }
}

fn create_str(str: &str, highlight: Option<HighLight>) -> Vec<TerminalChar> {
    let mut ret = Vec::new();
    for c in str.chars() {
        ret.push(TerminalChar { char: c as u8, highlight });
    }
    ret
}

fn token_to_char(token: &Token) -> char {
    match token {
        Token::Inc => '+',
        Token::Dec => '-',
        Token::IncPtr => '>',
        Token::DecPtr => '<',
        Token::LoopIn => '[',
        Token::LoopOut => ']',
        Token::Print => '.',
        Token::Read => ',',
    }
}
