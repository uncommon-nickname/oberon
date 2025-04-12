use std::io::Result as IoResult;
use std::sync::Arc;

use oberon::oberon_core::linalg::shapes::{Rectangle, Shape};
use oberon::oberon_core::linalg::{Point2, Vec2};
use oberon::oberon_core::style::Color;
use oberon::oberon_core::terminal::Cell;
use oberon::prelude::*;

struct App
{
    rectangle: Rectangle,
}

impl App
{
    fn new() -> Self
    {
        let origin = Point2::new(10, 10);
        let size = Vec2::new(10, 20);

        Self {
            rectangle: Rectangle::from_corner_and_size(origin, size),
        }
    }
}

impl ApplicationHandler for App
{
    fn frame(&mut self, mut canvas: Canvas<'_>, dt: f64, _: &mut Arc<Loop>)
    {
        canvas.erase();

        let angle = 360.0 * dt / 5.0;
        self.rectangle.rotate(angle);

        canvas.draw_shape_outline(self.rectangle, Cell::EMPTY.with_bg(Color::WHITE));
    }
}

fn main() -> IoResult<()>
{
    let app = App::new();
    run_oberon_application(app)
}
