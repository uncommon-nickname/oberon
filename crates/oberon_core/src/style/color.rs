use crate::style::{Hsl, Rgb};

#[derive(Clone, Copy, Debug)]
pub enum Color
{
    Rgb(Rgb),
    Hsl(Hsl),
    Default,
}

impl Color
{
    pub const fn hsl(h: f32, s: f32, l: f32) -> Self
    {
        let hsl = Hsl::new(h, s, l);
        Color::Hsl(hsl)
    }

    pub fn random() -> Self
    {
        let rgb = Rgb::random();
        Color::Rgb(rgb)
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self
    {
        let rgb = Rgb::new(r, g, b);
        Color::Rgb(rgb)
    }

    pub fn complementary(self) -> Self
    {
        match self
        {
            Self::Rgb(rgb) => Color::Rgb(rgb.complementary()),
            Self::Hsl(hsl) => Color::Hsl(hsl.complementary()),
            Self::Default => self,
        }
    }

    pub fn darken(self, ratio: f32) -> Self
    {
        match self
        {
            Self::Rgb(rgb) => Color::Rgb(rgb.darken(ratio)),
            Self::Hsl(hsl) => Color::Hsl(hsl.darken(ratio)),
            Self::Default => self,
        }
    }

    pub fn lighten(self, ratio: f32) -> Self
    {
        match self
        {
            Self::Rgb(rgb) => Color::Rgb(rgb.lighten(ratio)),
            Self::Hsl(hsl) => Color::Hsl(hsl.lighten(ratio)),
            Self::Default => self,
        }
    }
}
