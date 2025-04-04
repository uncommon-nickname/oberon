use crate::style::rgb::Rgb;

#[derive(Clone, Debug)]
pub struct Grayscale
{
    value: u8,
}

impl Grayscale
{
    pub const fn new(value: u8) -> Self
    {
        Self { value }
    }

    pub const fn from_rgb(color: Rgb) -> Self
    {
        let value = (0.299 * color.red() as f32
            + 0.587 * color.green() as f32
            + 0.114 * color.blue() as f32) as u8;
        Self { value }
    }

    pub const fn into_char(&self) -> char
    {
        const ASCII_SCALE: [char; 7] = [' ', '.', '-', '+', '*', '#', '@'];

        let index = (self.value as f32 / 255.0 * 7.0) as usize;

        ASCII_SCALE[index]
    }
}
