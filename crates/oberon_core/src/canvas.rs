use crate::linalg::{Point2, Rectangle, Shape, Vec2};
use crate::terminal::{Cell, Terminal};

pub struct Canvas<'a>
{
    terminal: &'a mut Terminal,
    working_area: Rectangle,
}

impl<'a> Canvas<'a>
{
    pub fn new(terminal: &'a mut Terminal, working_area: Rectangle) -> Self
    {
        Self {
            terminal,
            working_area,
        }
    }

    pub fn area(&self) -> usize
    {
        self.working_area.area()
    }

    pub fn size(&self) -> Vec2
    {
        self.working_area.size()
    }

    pub fn draw(&mut self, pos: Point2, cell: Cell)
    {
        self.terminal.at(pos).change_cell(cell);
    }

    pub fn draw_shape(&mut self, shape: impl Shape, cell: Cell)
    {
        for pos in shape.get_points()
        {
            self.draw(pos, cell);
        }
    }

    pub fn erase(&mut self)
    {
        self.terminal.fill(Cell::EMPTY);
    }

    pub fn fill(&mut self, cell: Cell)
    {
        self.terminal.fill(cell);
    }
}
