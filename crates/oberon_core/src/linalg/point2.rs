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

    pub const fn from_signed(x: isize, y: isize) -> Self
    {
        Self {
            x: x as usize,
            y: y as usize,
        }
    }
}

impl Add<Vec2> for Point2
{
    type Output = Point2;

    fn add(self, rhs: Vec2) -> Self::Output
    {
        let new_x = self.x.saturating_add_signed(rhs.x);
        let new_y = self.y.saturating_add_signed(rhs.y);

        Point2::new(new_x, new_y)
    }
}

impl AddAssign<Vec2> for Point2
{
    fn add_assign(&mut self, rhs: Vec2)
    {
        self.x = self.x.saturating_add_signed(rhs.x);
        self.y = self.y.saturating_add_signed(rhs.y);
    }
}
