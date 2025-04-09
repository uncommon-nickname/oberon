use std::env::args;
use std::fs::File;
use std::io::{BufReader, Result as IoResult};
use std::sync::Arc;

use oberon::image::codecs::gif::GifDecoder;
use oberon::image::{AnimationDecoder, Frame};
use oberon::oberon_core::linalg::Point2;
use oberon::oberon_core::style::{Color, Grayscale, Rgb};
use oberon::oberon_core::terminal::Cell;
use oberon::prelude::*;

struct App
{
    index: usize,
    use_grayscale: bool,
    frames: Vec<Frame>,
}

impl App
{
    fn new(use_grayscale: bool) -> Self
    {
        let file = File::open("./assets/smol-miku.gif").unwrap();
        let buf = BufReader::new(file);
        let decoder = GifDecoder::new(buf).unwrap();
        let frames = decoder.into_frames().collect_frames().unwrap();

        Self {
            index: 0,
            use_grayscale,
            frames,
        }
    }
}

impl ApplicationHandler for App
{
    fn setup(&mut self, config: &mut Config)
    {
        config.fps = 10.0;
    }

    fn frame(&mut self, mut canvas: Canvas<'_>, _: f32, _: &mut Arc<Loop>)
    {
        let frame = &self.frames[self.index];

        for (x, y, pixel) in frame.buffer().enumerate_pixels()
        {
            let [r, g, b, _] = pixel.0;
            let rgb = Rgb::new(r, g, b);

            let cell = if self.use_grayscale
            {
                let gs = Grayscale::from_rgb(rgb);
                Cell::new(gs.into_char())
            }
            else
            {
                Cell::EMPTY.with_bg(Color::rgb(r, g, b))
            };

            let pos = Point2::new(x as usize, y as usize);
            canvas.draw(pos, cell);
        }
        self.index = (self.index + 1) % self.frames.len();
    }
}

fn main() -> IoResult<()>
{
    let use_grayscale = args()
        .nth(1)
        .expect("no value given.")
        .parse::<bool>()
        .expect("a valid bool is needed");

    let app = App::new(use_grayscale);

    run_oberon_application(app)
}
