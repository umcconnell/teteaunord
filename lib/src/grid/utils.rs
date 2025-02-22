use super::{Grid, GRID_SIZE, SUBGRID_SIZE};

impl Grid {
    pub fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::with_capacity(3 * GRID_SIZE - 2);

        // Division w/ usize truncates result (== floor(a/b))
        let subgrid_row = (row / SUBGRID_SIZE) * SUBGRID_SIZE;
        let subgrid_col = (col / SUBGRID_SIZE) * SUBGRID_SIZE;

        for r in subgrid_row..subgrid_row + SUBGRID_SIZE {
            for c in subgrid_col..subgrid_col + SUBGRID_SIZE {
                if r != row && c != col {
                    neighbors.push((r, c))
                }
            }
        }

        for c in 0..GRID_SIZE {
            if c != col {
                neighbors.push((row, c));
            }
        }

        for r in 0..GRID_SIZE {
            if r != row {
                neighbors.push((r, col));
            }
        }

        neighbors
    }
}
