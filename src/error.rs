use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ProgramError<'a> {
    src: &'a str,
    pos: usize,
    message: &'a str
}

impl<'a> ProgramError<'a> {
    pub fn new(src: &'a str, pos: usize, message: &'a str) -> Self {
        ProgramError { src, pos, message }
    }
}

impl Display for ProgramError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let src = self.src.chars().collect::<Vec<char>>();

        let mut line = 0;
        let mut last_head = 0;
        for (p, c) in src.iter().enumerate() {
            if p == self.pos { break }

            if *c == '\n' {
                line += 1;
                last_head = p + 1;
            }
        }

        let mut p = last_head;
        let mut code = String::new();
        while p < src.len() && src[p] != '\n' {
            code.push(src[p]);
            p += 1;
        }

        let col = self.pos - last_head;

        writeln!(f, "an error occurred ({}:{}): {}", line + 1, col + 1, self.message)?;

        let number = format!("  {} | ", line + 1);
        writeln!(f, "{number}{code}")?;

        let mut space = String::new();
        for _ in 0..(col + number.chars().count() + 1) {
            space.push(' ');
        }
        write!(f, "{}^", space)
    }
}

impl Error for ProgramError<'_> {}