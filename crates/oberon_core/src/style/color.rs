use crate::style::{Hsl, Rgb};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color
{
    Rgb(Rgb),
    Hsl(Hsl),
    Default,
}

impl Color
{
    pub const BLACK: Self = Color::Rgb(Rgb::BLACK);
    pub const WHITE: Self = Color::Rgb(Rgb::WHITE);
    pub const RED: Self = Color::Rgb(Rgb::RED);
    pub const GREEN: Self = Color::Rgb(Rgb::GREEN);
    pub const BLUE: Self = Color::Rgb(Rgb::BLUE);

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

    pub fn darken(&self, ratio: f32) -> Self
    {
        match *self
        {
            Self::Rgb(rgb) => Color::Rgb(rgb.darken(ratio)),
            Self::Hsl(hsl) => Color::Hsl(hsl.darken(ratio)),
            Self::Default => *self,
        }
    }

    pub fn lighten(&self, ratio: f32) -> Self
    {
        match *self
        {
            Self::Rgb(rgb) => Color::Rgb(rgb.lighten(ratio)),
            Self::Hsl(hsl) => Color::Hsl(hsl.lighten(ratio)),
            Self::Default => *self,
        }
    }

    pub fn mix(&self, other: Color, ratio: f64) -> Self
    {
        let other = match other
        {
            Self::Rgb(rgb) => rgb,
            Self::Hsl(hsl) => hsl.to_rgb(),
            Self::Default => return *self,
        };
        match *self
        {
            Self::Rgb(rgb) => Color::Rgb(rgb.mix(other, ratio)),
            Self::Hsl(hsl) => Color::Hsl(hsl.to_rgb().mix(other, ratio).to_hsl()),
            Self::Default => *self,
        }
    }
}
