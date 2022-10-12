mod app_error;
mod config;
mod interpolation;
mod mode;
mod pwm_writer;
mod signals;
mod temperature_reader;

use app_error::AppError;
use clap::Parser;
use config::{ArgCommands, Config, SensorType};
use interpolation::TempToPwm;
use mode::ManualFanControl;
use pwm_writer::PwmWriter;
use std::{
    fs::{read_dir, read_to_string},
    path::Path,
    thread::sleep,
    time::Duration,
};
use temperature_reader::TemperatureReader;

const SLEEP_DURATION: Duration = Duration::from_secs(5);

fn main() -> Result<(), AppError> {
    match Config::parse().command {
        ArgCommands::Find => find(),
        ArgCommands::Run {
            config_path,
            hwmon_path,
            sensor_type,
        } => start_run(&config_path, Path::new(&hwmon_path), &sensor_type),
    }
}

fn find() -> Result<(), AppError> {
    read_dir("/sys/class/hwmon/")?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            std::fs::read_to_string(path.join("name"))
                .ok()
                .filter(|s| s.as_str() == "amdgpu\n")
                .is_some()
        })
        .for_each(|path| {
            if let Some(path) = path.to_str() {
                println!("Valid gpu path found: {path}")
            }
        });

    Ok(())
}

fn start_run(
    config_path: &str,
    hwmon_path: &Path,
    sensor_type: &SensorType,
) -> Result<(), AppError> {
    signals::listen();

    let temp_to_pwm =
        TempToPwm::from_config_lines(&read_to_string(config_path)?.lines().collect::<Vec<_>>())?;
    let mut temp_reader = TemperatureReader::new(hwmon_path, sensor_type)?;
    let mut pwm_writer = PwmWriter::new(hwmon_path)?;
    let _disabler = ManualFanControl::enable(hwmon_path)?;

    loop {
        let pwm = temp_to_pwm.interpolate(temp_reader.read()?);

        pwm_writer.set_pwm(pwm)?;

        sleep(SLEEP_DURATION);
    }
}
