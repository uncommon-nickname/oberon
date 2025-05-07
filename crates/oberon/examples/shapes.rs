use std::io::Result as IoResult;
use std::sync::Arc;

use oberon::oberon_core::linalg::shapes::{ConvexPolygon, Rectangle, Shape, Triangle};
use oberon::oberon_core::linalg::{Point2, Point2f, Vec2, Vec2f};
use oberon::oberon_core::style::Color;
use oberon::oberon_core::terminal::Cell;
use oberon::prelude::*;

struct App
{
    polygon: ConvexPolygon<5>,
    rectangle: Rectangle,
    triangle: Triangle,
    triangle_rot_point: Point2f,
}

impl App
{
    fn new() -> Self
    {
        Self {
            polygon: ConvexPolygon::from_vertices([
                Point2::new(30, 30),
                Point2::new(30, 40),
                Point2::new(40, 50),
                Point2::new(50, 40),
                Point2::new(50, 30),
            ]),
            rectangle: Rectangle::from_corner_and_size(Point2::new(10, 10), Vec2::new(10, 20)),
            triangle: Triangle::from_vertices([
                Point2::new(50, 5),
                Point2::new(40, 10),
                Point2::new(60, 15),
            ]),
            triangle_rot_point: Point2f::new(70.0, 20.0),
        }
    }
}

impl ApplicationHandler for App
{
    fn frame(&mut self, mut canvas: Canvas<'_>, dt: f64, _: &mut Arc<Loop>)
    {
        canvas.erase();

        self.polygon
            .transform()
            .rotate(-(360.0 * dt / 2.0))
            .finalize();

        self.rectangle
            .transform()
            .rotate(360.0 * dt / 10.0)
            .translate(Vec2f::new(1.0, 0.0))
            .finalize();

        self.triangle
            .transform()
            .rotate_around(self.triangle_rot_point, 360.0 * dt / 5.0)
            .finalize();

        canvas.draw_shape_outline(&self.rectangle, Cell::EMPTY.with_bg(Color::WHITE));
        canvas.draw_shape_outline(&self.polygon, Cell::new('@').with_fg(Color::RED));
        canvas.draw_shape_outline(&self.triangle, Cell::EMPTY.with_bg(Color::GREEN));
    }
}

fn main() -> IoResult<()>
{
    let app = App::new();
    run_oberon_application(app)
}
