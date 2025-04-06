#[derive(Debug)]
pub struct Config
{
    pub hide_cursor: bool,
    pub fps: f32,
    pub cursor_ratio: usize,
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
