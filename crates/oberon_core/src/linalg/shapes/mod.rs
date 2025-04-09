pub mod rectangle;

use crate::linalg::{Point2, Vec2};

pub trait Shape
{
    fn area(&self) -> usize;
    fn contains(&self, pos: Point2) -> bool;
    fn get_points(&self) -> Vec<Point2>;
    fn resize(&mut self, scale: f32);
    fn rotate(&mut self, angle: f32);
    fn translate(&mut self, vec: Vec2);
}
