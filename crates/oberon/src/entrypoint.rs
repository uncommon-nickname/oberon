use std::io::{stdout, BufWriter, Result as IoResult, Stdout};
use std::sync::Arc;

use oberon_core::renderer::Renderer;
use oberon_core::sys::current_window_size;
use oberon_core::terminal::term::Terminal;

use crate::app_loop::Loop;
use crate::application::ApplicationHandler;
use crate::config::Config;
use crate::timer::Timer;
use crate::utils::install_cleanup_handlers;

#[derive(Debug)]
pub struct Oberon
{
    renderer: Renderer<BufWriter<Stdout>>,
    terminal: Terminal,
    timer: Timer,
    config: Config,
    app_loop: Arc<Loop>,
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
        let app_loop = Arc::new(Loop::new());

        install_cleanup_handlers(app_loop.clone());

        Ok(Self {
            renderer,
            terminal,
            timer,
            config,
            app_loop,
        })
    }

    pub fn run_application(&mut self, mut app: impl ApplicationHandler) -> IoResult<()>
    {
        if self.config.hide_cursor
        {
            self.renderer.hide_cursor()?;
        }

        while self.app_loop.is_running()
        {
            let dt = self.timer.start_frame();

            app.frame(self.terminal.canvas(), dt, self.app_loop.clone());
            self.terminal.render_frame(&mut self.renderer)?;

            self.timer.end_frame();
        }
        Ok(())
    }
}

impl Drop for Oberon
{
    fn drop(&mut self)
    {
        let _ = self.renderer.clear();
        let _ = self.renderer.show_cursor();
    }
}
