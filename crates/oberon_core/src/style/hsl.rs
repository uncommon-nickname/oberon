use crate::style::Rgb;

#[derive(Copy, Clone, Debug)]
pub struct Hsl
{
    /// Hue in deegrees (0.0 - 360.0)
    pub h: f32,
    /// Saturation in percentage (0.0 - 1.0)
    pub s: f32,
    /// Lightness in percentage (0.0 - 1.0)
    pub l: f32,
}

impl Hsl
{
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(0.0, 0.0, 1.0);
    pub const RED: Self = Self::new(0.0, 1.0, 0.5);
    pub const GREEN: Self = Self::new(120.0, 1.0, 0.5);
    pub const BLUE: Self = Self::new(240.0, 1.0, 0.5);

    pub const fn new(h: f32, s: f32, l: f32) -> Self
    {
        Self { h, s, l }
    }

    pub fn complementary(&self) -> Self
    {
        let new_s = (self.s + 180.0) % 360.0;
        Self::new(self.h, new_s, self.l)
    }

    pub fn darken(&self, ratio: f32) -> Self
    {
        let new_l = (self.l - ratio).clamp(0.0, 1.0);
        Self::new(self.h, self.s, new_l)
    }

    pub fn lighten(&self, ratio: f32) -> Self
    {
        let new_l = (self.l + ratio).clamp(0.0, 1.0);
        Self::new(self.h, self.s, new_l)
    }

    // https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB_alternative
    pub fn to_rgb(&self) -> Rgb
    {
        let a = self.s * self.l.min(1.0 - self.l);

        let r = self.calculate_channel(a, 0.0);
        let g = self.calculate_channel(a, 8.0);
        let b = self.calculate_channel(a, 4.0);

        Rgb::new((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }

    fn calculate_channel(&self, a: f32, n: f32) -> f32
    {
        let k = (n + self.h / 30.0) % 12.0;
        self.l - a * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn check_red_rgb_conversion()
    {
        let hsl = Hsl::new(0.0, 1.0, 0.5);
        let rgb = hsl.to_rgb();

        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 0);
        assert_eq!(rgb.b, 0);
    }

    #[test]
    fn check_yellow_rgb_conversion()
    {
        let hsl = Hsl::new(60.0, 1.0, 0.5);
        let rgb = hsl.to_rgb();

        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 0);
    }

    #[test]
    fn check_navy_rgb_conversion()
    {
        let hsl = Hsl::new(240.0, 1.0, 0.25);
        let rgb = hsl.to_rgb();

        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 0);
        assert_eq!(rgb.b, 127);
    }
}
