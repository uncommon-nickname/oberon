use crate::linalg::shapes::{BoundingBox, ConvexPolygon, LazyShape, LazyTransformer, Shape};
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
            polygon: ConvexPolygon::from_vertices(vertices),
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

    fn bounding_box(&self) -> BoundingBox
    {
        self.polygon.bounding_box()
    }

    fn center(&self) -> Point2f
    {
        let [tl, _, _, br] = self.polygon.get_original_vertices();

        let center_x = (tl.x + br.x) as f64 / 2.0;
        let center_y = (tl.y + br.y) as f64 / 2.0;

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

impl LazyShape for Rectangle
{
    fn get_center(&self) -> Point2f
    {
        self.center()
    }

    fn get_rotations_mut(&mut self) -> &mut crate::linalg::Matrix3
    {
        self.polygon.get_rotations_mut()
    }

    fn get_translations_mut(&mut self) -> &mut crate::linalg::Matrix3
    {
        self.polygon.get_translations_mut()
    }

    fn perform_update(&mut self)
    {
        self.polygon.perform_update();
    }
}
