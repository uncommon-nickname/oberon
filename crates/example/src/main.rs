use std::fs::File;
use std::io::{BufReader, Result as IoResult};
use std::sync::Arc;

use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, Frame};
use oberon::oberon_core::canvas::Canvas;
use oberon::oberon_core::color::{Color, Rgb};
use oberon::oberon_core::linalg::Vec2;
use oberon::oberon_core::terminal::cell::Cell;
use oberon::prelude::*;

struct App
{
    index: usize,
    frames: Vec<Frame>,
}

impl App
{
    pub fn new() -> Self
    {
        let file = File::open("./smol-miku.gif").unwrap();
        let buf = BufReader::new(file);
        let decoder = GifDecoder::new(buf).unwrap();
        let frames = decoder.into_frames().collect_frames().unwrap();

        Self { index: 0, frames }
    }
}

impl ApplicationHandler for App
{
    fn frame(&mut self, mut canvas: Canvas<'_>, _: f32, _: Arc<Loop>)
    {
        canvas.erase();

        let frame = &self.frames[self.index];

        for (x, y, pixel) in frame.buffer().enumerate_pixels()
        {
            let [r, g, b, _] = pixel.0;

            let mut cell = Cell::EMPTY;
            cell.bg = Color::Rgb(Rgb::new(r, g, b));

            canvas.draw(Vec2::new(x as usize, y as usize), cell);
        }
        self.index = (self.index + 1) % self.frames.len();
    }
}

fn main() -> IoResult<()>
{
    let mut oberon = Oberon::new(Config::default().fps(10.0))?;
    let app = App::new();

    oberon.run_application(app)
}
