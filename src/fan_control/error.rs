use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to open file {0}")]
    OpenFile(io::Error),
    #[error("Failed to enable manual control {0}")]
    ManualControl(io::Error),
    #[error("Failed to read temperature: {0}")]
    ReadTemperature(io::Error),
    #[error("Failed to write pwm: {0}")]
    WritePwm(io::Error),
}
