mod app_error;
mod config;
mod hwmon_files;
mod interpolation;
mod signals;

use app_error::AppError;
use clap::Parser;
use config::{ArgCommands, Config, SensorType};
use hwmon_files::{change_fan_control_mode, Files, Mode};
use interpolation::TempToPwm;
use std::{
    fmt::Write,
    fs::read_to_string,
    path::Path,
    sync::atomic::{self, AtomicBool},
    thread::sleep,
    time::Duration,
};

const SLEEP_DURATION: Duration = Duration::from_secs(5);

pub static RUNNING: AtomicBool = AtomicBool::new(true);

fn main() -> Result<(), AppError> {
    match Config::parse().command {
        ArgCommands::Find => start_find(),
        ArgCommands::Run {
            config_path,
            hwmon_path,
            sensor_type,
        } => start_run(&config_path, &hwmon_path, &sensor_type),
    }
}

fn start_find() -> Result<(), AppError> {
    Ok(Files::print_valid_devices()?)
}

fn start_run(
    config_path: &str,
    hwmon_path: &str,
    sensor_type: &SensorType,
) -> Result<(), AppError> {
    signals::listen();

    let hwmon_path = Path::new(hwmon_path);

    let result = run(config_path, hwmon_path, sensor_type);

    change_fan_control_mode(hwmon_path, Mode::Automatic).expect(
        "WARNING: Failed to disable manual gpu control:\n\
         Try writing '2\\n' in '/sys/class/hwmon/hwmon(index)/pwm1_enable'",
    );

    result
}

fn run(config_path: &str, hwmon_path: &Path, sensor_type: &SensorType) -> Result<(), AppError> {
    let temp_to_pwm =
        TempToPwm::from_config_lines(&read_to_string(config_path)?.lines().collect::<Vec<_>>())?;
    let mut files = Files::new(hwmon_path, sensor_type)?;
    let mut buf = String::with_capacity(8);

    change_fan_control_mode(hwmon_path, Mode::Manual)?;

    while RUNNING.load(atomic::Ordering::Relaxed) {
        files.read_temp(&mut buf)?;

        let pwm = temp_to_pwm.interpolate(buf.parse()?);

        buf.clear();

        writeln!(buf, "{pwm}")?;

        files.set_pwm(&mut buf)?;

        buf.clear();

        sleep(SLEEP_DURATION);
    }

    Ok(())
}
