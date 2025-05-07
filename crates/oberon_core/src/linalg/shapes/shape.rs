use crate::linalg::shapes::LazyTransformer;
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
    fn center(&self) -> Point2f;
    fn points_outline(&self) -> impl Iterator<Item = Point2>;
    fn transform(&mut self) -> LazyTransformer<'_, Self>;
}
