use std::io::stdout;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::{execute, terminal};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crate::{Machine, Token};

pub struct Debugger {
    machine: Machine,
    terminal_row: u16,
    terminal_col: u16,
    output: String,
}

impl Debugger {
    pub fn new(machine: Machine) -> Self {
        Debugger { machine, terminal_row: 0, terminal_col: 0, output: Stirng::new() }
    }

    pub fn debug_run(&mut self) {
        enable_raw_mode().unwrap();
        execute!(
            stdout(),
            EnterAlternateScreen,
        ).unwrap();

        (self.terminal_col, self.terminal_row) = terminal::size().unwrap();

        self.render();

        let terminal_size_receiver = self.observe_terminal_size();
        let key_input_receiver = self.observe_key_input();
        loop {
            let current_terminal_size = terminal_size_receiver.try_recv();
            let key_input = key_input_receiver.try_recv();

            if let Ok((current_col, current_row)) = current_terminal_size {
                if current_row != self.terminal_row || current_col != self.terminal_col {
                    self.terminal_row = current_row;
                    self.terminal_col = current_col;
                    self.render();
                }
            }
            if let Ok(event) = key_input {
                match event {
                    KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL } => break,
                    _ => (),
                }
            }
        }

        execute!(
            stdout(),
            LeaveAlternateScreen,
        ).unwrap();
        disable_raw_mode().unwrap();
    }

    fn render(&self) {
        execute!(
            stdout(),
            Clear(ClearType::All),
        ).unwrap();

        // デバッグモードの説明
        execute!(
            stdout(),
            MoveTo(1, 0),
            Print(format!("\"▶\" : Next step     \"Ctrl+c\" : Terminate")),
        ).unwrap();

        // ソースコードの表示
        let mut code_str = String::new();
        for i in 0..self.machine.code.len() {
            if i != 0 && i % (self.terminal_col - 6) as usize == 0 {
                code_str.push_str("\r\n   ");
            }

            match self.machine.code[i] {
                Token::Inc => code_str.push('+'),
                Token::Dec => code_str.push('-'),
                Token::IncPtr => code_str.push('>'),
                Token::DecPtr => code_str.push('<'),
                Token::LoopIn => code_str.push('['),
                Token::LoopOut => code_str.push(']'),
                Token::Print => code_str.push('.'),
                Token::Read => code_str.push(','),
            }
        }
        execute!(
            stdout(),
            MoveTo(1, 2),
            Print("Code:"),
            MoveTo(3, 3),
            Print(code_str),
        ).unwrap();

        // メモリの表示
        let mut memory_str = String::new();
        for i in 0..self.machine.memory.len() {
            if i != 0 && i % ((self.terminal_col - 6) / 5) as usize == 0 {
                memory_str.push_str("\r\n   ");
                for _ in 0..(i / ((self.terminal_col - 6) / 5) as usize) {
                    memory_str.push(' ');
                }
            }

            memory_str.push_str(format!("{:>5}", self.machine.memory[i]).as_str());
        }
        execute!(
            stdout(),
            MoveTo(1, 11),
            Print("Memory:"),
            MoveTo(3, 12),
            Print(memory_str),
        ).unwrap();

        // 入力の表示
        let mut input_str = String::new();
        for i in 0..self.machine.input.len() {
            match self.machine.input[i] {
                '\n' => input_str.push_str("\n   "),
                _ => input_str.push(self.machine.input[i]),
            }
        }
        execute!(
            stdout(),
            MoveTo(1, 20),
            Print("Input:"),
            MoveTo(3, 21),
            Print(input_str),
        ).unwrap();

        // 出力の表示
        let mut output_str = String::new();
        for i in 0..self.output.len() {
            match self.output.chars().nth(i).unwrap() {
                '\n' => output_str.push_str("\n   "),
                _ => output_str.push(self.output.chars().nth(i).unwrap()),
            }
        }
        execute!(
            stdout(),
            MoveTo(1, 29),
            Print("Output:"),
            MoveTo(3, 30),
            Print(output_str),
        ).unwrap();

        // エラーの表示
    }

    fn observe_terminal_size(&self) -> Receiver<(u16, u16)> {
        let (tx, rx) = mpsc::channel::<(u16, u16)>();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(20));
                tx.send(terminal::size().unwrap()).unwrap();
            }
        });

        rx
    }

    fn observe_key_input(&self) -> Receiver<KeyEvent> {
        let (tx, rx) = mpsc::channel::<KeyEvent>();

        thread::spawn(move || {
            loop {
                let event = read().unwrap();

                if let Event::Key(e) = event {
                    tx.send(e).unwrap();
                }
            }
        });

        rx
    }
}