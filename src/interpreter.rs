use std::fmt::Debug;
use crate::error::InterpreterError;
use crate::machine::Machine;

pub struct Interpreter {
    machine: Machine,
}

impl Interpreter {
    pub fn new(machine: Machine) -> Self {
        Interpreter { machine }
    }

    pub fn run(&mut self) {
        println!("Output:");
        print!("  ");

        loop {
            let (end, output) = match self.machine.step() {
                Ok(r) => r,
                Err(e) => {
                    println!();
                    println!();
                    println!("Error:");
                    print!("  {}", e.to_string());

                    break;
                },
            };

            if let Some(o) = output {
                print!("{}", o);

                if o == '\n' {
                    print!("  ");
                }
            }

            if end {
                break;
            }
        }

        println!();
    }
}
