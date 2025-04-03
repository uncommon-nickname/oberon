use std::io::{Result as IoResult, Write};

use crate::canvas::Canvas;
use crate::linalg::Vec2;
use crate::renderer::Renderer;
use crate::terminal::cell::Cell;

#[derive(Debug)]
pub struct Terminal
{
    size: Vec2,
    cells: Vec<Cell>,
}

impl Terminal
{
    pub fn new(size: Vec2) -> Self
    {
        let cells = vec![Cell::EMPTY; size.scalar_product()];
        Self { size, cells }
    }

    pub fn at(&mut self, pos: Vec2) -> &mut Cell
    {
        let index = cell_position_to_cell_index(pos, self.size.x);
        &mut self.cells[index]
    }

    pub fn canvas(&mut self) -> Canvas<'_>
    {
        Canvas::new(self, self.size)
    }

    pub fn get_cells(&self) -> &Vec<Cell>
    {
        &self.cells
    }

    pub fn get_cells_mut(&mut self) -> &mut Vec<Cell>
    {
        &mut self.cells
    }

    pub fn render_frame<W: Write>(&self, renderer: &mut Renderer<W>) -> IoResult<()>
    {
        for (index, cell) in self.cells.iter().enumerate()
        {
            let position = cell_index_to_cell_position(index, self.size.x);

            renderer.move_cursor(position)?;
            renderer.change_bg(&cell.bg)?;
            renderer.change_fg(&cell.fg)?;
            renderer.write(cell.char)?;
        }
        renderer.move_cursor(Vec2::ZEROES)?;
        renderer.flush()
    }
}

pub fn cell_index_to_cell_position(index: usize, width: usize) -> Vec2
{
    Vec2::new(index % width, index / width)
}

fn cell_position_to_cell_index(pos: Vec2, width: usize) -> usize
{
    pos.x + pos.y * width
}
