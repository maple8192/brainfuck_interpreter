/// Represents a position in the source code.
#[derive(Debug, Clone, Copy)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    /// Moves the position to the next position.
    pub fn advance(&mut self) {
        self.col += 1;
    }

    /// Moves the position to the next line.
    pub fn next_line(&mut self) {
        self.row += 1;
        self.col = 1;
    }

    /// Returns the current row.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the current column.
    pub fn col(&self) -> usize {
        self.col
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { row: 1, col: 1 }
    }
}
