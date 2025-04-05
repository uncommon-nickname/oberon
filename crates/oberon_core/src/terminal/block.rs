use std::io::{Result as IoResult, Write};

use crate::linalg::{Point2, Vec2};
use crate::renderer::Renderer;
use crate::style::Color;
use crate::terminal::Cell;

#[derive(Clone, Debug)]
pub struct Block
{
    cell: Cell,
    cursor_ratio: usize,
}

impl Block
{
    pub const fn new(cell: Cell, cursor_ratio: usize) -> Self
    {
        Self { cell, cursor_ratio }
    }

    pub fn change_cell(&mut self, new_cell: Cell)
    {
        self.cell = new_cell;
    }

    pub fn render_cells<W: Write>(
        &self, mut position: Point2, renderer: &mut Renderer<W>,
    ) -> IoResult<()>
    {
        for _ in 0..self.cursor_ratio
        {
            renderer.move_cursor(position)?;

            match &self.cell.bg
            {
                Color::Rgb(rgb) => renderer.change_bg(rgb)?,
                Color::Default => renderer.reset_bg()?,
            };
            match &self.cell.fg
            {
                Color::Rgb(rgb) => renderer.change_fg(rgb)?,
                Color::Default => renderer.reset_fg()?,
            };
            renderer.write(self.cell.char)?;
            position += Vec2::RIGHT;
        }
        Ok(())
    }
}
