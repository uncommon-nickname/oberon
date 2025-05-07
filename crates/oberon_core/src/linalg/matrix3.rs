use std::ops::{Mul, MulAssign};

use crate::linalg::{Point2f, Vec2f};

#[derive(Clone, Copy, Debug)]
pub struct Matrix3
{
    pub data: [f64; 9],
}

impl Matrix3
{
    pub const IDENTITY: Self = Self::new([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);

    pub const fn new(data: [f64; 9]) -> Self
    {
        Self { data }
    }

    pub fn rotation_around(point: Point2f, angle: f64) -> Self
    {
        let (sin, cos) = angle.to_radians().sin_cos();

        let mut matrix = Self::IDENTITY;

        matrix.data[0] = cos;
        matrix.data[1] = -sin;
        matrix.data[2] = point.x - point.x * cos + point.y * sin;
        matrix.data[3] = sin;
        matrix.data[4] = cos;
        matrix.data[5] = point.y - point.x * sin - point.y * cos;

        matrix
    }

    pub fn translation(by: Vec2f) -> Self
    {
        let mut matrix = Self::IDENTITY;

        matrix.data[2] = by.x;
        matrix.data[5] = by.y;

        matrix
    }
}

impl Mul<Matrix3> for Matrix3
{
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output
    {
        let m = &self.data;
        let n = &rhs.data;

        // NOTE: this is ugly, but it will be fast.
        Self::new([
            m[0] * n[0] + m[1] * n[3] + m[2] * n[6],
            m[0] * n[1] + m[1] * n[4] + m[2] * n[7],
            m[0] * n[2] + m[1] * n[5] + m[2] * n[8],
            m[3] * n[0] + m[4] * n[3] + m[5] * n[6],
            m[3] * n[1] + m[4] * n[4] + m[5] * n[7],
            m[3] * n[2] + m[4] * n[5] + m[5] * n[8],
            m[6] * n[0] + m[7] * n[3] + m[8] * n[6],
            m[6] * n[1] + m[7] * n[4] + m[8] * n[7],
            m[6] * n[2] + m[7] * n[5] + m[8] * n[8],
        ])
    }
}

impl MulAssign<Matrix3> for Matrix3
{
    fn mul_assign(&mut self, rhs: Matrix3)
    {
        *self = *self * rhs;
    }
}
