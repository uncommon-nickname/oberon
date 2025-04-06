use rand::distr::{Distribution, Uniform};

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

    pub const fn mix(&self, other: Self, ratio: f32) -> Self
    {
        assert!(ratio >= 0.0 && ratio <= 1.0);

        let rest = 1.0 - ratio;

        let r = (rest * self.r as f32 + ratio * other.r as f32) as u8;
        let g = (rest * self.g as f32 + ratio * other.g as f32) as u8;
        let b = (rest * self.b as f32 + ratio * other.b as f32) as u8;

        Self::new(r, g, b)
    }
}
