use std::io::{BufWriter, Result as IoResult, Stdout};

use crate::renderer::Renderer;

pub struct Terminal
{
    renderer: Renderer<BufWriter<Stdout>>,
}

impl Terminal
{
    pub fn new() -> Self
    {
        let buffer = BufWriter::with_capacity(100, std::io::stdout());
        let renderer = Renderer::new(buffer);
        Self { renderer }
    }

    pub fn run(&mut self) -> IoResult<()>
    {
        self.renderer.clear()?;
        self.renderer.move_cursor(10, 10)?;
        self.renderer.write('#')?;
        self.renderer.flush()?;

        Ok(())
    }
}
