use crate::linalg::algorithms::{rotate_vertex, Bresenham};
use crate::linalg::shapes::Shape;
use crate::linalg::{Point2, Point2f, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Rectangle
{
    vertices_curr: [Point2; 4],
    // This allows us to avoid accumulation of floating points errors
    // which emerge when multiple rotations are applied to the same
    // set of points.
    vertices_orig: [Point2; 4],
    total_rotation: f64,
}

impl Rectangle
{
    pub fn from_corners(top_left: Point2, bottom_right: Point2) -> Self
    {
        let top_right = top_left + Vec2::from_signed(bottom_right.x - top_left.x, 0);
        let bottom_left = top_left + Vec2::from_signed(0, bottom_right.y - top_left.y);

        Self::from_vertices([top_left, top_right, bottom_left, bottom_right])
    }

    pub fn from_corner_and_size(top_left: Point2, size: Vec2) -> Self
    {
        let bottom_right = top_left + size;
        Self::from_corners(top_left, bottom_right)
    }

    pub fn from_vertices(vertices: [Point2; 4]) -> Self
    {
        Self {
            vertices_curr: vertices,
            vertices_orig: vertices,
            total_rotation: 0.0,
        }
    }

    pub fn width(&self) -> usize
    {
        let [tl, _, _, br] = self.vertices_orig;
        br.x - tl.x
    }

    pub fn height(&self) -> usize
    {
        let [tl, _, _, br] = self.vertices_orig;
        br.y - tl.y
    }

    pub fn size(&self) -> Vec2
    {
        Vec2::from_signed(self.width(), self.height())
    }

    pub fn center(&self) -> Point2f
    {
        let [tl, _, _, br] = self.vertices_orig;

        let center_x = (tl.x + br.x) as f64 / 2.0;
        let center_y = (tl.y + br.y) as f64 / 2.0;

        Point2f::new(center_x, center_y)
    }
}

impl Shape for Rectangle
{
    fn area(&self) -> usize
    {
        self.width() * self.height()
    }

    fn points_outline(&self) -> impl Iterator<Item = Point2>
    {
        let [tl, tr, bl, br] = self.vertices_curr;

        Bresenham::new(tl, tr)
            .chain(Bresenham::new(tr, br))
            .chain(Bresenham::new(br, bl))
            .chain(Bresenham::new(bl, tl))
    }

    fn rotate(&mut self, angle: f64)
    {
        self.total_rotation += angle;

        let (sin, cos) = self.total_rotation.to_radians().sin_cos();
        let center = self.center();

        self.vertices_curr
            .iter_mut()
            .zip(self.vertices_orig.iter())
            .for_each(|(curr, orig)| *curr = rotate_vertex(*orig, center, sin, cos));
    }
}
