use crate::linalg::{Point2, Point2f};

pub trait Shape
{
    fn area(&self) -> f64;
    fn center(&self) -> Point2f;
    fn points_outline(&self) -> impl Iterator<Item = Point2>;
    fn rotate(&mut self, angle: f64);
}
