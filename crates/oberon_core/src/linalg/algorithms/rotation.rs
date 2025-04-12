use crate::linalg::{Point2, Point2f};

pub fn rotate_vertex(vertex: Point2, around: Point2f, sin: f64, cos: f64) -> Point2
{
    let translated_x = vertex.x as f64 - around.x;
    let translated_y = vertex.y as f64 - around.y;

    let rotated_x = translated_x * cos - translated_y * sin;
    let rotated_y = translated_x * sin + translated_y * cos;

    let new_x = (rotated_x + around.x) as usize;
    let new_y = (rotated_y + around.y) as usize;

    Point2::new(new_x, new_y)
}
