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
    cache: Vec<Block>,
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
            blocks: blocks.clone(),
            cache: blocks,
        }
    }

    pub fn at(&mut self, position: Point2) -> &mut Block
    {
        let index = block_position_to_buffer_index(position, self.working_area.width());
        &mut self.blocks[index]
    }

    pub fn canvas(&mut self) -> Canvas<'_>
    {
        Canvas::new(self, self.working_area)
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
        // Iterate only over the blocks that were changed in the last
        // frame.
        for (index, (block, cache)) in self
            .blocks
            .iter()
            .zip(self.cache.iter_mut())
            .enumerate()
            .filter(|(_, (block, cache))| *block != *cache)
        {
            let width = self.working_area.width();
            let position = block_index_to_screen_position(index, self.cursor_ratio, width);

            // Render the changed blocks only.
            block.render_cells(position, renderer)?;

            // Cache the new changes for next frame.
            *cache = *block;
        }
        renderer.move_cursor(Point2::ZERO)?;
        renderer.flush()
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
