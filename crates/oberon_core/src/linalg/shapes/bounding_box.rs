use crate::linalg::Point2;

// TODO: Add OrientedBoundingBox and SimpleBoundingBox.

#[derive(Debug)]
pub struct BoundingBox
{
    x: usize,
    y: usize,
    min: Point2,
    max: Point2,
}

impl BoundingBox
{
    pub const fn new(min: Point2, max: Point2) -> Self
    {
        let x = min.x;
        let y = min.y;

        Self { x, y, min, max }
    }
}

impl Iterator for BoundingBox
{
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.y > self.max.y
        {
            return None;
        }

        let point = Point2::new(self.x, self.y);
        self.x += 1;

        if self.x > self.max.x
        {
            self.x = self.min.x;
            self.y += 1;
        }
        Some(point)
    }
}
