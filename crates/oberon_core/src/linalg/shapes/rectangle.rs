use crate::linalg::{Point2, Point2f, Shape, Vec2};

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

        // Axis-aligned bounding box.
        let max = tl.x.max(tr.x).max(bl.x).max(br.x);
        let min = tl.x.min(tr.x).min(bl.x).min(br.x);

        max - min
    }

    pub fn height(&self) -> usize
    {
        let [tl, tr, bl, br] = self.vertices;

        // Axis-aligned bounding box.
        let max = tl.y.max(tr.y).max(bl.y).max(br.y);
        let min = tl.y.min(tr.y).min(bl.y).min(br.y);

        max - min
    }

    pub fn size(&self) -> Vec2
    {
        let [tl, _, _, br] = self.vertices;
        Vec2::from_signed(br.x - tl.x, br.y - tl.y)
    }
}

impl Shape for Rectangle
{
    fn area(&self) -> usize
    {
        self.width() * self.height()
    }

    fn get_points(&self) -> Vec<Point2>
    {
        let [tl, tr, bl, br] = self.vertices;
        let sides = [(tl, tr), (tr, br), (br, bl), (bl, tl)];

        let mut points = Vec::with_capacity(self.height() * 2 + self.width() * 2);

        sides
            .into_iter()
            .for_each(|(start, end)| points.append(&mut interpolate_line(start, end)));

        points
    }

    fn rotate(&mut self, angle: f32)
    {
        let [tl, _, _, br] = self.vertices;

        let center_x = (tl.x as f32 + br.x as f32) / 2.0;
        let center_y = (tl.y as f32 + br.y as f32) / 2.0;

        let center = Point2f::new(center_x, center_y);
        let (sin, cos) = angle.to_radians().sin_cos();

        self.vertices.iter_mut().for_each(|vertex| {
            rotate_vertex(vertex, center, sin, cos);
        });
    }
}

fn rotate_vertex(vertex: &mut Point2, around: Point2f, sin: f32, cos: f32)
{
    let translated_x = vertex.x as f32 - around.x;
    let translated_y = vertex.y as f32 - around.y;

    let rotated_x = translated_x * cos - translated_y * sin;
    let rotated_y = translated_x * sin + translated_y * cos;

    vertex.x = (rotated_x + around.x) as usize;
    vertex.y = (rotated_y + around.y) as usize;
}

// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
fn interpolate_line(p0: Point2, p1: Point2) -> Vec<Point2>
{
    // TODO: This does not look very good.
    let dx = (p1.x as i32 - p0.x as i32).abs();
    let dy = (p1.y as i32 - p0.y as i32).abs();

    let sx = if p0.x < p1.x { 1 } else { -1 };
    let sy = if p0.y < p1.y { 1 } else { -1 };

    let mut err = dx - dy;
    let mut x = p0.x as i32;
    let mut y = p0.y as i32;

    let mut points = Vec::with_capacity((dx.max(dy) + 1) as usize);

    loop
    {
        points.push(Point2::new(x as usize, y as usize));

        if x as usize == p1.x && y as usize == p1.y
        {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy
        {
            err -= dy;
            x += sx;
        }
        if e2 < dx
        {
            err += dx;
            y += sy;
        }
    }
    points
}
