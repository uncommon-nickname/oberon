use crate::linalg::Point2;

#[derive(Copy, Clone, Debug)]
pub struct Point2f
{
    pub x: f32,
    pub y: f32,
}

impl Point2f
{
    pub const fn new(x: f32, y: f32) -> Self
    {
        Self { x, y }
    }

    pub fn to_point2(&self) -> Point2
    {
        Point2::new(self.x as usize, self.y as usize)
    }
}
