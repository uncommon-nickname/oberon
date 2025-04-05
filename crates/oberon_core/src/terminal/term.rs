use std::io::{Result as IoResult, Write};

use crate::canvas::Canvas;
use crate::linalg::point2::Point2;
use crate::linalg::vec2::Vec2;
use crate::renderer::Renderer;
use crate::terminal::block::Block;
use crate::terminal::cell::Cell;

#[derive(Debug)]
pub struct Terminal
{
    size: Vec2,
    blocks: Vec<Block>,
    cursor_ratio: usize,
}

impl Terminal
{
    pub fn new(size: Vec2, cursor_ratio: usize) -> Self
    {
        let blocks = vec![Block::new(Cell::EMPTY, cursor_ratio); size.x * size.y];
        Self {
            size,
            blocks,
            cursor_ratio,
        }
    }

    pub fn at(&mut self, position: Point2) -> &mut Block
    {
        let index = self.block_position_to_buffer_index(position);
        &mut self.blocks[index]
    }

    pub fn canvas(&mut self) -> Canvas<'_>
    {
        Canvas::new(self, self.size)
    }

    pub fn get_blocks_mut(&mut self) -> &mut Vec<Block>
    {
        &mut self.blocks
    }

    pub fn render_frame<W: Write>(&self, renderer: &mut Renderer<W>) -> IoResult<()>
    {
        for (index, block) in self.blocks.iter().enumerate()
        {
            let starting_position = self.block_index_to_screen_position(index);
            block.render_cells(starting_position, renderer)?;
        }
        renderer.move_cursor(Point2::ZERO)?;
        renderer.flush()
    }

    fn block_position_to_buffer_index(&self, position: Point2) -> usize
    {
        position.x + position.y * self.size.x
    }

    fn block_index_to_screen_position(&self, index: usize) -> Point2
    {
        Point2::new(
            (index * self.cursor_ratio) % (self.size.x * self.cursor_ratio),
            index / self.size.x,
        )
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_calculate_screen_position()
    {
        let terminal = Terminal::new(Vec2::new(10, 10), 2);

        let pos = terminal.block_index_to_screen_position(0);

        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);

        let pos = terminal.block_index_to_screen_position(9);

        assert_eq!(pos.x, 18);
        assert_eq!(pos.y, 0);

        let pos = terminal.block_index_to_screen_position(11);

        assert_eq!(pos.x, 2);
        assert_eq!(pos.y, 1);
    }
}
