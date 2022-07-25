use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crate::Machine;

pub struct Debugger {
    machine: Machine,
    count: u32,
}

impl Debugger {
    pub fn new(machine: Machine) -> Self {
        Debugger { machine, count: 0 }
    }

    pub fn debug_run(&mut self) {
        enable_raw_mode().unwrap();
        execute!(
            stdout(),
            EnterAlternateScreen,
        ).unwrap();

        self.render();

        loop {
            let event = read().unwrap();

            match event {
                Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL }) => break,
                Event::Key(_) => { self.count += 1; self.render(); },
                _ => (),
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
            Print(format!("Count: {}", self.count)),
        ).unwrap();
    }
}