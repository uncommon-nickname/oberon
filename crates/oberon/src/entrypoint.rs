use std::io::{stdout, BufWriter, Result as IoResult, Stdout};

use oberon_core::linalg::Vec2;
use oberon_core::renderer::Renderer;
use oberon_core::sys::current_window_size;
use oberon_core::terminal::Terminal;

use crate::application::ApplicationHandler;

#[derive(Debug)]
pub struct Oberon
{
    renderer: Renderer<BufWriter<Stdout>>,
    terminal: Terminal,
}

impl Oberon
{
    pub fn with_size(size: Vec2) -> Self
    {
        let buf = BufWriter::new(stdout());
        let renderer = Renderer::new(buf);
        let terminal = Terminal::new(size);

        Self { renderer, terminal }
    }

    pub fn with_automatic_size() -> IoResult<Self>
    {
        let size = current_window_size()?;
        Ok(Self::with_size(size))
    }

    pub fn run_application(&mut self, mut app: impl ApplicationHandler) -> IoResult<()>
    {
        loop
        {
            app.render_frame(self.terminal.canvas());
            self.terminal.render_frame(&mut self.renderer)?;
        }
    }
}
