use crate::linalg::Point2;

#[derive(Debug)]
pub struct Bresenham
{
    x: isize,
    y: isize,
    end_x: isize,
    end_y: isize,
    sx: isize,
    sy: isize,
    dx: isize,
    dy: isize,
    err: isize,
}

impl Bresenham
{
    pub const fn new(p0: Point2, p1: Point2) -> Self
    {
        let dx = (p1.x as isize - p0.x as isize).abs();
        let dy = (p1.y as isize - p0.y as isize).abs();

        let err = dx - dy;

        let sx = if p0.x < p1.x { 1 } else { -1 };
        let sy = if p0.y < p1.y { 1 } else { -1 };

        Self {
            x: p0.x as isize,
            y: p0.y as isize,
            end_x: p1.x as isize,
            end_y: p1.y as isize,
            sx,
            sy,
            dx,
            dy,
            err,
        }
    }
}

impl Iterator for Bresenham
{
    type Item = Point2;

    // Source: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.x == self.end_x && self.y == self.end_y
        {
            return None;
        }

        let curr_err = 2 * self.err;

        if curr_err > -self.dy
        {
            self.err -= self.dy;
            self.x += self.sx;
        }
        if curr_err < self.dx
        {
            self.err += self.dx;
            self.y += self.sy;
        }

        Some(Point2::from_signed(self.x, self.y))
    }
}
