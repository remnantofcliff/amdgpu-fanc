use std::{io, num::ParseIntError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bad line in config: '{0}'")]
    ConfigMissingDelimiter(String),
    #[error("Couldn't open config: {0}")]
    ConfigOpen(#[from] io::Error),
    #[error("Couldn't parse number in config: '{0}'")]
    Parse(#[from] ParseIntError),
}
