use std::io::{Result as IoResult, Write};

use crate::canvas::Canvas;
use crate::linalg::{Point2, Rectangle, Shape, Vec2};
use crate::renderer::Renderer;
use crate::terminal::block::Block;
use crate::terminal::cell::Cell;

#[derive(Debug)]
pub struct Terminal
{
    working_area: Rectangle,
    cursor_ratio: usize,
    blocks: Vec<Block>,
}

impl Terminal
{
    pub fn new(size: Vec2, cursor_ratio: usize) -> Self
    {
        let working_area = Rectangle::from_size(Point2::ZERO, size);
        let blocks = vec![Block::new(Cell::EMPTY, cursor_ratio); working_area.area()];

        Self {
            working_area,
            cursor_ratio,
            blocks,
        }
    }

    pub fn at(&mut self, position: Point2) -> &mut Block
    {
        let index = block_position_to_buffer_index(position, self.working_area.width());
        &mut self.blocks[index]
    }

    pub fn area(&self) -> usize
    {
        self.working_area.area()
    }

    pub fn canvas(&mut self) -> Canvas<'_>
    {
        Canvas::new(self)
    }

    pub fn fill(&mut self, cell: Cell)
    {
        for block in self.blocks.iter_mut()
        {
            block.change_cell(cell);
        }
    }

    pub fn render_frame<W: Write>(&mut self, renderer: &mut Renderer<W>) -> IoResult<()>
    {
        let width = self.working_area.width();

        for (index, block) in self
            .blocks
            .iter_mut()
            .enumerate()
            .filter(|(_, block)| block.is_dirty())
        {
            let position = block_index_to_screen_position(index, self.cursor_ratio, width);
            block.render_cells(position, renderer)?;
        }
        renderer.move_cursor(Point2::ZERO)?;
        renderer.flush()
    }

    pub fn size(&self) -> Vec2
    {
        self.working_area.size()
    }
}

#[inline]
fn block_position_to_buffer_index(position: Point2, width: usize) -> usize
{
    position.x + position.y * width
}

#[inline]
fn block_index_to_screen_position(index: usize, cursor_ratio: usize, width: usize) -> Point2
{
    Point2::new(
        (index * cursor_ratio) % (width * cursor_ratio),
        index / width,
    )
}
