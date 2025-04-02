use crate::color::Rgb;
use crate::terminal::Cell;

impl Cell
{
    pub const EMPTY: Self = Self::new(' ');

    pub const fn new(char: char) -> Self
    {
        Self {
            char,
            bg: Rgb::BLACK,
            fg: Rgb::BLACK,
        }
    }
}
