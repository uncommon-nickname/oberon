use std::io::{stdout, BufWriter, Result as IoResult, Stdout};

use oberon_core::canvas::Canvas;
use oberon_core::renderer::Renderer;
use oberon_core::sys::current_window_size;
use oberon_core::terminal::Terminal;

#[derive(Debug)]
pub struct Oberon
{
    renderer: Renderer<BufWriter<Stdout>>,
    terminal: Terminal,
}

impl Oberon
{
    pub fn new() -> IoResult<Self>
    {
        let size = current_window_size()?;
        let buf = BufWriter::new(stdout());
        let renderer = Renderer::new(buf);
        let terminal = Terminal::new(size);

        Ok(Self { renderer, terminal })
    }

    pub fn end_frame(&mut self) -> IoResult<()>
    {
        self.terminal.render_frame(&mut self.renderer)
    }

    pub fn frame(&mut self, mut f: impl FnMut(Canvas<'_>)) -> IoResult<()>
    {
        f(self.terminal.canvas());
        self.end_frame()
    }

    pub fn hide_cursor(&mut self) -> IoResult<()>
    {
        self.renderer.hide_cursor()
    }

    pub fn show_cursor(&mut self) -> IoResult<()>
    {
        self.renderer.show_cursor()
    }
}
