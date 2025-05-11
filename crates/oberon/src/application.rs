use std::sync::Arc;

use oberon_core::canvas::Canvas;

use crate::app_loop::Loop;
use crate::entrypoint::ThreadSafeLoop;

pub trait ApplicationHandler
{
    fn before_start(&mut self, _canvas: Canvas<'_>) {}
    fn frame(&mut self, canvas: Canvas<'_>, dt: f64, app_loop: &mut ThreadSafeLoop);
    fn after_frame(&mut self, _app_loop: &mut Arc<Loop>) {}
}
