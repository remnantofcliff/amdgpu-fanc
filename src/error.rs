use crate::{fan_control, interpolation};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Other(&'static str),
    #[error("No hwmon directory found: {0}")]
    NoHwmonDir(std::io::Error),
    #[error("No amdgpu hwmon path found")]
    NoAmdGpuHwmon,
    #[error("{0}")]
    FanControl(#[from] fan_control::Error),
    #[error("{0}")]
    Interpolation(#[from] interpolation::Error),
}
