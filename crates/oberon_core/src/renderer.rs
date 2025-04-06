use std::io::{Result as IoResult, Write};

use crate::linalg::Point2;
use crate::style::Rgb;

#[derive(Debug)]
pub struct Renderer<W: Write>
{
    buffer: W,
}

impl<W: Write> Renderer<W>
{
    pub fn new(buffer: W) -> Self
    {
        Self { buffer }
    }

    pub fn change_bg(&mut self, color: &Rgb) -> IoResult<()>
    {
        write!(
            self.buffer,
            "\x1B[48;2;{};{};{}m",
            color.r, color.g, color.b
        )
    }

    pub fn change_fg(&mut self, color: &Rgb) -> IoResult<()>
    {
        write!(
            self.buffer,
            "\x1B[38;2;{};{};{}m",
            color.r, color.g, color.b
        )
    }

    pub fn clear(&mut self) -> IoResult<()>
    {
        self.buffer.write_all(b"\x1B[2J")
    }

    pub fn flush(&mut self) -> IoResult<()>
    {
        self.buffer.flush()
    }

    pub fn hide_cursor(&mut self) -> IoResult<()>
    {
        self.buffer.write_all(b"\x1B[?25l")
    }

    pub fn move_cursor(&mut self, pos: Point2) -> IoResult<()>
    {
        write!(self.buffer, "\x1B[{};{}H", pos.y + 1, pos.x + 1)
    }

    pub fn reset_bg(&mut self) -> IoResult<()>
    {
        self.buffer.write_all(b"\x1B[49m")
    }

    pub fn reset_fg(&mut self) -> IoResult<()>
    {
        self.buffer.write_all(b"\x1B[39m")
    }

    pub fn show_cursor(&mut self) -> IoResult<()>
    {
        self.buffer.write_all(b"\x1B[?25h")
    }

    pub fn write(&mut self, c: char) -> IoResult<()>
    {
        self.buffer.write_all(c.encode_utf8(&mut [0; 2]).as_bytes())
    }
}
