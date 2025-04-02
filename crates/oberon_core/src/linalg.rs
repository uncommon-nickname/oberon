#[derive(Copy, Clone, Debug)]
pub struct Vec2
{
    pub x: usize,
    pub y: usize,
}

impl Vec2
{
    pub const ZEROES: Self = Self::new(0, 0);

    pub const fn new(x: usize, y: usize) -> Self
    {
        Self { x, y }
    }

    pub const fn scalar_product(&self) -> usize
    {
        self.x * self.y
    }
}
