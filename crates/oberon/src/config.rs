#[derive(Debug)]
pub struct Config
{
    pub(crate) hide_cursor: bool,
    pub(crate) fps: f32,
    pub(crate) cursor_ratio: usize,
}

impl Default for Config
{
    fn default() -> Self
    {
        Self {
            hide_cursor: true,
            fps: 60.0,
            cursor_ratio: 2,
        }
    }
}

impl Config
{
    pub fn hide_cursor(mut self, hide_cursor: bool) -> Self
    {
        self.hide_cursor = hide_cursor;
        self
    }

    pub fn fps(mut self, fps: f32) -> Self
    {
        self.fps = fps;
        self
    }

    pub fn cursor_ratio(mut self, cursor_ratio: usize) -> Self
    {
        self.cursor_ratio = cursor_ratio;
        self
    }
}
