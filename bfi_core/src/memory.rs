use thiserror::Error;

/// Represents a memory.
pub struct Memory {
    mem: Vec<u8>,
    ptr: usize,
}

impl Memory {
    /// Returns the value at the current pointer.
    pub fn get(&self) -> u8 {
        self.mem[self.ptr]
    }

    /// Sets the value at the current pointer.
    pub fn set(&mut self, value: u8) {
        self.mem[self.ptr] = value;
    }

    /// Increments the value at the current pointer.
    pub fn inc(&mut self) {
        self.set(self.get().wrapping_add(1));
    }

    /// Decrements the value at the current pointer.
    pub fn dec(&mut self) {
        self.set(self.get().wrapping_sub(1));
    }

    /// Moves the pointer to the right.
    /// If the pointer is at the end of the memory, it will extend the memory.
    pub fn shr(&mut self) {
        self.ptr += 1;
        if self.ptr >= self.mem.len() {
            self.mem.push(0);
        }
    }

    /// Moves the pointer to the left.
    ///
    /// # Errors
    ///
    /// Returns an error if the pointer is already at the beginning of the memory.
    pub fn shl(&mut self) -> Result<(), NegPtrError> {
        if self.ptr == 0 {
            return Err(NegPtrError);
        }
        self.ptr -= 1;
        Ok(())
    }

    /// Returns the current memory state.
    pub fn mem(&self) -> &[u8] {
        &self.mem
    }

    /// Returns the current pointer position.
    pub fn ptr(&self) -> usize {
        self.ptr
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            mem: vec![0],
            ptr: 0,
        }
    }
}

/// Represents an error caused by moving the pointer to a negative address.
#[derive(Debug, Error)]
#[error("Pointer moved to a negative address")]
pub struct NegPtrError;
