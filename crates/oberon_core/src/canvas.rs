use crate::linalg::point2::Point2;
use crate::linalg::vec2::Vec2;
use crate::terminal::cell::Cell;
use crate::terminal::term::Terminal;

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
        self.size.x * self.size.y
    }

    pub fn draw(&mut self, pos: Point2, cell: Cell)
    {
        self.terminal.at(pos).change_cell(cell);
    }

    pub fn erase(&mut self)
    {
        self.fill(Cell::EMPTY);
    }

    pub fn fill(&mut self, cell: Cell)
    {
        for block in self.terminal.get_blocks_mut()
        {
            block.change_cell(cell.clone());
        }
    }

    pub fn size(&self) -> Vec2
    {
        self.size
    }
}
