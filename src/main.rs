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

    let mut string_buffer_mut = String::with_capacity(8);
    let mut pwm_container = vec![255; files.len()];

    loop {
        if signals::should_close() {
            break Ok(());
        }

        files.read_temps_to_pwms(&mut string_buffer_mut, &mut pwm_container, temp_to_pwm)?;
        files.write_pwms(&mut string_buffer_mut, &pwm_container)?;

        std::thread::sleep(SLEEP_DURATION);
    }
}
