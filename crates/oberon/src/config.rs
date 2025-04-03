use oberon_core::linalg::Vec2;

#[derive(Debug)]
pub struct Config
{
    pub(crate) hide_cursor: bool,
    pub(crate) fps: f32,
    pub(crate) size: Option<Vec2>,
}

impl Default for Config
{
    fn default() -> Self
    {
        Self {
            hide_cursor: true,
            fps: 60.0,
            size: None,
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

    pub fn size(mut self, size: Vec2) -> Self
    {
        self.size = Some(size);
        self
    }
}
