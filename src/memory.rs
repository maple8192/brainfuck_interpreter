use std::fmt::{self, Display, Formatter};

/// Represents a memory.
#[derive(Debug)]
pub struct Memory {
    memory: Vec<u8>,
    pointer: usize,
}

impl Memory {
    /// Returns the current value at the pointer.
    pub fn get(&self) -> u8 {
        self.memory[self.pointer]
    }

    /// Increments the value at the pointer.
    /// If the value overflows, it will be 0.
    pub fn increment(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
    }

    /// Decrements the value at the pointer.
    /// If the value underflows, it will be 255.
    pub fn decrement(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
    }

    /// Shifts the pointer to the right.
    pub fn shift_right(&mut self) {
        self.pointer += 1;
        if self.pointer >= self.memory.len() {
            self.memory.push(0);
        }
    }

    /// Shifts the pointer to the left.
    ///
    /// # Errors
    ///
    /// Returns an error if the pointer underflows.
    pub fn shift_left(&mut self) -> anyhow::Result<()> {
        if self.pointer == 0 {
            return Err(anyhow::anyhow!("Pointer out of bounds"));
        }
        self.pointer -= 1;
        Ok(())
    }

    /// Sets the value at the pointer.
    pub fn set(&mut self, value: u8) {
        self.memory[self.pointer] = value;
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: vec![0],
            pointer: 0,
        }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        write!(
            f,
            "{}",
            self.memory
                .iter()
                .enumerate()
                .map(|(i, &v)| {
                    if i == self.pointer {
                        format!("<{v}>")
                    } else {
                        format!("{v}")
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        write!(f, "]")?;
        Ok(())
    }
}
