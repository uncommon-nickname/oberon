mod rectangle;
pub use rectangle::Rectangle;

use crate::linalg::Point2;

pub trait Shape
{
    fn area(&self) -> usize;
    fn points_outline(&self) -> impl Iterator<Item = Point2>;
    fn rotate(&mut self, angle: f32);
}
