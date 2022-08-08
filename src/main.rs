mod config;
mod hwmon_files;
mod interpolation;
mod signals;

use crate::hwmon_files::Files;
use clap::Parser;
use config::{ArgCommands, Config};
use hwmon_files::{change_fan_control_mode, Mode};
use interpolation::TempToPwm;
use std::{
    fmt::Write,
    fs::read_to_string,
    os::raw::c_int,
    path::Path,
    sync::atomic::{self, AtomicBool},
    thread::sleep,
    time::Duration,
};

const SLEEP_DURATION: Duration = Duration::from_secs(5);

static RUNNING: AtomicBool = AtomicBool::new(true);

///
/// Signal handler function
///
extern "C" fn signal_handler(_: c_int) {
    RUNNING.store(false, atomic::Ordering::Relaxed);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Config::parse().command {
        ArgCommands::Find => Files::print_valid_devices()?,
        ArgCommands::Run {
            config_path,
            hwmon_path,
            sensor_type,
        } => {
            // Set listener for shutdown signals
            signals::listen(signal_handler)?;

            let temp_to_pwm = TempToPwm::from_config_lines(
                &read_to_string(config_path)?.lines().collect::<Vec<_>>(),
            )?;

            let path = Path::new(&hwmon_path);

            // Get necessary files
            let mut files = Files::new(path, &sensor_type)?;

            // Avoid allocations by reusing buffer
            let mut buf = String::with_capacity(8);

            change_fan_control_mode(path, Mode::Manual)?;

            while RUNNING.load(atomic::Ordering::Relaxed) {
                if let Err(e) = files.read_temp(&mut buf) {
                    eprintln!("{e}");

                    break;
                }

                match buf.parse() {
                    Ok(temp) => {
                        let pwm = temp_to_pwm.interpolate(temp);

                        println!("Temperature: {temp}, PWM: {pwm}");

                        buf.clear();

                        if let Err(e) = writeln!(buf, "{pwm}") {
                            eprintln!("{e}");

                            break;
                        } else if let Err(e) = files.set_pwm(&mut buf) {
                            eprintln!("{e}");

                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("{e}");

                        break;
                    }
                }

                buf.clear();

                sleep(SLEEP_DURATION);
            }

            change_fan_control_mode(path, Mode::Automatic).expect(
                "WARNING: Failed to disable manual gpu control:\n
                Try writing '2\\n' in '/sys/class/hwmon/hwmon(index)/pwm1_enable'",
            );
        }
    }
    Ok(())
}
