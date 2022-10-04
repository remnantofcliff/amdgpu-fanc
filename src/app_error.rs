use std::num::ParseIntError;

use crate::interpolation::TempToPwmError;

#[derive(Debug)]
pub enum AppError {
    Fmt(std::fmt::Error),
    Interpolation(TempToPwmError),
    Io(std::io::Error),
    Parse(ParseIntError),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fmt(e) => write!(f, "{}", e),
            Self::Interpolation(e) => write!(f, "{}", e),
            Self::Io(e) => write!(f, "{}", e),
            Self::Parse(e) => write!(f, "{}", e),
        }
    }
}

impl From<std::fmt::Error> for AppError {
    fn from(e: std::fmt::Error) -> Self {
        Self::Fmt(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<TempToPwmError> for AppError {
    fn from(e: TempToPwmError) -> Self {
        Self::Interpolation(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        Self::Parse(e)
    }
}
