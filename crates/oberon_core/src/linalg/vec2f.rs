#[derive(Clone, Copy, Debug)]
pub struct Vec2f
{
    pub x: f64,
    pub y: f64,
}

impl Vec2f
{
    pub const fn new(x: f64, y: f64) -> Self
    {
        Self { x, y }
    }

    pub const fn opposite(&self) -> Self
    {
        Self::new(-self.x, -self.y)
    }
}
