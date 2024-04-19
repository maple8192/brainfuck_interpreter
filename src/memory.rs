#[derive(Debug)]
pub struct Memory {
    memory: Vec<u8>,
    pointer: usize
}

impl Memory {
    pub fn new() -> Self {
        Memory { memory: vec![0], pointer: 0 }
    }

    pub fn get(&self) -> u8 {
        self.memory[self.pointer]
    }

    pub fn set(&mut self, value: u8) {
        self.memory[self.pointer] = value;
    }

    pub fn increment(&mut self) {
        self.set(self.memory[self.pointer].wrapping_add(1));
    }

    pub fn decrement(&mut self) {
        self.set(self.memory[self.pointer].wrapping_sub(1));
    }
    
    pub fn shift_right(&mut self) {
        self.pointer += 1;
        if self.pointer >= self.memory.len() {
            self.memory.push(0);
        }
    }
    
    pub fn shift_left(&mut self) -> Result<(), ()> {
        if self.pointer == 0 {
            return Err(());
        }
        self.pointer -= 1;
        Ok(())
    }

    pub fn memory(&self) -> &Vec<u8> {
        &self.memory
    }

    pub fn pointer(&self) -> usize {
        self.pointer
    }
}