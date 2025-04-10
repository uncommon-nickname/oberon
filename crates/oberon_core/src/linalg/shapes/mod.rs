pub mod rectangle;

use crate::linalg::Point2;

pub trait Shape
{
    fn area(&self) -> usize;
    fn get_points(&self) -> Vec<Point2>;
    fn rotate(&mut self, angle: f32);
}
