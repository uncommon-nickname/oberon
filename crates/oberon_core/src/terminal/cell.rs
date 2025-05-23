use crate::style::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell
{
    pub char: char,
    pub bg: Color,
    pub fg: Color,
}

impl Cell
{
    pub const EMPTY: Self = Self::new(' ');

    pub const fn new(char: char) -> Self
    {
        Self {
            char,
            bg: Color::Default,
            fg: Color::Default,
        }
    }

    pub const fn bg(mut self, bg: Color) -> Self
    {
        self.bg = bg;
        self
    }

    pub const fn fg(mut self, fg: Color) -> Self
    {
        self.fg = fg;
        self
    }
}
