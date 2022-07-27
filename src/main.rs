use crate::hwmon_files::HwmonFiles;
use interpolation::TempToPwm;
use std::time::Duration;

mod hwmon_files;
mod interpolation;
mod signals;

const SLEEP_DURATION: Duration = Duration::from_secs(5);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // May panic during here, otherwise the program should never realistically
    // panic.
    let temp_to_pwm = &TempToPwm::from_args();
    let mut files = HwmonFiles::new()?;

    signals::listen()?;

    // Avoid allocations by reusing buffer
    let mut string_buffer = String::with_capacity(8);

    loop {
        if signals::should_close() {
            break Ok(());
        }

        files.update_pwms(&mut string_buffer, temp_to_pwm)?;

        std::thread::sleep(SLEEP_DURATION);
    }
}
