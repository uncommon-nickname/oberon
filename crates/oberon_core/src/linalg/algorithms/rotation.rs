use crate::linalg::{Point2, Point2f};

pub fn rotate_vertex(vertex: &mut Point2, around: &Point2f, sin: f32, cos: f32)
{
    let translated_x = vertex.x as f32 - around.x;
    let translated_y = vertex.y as f32 - around.y;

    let rotated_x = translated_x * cos - translated_y * sin;
    let rotated_y = translated_x * sin + translated_y * cos;

    vertex.x = (rotated_x + around.x) as usize;
    vertex.y = (rotated_y + around.y) as usize;
}
