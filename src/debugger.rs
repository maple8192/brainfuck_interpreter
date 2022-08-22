mod tick_pulsar;
mod key_event_listener;
mod renderer;

use std::io::stdout;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::{execute, terminal};
use crossterm::cursor::Hide;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crate::Machine;
use crate::debugger::renderer::Renderer;

pub struct Debugger {
    machine: Machine,
    terminal_row: u16,
    terminal_col: u16,
    output: String,
    error: Option<String>,
    tick_count: u32,
    auto_step_level: u32,
    renderer: Renderer,
}

impl Debugger {
    pub fn new(machine: Machine) -> Self {
        Debugger { machine, terminal_row: 0, terminal_col: 0, output: String::new(), error: None, tick_count: 0, auto_step_level: 0, renderer: Renderer::new() }
    }

    pub fn debug_run(&mut self) {
        enable_raw_mode().unwrap();
        execute!(
            stdout(),
            EnterAlternateScreen,
            Hide,
        ).unwrap();

        (self.terminal_col, self.terminal_row) = terminal::size().unwrap();

        self.renderer.set_terminal_size(self.terminal_col, self.terminal_row);
        self.renderer.render(&self.machine, self.auto_step_level, self.output.clone(), self.error.clone());

        let tick_receiver = tick_pulsar::create_tick_receiver();
        let key_event_receiver = key_event_listener::create_key_event_receiver();

        loop {
            let tick = tick_receiver.try_recv();
            let key_event = key_event_receiver.try_recv();

            if let Ok(()) = tick {
                let (current_terminal_col, current_terminal_row) = terminal::size().unwrap();
                if self.terminal_col != current_terminal_col || self.terminal_row != current_terminal_row {
                    (self.terminal_col, self.terminal_row) = (current_terminal_col, current_terminal_row);
                    self.renderer.set_terminal_size(self.terminal_col, self.terminal_row);
                    self.renderer.render(&self.machine, self.auto_step_level, self.output.clone(), self.error.clone());
                }

                match self.auto_step_level {
                    1 => if self.tick_count % 25 == 0 { self.next_step(true); }
                    2 => if self.tick_count % 12 == 0 { self.next_step(true); }
                    3 => if self.tick_count % 6 == 0 { self.next_step(true); }
                    4 => if self.tick_count % 3 == 0 { self.next_step(true); }
                    5 => self.next_step(true),
                    6 => { self.next_step(false); self.next_step(true); }
                    7 => { for _ in 0..4 { self.next_step(false); } self.renderer.render(&self.machine, self.auto_step_level, self.output.clone(), self.error.clone()); }
                    8 => { for _ in 0..8 { self.next_step(false); } self.renderer.render(&self.machine, self.auto_step_level, self.output.clone(), self.error.clone()); }
                    9 => { for _ in 0..16 { self.next_step(false); } self.renderer.render(&self.machine, self.auto_step_level, self.output.clone(), self.error.clone()); }
                    10 => { for _ in 0..100 { self.next_step(false); } self.renderer.render(&self.machine, self.auto_step_level, self.output.clone(), self.error.clone()); }
                    _ => (),
                }

                self.tick_count += 1;
            }
            if let Ok(event) = key_event {
                match event {
                    KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::NONE } => break,
                    KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::NONE } => self.next_step(true),
                    KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE } => if self.auto_step_level < 10 { self.auto_step_level += 1; self.renderer.render(&self.machine, self.auto_step_level, self.output.clone(), self.error.clone()); },
                    KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::NONE } => if self.auto_step_level > 0 { self.auto_step_level -= 1; self.renderer.render(&self.machine, self.auto_step_level, self.output.clone(), self.error.clone()); },
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

    fn next_step(&mut self, render: bool) {
        if let None = self.error {
            let (end, output) = match self.machine.step() {
                Ok(r) => r,
                Err(e) => {
                    self.error = Some(e.to_string());
                    if render { self.renderer.render(&self.machine, self.auto_step_level, self.output.clone(), self.error.clone()); }
                    return;
                },
            };

            if let Some(o) = output {
                self.output.push(o);
            }

            if end {
                return;
            }

            if render { self.renderer.render(&self.machine, self.auto_step_level, self.output.clone(), self.error.clone()); }
        }
    }
}