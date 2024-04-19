use std::io;
use std::io::Write;
use crate::memory::Memory;
use crate::token::Token;

pub enum IO {
    Out,
    In
}

pub struct Logger<L: Write> {
    log: L,
    src: Vec<char>,
    is_first_line: bool
}

impl<L: Write> Logger<L> {
    pub fn new(log: L, src: &str) -> Self {
        Logger { log, src: src.chars().collect(), is_first_line: true }
    }

    pub fn log(&mut self, step: usize, token: Token, memory: &Memory, io: Option<IO>) -> io::Result<()> {
        if !self.is_first_line {
            writeln!(self.log)?;
            writeln!(self.log, "----------------------------------------")?;
            writeln!(self.log)?;
        }
        self.is_first_line = false;
        
        writeln!(self.log, "step: {}", step)?;
        writeln!(self.log)?;

        writeln!(self.log, "code:")?;
        self.write_program(token)?;
        writeln!(self.log)?;

        writeln!(self.log, "memory({}):", memory.pointer())?;
        self.write_memory(memory)?;

        if let Some(io) = io {
            writeln!(self.log)?;
            match io {
                IO::Out => writeln!(self.log, "output: '{}' ({})", memory.get() as char, memory.get())?,
                IO::In => writeln!(self.log, "input: '{}' ({})", memory.get() as char, memory.get())?
            }
        }

        Ok(())
    }

    fn write_program(&mut self, token: Token) -> io::Result<()> {
        let mut line = 0;
        let mut last_head = 0;
        for (p, &c) in self.src.iter().enumerate() {
            if p == token.pos { break }

            if c == '\n' {
                line += 1;
                last_head = p + 1;
            }
        }

        let mut p = last_head;
        let mut code = String::new();
        while p < self.src.len() && self.src[p] != '\n' {
            code.push(self.src[p]);
            p += 1;
        }

        let col = token.pos - last_head;

        let number = format!("  {} | ", line + 1);
        writeln!(self.log, "{number}{code}")?;

        let mut space = String::new();
        for _ in 0..(col + number.chars().count()) {
            space.push(' ');
        }
        write!(self.log, "{}^", space)?;
        writeln!(self.log)?;

        Ok(())
    }

    fn write_memory(&mut self, memory: &Memory) -> io::Result<()> {
        let mem = memory.memory();
        let ptr = memory.pointer();

        for &m in mem {
            write!(self.log, "[{m:>3}]")?;
        }
        writeln!(self.log)?;

        for _ in 0..ptr {
            write!(self.log, "     ")?;
        }
        writeln!(self.log, "  ^")?;

        Ok(())
    }
}