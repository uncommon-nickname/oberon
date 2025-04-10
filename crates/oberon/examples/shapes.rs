use std::io::Result as IoResult;
use std::sync::Arc;

use oberon::oberon_core::linalg::{Point2, Rectangle, Shape};
use oberon::oberon_core::style::Color;
use oberon::oberon_core::terminal::Cell;
use oberon::prelude::*;

struct App
{
    elapsed: f32,
    end_time: f32,
}

impl App
{
    fn new() -> Self
    {
        Self {
            elapsed: 0.0,
            end_time: 5.0,
        }
    }

    fn interpolate(&mut self, dt: f32) -> f32
    {
        let time_factor = (self.elapsed / self.end_time).clamp(0.0, 1.0);

        if time_factor == 1.0
        {
            self.elapsed = 0.0;
            return self.interpolate(dt);
        }
        self.elapsed += dt;

        360.0 * time_factor
    }
}

impl ApplicationHandler for App
{
    fn frame(&mut self, mut canvas: Canvas<'_>, dt: f32, _: &mut Arc<Loop>)
    {
        canvas.erase();

        let angle = self.interpolate(dt);

        let origin = Point2::new(10, 10);
        let end = Point2::new(30, 20);

        let mut rectangle = Rectangle::from_corners(origin, end);
        rectangle.rotate(angle);

        canvas.draw_shape(rectangle, Cell::EMPTY.with_bg(Color::WHITE));
    }
}

fn main() -> IoResult<()>
{
    let app = App::new();
    run_oberon_application(app)
}
