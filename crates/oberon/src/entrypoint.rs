use std::io::{stdout, BufWriter, Result as IoResult, Stdout};
use std::sync::Arc;

use oberon_core::renderer::Renderer;
use oberon_core::sys::current_window_size;
use oberon_core::terminal::Terminal;

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
        let mut size = current_window_size()?;
        // We have to divide the amount of columns by the cursor ratio
        // to make sure that we can fit with the block rendering.
        size.x /= config.cursor_ratio;

        let buf = BufWriter::new(stdout());
        let renderer = Renderer::new(buf);
        let terminal = Terminal::new(size, config.cursor_ratio);
        let timer = Timer::new(config.fps);
        let app_loop = Arc::new(Loop::default());

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

        app.before_start(self.terminal.canvas());

        while self.app_loop.is_running()
        {
            let dt = self.timer.start_frame();

            app.frame(self.terminal.canvas(), dt, &mut self.app_loop);
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
