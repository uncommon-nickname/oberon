use std::io::{stdout, BufWriter, Result as IoResult, Stdout};

use oberon_core::renderer::Renderer;
use oberon_core::sys::current_window_size;
use oberon_core::terminal::term::Terminal;

use crate::application::ApplicationHandler;
use crate::config::Config;
use crate::timer::Timer;

#[derive(Debug)]
pub struct Oberon
{
    renderer: Renderer<BufWriter<Stdout>>,
    terminal: Terminal,
    timer: Timer,
    config: Config,
}

impl Oberon
{
    pub fn new(config: Config) -> IoResult<Self>
    {
        let size = match config.size
        {
            Some(size) => size,
            None => current_window_size()?,
        };

        let buf = BufWriter::new(stdout());
        let renderer = Renderer::new(buf);
        let terminal = Terminal::new(size);
        let timer = Timer::new(config.fps);

        Ok(Self {
            renderer,
            terminal,
            timer,
            config,
        })
    }

    pub fn run_application(&mut self, mut app: impl ApplicationHandler) -> IoResult<()>
    {
        if self.config.hide_cursor
        {
            self.renderer.hide_cursor()?;
        }

        loop
        {
            let dt = self.timer.start_frame();

            app.frame(self.terminal.canvas(), dt);
            self.terminal.render_frame(&mut self.renderer)?;

            self.timer.end_frame();
        }
    }
}
