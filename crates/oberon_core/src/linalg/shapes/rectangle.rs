use crate::linalg::{Point2, Shape, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Rectangle
{
    // Left top vertex of the rectangle.
    left_top: Point2,
    // Right bottom vertex of the rectangle.
    right_bottom: Point2,
}

impl Rectangle
{
    pub const fn new(left_top: Point2, right_bottom: Point2) -> Self
    {
        Self {
            left_top,
            right_bottom,
        }
    }

    pub fn from_size(left_top: Point2, size: Vec2) -> Self
    {
        let right_bottom = left_top + size;

        Self {
            left_top,
            right_bottom,
        }
    }

    pub fn width(&self) -> usize
    {
        self.right_bottom.x - self.left_top.x
    }

    pub fn height(&self) -> usize
    {
        self.right_bottom.y - self.left_top.y
    }

    pub fn size(&self) -> Vec2
    {
        Vec2::new(self.width(), self.height())
    }
}

impl Shape for Rectangle
{
    fn area(&self) -> usize
    {
        self.width() * self.height()
    }

    fn contains(&self, pos: Point2) -> bool
    {
        self.left_top.x <= pos.x
            && pos.x < self.right_bottom.x
            && self.left_top.y <= pos.y
            && pos.y < self.right_bottom.y
    }

    fn get_points(&self) -> Vec<Point2>
    {
        let mut result = Vec::with_capacity(self.area());

        for x in self.left_top.x..self.right_bottom.x
        {
            for y in self.left_top.y..self.right_bottom.y
            {
                result.push(Point2::new(x, y));
            }
        }
        result
    }

    fn resize(&mut self, _scale: f32)
    {
        todo!();
    }

    fn rotate(&mut self, _angle: f32)
    {
        todo!();
    }

    fn translate(&mut self, _vec: Vec2)
    {
        todo!();
    }
}
