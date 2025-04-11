use crate::linalg::algorithms::{rotate_vertex, Bresenham};
use crate::linalg::shapes::Shape;
use crate::linalg::{Point2, Point2f, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Rectangle
{
    vertices: [Point2; 4],
}

impl Rectangle
{
    pub fn from_corners(top_left: Point2, bottom_right: Point2) -> Self
    {
        let top_right = top_left + Vec2::from_signed(bottom_right.x - top_left.x, 0);
        let bottom_left = top_left + Vec2::from_signed(0, bottom_right.y - top_left.y);

        Self {
            vertices: [top_left, top_right, bottom_left, bottom_right],
        }
    }

    pub fn from_corner_and_size(top_left: Point2, size: Vec2) -> Self
    {
        // FIXME: This is bugged af and will only work for top_left = 0,0
        let top_right = top_left + Vec2::new(size.x, 0);
        let bottom_left = top_left + Vec2::new(0, size.y);
        let bottom_right = top_left + size;

        Self {
            vertices: [top_left, top_right, bottom_left, bottom_right],
        }
    }

    pub fn width(&self) -> usize
    {
        let [tl, tr, bl, br] = self.vertices;

        let max = tl.x.max(tr.x).max(bl.x).max(br.x);
        let min = tl.x.min(tr.x).min(bl.x).min(br.x);

        max - min
    }

    pub fn height(&self) -> usize
    {
        let [tl, tr, bl, br] = self.vertices;

        let max = tl.y.max(tr.y).max(bl.y).max(br.y);
        let min = tl.y.min(tr.y).min(bl.y).min(br.y);

        max - min
    }

    pub fn size(&self) -> Vec2
    {
        let [tl, _, _, br] = self.vertices;
        Vec2::from_signed(br.x - tl.x, br.y - tl.y)
    }

    pub fn center(&self) -> Point2f
    {
        // FIXME: This should work on a rotated rectangle.
        let [tl, _, _, br] = self.vertices;

        let center_x = (tl.x as f32 + br.x as f32) / 2.0;
        let center_y = (tl.y as f32 + br.y as f32) / 2.0;

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
        let [tl, tr, bl, br] = self.vertices;

        Bresenham::new(tl, tr)
            .chain(Bresenham::new(tr, br))
            .chain(Bresenham::new(br, bl))
            .chain(Bresenham::new(bl, tl))
    }

    fn rotate(&mut self, angle: f32)
    {
        let (sin, cos) = angle.to_radians().sin_cos();
        let center = self.center();

        self.vertices
            .iter_mut()
            .for_each(|vertex| rotate_vertex(vertex, &center, sin, cos));
    }
}
