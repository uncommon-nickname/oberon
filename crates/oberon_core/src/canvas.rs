use crate::linalg::Vec2;
use crate::terminal::{Cell, Terminal};

pub struct Canvas<'a>
{
    terminal: &'a mut Terminal,
    size: Vec2,
}

impl<'a> Canvas<'a>
{
    pub fn new(terminal: &'a mut Terminal, size: Vec2) -> Self
    {
        Self { terminal, size }
    }

    pub fn area(&self) -> usize
    {
        self.size.scalar_product()
    }

    pub fn draw(&mut self, pos: Vec2, cell: Cell)
    {
        *self.terminal.at(pos) = cell;
    }

    pub fn erase(&mut self)
    {
        self.fill(Cell::EMPTY);
    }

    pub fn fill(&mut self, cell: Cell)
    {
        self.terminal.get_cells_mut().fill(cell);
    }

    pub fn size(&self) -> Vec2
    {
        self.size
    }
}
