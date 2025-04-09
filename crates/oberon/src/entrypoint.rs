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
    pub(crate) renderer: Renderer<BufWriter<Stdout>>,
    pub(crate) terminal: Terminal,
    pub(crate) timer: Timer,
    pub(crate) app_loop: Arc<Loop>,
}

impl Oberon
{
    pub fn new(config: Config) -> IoResult<Self>
    {
        let mut size = current_window_size()?;
        // We have to divide the amount of columns by the cursor ratio
        // to make sure that we can fit with the block rendering.

        size.x = (size.x + config.cursor_ratio - 1) / config.cursor_ratio;

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
}

impl Drop for Oberon
{
    fn drop(&mut self)
    {
        let _ = self.renderer.clear();
        let _ = self.renderer.show_cursor();
    }
}

pub fn run_oberon_application(mut app: impl ApplicationHandler) -> IoResult<()>
{
    let mut config = Config::default();

    app.setup(&mut config);

    let mut oberon = Oberon::new(config)?;

    app.before_start(oberon.terminal.canvas());

    while oberon.app_loop.is_running()
    {
        let dt = oberon.timer.start_frame();

        app.frame(oberon.terminal.canvas(), dt, &mut oberon.app_loop);
        oberon.terminal.render_frame(&mut oberon.renderer)?;

        oberon.timer.end_frame();
    }
    Ok(())
}
