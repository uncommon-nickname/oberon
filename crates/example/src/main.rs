use std::io::Result as IoResult;
use std::sync::Arc;

use oberon::oberon_core::canvas::Canvas;
use oberon::oberon_core::color::{Color, Rgb};
use oberon::oberon_core::terminal::cell::Cell;
use oberon::prelude::*;

#[derive(Default)]
struct App
{
    cntr: u8,
}

impl ApplicationHandler for App
{
    fn frame(&mut self, mut canvas: Canvas<'_>, _: f32, _: Arc<Loop>)
    {
        canvas.erase();

        let mut cell = Cell::EMPTY;
        cell.bg = Color::Rgb(Rgb::new(1, 1, self.cntr));
        canvas.fill(cell);

        self.cntr = self.cntr.wrapping_add(1);
    }
}

fn main() -> IoResult<()>
{
    let mut oberon = Oberon::new(Config::default())?;
    let app = App::default();

    oberon.run_application(app)
}
