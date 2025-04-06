use rand::distr::{Distribution, Uniform};

#[derive(Copy, Clone, Debug)]
pub struct Rgb
{
    r: u8,
    g: u8,
    b: u8,
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

    pub const fn from_float(r: f32, g: f32, b: f32) -> Self
    {
        let r = (255.0 * r) as u8;
        let g = (255.0 * g) as u8;
        let b = (255.0 * b) as u8;

        Self::new(r, g, b)
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

    #[inline]
    pub const fn red(&self) -> u8
    {
        self.r
    }

    #[inline]
    pub const fn green(&self) -> u8
    {
        self.g
    }

    #[inline]
    pub const fn blue(&self) -> u8
    {
        self.b
    }
}
