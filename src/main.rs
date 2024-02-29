#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]

mod config;
mod error;
mod fan_control;
mod interpolation;
mod signals;

use clap::Parser;
use config::{ArgCommands, Args, SensorType};
use error::Error;
use fan_control::FanControl;
use interpolation::Interpolator;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
    thread::sleep,
    time::Duration,
};

///
/// Macro that gets replaced by the path to top hwmon directory.
///
macro_rules! hwmon_top_dir {
    () => {
        "/sys/class/hwmon"
    };
}

///
/// Duration to sleep between updates.
///
const SLEEP_DURATION: Duration = Duration::from_secs(5);
///
/// Hwmon identifier file name.
///
const HWMON_NAME_FILE_NAME: &str = "name";
///
/// Amdgpu name in `HWMON_NAME_FILE_NAME`.
///
const AMDGPU_HWMON_NAME: &str = "amdgpu\n";

fn main() -> Result<(), Error> {
    match Args::parse().command {
        ArgCommands::Find => find()?,
        ArgCommands::Run {
            config_path,
            hwmon_path,
            sensor_type,
        } => {
            let hwmon_path = if let Some(path) = hwmon_path {
                PathBuf::from(path)
            } else {
                get_amdgpu_hwmon_folders()?
                    .next()
                    .ok_or(Error::NoAmdGpuHwmon)?
            };

            println!("Using amdgpu hwmon path {hwmon_path:?}");

            run(Path::new(&config_path), &hwmon_path, sensor_type)?;
        }
    }

    Ok(())
}

///
/// Find and print amdgpu hwmon paths.
///
fn find() -> Result<(), Error> {
    let hwmon_folders = get_amdgpu_hwmon_folders()?;

    for path in hwmon_folders {
        let path = path.to_string_lossy();
        println!("Valid gpu path found: {path}");
    }

    Ok(())
}

///
/// Run fan control
///
fn run(config_path: &Path, hwmon_path: &Path, sensor_type: SensorType) -> Result<(), Error> {
    signals::listen().map_err(|()| Error::Other("Failed to listen to signals"))?;

    let temp_to_pwm = Interpolator::from_config(config_path)?;
    let mut fan_control = FanControl::enable(hwmon_path, sensor_type)?;

    loop {
        let pwm = temp_to_pwm.interpolate(fan_control.read_temperature()?);

        fan_control.set_pwm(pwm)?;

        sleep(SLEEP_DURATION);
    }
}

fn get_amdgpu_hwmon_folders() -> Result<impl Iterator<Item = PathBuf>, Error> {
    let hwmon_dir_contents = read_dir(hwmon_top_dir!()).map_err(Error::NoHwmonDir)?;

    Ok(hwmon_dir_contents
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            std::fs::read_to_string(path.join(HWMON_NAME_FILE_NAME))
                .ok()
                .filter(|s| s.as_str() == AMDGPU_HWMON_NAME)
                .is_some()
        }))
}
