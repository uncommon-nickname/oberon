use crate::linalg::algorithms::{rotate_vertex, Bresenham};
use crate::linalg::shapes::Shape;
use crate::linalg::{Point2, Point2f};

#[derive(Clone, Copy, Debug)]
pub struct ConvexPolygon<const N: usize>
{
    vertices_curr: [Point2; N],
    vertices_orig: [Point2; N],
    total_rotation: f64,
}

impl<const N: usize> ConvexPolygon<N>
{
    pub fn new(vertices: [Point2; N]) -> Self
    {
        Self {
            vertices_curr: vertices,
            vertices_orig: vertices,
            total_rotation: 0.0,
        }
    }

    pub fn get_original_vertices(&self) -> &[Point2; N]
    {
        &self.vertices_orig
    }

    pub fn get_current_vertices(&self) -> &[Point2; N]
    {
        &self.vertices_curr
    }
}

impl<const N: usize> Shape for ConvexPolygon<N>
{
    // Source: https://en.wikipedia.org/wiki/Shoelace_formula
    fn area(&self) -> f64
    {
        let mut area = 0.0;

        for index in 0..N
        {
            let p1 = self.vertices_orig[index];
            let p2 = self.vertices_orig[(index + 1) % N];

            let cross = (p1.x * p2.y) as f64 - (p2.x * p1.y) as f64;
            area += cross;
        }
        area * 0.5
    }

    fn center(&self) -> Point2f
    {
        let mut area = 0.0;
        let mut center_x = 0.0;
        let mut center_y = 0.0;

        for index in 0..N
        {
            let p1 = self.vertices_orig[index];
            let p2 = self.vertices_orig[(index + 1) % N];
            let cross = (p1.x * p2.y) as f64 - (p2.x * p1.y) as f64;

            area += cross;
            center_x += (p1.x + p2.x) as f64 * cross;
            center_y += (p1.y + p2.y) as f64 * cross;
        }
        area *= 0.5;

        let factor = 1.0 / (6.0 * area);

        Point2f::new(center_x * factor, center_y * factor)
    }

    fn points_outline(&self) -> impl Iterator<Item = Point2>
    {
        let mut points = Vec::new();

        for index in 0..N
        {
            let p0 = self.vertices_curr[index];
            let p1 = self.vertices_curr[(index + 1) % N];

            points.extend(Bresenham::new(p0, p1));
        }
        points.into_iter()
    }

    fn rotate(&mut self, angle: f64)
    {
        self.total_rotation = (self.total_rotation + angle) % 360.0;

        let (sin, cos) = self.total_rotation.to_radians().sin_cos();
        let center = self.center();

        self.vertices_curr
            .iter_mut()
            .zip(self.vertices_orig.iter())
            .for_each(|(curr, orig)| *curr = rotate_vertex(*orig, center, sin, cos));
    }
}
