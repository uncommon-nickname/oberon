use std::sync::Arc;

use oberon_core::canvas::Canvas;

use crate::app_loop::Loop;

pub trait ApplicationHandler
{
    fn frame(&mut self, canvas: Canvas<'_>, dt: f32, app_loop: &mut Arc<Loop>);
}
