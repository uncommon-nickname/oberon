use std::io::Result as IoResult;
use std::sync::Arc;

use oberon::oberon_core::linalg::shapes::{ConvexPolygon, Rectangle, Shape};
use oberon::oberon_core::linalg::{Point2, Vec2};
use oberon::oberon_core::style::Color;
use oberon::oberon_core::terminal::Cell;
use oberon::prelude::*;

struct App
{
    polygon: ConvexPolygon<5>,
    rectangle: Rectangle,
}

impl App
{
    fn new() -> Self
    {
        Self {
            polygon: ConvexPolygon::new([
                Point2::new(30, 30),
                Point2::new(30, 40),
                Point2::new(40, 50),
                Point2::new(50, 40),
                Point2::new(50, 30),
            ]),
            rectangle: Rectangle::from_corner_and_size(Point2::new(10, 10), Vec2::new(10, 20)),
        }
    }
}

impl ApplicationHandler for App
{
    fn frame(&mut self, mut canvas: Canvas<'_>, dt: f64, _: &mut Arc<Loop>)
    {
        canvas.erase();

        self.polygon.rotate(-(360.0 * dt / 2.0));
        self.rectangle.rotate(360.0 * dt / 10.0);

        canvas.draw_shape_outline(self.rectangle, Cell::EMPTY.with_bg(Color::WHITE));
        canvas.draw_shape_outline(self.polygon, Cell::new('@').with_fg(Color::RED));
    }
}

fn main() -> IoResult<()>
{
    let app = App::new();
    run_oberon_application(app)
}
