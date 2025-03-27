use std::io::{Result as IoResult, Write};

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

    pub fn clear(&mut self) -> IoResult<()>
    {
        self.buffer.write_all(b"\x1B[2J")
    }

    pub fn flush(&mut self) -> IoResult<()>
    {
        self.buffer.flush()
    }

    pub fn move_cursor(&mut self, x: u16, y: u16) -> IoResult<()>
    {
        write!(self.buffer, "\x1B[{};{}H", x + 1, y + 1)
    }

    pub fn write(&mut self, c: char) -> IoResult<()>
    {
        self.buffer.write_all(c.encode_utf8(&mut [0; 2]).as_bytes())
    }
}
