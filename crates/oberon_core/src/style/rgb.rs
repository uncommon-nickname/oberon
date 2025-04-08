use rand::distr::{Distribution, Uniform};

use crate::style::utils::{calculate_hue, calculate_saturation};
use crate::style::Hsl;

#[derive(Copy, Clone, Debug)]
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

    pub fn random() -> Self
    {
        let distr = Uniform::new_inclusive::<u8, u8>(0, 255).unwrap();
        let mut rng = rand::rng();

        Self::new(
            distr.sample(&mut rng),
            distr.sample(&mut rng),
            distr.sample(&mut rng),
        )
    }

    pub fn complementary(&self) -> Self
    {
        self.to_hsl().complementary().to_rgb()
    }

    pub fn darken(&self, ratio: f32) -> Self
    {
        self.to_hsl().darken(ratio).to_rgb()
    }

    pub fn lighten(&self, ratio: f32) -> Self
    {
        self.to_hsl().lighten(ratio).to_rgb()
    }

    pub fn mix(&self, other: Self, ratio: f32) -> Self
    {
        let rest = 1.0 - ratio;

        let r = (rest * self.r as f32 + ratio * other.r as f32) as u8;
        let g = (rest * self.g as f32 + ratio * other.g as f32) as u8;
        let b = (rest * self.b as f32 + ratio * other.b as f32) as u8;

        Self::new(r, g, b)
    }

    pub fn to_hsl(&self) -> Hsl
    {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let l = 0.5 * (max + min);
        let h = calculate_hue(max, min, r, g, b);
        let s = calculate_saturation(max, min, l);

        Hsl::new(h, s, l)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn check_red_hsl_conversion()
    {
        let rgb = Rgb::new(255, 0, 0);
        let hsl = rgb.to_hsl();

        assert_eq!(hsl.h, 0.0);
        assert_eq!(hsl.s, 1.0);
        assert_eq!(hsl.l, 0.5);
    }

    #[test]
    fn check_yellow_hsl_conversion()
    {
        let rgb = Rgb::new(255, 255, 0);
        let hsl = rgb.to_hsl();

        assert_eq!(hsl.h, 60.0);
        assert_eq!(hsl.s, 1.0);
        assert_eq!(hsl.l, 0.5);
    }

    #[test]
    fn check_navy_hsl_conversion()
    {
        let rgb = Rgb::new(0, 0, 128);
        let hsl = rgb.to_hsl();

        assert_eq!(hsl.h, 240.0);
        assert_eq!(hsl.s, 1.0);
        assert_eq!(hsl.l, 0.2509804);
    }
}
