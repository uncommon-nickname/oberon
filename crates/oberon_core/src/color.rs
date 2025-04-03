#[derive(Clone, Debug)]
pub enum Color
{
    Rgb(Rgb),
    Restore,
}

#[derive(Clone, Debug)]
pub struct Rgb
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb
{
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self
    {
        Self { r, g, b }
    }
}
