use std::io::Result as IoResult;

use oberon_core::linalg::Vec2;
use oberon_core::sys::current_window_size;

#[derive(Debug)]
pub struct Config
{
    pub hide_cursor: bool,
    pub fps: f32,
    pub cursor_ratio: usize,
    pub size: Vec2,
}

impl Config
{
    pub fn new() -> IoResult<Self>
    {
        let size = current_window_size()?;

        Ok(Self {
            hide_cursor: true,
            fps: 60.0,
            cursor_ratio: 2,
            size,
        })
    }

    pub fn cursor_ratio(mut self, ratio: usize) -> Self
    {
        self.cursor_ratio = ratio;
        self
    }

    pub fn fps(mut self, fps: f32) -> Self
    {
        self.fps = fps;
        self
    }

    pub fn hide_cursor(mut self, value: bool) -> Self
    {
        self.hide_cursor = value;
        self
    }

    pub fn size(mut self, value: Vec2) -> Self
    {
        self.size = value;
        self
    }
}
