mod config;
mod error;
mod fan_control;
mod interpolation;
mod signals;

use clap::Parser;
use config::{ArgCommands, Args, SensorType};
use error::Error;
use fan_control::FanControl;
use interpolation::TempToPwm;
use std::{fs::read_dir, path::Path, thread::sleep, time::Duration};

///
/// Macro that gets replaced by the path to hwmon top folder.
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
/// Directory to look for amdgpu hwmon directory.
///
const HWMON_TOP_DIR_FAIL: &str = concat!("Failed to read ", hwmon_top_dir!(), " directory");
const HWMON_NAME_FILE_NAME: &str = "name";
const AMDGPU_HWMON_NAME: &str = "amdgpu\n";

fn main() -> Result<(), Error> {
    match Args::parse().command {
        ArgCommands::Find => find(),
        ArgCommands::Run {
            config_path,
            hwmon_path,
            sensor_type,
        } => run(
            Path::new(&config_path),
            Path::new(&hwmon_path),
            &sensor_type,
        ),
    }
}

fn find() -> Result<(), Error> {
    read_dir(hwmon_top_dir!())
        .expect(HWMON_TOP_DIR_FAIL)
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            std::fs::read_to_string(path.join(HWMON_NAME_FILE_NAME))
                .ok()
                .filter(|s| s.as_str() == AMDGPU_HWMON_NAME)
                .is_some()
        })
        .for_each(|path| {
            if let Some(path) = path.to_str() {
                println!("Valid gpu path found: {path}")
            }
        });

    Ok(())
}

fn run(config_path: &Path, hwmon_path: &Path, sensor_type: &SensorType) -> Result<(), Error> {
    signals::listen();

    let temp_to_pwm = TempToPwm::from_config(config_path)?;
    let mut fan_control = FanControl::enable(hwmon_path, sensor_type)?;

    loop {
        let pwm = temp_to_pwm.interpolate(fan_control.read_temperature()?);

        fan_control.set_pwm(pwm)?;

        sleep(SLEEP_DURATION);
    }
}
