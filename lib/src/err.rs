use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub enum SudokuError {
    Contradiction(&'static str),
    ParseError(&'static str),
}

impl From<ParseIntError> for SudokuError {
    fn from(_e: ParseIntError) -> Self {
        Self::ParseError("Invalid digit")
    }
}

impl std::error::Error for SudokuError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Contradiction(_) => None,
            Self::ParseError(_) => None,
        }
    }
}

impl fmt::Display for SudokuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SudokuError::Contradiction(msg) => write!(f, "{}", msg),
            SudokuError::ParseError(msg) => write!(f, "{}", msg),
        }
    }
}

pub type SudokuResult<T> = Result<T, SudokuError>;
