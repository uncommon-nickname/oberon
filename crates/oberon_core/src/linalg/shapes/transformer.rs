use crate::linalg::shapes::LazyShape;
use crate::linalg::{Matrix3, Point2f, Vec2f};

pub struct LazyTransformer<'a, S: LazyShape>
{
    shape: &'a mut S,
}

impl<'a, S: LazyShape> LazyTransformer<'a, S>
{
    pub fn new(shape: &'a mut S) -> Self
    {
        Self { shape }
    }

    pub fn rotate(self, by: f64) -> Self
    {
        let center = self.shape.get_center();
        self.rotate_around(center, by)
    }

    pub fn rotate_around(self, around: Point2f, by: f64) -> Self
    {
        let rotations = self.shape.get_rotations();
        *rotations *= Matrix3::rotation_around(around, by);
        self
    }

    pub fn translate(self, by: Vec2f) -> Self
    {
        let translations = self.shape.get_translations();
        *translations *= Matrix3::translation(by);
        self
    }

    pub fn finalize(&mut self)
    {
        self.shape.perform_update();
    }
}
