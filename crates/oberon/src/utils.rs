use std::io::{stdout, BufWriter};
use std::panic::{set_hook, take_hook};
use std::sync::Arc;

use ctrlc::set_handler;
use oberon_core::renderer::Renderer;

use crate::app_loop::Loop;

pub(crate) fn install_cleanup_handlers(app_loop: Arc<Loop>)
{
    set_handler(move || {
        app_loop.shutdown();
    })
    .expect("Could not setup the ctrl+c handler.");

    let default_panic_hook = take_hook();
    set_hook(Box::new(move |info| {
        let mut renderer = Renderer::new(BufWriter::new(stdout()));
        let _ = renderer.show_cursor();
        let _ = renderer.clear();

        default_panic_hook(info);
    }));
}
