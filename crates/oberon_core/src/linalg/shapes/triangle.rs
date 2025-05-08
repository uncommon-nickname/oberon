use crate::linalg::shapes::{BoundingBox, ConvexPolygon, LazyShape, LazyTransformer, Shape};
use crate::linalg::{Matrix3, Point2, Point2f};

#[derive(Clone, Copy, Debug)]
pub struct Triangle
{
    polygon: ConvexPolygon<3>,
}

impl Triangle
{
    pub fn from_vertices(vertices: [Point2; 3]) -> Self
    {
        Self {
            polygon: ConvexPolygon::from_vertices(vertices),
        }
    }
}

impl Shape for Triangle
{
    fn area(&self) -> f64
    {
        self.polygon.area()
    }

    fn bounding_box(&self) -> BoundingBox
    {
        self.polygon.bounding_box()
    }

    fn center(&self) -> Point2f
    {
        let [f, s, t] = self.polygon.get_original_vertices();

        let center_x = (f.x + s.x + t.x) as f64 / 3.0;
        let center_y = (f.y + s.y + t.y) as f64 / 3.0;

        Point2f::new(center_x, center_y)
    }

    fn contains(&self, point: Point2) -> bool
    {
        self.polygon.contains(point)
    }

    fn points_filled(&self) -> Vec<Point2>
    {
        self.polygon.points_filled()
    }

    fn points_outline(&self) -> impl Iterator<Item = Point2>
    {
        self.polygon.points_outline()
    }

    fn transform(&mut self) -> LazyTransformer<'_, Self>
    {
        LazyTransformer::new(self)
    }
}

impl LazyShape for Triangle
{
    fn get_center(&self) -> Point2f
    {
        self.center()
    }

    fn get_rotations_mut(&mut self) -> &mut Matrix3
    {
        self.polygon.get_rotations_mut()
    }

    fn get_translations_mut(&mut self) -> &mut Matrix3
    {
        self.polygon.get_translations_mut()
    }

    fn perform_update(&mut self)
    {
        self.polygon.perform_update();
    }
}
