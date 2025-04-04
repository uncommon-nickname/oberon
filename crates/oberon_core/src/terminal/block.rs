use std::io::{Result as IoResult, Write};

use crate::color::Color;
use crate::linalg::Vec2;
use crate::renderer::Renderer;
use crate::terminal::cell::Cell;

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
        &self, starting_position: Vec2, renderer: &mut Renderer<W>,
    ) -> IoResult<()>
    {
        for index in 0..self.cursor_ratio
        {
            let new_position = Vec2::new(starting_position.x + index, starting_position.y);

            renderer.move_cursor(new_position)?;

            match &self.cell.bg
            {
                Color::Rgb(rgb) => renderer.change_bg(rgb)?,
                Color::Restore => renderer.reset_bg()?,
            };
            match &self.cell.fg
            {
                Color::Rgb(rgb) => renderer.change_fg(rgb)?,
                Color::Restore => renderer.reset_fg()?,
            };
            renderer.write(self.cell.char)?;
        }
        Ok(())
    }
}
