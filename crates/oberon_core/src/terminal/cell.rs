use crate::style::color::Color;

#[derive(Clone, Debug)]
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

    pub const fn with_bg(mut self, bg: Color) -> Self
    {
        self.bg = bg;
        self
    }

    pub const fn with_fg(mut self, fg: Color) -> Self
    {
        self.fg = fg;
        self
    }
}
