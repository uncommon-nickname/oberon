use std::io::{Result as IoResult, Write};

use crate::linalg::{Point2, Vec2};
use crate::renderer::Renderer;
use crate::style::Color;
use crate::terminal::Cell;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Block
{
    cell: Cell,
    cursor_ratio: usize,
    dirty: bool,
}

impl Block
{
    pub const fn new(cell: Cell, cursor_ratio: usize) -> Self
    {
        Self {
            cell,
            cursor_ratio,
            dirty: true,
        }
    }

    pub fn change_cell(&mut self, new_cell: Cell)
    {
        // Changed block becomes dirty, should not be cached.
        self.dirty = true;
        self.cell = new_cell;
    }

    pub fn is_dirty(&self) -> bool
    {
        self.dirty
    }

    pub fn render_cells<W: Write>(
        &mut self, mut position: Point2, renderer: &mut Renderer<W>,
    ) -> IoResult<()>
    {
        // Rendered block is no longer dirty, can be cached.
        self.dirty = false;

        for _ in 0..self.cursor_ratio
        {
            renderer.move_cursor(position)?;

            match &self.cell.bg
            {
                Color::Rgb(rgb) => renderer.change_bg(rgb)?,
                Color::Hsl(hsl) => renderer.change_bg(&hsl.to_rgb())?,
                Color::Default => renderer.reset_bg()?,
            };
            match &self.cell.fg
            {
                Color::Rgb(rgb) => renderer.change_fg(rgb)?,
                Color::Hsl(hsl) => renderer.change_fg(&hsl.to_rgb())?,
                Color::Default => renderer.reset_fg()?,
            };
            renderer.write(self.cell.char)?;
            position += Vec2::RIGHT;
        }
        Ok(())
    }
}
