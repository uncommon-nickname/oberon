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
}
