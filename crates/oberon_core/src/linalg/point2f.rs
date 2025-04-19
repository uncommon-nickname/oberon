use crate::linalg::Vec2f;

#[derive(Copy, Clone, Debug)]
pub struct Point2f
{
    pub x: f64,
    pub y: f64,
}

impl Point2f
{
    pub const fn new(x: f64, y: f64) -> Self
    {
        Self { x, y }
    }

    pub const fn to_vec2f(&self) -> Vec2f
    {
        Vec2f::new(self.x, self.y)
    }
}
