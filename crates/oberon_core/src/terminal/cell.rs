use crate::color::Rgb;

#[derive(Clone, Debug)]
pub struct Cell
{
    pub char: char,
    pub bg: Rgb,
    pub fg: Rgb,
}

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
