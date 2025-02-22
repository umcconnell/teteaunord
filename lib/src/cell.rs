use std::fmt;

use crate::err::{SudokuError, SudokuResult};

#[derive(Debug)]
pub enum GridCell {
    Digit(usize),
    Options(Vec<usize>),
}

impl GridCell {
    pub fn empty() -> Self {
        Self::Options((1..=9).collect::<Vec<usize>>())
    }

    pub fn eliminate(&mut self, target: usize) -> SudokuResult<bool> {
        match self {
            Self::Options(opts) => {
                opts.retain(|d| *d != target);

                if opts.len() == 1 {
                    *self = Self::Digit(*opts.first().unwrap());
                    return Ok(true);
                }
            }
            Self::Digit(d) if *d == target => {
                return Err(SudokuError::Contradiction(
                    "Invalid sudoku grid! Contradiction detected.",
                ))
            }
            _ => (),
        };

        Ok(false)
    }
}

impl fmt::Display for GridCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Digit(d) => d.to_string(),
                Self::Options(_) => ".".into(),
            }
        )
    }
}
