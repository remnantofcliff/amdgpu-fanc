use std::{
    io,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bad line in config: '{0}'")]
    ConfigMissingDelimiter(String),
    #[error("Couldn't open config: {0}")]
    ConfigOpen(#[from] io::Error),
    #[error("Couldn't parse temperature in config: '{0}'")]
    ParseTemperature(#[from] ParseIntError),
    #[error("Couldn't parse fan percent in config: '{0}'")]
    ParseFanPercent(#[from] ParseFloatError),
}
