use crate::linalg::shapes::{ConvexPolygon, Shape};
use crate::linalg::{Point2, Point2f, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Rectangle
{
    polygon: ConvexPolygon<4>,
}

impl Rectangle
{
    pub fn from_corners(top_left: Point2, bottom_right: Point2) -> Self
    {
        let top_right = top_left + Vec2::from_signed(bottom_right.x - top_left.x, 0);
        let bottom_left = top_left + Vec2::from_signed(0, bottom_right.y - top_left.y);
        Self::from_vertices([top_left, top_right, bottom_right, bottom_left])
    }

    pub fn from_corner_and_size(top_left: Point2, size: Vec2) -> Self
    {
        let bottom_right = top_left + size;
        Self::from_corners(top_left, bottom_right)
    }

    pub fn from_vertices(vertices: [Point2; 4]) -> Self
    {
        Self {
            polygon: ConvexPolygon::new(vertices),
        }
    }

    pub fn width(&self) -> usize
    {
        let [tl, _, br, _] = self.polygon.get_original_vertices();
        br.x - tl.x
    }

    pub fn height(&self) -> usize
    {
        let [tl, _, br, _] = self.polygon.get_original_vertices();
        br.y - tl.y
    }

    pub fn size(&self) -> Vec2
    {
        Vec2::from_signed(self.width(), self.height())
    }
}

impl Shape for Rectangle
{
    fn area(&self) -> f64
    {
        (self.width() * self.height()) as f64
    }

    fn center(&self) -> Point2f
    {
        let [tl, _, _, br] = self.polygon.get_original_vertices();

        let center_x = (tl.x + br.x) as f64 / 2.0;
        let center_y = (tl.y + br.y) as f64 / 2.0;

        Point2f::new(center_x, center_y)
    }

    fn points_outline(&self) -> impl Iterator<Item = Point2>
    {
        self.polygon.points_outline()
    }

    fn rotate(&mut self, angle: f64)
    {
        self.polygon.rotate(angle);
    }
}
