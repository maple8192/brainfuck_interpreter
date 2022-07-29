use std::io::stdout;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::cmp::min;
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
    auto_step_tick_count: u32,
    auto_step_speed: u32,
}

impl Debugger {
    pub fn new(machine: Machine) -> Self {
        Debugger { machine, terminal_row: 0, terminal_col: 0, output: String::new(), error: None, terminal_cache: Vec::new(), auto_step_tick_count: 0, auto_step_speed: 0 }
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
        let auto_step_tick_receiver = self.auto_step_tick();
        loop {
            let current_terminal_size = terminal_size_receiver.try_recv();
            let key_input = key_input_receiver.try_recv();
            let auto_step_tick = auto_step_tick_receiver.try_recv();

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
                    KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::NONE } => self.next_step(true),
                    KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE } => if self.auto_step_speed < 10 { self.auto_step_speed += 1; self.render(); },
                    KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::NONE } => if self.auto_step_speed > 0 { self.auto_step_speed -= 1; self.render(); },
                    _ => (),
                }
            }
            if let Ok(()) = auto_step_tick {
                match self.auto_step_speed {
                    1 => if self.auto_step_tick_count % 25 == 0 { self.next_step(true); }
                    2 => if self.auto_step_tick_count % 12 == 0 { self.next_step(true); }
                    3 => if self.auto_step_tick_count % 6 == 0 { self.next_step(true); }
                    4 => if self.auto_step_tick_count % 3 == 0 { self.next_step(true); }
                    5 => self.next_step(true),
                    6 => { self.next_step(false); self.next_step(true); }
                    7 => { for _ in 0..4 { self.next_step(false); } self.render(); }
                    8 => { for _ in 0..8 { self.next_step(false); } self.render(); }
                    9 => { for _ in 0..16 { self.next_step(false); } self.render(); }
                    10 => { for _ in 0..100 { self.next_step(false); } self.render(); }
                    _ => (),
                }

                self.auto_step_tick_count += 1;
            }
        }

        execute!(
            stdout(),
            LeaveAlternateScreen,
        ).unwrap();
        disable_raw_mode().unwrap();
    }

    fn next_step(&mut self, render: bool) {
        if let None = self.error {
            let (end, output) = match self.machine.step() {
                Ok(r) => r,
                Err(e) => {
                    self.error = Some(e.to_string());
                    if render { self.render(); }
                    return;
                },
            };

            if let Some(o) = output {
                self.output.push(o);
            }

            if end {
                return;
            }

            if render {self.render(); }
        }
    }

    fn render(&mut self) {
        let mut display = Vec::<String>::new();
        for _ in 0..self.terminal_row { display.push("".to_string()); }

        let mut current_line;

        // デバッグモードの説明
        display[0] = format!("\"▶\" : Next step     \"▲\" : Auto Step Speed Up     \"▼\" : Auto Step Speed Down     (Current Auto Step : {} step/s)     \"Esc\" : Exit", match self.auto_step_speed {
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
        });

        // ソースコードの表示
        display[2] = "Code:".to_string();

        let mut code_lines = Vec::<String>::new();
        let mut code_line_count = 0;
        for i in 0..self.machine.code.len() {
            if i % (self.terminal_col - 6) as usize == 0 {
                code_lines.push("".to_string());

                if i != 0 {
                    code_line_count += 1;
                }
            }

            match self.machine.code[i] {
                Token::Inc => code_lines[code_line_count].push('+'),
                Token::Dec => code_lines[code_line_count].push('-'),
                Token::IncPtr => code_lines[code_line_count].push('>'),
                Token::DecPtr => code_lines[code_line_count].push('<'),
                Token::LoopIn => code_lines[code_line_count].push('['),
                Token::LoopOut => code_lines[code_line_count].push(']'),
                Token::Print => code_lines[code_line_count].push('.'),
                Token::Read => code_lines[code_line_count].push(','),
            }
        }

        let code_pointer_line = self.machine.program_pointer / (self.terminal_col - 6) as usize;
        if code_lines.len() <= 7 {
            for i in 0..min(code_lines.len(), 7) {
                display[i + 3] = format!("  {}", code_lines[i]);
            }
        } else {
            if code_pointer_line <= 6 {
                for i in 0..6 {
                    display[i + 3] = format!("  {}", code_lines[i]);
                }
                display[9] = "  ...".to_string();
            } else if code_pointer_line > code_lines.len() - 6 {
                display[3] = "  ...".to_string();
                let len = code_lines.len();
                for i in 0..6 {
                    display[i + 4] = format!("  {}", code_lines[len - 6 + i]);
                }
            } else {
                let code_block = (code_pointer_line + 4) / 5 - 1;
                let start_line = 1 + code_block * 5;
                display[3] = "  ...".to_string();
                for i in 0..5 {
                    display[i + 4] = format!("  {}", code_lines[start_line + i]);
                }
                display[9] = "  ...".to_string();
            }
        }

        // メモリの表示
        display[11] = "Memory:".to_string();

        let mut memory_lines = Vec::<String>::new();
        let mut memory_line_count = 0;
        for i in 0..self.machine.memory.len() {
            if i % ((self.terminal_col - 6) / 5) as usize == 0 {
                memory_lines.push("".to_string());

                if i != 0 {
                    memory_line_count += 1;
                }
            }

            memory_lines[memory_line_count].push_str(format!("{:>5}", self.machine.memory[i]).as_str());
        }

        let memory_pointer_line = self.machine.pointer / ((self.terminal_col - 6) / 5) as isize;
        if memory_lines.len() <= 7 {
            for i in 0..min(memory_lines.len(), 7) {
                display[i + 12] = format!("  {}", memory_lines[i]);
            }
        } else {
            if memory_pointer_line <= 6 {
                for i in 0..6 {
                    display[i + 12] = format!("  {}", memory_lines[i]);
                }
                display[18] = "  ...".to_string();
            } else if memory_pointer_line > (memory_lines.len() - 6) as isize {
                display[12] = "  ...".to_string();
                let len = memory_lines.len();
                for i in 0..6 {
                    display[i + 13] = format!("  {}", memory_lines[len - 6 + i]);
                }
            } else {
                let memory_block = (memory_pointer_line + 4) / 5 - 1;
                let start_line = 1 + memory_block * 5;
                display[12] = "  ...".to_string();
                for i in 0..5 {
                    display[i + 13] = format!("  {}", memory_lines[(start_line + i as isize) as usize]);
                }
                display[18] = "  ...".to_string();
            }
        }

        // 入力の表示
        display[20] = "Input:".to_string();
        display[21] = "  ".to_string();

        current_line = 21;
        for i in 0..self.machine.input.len() {
            match self.machine.input[i] {
                '\n' => {
                    current_line += 1;
                    display[current_line] = "  ".to_string();

                    if current_line >= 21 + 6 {
                        display[current_line].push_str("...");
                        break;
                    }
                }
                _ => display[current_line].push(self.machine.input[i]),
            }
        }

        // 出力の表示
        display[29] = "Output:".to_string();
        display[30] = "  ".to_string();

        current_line = 30;
        for i in 0..self.output.len() {
            match self.output.chars().nth(i).unwrap() {
                '\n' => {
                    current_line += 1;
                    display[current_line] = "  ".to_string();

                    if current_line >= 30 + 6 {
                        display[current_line].push_str("...");
                        break;
                    }
                },
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

            execute!(
                stdout(),
                Hide,
            ).unwrap();
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

            execute!(
                stdout(),
                Hide,
            ).unwrap();
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

    fn auto_step_tick(&self) -> Receiver<()> {
        let (tx, rx) = mpsc::channel::<()>();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(20));
                tx.send(()).unwrap();
            }
        });

        rx
    }
}