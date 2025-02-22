use std::fmt;

use super::{Grid, GRID_SIZE, NUM_SEPERATORS, SUBGRID_SIZE};

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row_seperator = String::from("\n") + &"-".repeat(3 * (GRID_SIZE + NUM_SEPERATORS) - 2);
        let col_seperator = "  |";

        let grid = self
            .0
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, col)| {
                        col.to_string()
                            + (if (c + 1) % SUBGRID_SIZE == 0 && (c + 1) < GRID_SIZE {
                                col_seperator
                            } else {
                                ""
                            })
                    })
                    .collect::<Vec<_>>()
                    .join("  ")
                    + (if (r + 1) % SUBGRID_SIZE == 0 && (r + 1) < GRID_SIZE {
                        &row_seperator
                    } else {
                        ""
                    })
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", grid)
    }
}
