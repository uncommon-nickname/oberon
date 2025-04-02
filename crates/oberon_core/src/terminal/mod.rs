pub mod cell;
pub mod term;

use crate::color::Rgb;
use crate::linalg::Vec2;

pub fn cell_index_to_cell_position(index: usize, width: usize) -> Vec2
{
    Vec2::new(index % width, index / width)
}

fn cell_position_to_cell_index(pos: Vec2, width: usize) -> usize
{
    pos.x + pos.y * width
}

#[derive(Clone, Debug)]
pub struct Cell
{
    pub char: char,
    pub bg: Rgb,
    pub fg: Rgb,
}

#[derive(Debug)]
pub struct Terminal
{
    size: Vec2,
    cells: Vec<Cell>,
}
