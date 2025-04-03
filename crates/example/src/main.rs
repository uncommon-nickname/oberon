use std::io::Result as IoResult;

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
    fn render_frame(&mut self, mut canvas: Canvas<'_>)
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
    let mut oberon = Oberon::with_automatic_size()?;
    let app = App::default();

    oberon.run_application(app)
}
