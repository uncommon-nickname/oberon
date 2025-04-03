use crate::color::Color;

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
            bg: Color::Restore,
            fg: Color::Restore,
        }
    }
}
