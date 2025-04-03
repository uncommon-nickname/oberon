use oberon_core::canvas::Canvas;

pub trait ApplicationHandler
{
    fn render_frame(&mut self, canvas: Canvas<'_>);
}
