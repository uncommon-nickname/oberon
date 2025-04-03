use oberon_core::canvas::Canvas;

pub trait ApplicationHandler
{
    fn frame(&mut self, canvas: Canvas<'_>, dt: f32);
}
