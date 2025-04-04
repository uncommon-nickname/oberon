use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug)]
pub struct Vec2
{
    pub x: usize,
    pub y: usize,
}

impl Vec2
{
    pub const ZERO: Self = Self::new(0, 0);
    pub const RIGHT: Self = Self::new(1, 0);
    pub const UP: Self = Self::new(0, 1);

    pub const fn new(x: usize, y: usize) -> Self
    {
        Self { x, y }
    }

    pub const fn scalar_product(&self) -> usize
    {
        self.x * self.y
    }
}

impl Add<Vec2> for Vec2
{
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output
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

impl Add<usize> for Vec2
{
    type Output = Vec2;

    fn add(self, rhs: usize) -> Self::Output
    {
        Vec2::new(self.x + rhs, self.y + rhs)
    }
}

impl AddAssign<usize> for Vec2
{
    fn add_assign(&mut self, rhs: usize)
    {
        self.x += rhs;
        self.y += rhs;
    }
}
