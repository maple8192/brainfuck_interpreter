use crate::debugger::renderer::highlight::HighLight;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct TerminalChar {
    pub char: u8,
    pub highlight: Option<HighLight>,
}
