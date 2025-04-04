use std::fs::File;
use std::io::{BufReader, Result as IoResult};
use std::sync::Arc;

use oberon::image::codecs::gif::GifDecoder;
use oberon::image::{AnimationDecoder, Frame};
use oberon::oberon_core::linalg::Vec2;
use oberon::oberon_core::style::color::Color;
use oberon::oberon_core::style::grayscale::Grayscale;
use oberon::oberon_core::style::rgb::Rgb;
use oberon::oberon_core::terminal::cell::Cell;
use oberon::prelude::*;

struct App
{
    index: usize,
    frames: Vec<Frame>,
}

impl App
{
    fn new() -> Self
    {
        let file = File::open("./assets/smol-miku.gif").unwrap();
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
            let rgb = Rgb::new(r, g, b);

            let gs = Grayscale::from_rgb(rgb.clone());
            let cell = Cell::new(gs.into_char()).with_fg(Color::Rgb(rgb));
            let pos = Vec2::new(x as usize, y as usize);

            canvas.draw(pos, cell);
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
