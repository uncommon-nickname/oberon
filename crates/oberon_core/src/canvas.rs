use crate::linalg::shapes::{LazyShape, Shape};
use crate::linalg::{Point2, Vec2};
use crate::terminal::{Cell, Terminal};

pub struct Canvas<'a>
{
    terminal: &'a mut Terminal,
}

impl<'a> Canvas<'a>
{
    pub fn new(terminal: &'a mut Terminal) -> Self
    {
        Self { terminal }
    }

    pub fn area(&self) -> f64
    {
        self.terminal.area()
    }

    pub fn size(&self) -> Vec2
    {
        self.terminal.size()
    }

    pub fn draw(&mut self, pos: Point2, cell: Cell)
    {
        self.terminal.at(pos).change_cell(cell);
    }

    pub fn draw_shape_outline<S: Shape<S> + LazyShape>(&mut self, shape: &S, cell: Cell)
    {
        shape
            .points_outline()
            .for_each(|point| self.draw(point, cell));
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
