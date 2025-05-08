use crate::linalg::shapes::{BoundingBox, LazyTransformer};
use crate::linalg::{Matrix3, Point2, Point2f};

pub trait LazyShape
{
    fn get_center(&self) -> Point2f;
    fn get_rotations_mut(&mut self) -> &mut Matrix3;
    fn get_translations_mut(&mut self) -> &mut Matrix3;
    fn perform_update(&mut self);
}

pub trait Shape
where
    Self: LazyShape + Sized,
{
    fn area(&self) -> f64;
    fn bounding_box(&self) -> BoundingBox;
    fn center(&self) -> Point2f;
    fn contains(&self, point: Point2) -> bool;
    fn points_filled(&self) -> Vec<Point2>;
    fn points_outline(&self) -> impl Iterator<Item = Point2>;
    fn transform(&mut self) -> LazyTransformer<'_, Self>;
}
