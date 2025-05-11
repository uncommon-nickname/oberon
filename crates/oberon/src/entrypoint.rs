use std::io::{stdout, BufWriter, Result as IoResult, Stdout};
use std::sync::Arc;

use oberon_core::renderer::Renderer;
use oberon_core::terminal::Terminal;

use crate::app_loop::Loop;
use crate::application::ApplicationHandler;
use crate::config::Config;
use crate::timer::Timer;
use crate::utils::install_cleanup_handlers;

pub type ThreadSafeLoop = Arc<Loop>;
type TerminalRenderer = Renderer<BufWriter<Stdout>>;

#[derive(Debug)]
pub struct Oberon
{
    renderer: TerminalRenderer,
    terminal: Terminal,
    timer: Timer,
    app_loop: ThreadSafeLoop,
}

impl Oberon
{
    pub fn new(config: Config) -> IoResult<Self>
    {
        let mut size = config.size;
        size.x /= config.cursor_ratio as isize;

        let buf = BufWriter::new(stdout());
        let mut renderer = Renderer::new(buf);
        let terminal = Terminal::new(size, config.cursor_ratio);
        let timer = Timer::new(config.fps);
        let app_loop = Arc::new(Loop::default());

        install_cleanup_handlers(app_loop.clone());

        if config.hide_cursor
        {
            renderer.hide_cursor()?;
        }

        Ok(Self {
            renderer,
            terminal,
            timer,
            app_loop,
        })
    }

    pub fn run<A: ApplicationHandler>(&mut self, mut app: A) -> IoResult<()>
    {
        app.before_start(self.terminal.canvas());

        while self.app_loop.is_running()
        {
            let dt = self.timer.start_frame();

            app.frame(self.terminal.canvas(), dt, &mut self.app_loop);
            self.terminal.render_frame(&mut self.renderer)?;
            app.after_frame(&mut self.app_loop);

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
