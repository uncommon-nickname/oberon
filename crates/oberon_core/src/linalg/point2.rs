use std::ops::{Add, AddAssign};

use crate::linalg::{Matrix3, Point2f, Vec2};

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

    pub fn from_signed(x: isize, y: isize) -> Self
    {
        Self {
            x: usize::try_from(x).unwrap_or(0),
            y: usize::try_from(y).unwrap_or(0),
        }
    }

    pub const fn to_point2f(&self) -> Point2f
    {
        Point2f::new(self.x as f64, self.y as f64)
    }

    pub fn transform(&self, transform: &Matrix3) -> Self
    {
        let m = &transform.data;
        let x = self.x as f64;
        let y = self.y as f64;

        let nx = m[0] * x + m[1] * y + m[2];
        let ny = m[3] * x + m[4] * y + m[5];
        let w = m[6] * x + m[7] * y + m[8];

        if w != 0.0
        {
            let new_x = (nx / w).clamp(0.0, usize::MAX as f64) as usize;
            let new_y = (ny / w).clamp(0.0, usize::MAX as f64) as usize;
            Point2::new(new_x, new_y)
        }
        else
        {
            let new_x = nx.clamp(0.0, usize::MAX as f64) as usize;
            let new_y = ny.clamp(0.0, usize::MAX as f64) as usize;
            Point2::new(new_x, new_y)
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
