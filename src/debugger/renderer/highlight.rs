use crossterm::style::Color;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum HighLight {
    Code,
    Memory,
}

impl HighLight {
    pub fn get_color(color: HighLight) -> (Color, Color) {
        match color {
            HighLight::Code => (Color::Red, Color::Black),
            HighLight::Memory => (Color::Cyan, Color::Black),
        }
    }
}
