use std::ops::{Add, AddAssign};

use crate::linalg::vec2::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct Point2
{
    pub x: usize,
    pub y: usize,
}

impl Point2
{
    pub const ZERO: Self = Self::new(0, 0);

    pub const fn new(x: usize, y: usize) -> Self
    {
        Self { x, y }
    }
}

impl Add<Vec2> for Point2
{
    type Output = Point2;

    fn add(self, rhs: Vec2) -> Self::Output
    {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Vec2> for Point2
{
    fn add_assign(&mut self, rhs: Vec2)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
