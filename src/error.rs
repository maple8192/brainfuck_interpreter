use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct InterpreterError {
    message: String,
}

impl InterpreterError {
    pub fn new(message: String) -> Self {
        InterpreterError { message }
    }
}

impl Debug for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InterpreterError { }
