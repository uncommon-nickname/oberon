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
}
