use std::io::stdout;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use crossterm::cursor::{Hide, MoveTo};
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
    error: Option<String>,
    terminal_cache: Vec<String>,
}

impl Debugger {
    pub fn new(machine: Machine) -> Self {
        Debugger { machine, terminal_row: 0, terminal_col: 0, output: String::new(), error: None, terminal_cache: Vec::new() }
    }

    pub fn debug_run(&mut self) {
        enable_raw_mode().unwrap();
        execute!(
            stdout(),
            EnterAlternateScreen,
            Hide,
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
                    KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE } => break,
                    KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::NONE } => self.next_step(),
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

    fn next_step(&mut self) {
        if let None = self.error {
            let (end, output) = match self.machine.step() {
                Ok(r) => r,
                Err(e) => {
                    self.error = Some(e.to_string());
                    self.render();
                    return;
                },
            };

            if let Some(o) = output {
                self.output.push(o);
            }

            if end {
                return;
            }

            self.render();
        }
    }

    fn render(&mut self) {
        let mut display = Vec::<String>::new();
        for _ in 0..self.terminal_row { display.push("".to_string()); }

        let mut current_line;

        // デバッグモードの説明
        display[0] = "\"▶\" : Next step     \"Esc\" : Exit".to_string();

        // ソースコードの表示
        display[2] = "Code:".to_string();
        display[3] = "  ".to_string();

       current_line = 3;
        for i in 0..self.machine.code.len() {
            if i != 0 && i % (self.terminal_col - 6) as usize == 0 {
                current_line += 1;
                display[current_line] = "  ".to_string();
            }

            match self.machine.code[i] {
                Token::Inc => display[current_line].push('+'),
                Token::Dec => display[current_line].push('-'),
                Token::IncPtr => display[current_line].push('>'),
                Token::DecPtr => display[current_line].push('<'),
                Token::LoopIn => display[current_line].push('['),
                Token::LoopOut => display[current_line].push(']'),
                Token::Print => display[current_line].push('.'),
                Token::Read => display[current_line].push(','),
            }
        }

        // メモリの表示
        display[11] = "Memory:".to_string();
        display[12] = "  ".to_string();

        current_line = 12;
        for i in 0..self.machine.memory.len() {
            if i != 0 && i % ((self.terminal_col - 6) / 5) as usize == 0 {
                current_line += 1;
                display[current_line] = "  ".to_string();
            }

            display[current_line].push_str(format!("{:>5}", self.machine.memory[i]).as_str());
        }

        // 入力の表示
        display[20] = "Input:".to_string();
        display[21] = "  ".to_string();

        current_line = 21;
        for i in 0..self.machine.input.len() {
            match self.machine.input[i] {
                '\n' => { current_line += 1; display[current_line] = "  ".to_string(); }
                _ => display[current_line].push(self.machine.input[i]),
            }
        }

        // 出力の表示
        display[29] = "Output:".to_string();
        display[30] = "  ".to_string();

        current_line = 30;
        for i in 0..self.output.len() {
            match self.output.chars().nth(i).unwrap() {
                '\n' => { current_line += 1; display[current_line] = "  ".to_string() },
                _ => display[current_line].push(self.output.chars().nth(i).unwrap()),
            }
        }

        // エラーの表示
        display[38] = "Error:".to_string();
        display[39] = if let Some(e) = self.error.clone() { format!("  {}", e) } else { "".to_string() };

        // 表示
        if self.terminal_cache.len() != display.len() {
            execute!(
                stdout(),
                Clear(ClearType::All),
            ).unwrap();

            for i in 0..display.len() {
                execute!(
                    stdout(),
                    MoveTo(1, i as u16),
                    Print(display[i].clone()),
                ).unwrap();
            }
        } else {
            for i in 0..display.len() {
                if self.terminal_cache[i] != display[i] {
                    execute!(
                        stdout(),
                        MoveTo(0, i as u16),
                        Clear(ClearType::CurrentLine),
                        MoveTo(1, i as u16),
                        Print(display[i].clone()),
                    ).unwrap();
                }
            }
        }

        self.terminal_cache = display;
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