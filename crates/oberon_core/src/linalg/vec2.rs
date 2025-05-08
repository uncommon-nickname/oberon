use std::ops::{Add, AddAssign};

use crate::linalg::Vec2f;

#[derive(Copy, Clone, Debug)]
pub struct Vec2
{
    pub x: isize,
    pub y: isize,
}

impl Vec2
{
    pub const ZEROES: Self = Self::new(0, 0);
    pub const ONES: Self = Self::new(1, 1);
    pub const RIGHT: Self = Self::new(1, 0);
    pub const LEFT: Self = Self::new(-1, 0);
    pub const UP: Self = Self::new(0, 1);

    pub const fn new(x: isize, y: isize) -> Self
    {
        Self { x, y }
    }

    pub const fn from_signed(x: usize, y: usize) -> Self
    {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    pub const fn to_vec2f(&self) -> Vec2f
    {
        Vec2f::new(self.x as f64, self.y as f64)
    }
}

impl Add<Vec2> for Vec2
{
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output
    {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Vec2> for Vec2
{
    fn add_assign(&mut self, rhs: Vec2)
    {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<isize> for Vec2
{
    type Output = Vec2;

    fn add(self, rhs: isize) -> Self::Output
    {
        Vec2::new(self.x + rhs, self.y + rhs)
    }
}

impl AddAssign<isize> for Vec2
{
    fn add_assign(&mut self, rhs: isize)
    {
        self.x += rhs;
        self.y += rhs;
    }
}
