use super::{Grid, GridCell, GRID_SIZE};
use crate::err::SudokuError;
use std::convert::TryFrom;

impl Grid {
    pub fn parse(grid: &str) -> Result<Self, SudokuError> {
        let grid = grid
            .lines()
            .map(|l| l.trim())
            .map(|l| {
                if l.is_empty() || l.starts_with('-') {
                    return Ok(None);
                }

                let line = l
                    .split("  ")
                    .map(|c| match c {
                        "." => Ok(Some(GridCell::empty())),
                        "|" => Ok(None),
                        c if {
                            let digit = c.parse::<usize>()?;
                            digit > 0 && digit <= 9
                        } =>
                        {
                            Ok(Some(GridCell::Digit(c.parse::<usize>().unwrap())))
                        }
                        _ => Err(SudokuError::ParseError("Unknown character")),
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .filter(|el| el.is_some())
                    .map(|el| el.unwrap())
                    .collect::<Vec<_>>();

                if line.len() != GRID_SIZE {
                    Err(SudokuError::ParseError("Invalid grid dimensions"))
                } else {
                    Ok(Some(line))
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|el| el.is_some())
            .map(|el| el.unwrap())
            .collect::<Vec<_>>();

        if grid.len() != GRID_SIZE {
            Err(SudokuError::ParseError("Invalid grid dimensions"))
        } else {
            Ok(Grid(grid))
        }
    }
}

impl TryFrom<&str> for Grid {
    type Error = SudokuError;

    fn try_from(grid: &str) -> Result<Self, Self::Error> {
        Self::parse(grid)
    }
}
