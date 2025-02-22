use super::cell::GridCell;

const SUBGRID_SIZE: usize = 3;
const GRID_SIZE: usize = 3 * SUBGRID_SIZE;
const NUM_SEPERATORS: usize = GRID_SIZE / SUBGRID_SIZE - 1;

#[derive(Debug)]
pub struct Grid(Vec<Vec<GridCell>>);

mod display;
mod logic;
mod parse;
mod utils;
