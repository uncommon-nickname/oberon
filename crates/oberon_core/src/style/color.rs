use crate::style::rgb::Rgb;

#[derive(Clone, Debug)]
pub enum Color
{
    Rgb(Rgb),
    Default,
}

impl Color
{
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self
    {
        let rgb = Rgb::new(r, g, b);
        Color::Rgb(rgb)
    }

    pub fn random() -> Self
    {
        let rgb = Rgb::random();
        Color::Rgb(rgb)
    }
}
