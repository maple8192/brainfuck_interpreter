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
use crate::Machine;

pub struct Debugger {
    machine: Machine,
    terminal_row: u16,
    terminal_col: u16,
}

impl Debugger {
    pub fn new(machine: Machine) -> Self {
        Debugger { machine, terminal_row: 0, terminal_col: 0 }
    }

    pub fn debug_run(&mut self) {
        enable_raw_mode().unwrap();
        execute!(
            stdout(),
            EnterAlternateScreen,
        ).unwrap();

        (self.terminal_row, self.terminal_col) = terminal::size().unwrap();

        self.render();

        let terminal_size_receiver = self.observe_terminal_size();
        let key_input_receiver = self.observe_key_input();
        loop {
            let current_terminal_size = terminal_size_receiver.try_recv();
            let key_input = key_input_receiver.try_recv();

            if let Ok((current_row, current_col)) = current_terminal_size {
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
            MoveTo(5, 5),
            Print(format!("Row: {}, Column: {}", self.terminal_row, self.terminal_col)),
        ).unwrap();
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