use std::mem;

use super::{Grid, GridCell, GRID_SIZE};
use crate::err::SudokuResult;

impl Grid {
    // Constraint propagation
    pub fn eliminate_neighbors(&mut self, row: usize, col: usize) -> SudokuResult<()> {
        if let GridCell::Digit(d) = self.0[row][col] {
            for (nrow, ncol) in self.neighbors(row, col) {
                // Propagate new constraint if options has been turned into a
                // certain digit
                if self.0[nrow][ncol].eliminate(d)? {
                    self.eliminate_neighbors(nrow, ncol)?
                }
            }
        }

        Ok(())
    }

    pub fn backtrack(&mut self) -> bool {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if let GridCell::Options(opts) = &self.0[row][col] {
                    let possible_opts: Vec<usize> = opts
                        .iter()
                        .copied()
                        .filter(|target| {
                            self.neighbors(row, col)
                                .into_iter()
                                .all(|(nr, nc)| match &self.0[nr][nc] {
                                    GridCell::Digit(d) => d != target,
                                    _ => true,
                                })
                        })
                        .collect();

                    for i in possible_opts {
                        let old = mem::replace(&mut self.0[row][col], GridCell::Digit(i));
                        if self.backtrack() {
                            return true; // Propagate backtrack solution
                        }
                        self.0[row][col] = old;
                    }

                    return false; // All possible options tried, no solution found
                }
            }
        }

        true // Grid filled with valid solution
    }

    pub fn solve(&mut self) -> SudokuResult<()> {
        // 1. Constraint propagation
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                self.eliminate_neighbors(row, col)?;
            }
        }

        // 2. Backtracking
        self.backtrack();

        Ok(())
    }
}
