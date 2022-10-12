use crate::{fan_control, interpolation};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Interpolation(#[from] interpolation::Error),
    #[error("{0}")]
    FanControl(#[from] fan_control::Error),
}
