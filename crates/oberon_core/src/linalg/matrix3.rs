use std::ops::{Mul, MulAssign};

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

    pub const fn transformation(sin: f64, cos: f64, x: f64, y: f64) -> Self
    {
        Self::new([
            cos,
            -sin,
            x - x * cos + y * sin,
            sin,
            cos,
            y - x * sin - y * cos,
            0.0,
            0.0,
            1.0,
        ])
    }
}

impl Mul<Matrix3> for Matrix3
{
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output
    {
        let m = &self.data;
        let n = &rhs.data;

        // Ugly, but fast.
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
