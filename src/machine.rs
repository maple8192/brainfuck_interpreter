use crate::error::InterpreterError;
use crate::Token;

pub struct Machine {
    pub code: Vec<Token>,
    pub input: Vec<char>,
    pub program_pointer: usize,
    pub pointer: isize,
    pub memory: Vec<u8>,
    pub input_pointer: usize,
}

impl Machine {
    pub fn new(code: Vec<Token>, input: String) -> Self {
        Machine {
            code,
            input: input.chars().collect(),
            program_pointer: 0,
            pointer: 0,
            memory: vec![0],
            input_pointer: 0,
        }
    }

    fn increment(&mut self) {
        self.memory[self.pointer as usize] = self.memory[self.pointer as usize].wrapping_add(1);
    }

    fn decrement(&mut self) {
        self.memory[self.pointer as usize] = self.memory[self.pointer as usize].wrapping_sub(1);
    }

    fn increment_pointer(&mut self) {
        self.pointer += 1;
        if self.pointer >= self.memory.len() as isize {
            self.memory.push(0);
        }
    }

    fn decrement_pointer(&mut self) -> Result<(), InterpreterError> {
        self.pointer -= 1;
        if self.pointer < 0 {
            return Err(InterpreterError::new("ポインタの値が負です。".to_string()));
        }

        Ok(())
    }

    fn is_zero(&self) -> bool {
        self.memory[self.pointer as usize] == 0
    }

    fn jump_to_close_bracket(&mut self) -> Result<(), InterpreterError> {
        let mut count = 0;
        for i in (self.program_pointer + 1)..=self.code.len() {
            if i == self.code.len() {
                return Err(InterpreterError::new("対応する]が見つかりません。".to_string()));
            }

            match self.code[i] {
                Token::LoopIn => count += 1,
                Token::LoopOut => if count == 0 {
                    self.program_pointer = i;
                    break;
                } else {
                    count -= 1;
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn jump_to_open_bracket(&mut self) -> Result<(), InterpreterError> {
        let mut count = 0;
        for i in (0..=self.program_pointer).rev() {
            if i == 0 {
                return Err(InterpreterError::new("対応する[が見つかりません。".to_string()));
            }

            match self.code[i - 1] {
                Token::LoopIn => if count == 0 {
                    self.program_pointer = i - 1;
                    break;
                } else {
                    count -= 1;
                }
                Token::LoopOut => count += 1,
                _ => (),
            }
        }

        Ok(())
    }

    fn get_character(&self) -> char {
        self.memory[self.pointer as usize] as char
    }

    fn set_character(&mut self, c: char) {
        self.memory[self.pointer as usize] = c as u8;
    }

    pub fn step(&mut self) -> Result<(bool, Option<char>), InterpreterError> {
        if self.program_pointer >= self.code.len() {
            return Ok((true, None));
        }

        let current_command = &self.code[self.program_pointer];

        let mut output = None;

        match current_command {
            Token::Inc => { self.increment(); Ok(()) },
            Token::Dec => { self.decrement(); Ok(()) },
            Token::IncPtr => { self.increment_pointer(); Ok(()) },
            Token::DecPtr => self.decrement_pointer(),
            Token::LoopIn => if self.is_zero() { self.jump_to_close_bracket() } else { Ok(()) },
            Token::LoopOut => if self.is_zero() { Ok(()) } else { self.jump_to_open_bracket() },
            Token::Print => { output = Some(self.get_character()); Ok(()) },
            Token::Read => {
                self.set_character(if self.input_pointer < self.input.len() { self.input[self.input_pointer] } else { 0 as char });
                self.input_pointer += 1; Ok(())
            },
        }?;

        self.program_pointer += 1;

        Ok((false, output))
    }
}
