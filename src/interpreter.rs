use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
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
            let (end, output) = match self.machine.step::<MyError>() {
                Ok(r) => r,
                Err(_) => break,
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

// 暫定の自作Error
struct MyError;

impl Debug for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for MyError {

}
